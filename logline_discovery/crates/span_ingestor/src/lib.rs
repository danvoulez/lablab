use chrono::{DateTime, Utc};
use serde_json::Value;
use spans_core::{span_from_json, UniversalSpan};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IngestError {
    #[error("span parsing failed: {0}")]
    Parse(#[from] anyhow::Error),
    #[error("required field missing: {0}")]
    MissingField(&'static str),
}

#[derive(Debug, Clone)]
pub struct IngestOptions {
    pub flow: String,
    pub workflow: String,
    pub default_start: DateTime<Utc>,
}

impl IngestOptions {
    pub fn new(
        flow: impl Into<String>,
        workflow: impl Into<String>,
        default_start: DateTime<Utc>,
    ) -> Self {
        Self {
            flow: flow.into(),
            workflow: workflow.into(),
            default_start,
        }
    }
}

/// Ingest a payload into a UniversalSpan, applying LogLine Discovery conventions.
pub fn ingest_json(mut value: Value, opts: &IngestOptions) -> Result<UniversalSpan, IngestError> {
    value
        .as_object_mut()
        .ok_or(IngestError::MissingField("payload must be an object"))?
        .entry("flow")
        .or_insert(Value::String(opts.flow.clone()));

    value
        .as_object_mut()
        .expect("object checked")
        .entry("workflow")
        .or_insert(Value::String(opts.workflow.clone()));

    if !value.as_object().unwrap().contains_key("started_at")
        && !value.as_object().unwrap().contains_key("timestamp")
    {
        value.as_object_mut().unwrap().insert(
            "started_at".into(),
            Value::String(opts.default_start.to_rfc3339()),
        );
    }

    Ok(span_from_json(value)?)
}

/// Specialized adapter for LogLine Fold span payloads.
pub fn ingest_fold_json(value: Value, opts: &IngestOptions) -> Result<UniversalSpan, IngestError> {
    let mut value = value;
    let started_at = value
        .get("execution")
        .and_then(|exec| exec.get("timestamp"))
        .and_then(|ts| ts.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| opts.default_start.to_rfc3339());

    let name_hint = value
        .get("protein_metadata")
        .and_then(|meta| meta.get("target"))
        .and_then(|target| target.as_str())
        .map(|target| format!("{} folding", target));

    if let Some(obj) = value.as_object_mut() {
        obj.entry("name").or_insert_with(|| {
            Value::String(
                name_hint
                    .clone()
                    .unwrap_or_else(|| "folding simulation".to_string()),
            )
        });
        obj.insert("timestamp".into(), Value::String(started_at.clone()));
        obj.entry("started_at")
            .or_insert_with(|| Value::String(started_at.clone()));
    } else {
        return Err(IngestError::MissingField("payload must be an object"));
    }

    ingest_json(value, opts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;

    #[test]
    fn ingests_fold_payload() {
        let raw = json!({
            "span_id": "test_span",
            "protein_metadata": {
                "target": "chignolin"
            },
            "execution": {
                "timestamp": "2025-09-27T19:51:10Z"
            },
            "results": {
                "final_rmsd_angstrom": 0.42
            }
        });

        let opts = IngestOptions::new("protein_folding", "fold_pipeline", Utc::now());
        let span = ingest_fold_json(raw, &opts).expect("span ingest");

        assert_eq!(span.id.0, "test_span");
        assert_eq!(span.flow, "protein_folding");
        assert_eq!(span.name, "chignolin folding");
        assert_eq!(span.workflow, "fold_pipeline");
        assert_eq!(span.started_at.to_rfc3339(), "2025-09-27T19:51:10+00:00");
    }
}
