use std::collections::HashMap;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use digital_twin_bridge::{BidirectionalTwinBridge, SyncConfig, TwinObservation, TwinSide};
use once_cell::sync::Lazy;
use serde_json::{json, Value};
use spans_core::UniversalSpan;
use tokio::sync::Mutex;
use tracing::{info, warn};

static TWIN_COORDINATOR: Lazy<Mutex<TwinCoordinator>> =
    Lazy::new(|| Mutex::new(TwinCoordinator::new()));

#[derive(Debug)]
struct ParsedObservation {
    cycle_id: String,
    observation: TwinObservation,
    source_span: UniversalSpan,
}

#[derive(Default, Debug)]
struct PendingObservation {
    physical: Option<(TwinObservation, UniversalSpan)>,
    digital: Option<(TwinObservation, UniversalSpan)>,
}

#[derive(Debug)]
struct TwinCoordinator {
    bridge: BidirectionalTwinBridge,
    pending: HashMap<String, PendingObservation>,
}

impl TwinCoordinator {
    fn new() -> Self {
        Self {
            bridge: BidirectionalTwinBridge::with_config(SyncConfig {
                default_metric_tolerance: 0.10,
                ..SyncConfig::default()
            }),
            pending: HashMap::new(),
        }
    }

    fn ingest(&mut self, parsed: ParsedObservation) -> Result<Vec<UniversalSpan>> {
        let ParsedObservation {
            cycle_id,
            observation,
            source_span,
        } = parsed;

        let entry = self
            .pending
            .entry(cycle_id.clone())
            .or_insert_with(PendingObservation::default);

        match observation.side {
            TwinSide::Physical => {
                if entry.physical.is_some() {
                    warn!(cycle = %cycle_id, "overwriting_existing_physical_observation");
                }
                entry.physical = Some((observation, source_span));
            }
            TwinSide::Digital => {
                if entry.digital.is_some() {
                    warn!(cycle = %cycle_id, "overwriting_existing_digital_observation");
                }
                entry.digital = Some((observation, source_span));
            }
        }

        if entry.physical.is_none() || entry.digital.is_none() {
            return Ok(Vec::new());
        }

        let (physical_obs, _physical_span) = entry.physical.as_ref().expect("physical set");
        let (digital_obs, digital_span) = entry.digital.as_ref().expect("digital set");

        let execution_hint = digital_span
            .payload
            .get("metadata")
            .and_then(|meta| meta.get("execution_span"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or_else(|| {
                digital_span
                    .payload
                    .get("execution_span")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .or_else(|| {
                digital_span
                    .causal
                    .parent_id
                    .as_ref()
                    .map(|id| id.0.clone())
            });

        let comparison = self
            .bridge
            .analyze_cycle(physical_obs.clone(), digital_obs.clone());
        self.bridge.record_comparison(&comparison);

        let mut generated = Vec::new();
        if comparison.has_divergences() {
            let aligned_metrics = comparison.aligned_metrics.clone();
            for divergence in &comparison.divergences {
                let mut span = self
                    .bridge
                    .emit_divergence_span(digital_span, divergence)
                    .context("emit divergence span")?;

                span.flow = "twin_divergence".to_string();
                span.workflow = digital_span.workflow.clone();
                span.name = format!("Twin divergence {} ({})", divergence.metric, cycle_id);

                if let Some(obj) = span.payload.as_object_mut() {
                    obj.insert("cycle_id".into(), Value::String(cycle_id.clone()));
                    obj.insert(
                        "aligned_metrics".into(),
                        Value::Array(aligned_metrics.iter().cloned().map(Value::String).collect()),
                    );
                    let exec_clone = execution_hint.clone();
                    if let Some(exec) = exec_clone.clone() {
                        obj.entry("execution_span").or_insert(Value::String(exec));
                    }
                    obj.insert(
                        "metadata".into(),
                        json!({
                            "cycle_id": cycle_id,
                            "detected_at": divergence.detected_at.to_rfc3339(),
                            "divergence_metric": divergence.metric,
                            "severity": format!("{:?}", divergence.severity),
                            "physical_span": divergence.physical_span.0,
                            "digital_span": divergence.digital_span.0,
                            "divergence_count": comparison.divergences.len(),
                            "execution_span": exec_clone,
                        }),
                    );
                }

                generated.push(span);
            }

            info!(
                cycle = %cycle_id,
                divergences = generated.len(),
                "twin_divergence_detected"
            );
        } else {
            info!(cycle = %cycle_id, "twin_cycle_aligned");
        }

        self.pending.remove(&cycle_id);
        Ok(generated)
    }
}

pub async fn handle_twin_observation(span: &UniversalSpan) -> Result<Vec<UniversalSpan>> {
    if span.flow != "twin_observation" {
        return Ok(Vec::new());
    }

    let Some(parsed) = parse_observation(span) else {
        return Ok(Vec::new());
    };

    let mut guard = TWIN_COORDINATOR.lock().await;
    guard.ingest(parsed)
}

fn parse_observation(span: &UniversalSpan) -> Option<ParsedObservation> {
    let twin_block = span
        .payload
        .get("twin")
        .or_else(|| span.payload.get("metadata").and_then(|m| m.get("twin")))?;

    let cycle_id = twin_block.get("cycle_id")?.as_str()?.to_string();
    let side = twin_block
        .get("side")
        .and_then(|v| v.as_str())
        .and_then(parse_side)?;

    let recorded_at = twin_block
        .get("recorded_at")
        .and_then(|v| v.as_str())
        .and_then(parse_timestamp)
        .or(span.finished_at)
        .unwrap_or(span.started_at);

    let metrics_value = twin_block
        .get("metrics")
        .or_else(|| span.payload.get("metrics"))?;
    let metrics = parse_metrics(metrics_value)?;

    if metrics.is_empty() {
        warn!(span = %span.id.0, "twin_metrics_missing");
        return None;
    }

    let observation = TwinObservation {
        span_id: span.id.clone(),
        side,
        recorded_at,
        metrics,
    };

    Some(ParsedObservation {
        cycle_id,
        observation,
        source_span: span.clone(),
    })
}

fn parse_metrics(value: &Value) -> Option<HashMap<String, f64>> {
    let map = value.as_object()?;
    let mut metrics = HashMap::new();
    for (key, val) in map.iter() {
        if let Some(num) = val.as_f64() {
            metrics.insert(key.clone(), num);
        } else if let Some(s) = val.as_str() {
            if let Ok(parsed) = s.parse::<f64>() {
                metrics.insert(key.clone(), parsed);
            }
        }
    }
    Some(metrics)
}

fn parse_side(raw: &str) -> Option<TwinSide> {
    match raw.to_lowercase().as_str() {
        "physical" | "real" | "ground_truth" => Some(TwinSide::Physical),
        "digital" | "simulated" | "twin" => Some(TwinSide::Digital),
        _ => {
            warn!(side = raw, "unknown_twin_side");
            None
        }
    }
}

fn parse_timestamp(raw: &str) -> Option<DateTime<Utc>> {
    raw.parse::<DateTime<Utc>>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use spans_core::UniversalSpan;

    fn make_span(id: &str, side: &str, metrics: Value) -> UniversalSpan {
        let payload = json!({
            "twin": {
                "cycle_id": "cycle::001",
                "side": side,
                "recorded_at": "2025-09-29T12:00:00Z",
                "metrics": metrics,
            }
        });
        UniversalSpan::new(
            id,
            "twin observation",
            "twin_observation",
            "twin_workflow",
            Utc::now(),
            payload,
        )
    }

    #[tokio::test]
    async fn matches_physical_and_digital_observations() {
        let physical = make_span("span::phys", "physical", json!({ "temperature": 300.0 }));
        let digital = make_span("span::dig", "digital", json!({ "temperature": 360.0 }));

        handle_twin_observation(&physical).await.unwrap();
        let divergences = handle_twin_observation(&digital).await.unwrap();

        assert_eq!(divergences.len(), 1);
        assert_eq!(divergences[0].flow, "twin_divergence");
    }

    #[tokio::test]
    async fn ignores_spans_without_twin_payload() {
        let mut span = make_span("span::other", "physical", json!({ "temperature": 300.0 }));
        span.flow = "metric".into();
        let divergences = handle_twin_observation(&span).await.unwrap();
        assert!(divergences.is_empty());
    }
}
