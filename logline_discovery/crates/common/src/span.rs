use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanMetadata {
    pub span_id: String,
    pub flow: String,
    pub workflow: String,
    pub created_at: DateTime<Utc>,
    pub tags: std::collections::HashMap<String, String>,
}

impl SpanMetadata {
    pub fn new(span_id: String, flow: String, workflow: String) -> Self {
        Self {
            span_id,
            flow,
            workflow,
            created_at: Utc::now(),
            tags: std::collections::HashMap::new(),
        }
    }

    pub fn with_tag(mut self, key: String, value: String) -> Self {
        self.tags.insert(key, value);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpanKind {
    Execution,
    Metric,
    Artifact,
    Analysis,
    Manuscript,
    TwinObservation,
    TwinDivergence,
}

impl SpanKind {
    pub fn from_flow(flow: &str) -> Self {
        match flow {
            "execution_run" | "execution" => SpanKind::Execution,
            "metric" => SpanKind::Metric,
            "artifact" => SpanKind::Artifact,
            "analysis" => SpanKind::Analysis,
            "manuscript" => SpanKind::Manuscript,
            "twin_observation" => SpanKind::TwinObservation,
            "twin_divergence" => SpanKind::TwinDivergence,
            _ => SpanKind::Analysis, // Default fallback
        }
    }
}
