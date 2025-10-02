use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use spans_core::UniversalSpan;

#[derive(Debug, Clone, Deserialize)]
pub struct SubjectMetadata {
    pub subject_type: String,
    pub subject_identifier: String,
    pub intent: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProtocolMetadata {
    pub recipe_name: String,
    pub parameters: Value,
    pub checksum: Option<String>,
    #[serde(default)]
    pub subject_span: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionMetadata {
    pub status: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub protocol_span: Option<String>,
    pub subject_span: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AcquisitionMetadata {
    pub execution_span: Option<String>,
    pub source: String,
    pub artifact_type: String,
    pub artifact_reference: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FrameMetadata {
    pub execution_span: Option<String>,
    pub frame_index: i32,
    pub recorded_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub payload: Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MetricMetadata {
    pub execution_span: Option<String>,
    pub metric_name: String,
    pub value: Option<f64>,
    pub unit: Option<String>,
    pub recorded_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub metadata: Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnalysisMetadata {
    pub execution_span: Option<String>,
    pub analysis_type: String,
    #[serde(default)]
    pub summary: Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReviewMetadata {
    pub execution_span: Option<String>,
    pub reviewer: String,
    pub verdict: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ManuscriptMetadata {
    pub execution_span: Option<String>,
    pub title: String,
    pub format: String,
    pub storage_path: String,
    pub checksum: String,
    #[serde(default)]
    pub metadata: Value,
}

#[derive(Debug, Clone)]
pub struct TwinObservationMetadata {
    pub cycle_id: String,
    pub side: String,
    pub recorded_at: DateTime<Utc>,
    pub metrics: Value,
    pub execution_span: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TwinDivergenceMetadata {
    pub cycle_id: String,
    pub metric: String,
    pub severity: String,
    pub absolute_delta: f64,
    pub percent_delta: f64,
    pub detected_at: DateTime<Utc>,
    pub physical_span: Option<String>,
    pub digital_span: Option<String>,
    pub execution_span: Option<String>,
    pub payload: Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArtifactMetadata {
    pub execution_span: Option<String>,
    pub artifact_kind: String,
    pub storage_path: String,
    pub checksum: String,
}

#[derive(Debug, Clone)]
pub enum SpanKind {
    Subject(SubjectMetadata),
    Protocol(ProtocolMetadata),
    Execution(ExecutionMetadata),
    Acquisition(AcquisitionMetadata),
    Frame(FrameMetadata),
    Metric(MetricMetadata),
    Analysis(AnalysisMetadata),
    Review(ReviewMetadata),
    Manuscript(ManuscriptMetadata),
    Artifact(ArtifactMetadata),
    TwinObservation(TwinObservationMetadata),
    TwinDivergence(TwinDivergenceMetadata),
    Unknown,
}

pub fn classify_span(span: &UniversalSpan) -> SpanKind {
    match span.flow.as_str() {
        "subject_init" => span
            .payload
            .get("metadata")
            .and_then(|v| serde_json::from_value::<SubjectMetadata>(v.clone()).ok())
            .map(SpanKind::Subject)
            .unwrap_or(SpanKind::Unknown),
        "protocol_config" => span
            .payload
            .get("metadata")
            .and_then(|v| serde_json::from_value::<ProtocolMetadata>(v.clone()).ok())
            .map(SpanKind::Protocol)
            .unwrap_or(SpanKind::Unknown),
        "execution_run" => span
            .payload
            .get("metadata")
            .and_then(|v| serde_json::from_value::<ExecutionMetadata>(v.clone()).ok())
            .map(SpanKind::Execution)
            .unwrap_or(SpanKind::Unknown),
        "acquisition" => span
            .payload
            .get("metadata")
            .or_else(|| span.payload.get("acquisition"))
            .and_then(|v| serde_json::from_value::<AcquisitionMetadata>(v.clone()).ok())
            .map(SpanKind::Acquisition)
            .unwrap_or(SpanKind::Unknown),
        "frame" => span
            .payload
            .get("metadata")
            .and_then(|v| serde_json::from_value::<FrameMetadata>(v.clone()).ok())
            .map(SpanKind::Frame)
            .unwrap_or(SpanKind::Unknown),
        "metric" => span
            .payload
            .get("metadata")
            .and_then(|v| serde_json::from_value::<MetricMetadata>(v.clone()).ok())
            .map(SpanKind::Metric)
            .unwrap_or(SpanKind::Unknown),
        "analysis" => span
            .payload
            .get("metadata")
            .and_then(|v| serde_json::from_value::<AnalysisMetadata>(v.clone()).ok())
            .map(SpanKind::Analysis)
            .unwrap_or(SpanKind::Unknown),
        "review" => span
            .payload
            .get("metadata")
            .and_then(|v| serde_json::from_value::<ReviewMetadata>(v.clone()).ok())
            .map(SpanKind::Review)
            .unwrap_or(SpanKind::Unknown),
        "manuscript" => span
            .payload
            .get("metadata")
            .and_then(|v| serde_json::from_value::<ManuscriptMetadata>(v.clone()).ok())
            .map(SpanKind::Manuscript)
            .unwrap_or(SpanKind::Unknown),
        "artifact_manifest" => span
            .payload
            .get("metadata")
            .and_then(|v| serde_json::from_value::<ArtifactMetadata>(v.clone()).ok())
            .map(SpanKind::Artifact)
            .unwrap_or(SpanKind::Unknown),
        "twin_observation" => parse_twin_observation(span)
            .map(SpanKind::TwinObservation)
            .unwrap_or(SpanKind::Unknown),
        "twin_divergence" => parse_twin_divergence(span)
            .map(SpanKind::TwinDivergence)
            .unwrap_or(SpanKind::Unknown),
        _ => SpanKind::Unknown,
    }
}

pub fn resolve_subject_span(span: &UniversalSpan, meta: &ExecutionMetadata) -> Result<String> {
    if let Some(subject_span) = &meta.subject_span {
        return Ok(subject_span.clone());
    }
    if let Some(parent) = &span.causal.parent_id {
        return Ok(parent.0.clone());
    }
    Err(anyhow!(
        "subject reference missing for execution span {}",
        span.id.0
    ))
}

pub fn resolve_protocol_span(span: &UniversalSpan, meta: &ExecutionMetadata) -> Option<String> {
    meta.protocol_span
        .clone()
        .or_else(|| span.causal.related_ids.first().map(|id| id.0.clone()))
}

pub fn resolve_protocol_subject(meta: &ProtocolMetadata, span: &UniversalSpan) -> Option<String> {
    meta.subject_span
        .clone()
        .or_else(|| span.causal.parent_id.as_ref().map(|id| id.0.clone()))
}

pub fn resolve_execution_reference(
    span: &UniversalSpan,
    execution_span: &Option<String>,
) -> Option<String> {
    execution_span
        .clone()
        .or_else(|| span.causal.parent_id.as_ref().map(|id| id.0.clone()))
        .or_else(|| span.causal.related_ids.first().map(|id| id.0.clone()))
}

fn parse_twin_observation(span: &UniversalSpan) -> Option<TwinObservationMetadata> {
    // Try the nested twin format first (for backwards compatibility)
    if let Some(twin) = span
        .payload
        .get("twin")
        .or_else(|| span.payload.get("metadata").and_then(|m| m.get("twin")))
    {
        if let (Some(cycle_id), Some(side)) = (
            twin.get("cycle_id").and_then(|v| v.as_str()),
            twin.get("side").and_then(|v| v.as_str()),
        ) {
            let recorded_at = twin
                .get("recorded_at")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse::<DateTime<Utc>>().ok())
                .or(span.finished_at)
                .unwrap_or(span.started_at);

            let metrics = twin.get("metrics")?.clone();

            let execution_span = twin
                .get("execution_span")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .or_else(|| {
                    span.payload
                        .get("execution_span")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                })
                .or_else(|| {
                    span.payload
                        .get("metadata")
                        .and_then(|m| m.get("execution_span"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                })
                .or_else(|| span.causal.parent_id.as_ref().map(|id| id.0.clone()));

            return Some(TwinObservationMetadata {
                cycle_id: cycle_id.to_string(),
                side: side.to_string(),
                recorded_at,
                metrics,
                execution_span,
            });
        }
    }

    // Try the flat metadata format (our current format)
    if let Some(metadata) = span.payload.get("metadata") {
        if let (Some(twin_type), Some(observations)) = (
            metadata.get("twin_type").and_then(|v| v.as_str()),
            metadata.get("observations"),
        ) {
            let recorded_at = metadata
                .get("recorded_at")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse::<DateTime<Utc>>().ok())
                .or(span.finished_at)
                .unwrap_or(span.started_at);

            let metrics = observations.clone();

            let execution_span = metadata
                .get("execution_span")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .or_else(|| span.causal.parent_id.as_ref().map(|id| id.0.clone()));

            return Some(TwinObservationMetadata {
                cycle_id: format!("cycle::{}", span.id.0),
                side: twin_type.to_string(),
                recorded_at,
                metrics,
                execution_span,
            });
        }
    }

    None
}

fn parse_twin_divergence(span: &UniversalSpan) -> Option<TwinDivergenceMetadata> {
    let payload = &span.payload;
    let metadata = payload.get("metadata");

    let cycle_id = payload
        .get("cycle_id")
        .and_then(|v| v.as_str())
        .or_else(|| {
            metadata
                .and_then(|m| m.get("cycle_id"))
                .and_then(|v| v.as_str())
        })?
        .to_string();
    let metric = payload
        .get("metric")
        .and_then(|v| v.as_str())
        .or_else(|| {
            metadata
                .and_then(|m| m.get("divergence_metric"))
                .and_then(|v| v.as_str())
        })?
        .to_string();
    let severity = payload
        .get("severity")
        .and_then(|v| v.as_str())
        .or_else(|| {
            metadata
                .and_then(|m| m.get("severity"))
                .and_then(|v| v.as_str())
        })?
        .to_string();
    let absolute_delta = payload
        .get("absolute_delta")
        .and_then(|v| v.as_f64())
        .or_else(|| {
            metadata
                .and_then(|m| m.get("absolute_delta"))
                .and_then(|v| v.as_f64())
        })?;
    let percent_delta = payload
        .get("percent_delta")
        .and_then(|v| v.as_f64())
        .or_else(|| {
            metadata
                .and_then(|m| m.get("percent_delta"))
                .and_then(|v| v.as_f64())
        })?;

    let detected_at = payload
        .get("detected_at")
        .and_then(|v| v.as_str())
        .or_else(|| {
            metadata
                .and_then(|m| m.get("detected_at"))
                .and_then(|v| v.as_str())
        })
        .and_then(|s| s.parse::<DateTime<Utc>>().ok())
        .or(span.finished_at)
        .unwrap_or(span.started_at);

    let physical_span = payload
        .get("physical_span")
        .and_then(|v| v.as_str())
        .or_else(|| {
            metadata
                .and_then(|m| m.get("physical_span"))
                .and_then(|v| v.as_str())
        })
        .map(|s| s.to_string());
    let digital_span = payload
        .get("digital_span")
        .and_then(|v| v.as_str())
        .or_else(|| {
            metadata
                .and_then(|m| m.get("digital_span"))
                .and_then(|v| v.as_str())
        })
        .map(|s| s.to_string());

    let execution_span = payload
        .get("execution_span")
        .and_then(|v| v.as_str())
        .or_else(|| {
            metadata
                .and_then(|m| m.get("execution_span"))
                .and_then(|v| v.as_str())
        })
        .map(|s| s.to_string())
        .or_else(|| span.causal.parent_id.as_ref().map(|id| id.0.clone()));

    Some(TwinDivergenceMetadata {
        cycle_id,
        metric,
        severity,
        absolute_delta,
        percent_delta,
        detected_at,
        physical_span,
        digital_span,
        execution_span,
        payload: span.payload.clone(),
    })
}

pub fn metadata_payload(span: &UniversalSpan) -> Value {
    span.payload
        .get("metadata")
        .cloned()
        .unwrap_or_else(|| Value::Object(Default::default()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use serde_json::json;
    use spans_core::{SpanId, UniversalSpan};

    fn base_span(flow: &str, payload: Value) -> UniversalSpan {
        UniversalSpan::new(
            "span::test",
            "test span",
            flow,
            "workflow",
            Utc.timestamp_opt(1_700_000_000, 0).unwrap(),
            payload,
        )
    }

    #[test]
    fn classifies_acquisition() {
        let payload = json!({
            "metadata": {
                "execution_span": "span::exec",
                "source": "PDB",
                "artifact_type": "structure",
                "artifact_reference": "1ABC"
            }
        });
        let mut span = base_span("acquisition", payload);
        span.causal.parent_id = Some(SpanId::new("span::parent"));

        match classify_span(&span) {
            SpanKind::Acquisition(meta) => {
                assert_eq!(meta.source, "PDB");
                assert_eq!(meta.artifact_reference, "1ABC");
            }
            other => panic!("unexpected span kind: {:?}", other),
        }
    }

    #[test]
    fn classifies_twin_observation() {
        let payload = json!({
            "twin": {
                "cycle_id": "cycle::001",
                "side": "physical",
                "recorded_at": "2025-09-29T12:00:00Z",
                "metrics": {"temperature": 300.0},
                "execution_span": "span::exec"
            }
        });
        let span = base_span("twin_observation", payload);
        match classify_span(&span) {
            SpanKind::TwinObservation(meta) => {
                assert_eq!(meta.cycle_id, "cycle::001");
                assert_eq!(meta.side, "physical");
                assert_eq!(meta.execution_span.as_deref(), Some("span::exec"));
            }
            other => panic!("unexpected span kind: {:?}", other),
        }
    }

    #[test]
    fn classifies_twin_divergence() {
        let payload = json!({
            "cycle_id": "cycle::001",
            "metric": "temperature",
            "severity": "Critical",
            "absolute_delta": 60.0,
            "percent_delta": 0.2,
            "detected_at": "2025-09-29T12:05:00Z",
            "physical_span": "span::phys",
            "digital_span": "span::dig",
            "execution_span": "span::exec"
        });
        let span = base_span("twin_divergence", payload);
        match classify_span(&span) {
            SpanKind::TwinDivergence(meta) => {
                assert_eq!(meta.metric, "temperature");
                assert_eq!(meta.severity, "Critical");
                assert_eq!(meta.execution_span.as_deref(), Some("span::exec"));
            }
            other => panic!("unexpected span kind: {:?}", other),
        }
    }

    #[test]
    fn resolve_execution_reference_prefers_metadata() {
        let span = base_span(
            "metric",
            json!({"metadata": {"execution_span": "span::exec", "metric_name": "rmsd"}}),
        );
        let resolved = resolve_execution_reference(&span, &Some("span::exec".into()));
        assert_eq!(resolved.unwrap(), "span::exec");
    }
}
