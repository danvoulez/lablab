use std::collections::{HashMap, HashSet};

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use spans_core::{SpanId, UniversalSpan};
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub max_divergences: usize,
    pub divergence_threshold: f64,
    pub auto_reconcile: bool,
    pub default_metric_tolerance: f64,
    #[serde(default)]
    pub metric_tolerance: HashMap<String, f64>,
}

impl SyncConfig {
    pub fn tolerance_for(&self, metric: &str) -> f64 {
        self.metric_tolerance
            .get(metric)
            .copied()
            .unwrap_or(self.default_metric_tolerance)
    }

    pub fn with_metric_tolerance(mut self, metric: impl Into<String>, tolerance: f64) -> Self {
        self.metric_tolerance.insert(metric.into(), tolerance);
        self
    }
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            max_divergences: 3,
            divergence_threshold: 0.25,
            auto_reconcile: false,
            default_metric_tolerance: 0.10,
            metric_tolerance: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinSyncCycle {
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub divergences: Vec<SpanId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TwinSide {
    Physical,
    Digital,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinObservation {
    pub span_id: SpanId,
    pub side: TwinSide,
    pub recorded_at: DateTime<Utc>,
    pub metrics: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DivergenceSeverity {
    Minor,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinDivergence {
    pub span_id: SpanId,
    pub metric: String,
    pub physical_value: Option<f64>,
    pub digital_value: Option<f64>,
    pub absolute_delta: f64,
    pub percent_delta: f64,
    pub severity: DivergenceSeverity,
    pub detected_at: DateTime<Utc>,
    pub physical_span: SpanId,
    pub digital_span: SpanId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinComparison {
    pub physical: TwinObservation,
    pub digital: TwinObservation,
    pub divergences: Vec<TwinDivergence>,
    pub aligned_metrics: Vec<String>,
}

impl TwinComparison {
    pub fn has_divergences(&self) -> bool {
        !self.divergences.is_empty()
    }
}

#[derive(Debug)]
pub struct BidirectionalTwinBridge {
    cycles: Vec<TwinSyncCycle>,
    config: SyncConfig,
}

impl BidirectionalTwinBridge {
    pub fn new() -> Self {
        Self {
            cycles: Vec::new(),
            config: SyncConfig::default(),
        }
    }

    pub fn with_config(config: SyncConfig) -> Self {
        Self {
            cycles: Vec::new(),
            config,
        }
    }

    pub fn analyze_cycle(
        &self,
        physical: TwinObservation,
        digital: TwinObservation,
    ) -> TwinComparison {
        let mut metrics: HashSet<String> = HashSet::new();
        metrics.extend(physical.metrics.keys().cloned());
        metrics.extend(digital.metrics.keys().cloned());

        let mut divergences = Vec::new();
        let mut aligned = Vec::new();
        let detected_at = if physical.recorded_at >= digital.recorded_at {
            physical.recorded_at
        } else {
            digital.recorded_at
        };

        for metric in metrics {
            let physical_value = physical.metrics.get(&metric).copied();
            let digital_value = digital.metrics.get(&metric).copied();

            if physical_value.is_some() && digital_value.is_some() {
                aligned.push(metric.clone());
            }

            if let Some(divergence) = compute_divergence(
                &metric,
                physical_value,
                digital_value,
                self.config.tolerance_for(&metric),
                detected_at,
                &physical.span_id,
                &digital.span_id,
            ) {
                divergences.push(divergence);
            }
        }

        aligned.sort();

        TwinComparison {
            physical,
            digital,
            divergences,
            aligned_metrics: aligned,
        }
    }

    pub fn record_cycle(&mut self, cycle: TwinSyncCycle) {
        let mut stored = cycle;
        if stored.divergences.len() > self.config.max_divergences {
            stored.divergences.truncate(self.config.max_divergences);
            info!(
                "divergence_truncated" = true,
                "cycle divergences truncated to max_divergences"
            );
        }

        if self.config.auto_reconcile && requires_reconciliation(&self.config, &stored) {
            info!(
                "auto_reconcile_triggered" = true,
                "divergences" = stored.divergences.len(),
                "threshold" = self.config.divergence_threshold
            );
        }

        info!("twin_sync_cycle" = ?stored, "Twin cycle recorded");
        self.cycles.push(stored);
    }

    pub fn record_comparison(&mut self, comparison: &TwinComparison) -> TwinSyncCycle {
        let started_at = if comparison.physical.recorded_at <= comparison.digital.recorded_at {
            comparison.physical.recorded_at
        } else {
            comparison.digital.recorded_at
        };
        let finished_at = Some(
            if comparison.physical.recorded_at >= comparison.digital.recorded_at {
                comparison.physical.recorded_at
            } else {
                comparison.digital.recorded_at
            },
        );

        let divergences = comparison
            .divergences
            .iter()
            .map(|d| d.span_id.clone())
            .collect();

        let cycle = TwinSyncCycle {
            started_at,
            finished_at,
            divergences,
        };

        self.record_cycle(cycle.clone());
        cycle
    }

    pub fn emit_divergence_span(
        &self,
        base: &UniversalSpan,
        divergence: &TwinDivergence,
    ) -> Result<UniversalSpan> {
        let payload = json!({
            "metric": divergence.metric,
            "physical_value": divergence.physical_value,
            "digital_value": divergence.digital_value,
            "absolute_delta": divergence.absolute_delta,
            "percent_delta": divergence.percent_delta,
            "severity": format!("{:?}", divergence.severity),
            "detected_at": divergence.detected_at.to_rfc3339(),
            "physical_span": divergence.physical_span.0,
            "digital_span": divergence.digital_span.0,
        });

        let mut span = UniversalSpan::new(
            divergence.span_id.0.clone(),
            format!("Twin divergence [{}]", divergence.metric),
            base.flow.clone(),
            base.workflow.clone(),
            divergence.detected_at,
            payload,
        )
        .with_parent(base.id.clone());

        span = span.add_related(divergence.physical_span.clone());
        span = span.add_related(divergence.digital_span.clone());
        Ok(span)
    }

    pub fn summarize(&self) -> TwinSummary {
        TwinSummary {
            total_cycles: self.cycles.len(),
            divergence_events: self
                .cycles
                .iter()
                .map(|cycle| cycle.divergences.len())
                .sum(),
            recent_cycle: self.cycles.last().cloned(),
            requires_reconciliation: self
                .cycles
                .last()
                .map(|cycle| requires_reconciliation(&self.config, cycle))
                .unwrap_or(false),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinSummary {
    pub total_cycles: usize,
    pub divergence_events: usize,
    pub recent_cycle: Option<TwinSyncCycle>,
    pub requires_reconciliation: bool,
}

fn requires_reconciliation(config: &SyncConfig, cycle: &TwinSyncCycle) -> bool {
    if config.max_divergences == 0 {
        return false;
    }
    let ratio = cycle.divergences.len() as f64 / config.max_divergences as f64;
    ratio >= config.divergence_threshold
}

fn compute_divergence(
    metric: &str,
    physical_value: Option<f64>,
    digital_value: Option<f64>,
    tolerance: f64,
    detected_at: DateTime<Utc>,
    physical_span: &SpanId,
    digital_span: &SpanId,
) -> Option<TwinDivergence> {
    match (physical_value, digital_value) {
        (Some(p), Some(d)) => {
            let delta = d - p;
            let absolute_delta = delta.abs();
            let baseline = p.abs().max(1e-9);
            let percent_delta = absolute_delta / baseline;
            if percent_delta <= tolerance {
                return None;
            }
            let severity = if percent_delta <= tolerance * 2.0 {
                DivergenceSeverity::Minor
            } else {
                DivergenceSeverity::Critical
            };
            Some(TwinDivergence {
                span_id: SpanId::new(format!("span::twin_divergence::{}", Uuid::new_v4())),
                metric: metric.to_string(),
                physical_value: Some(p),
                digital_value: Some(d),
                absolute_delta,
                percent_delta,
                severity,
                detected_at,
                physical_span: physical_span.clone(),
                digital_span: digital_span.clone(),
            })
        }
        (Some(p), None) => Some(TwinDivergence {
            span_id: SpanId::new(format!("span::twin_divergence::{}", Uuid::new_v4())),
            metric: metric.to_string(),
            physical_value: Some(p),
            digital_value: None,
            absolute_delta: p.abs(),
            percent_delta: f64::MAX,
            severity: DivergenceSeverity::Critical,
            detected_at,
            physical_span: physical_span.clone(),
            digital_span: digital_span.clone(),
        }),
        (None, Some(d)) => Some(TwinDivergence {
            span_id: SpanId::new(format!("span::twin_divergence::{}", Uuid::new_v4())),
            metric: metric.to_string(),
            physical_value: None,
            digital_value: Some(d),
            absolute_delta: d.abs(),
            percent_delta: f64::MAX,
            severity: DivergenceSeverity::Critical,
            detected_at,
            physical_span: physical_span.clone(),
            digital_span: digital_span.clone(),
        }),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn observation(span: &str, side: TwinSide, value: f64) -> TwinObservation {
        let mut metrics = HashMap::new();
        metrics.insert("temperature".to_string(), value);
        TwinObservation {
            span_id: SpanId::new(span),
            side,
            recorded_at: Utc::now(),
            metrics,
        }
    }

    #[test]
    fn record_cycle_truncates_divergences() {
        let mut bridge = BidirectionalTwinBridge::with_config(SyncConfig {
            max_divergences: 2,
            divergence_threshold: 0.5,
            auto_reconcile: false,
            ..Default::default()
        });

        bridge.record_cycle(TwinSyncCycle {
            started_at: Utc::now(),
            finished_at: None,
            divergences: vec![SpanId("a".into()), SpanId("b".into()), SpanId("c".into())],
        });

        let summary = bridge.summarize();
        assert_eq!(summary.total_cycles, 1);
        assert_eq!(summary.divergence_events, 2);
        assert_eq!(summary.recent_cycle.unwrap().divergences.len(), 2);
    }

    #[test]
    fn summary_flags_reconciliation_when_threshold_met() {
        let mut bridge = BidirectionalTwinBridge::with_config(SyncConfig {
            max_divergences: 4,
            divergence_threshold: 0.5,
            auto_reconcile: true,
            ..Default::default()
        });

        bridge.record_cycle(TwinSyncCycle {
            started_at: Utc::now(),
            finished_at: None,
            divergences: vec![SpanId("a".into()), SpanId("b".into())],
        });

        let summary = bridge.summarize();
        assert!(summary.requires_reconciliation);
    }

    #[test]
    fn analyze_cycle_detects_critical_divergence() {
        let mut bridge = BidirectionalTwinBridge::with_config(SyncConfig {
            default_metric_tolerance: 0.05,
            ..Default::default()
        });

        let physical = observation("span::phys", TwinSide::Physical, 300.0);
        let digital = observation("span::dig", TwinSide::Digital, 360.0);

        let comparison = bridge.analyze_cycle(physical.clone(), digital.clone());
        assert!(comparison.has_divergences());
        let divergence = &comparison.divergences[0];
        assert_eq!(divergence.metric, "temperature");
        assert_eq!(divergence.physical_value, Some(300.0));
        assert_eq!(divergence.digital_value, Some(360.0));
        assert_eq!(divergence.severity, DivergenceSeverity::Critical);

        let cycle = bridge.record_comparison(&comparison);
        assert_eq!(cycle.divergences.len(), 1);
    }

    #[test]
    fn emit_divergence_span_serializes_payload() {
        let bridge = BidirectionalTwinBridge::new();
        let base = UniversalSpan::new(
            "span::twin_base",
            "Twin base",
            "digital_twin",
            "twin_workflow",
            Utc::now(),
            json!({}),
        );

        let comparison = bridge.analyze_cycle(
            observation("span::phys", TwinSide::Physical, 300.0),
            observation("span::dig", TwinSide::Digital, 360.0),
        );
        let divergence = &comparison.divergences[0];

        let span = bridge
            .emit_divergence_span(&base, divergence)
            .expect("divergence span");
        assert_eq!(span.causal.parent_id, Some(base.id.clone()));
        assert_eq!(span.flow, base.flow);
        assert!(span.payload.get("metric").is_some());
    }

    #[test]
    fn analyze_cycle_ignores_within_tolerance_metrics() {
        let bridge = BidirectionalTwinBridge::with_config(SyncConfig {
            default_metric_tolerance: 0.2,
            ..Default::default()
        });

        let mut phys = observation("span::phys", TwinSide::Physical, 300.0);
        phys.metrics.insert("pressure".to_string(), 1000.0);
        let mut dig = observation("span::dig", TwinSide::Digital, 370.0);
        dig.metrics.insert("pressure".to_string(), 1020.0);

        let comparison = bridge.analyze_cycle(phys, dig);
        assert!(comparison.has_divergences());
        assert_eq!(comparison.divergences.len(), 1);
        assert!(comparison
            .aligned_metrics
            .iter()
            .any(|metric| metric == "pressure"));
    }
}
