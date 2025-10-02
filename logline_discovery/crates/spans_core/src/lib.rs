use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Canonical identifier for spans emitted inside the lab.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpanId(pub String);

impl SpanId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Minimal causal metadata shared across engines.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct CausalLinks {
    pub parent_id: Option<SpanId>,
    pub related_ids: Vec<SpanId>,
}


/// Universal span payload flowing between Warp, Fold, and the Discovery Lab.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalSpan {
    pub id: SpanId,
    pub name: String,
    pub flow: String,
    pub workflow: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub payload: serde_json::Value,
    pub causal: CausalLinks,
}

impl UniversalSpan {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        flow: impl Into<String>,
        workflow: impl Into<String>,
        started_at: DateTime<Utc>,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            id: SpanId::new(id),
            name: name.into(),
            flow: flow.into(),
            workflow: workflow.into(),
            started_at,
            finished_at: None,
            payload,
            causal: CausalLinks::default(),
        }
    }

    pub fn with_finish_time(mut self, ts: DateTime<Utc>) -> Self {
        self.finished_at = Some(ts);
        self
    }

    pub fn with_parent(mut self, parent: SpanId) -> Self {
        self.causal.parent_id = Some(parent);
        self
    }

    pub fn add_related(mut self, related: SpanId) -> Self {
        self.causal.related_ids.push(related);
        self
    }
}

/// Helper to construct spans that wrap external systems (e.g., LogLine Warp spans).
pub fn span_from_json(value: serde_json::Value) -> anyhow::Result<UniversalSpan> {
    let id = value
        .get("span_id")
        .or_else(|| value.get("id"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("span identifier missing"))?;

    let name = value
        .get("name")
        .or_else(|| value.get("label"))
        .and_then(|v| v.as_str())
        .unwrap_or("unnamed");

    let flow = value
        .get("flow")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown_flow");

    let workflow = value
        .get("workflow")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown_workflow");

    let started_at = value
        .get("started_at")
        .or_else(|| value.get("timestamp"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("span timestamp missing"))?;

    let started_at = started_at.parse::<DateTime<Utc>>()?;

    let finished_at = value
        .get("finished_at")
        .and_then(|v| v.as_str())
        .and_then(|ts| ts.parse::<DateTime<Utc>>().ok());

    let parent_id = value
        .get("parent_id")
        .and_then(|v| v.as_str())
        .map(SpanId::new);

    let related_ids = value
        .get("related_ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(SpanId::new)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let mut span = UniversalSpan::new(id, name, flow, workflow, started_at, value.clone());

    if let Some(ts) = finished_at {
        span = span.with_finish_time(ts);
    }

    if let Some(parent) = parent_id {
        span = span.with_parent(parent);
    }

    if !related_ids.is_empty() {
        for rel in related_ids {
            span = span.add_related(rel);
        }
    }

    Ok(span)
}
