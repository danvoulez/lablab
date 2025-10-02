use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use causal_engine::{CausalChain, CausalEngine};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use spans_core::UniversalSpan;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::commands::sync_ledger;
use crate::config::RunnerConfig;
use crate::mapping::{self, SpanKind};

#[derive(Clone)]
struct AppState {
    ledger: Arc<LedgerCache>,
}

pub async fn serve(cfg: RunnerConfig, address: SocketAddr) -> Result<()> {
    let ledger = Arc::new(LedgerCache::new(cfg.ledger_path.clone()));
    let state = AppState { ledger };

    let app = Router::new()
        .route("/health", get(health))
        .route("/executions", get(list_executions))
        .route("/executions/:execution_id/spans", get(execution_spans))
        .route("/executions/:execution_id/causal", get(execution_causal))
        .route("/executions/:execution_id/twin", get(execution_twin))
        .route("/executions/:execution_id/twin-observations", get(execution_twin_observations))
        .route("/executions/:execution_id/twin-divergences", get(execution_twin_divergences))
        .route("/twin-observations", get(all_twin_observations))
        .route("/twin-divergences", get(all_twin_divergences))
        .with_state(state);

    let listener = TcpListener::bind(address).await?;
    info!(address = %address, "runner_service_listening");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "ok" })))
}

async fn list_executions(
    State(state): State<AppState>,
) -> Result<Json<Vec<ExecutionSummary>>, AppError> {
    let spans = state.ledger.spans().await.map_err(AppError::from)?;
    let mut executions = Vec::new();

    for span in spans {
        if matches!(span.flow.as_str(), "execution_run") {
            let summary = build_execution_summary(&span);
            executions.push(summary);
        }
    }

    executions.sort_by(|a, b| b.started_at.cmp(&a.started_at));
    Ok(Json(executions))
}

async fn execution_spans(
    State(state): State<AppState>,
    Path(execution_id): Path<String>,
) -> Result<Json<Vec<UniversalSpan>>, AppError> {
    let spans = state.ledger.spans().await.map_err(AppError::from)?;
    let matching = spans_for_execution(&spans, &execution_id);
    if matching.is_empty() {
        return Err(AppError::not_found(format!(
            "execution span {} not found",
            execution_id
        )));
    }
    Ok(Json(matching))
}

async fn execution_causal(
    State(state): State<AppState>,
    Path(execution_id): Path<String>,
) -> Result<Json<Vec<CausalChain>>, AppError> {
    let spans = state.ledger.spans().await.map_err(AppError::from)?;
    let matching = spans_for_execution(&spans, &execution_id);
    if matching.is_empty() {
        return Err(AppError::not_found(format!(
            "execution span {} not found",
            execution_id
        )));
    }

    let mut engine = CausalEngine::new();
    engine.ingest(matching);
    let chains = engine.infer();
    Ok(Json(chains))
}

async fn execution_twin(
    State(state): State<AppState>,
    Path(execution_id): Path<String>,
) -> Result<Json<TwinSummary>, AppError> {
    let spans = state.ledger.spans().await.map_err(AppError::from)?;
    let matching = spans_for_execution(&spans, &execution_id);
    if matching.is_empty() {
        return Err(AppError::not_found(format!(
            "execution span {} not found",
            execution_id
        )));
    }

    let summary = build_twin_summary(&spans, &execution_id).ok_or_else(|| {
        AppError::not_found(format!(
            "no twin telemetry available for execution {}",
            execution_id
        ))
    })?;

    Ok(Json(summary))
}

async fn all_twin_observations(
    State(state): State<AppState>,
) -> Result<Json<Vec<TwinObservationSummary>>, AppError> {
    let spans = state.ledger.spans().await.map_err(AppError::from)?;
    let mut observations = Vec::new();

    for span in &spans {
        if matches!(span.flow.as_str(), "twin_observation") {
            if let SpanKind::TwinObservation(meta) = mapping::classify_span(span) {
                let metric_count = meta
                    .metrics
                    .as_object()
                    .map(|m| m.len())
                    .or_else(|| meta.metrics.as_array().map(|arr| arr.len()))
                    .unwrap_or(0);

                observations.push(TwinObservationSummary {
                    span_id: span.id.0.clone(),
                    cycle_id: meta.cycle_id.clone(),
                    side: meta.side.clone(),
                    recorded_at: meta.recorded_at,
                    metric_count,
                    metrics: meta.metrics.clone(),
                });
            }
        }
    }

    observations.sort_by_key(|obs| obs.recorded_at);
    Ok(Json(observations))
}

async fn all_twin_divergences(
    State(state): State<AppState>,
) -> Result<Json<Vec<TwinDivergenceSummary>>, AppError> {
    let spans = state.ledger.spans().await.map_err(AppError::from)?;
    let mut divergences = Vec::new();

    for span in &spans {
        if matches!(span.flow.as_str(), "twin_divergence") {
            if let SpanKind::TwinDivergence(meta) = mapping::classify_span(span) {
                divergences.push(TwinDivergenceSummary {
                    span_id: span.id.0.clone(),
                    cycle_id: meta.cycle_id.clone(),
                    metric: meta.metric.clone(),
                    severity: meta.severity.clone(),
                    absolute_delta: meta.absolute_delta,
                    percent_delta: meta.percent_delta,
                    detected_at: meta.detected_at,
                    physical_span: meta.physical_span.clone(),
                    digital_span: meta.digital_span.clone(),
                    payload: meta.payload.clone(),
                });
            }
        }
    }

    divergences.sort_by_key(|div| div.detected_at);
    Ok(Json(divergences))
}

async fn execution_twin_observations(
    State(state): State<AppState>,
    Path(execution_id): Path<String>,
) -> Result<Json<Vec<TwinObservationSummary>>, AppError> {
    let spans = state.ledger.spans().await.map_err(AppError::from)?;
    let matching = spans_for_execution(&spans, &execution_id);
    if matching.is_empty() {
        return Err(AppError::not_found(format!(
            "execution span {} not found",
            execution_id
        )));
    }

    let mut observations = Vec::new();
    for span in &matching {
        if matches!(span.flow.as_str(), "twin_observation") {
            if let SpanKind::TwinObservation(meta) = mapping::classify_span(span) {
                if !span_links_execution(span, &execution_id, meta.execution_span.as_ref()) {
                    continue;
                }

                let metric_count = meta
                    .metrics
                    .as_object()
                    .map(|m| m.len())
                    .or_else(|| meta.metrics.as_array().map(|arr| arr.len()))
                    .unwrap_or(0);

                observations.push(TwinObservationSummary {
                    span_id: span.id.0.clone(),
                    cycle_id: meta.cycle_id.clone(),
                    side: meta.side.clone(),
                    recorded_at: meta.recorded_at,
                    metric_count,
                    metrics: meta.metrics.clone(),
                });
            }
        }
    }

    observations.sort_by_key(|obs| obs.recorded_at);
    Ok(Json(observations))
}

async fn execution_twin_divergences(
    State(state): State<AppState>,
    Path(execution_id): Path<String>,
) -> Result<Json<Vec<TwinDivergenceSummary>>, AppError> {
    let spans = state.ledger.spans().await.map_err(AppError::from)?;
    let matching = spans_for_execution(&spans, &execution_id);
    if matching.is_empty() {
        return Err(AppError::not_found(format!(
            "execution span {} not found",
            execution_id
        )));
    }

    let mut divergences = Vec::new();
    for span in &matching {
        if matches!(span.flow.as_str(), "twin_divergence") {
            if let SpanKind::TwinDivergence(meta) = mapping::classify_span(span) {
                if !span_links_execution(span, &execution_id, meta.execution_span.as_ref()) {
                    continue;
                }

                divergences.push(TwinDivergenceSummary {
                    span_id: span.id.0.clone(),
                    cycle_id: meta.cycle_id.clone(),
                    metric: meta.metric.clone(),
                    severity: meta.severity.clone(),
                    absolute_delta: meta.absolute_delta,
                    percent_delta: meta.percent_delta,
                    detected_at: meta.detected_at,
                    physical_span: meta.physical_span.clone(),
                    digital_span: meta.digital_span.clone(),
                    payload: meta.payload.clone(),
                });
            }
        }
    }

    divergences.sort_by_key(|div| div.detected_at);
    Ok(Json(divergences))
}

fn build_execution_summary(span: &UniversalSpan) -> ExecutionSummary {
    let default_started = span.started_at;
    let default_finished = span.finished_at;
    match mapping::classify_span(span) {
        SpanKind::Execution(meta) => ExecutionSummary {
            span_id: span.id.0.clone(),
            workflow: span.workflow.clone(),
            status: meta.status.clone(),
            started_at: meta.started_at.unwrap_or(default_started),
            finished_at: meta.finished_at.or(default_finished),
            subject_span: meta
                .subject_span
                .clone()
                .or_else(|| span.causal.parent_id.as_ref().map(|id| id.0.clone())),
            protocol_span: meta
                .protocol_span
                .clone()
                .or_else(|| span.causal.related_ids.first().map(|id| id.0.clone())),
        },
        _ => ExecutionSummary {
            span_id: span.id.0.clone(),
            workflow: span.workflow.clone(),
            status: None,
            started_at: default_started,
            finished_at: default_finished,
            subject_span: span.causal.parent_id.as_ref().map(|id| id.0.clone()),
            protocol_span: span.causal.related_ids.iter().map(|id| id.0.clone()).next(),
        },
    }
}

fn spans_for_execution(spans: &[UniversalSpan], execution_id: &str) -> Vec<UniversalSpan> {
    spans
        .iter()
        .filter_map(|span| {
            if span.id.0 == execution_id {
                return Some(span.clone());
            }

            if span
                .causal
                .parent_id
                .as_ref()
                .map(|parent| parent.0.as_str() == execution_id)
                .unwrap_or(false)
            {
                return Some(span.clone());
            }

            if span
                .causal
                .related_ids
                .iter()
                .any(|related| related.0 == execution_id)
            {
                return Some(span.clone());
            }

            match mapping::classify_span(span) {
                SpanKind::Analysis(meta) => meta
                    .execution_span
                    .as_deref()
                    .filter(|span_id| *span_id == execution_id)
                    .map(|_| span.clone()),
                SpanKind::Metric(meta) => meta
                    .execution_span
                    .as_deref()
                    .filter(|span_id| *span_id == execution_id)
                    .map(|_| span.clone()),
                SpanKind::Frame(meta) => meta
                    .execution_span
                    .as_deref()
                    .filter(|span_id| *span_id == execution_id)
                    .map(|_| span.clone()),
                SpanKind::Acquisition(meta) => meta
                    .execution_span
                    .as_deref()
                    .filter(|span_id| *span_id == execution_id)
                    .map(|_| span.clone()),
                SpanKind::Review(meta) => meta
                    .execution_span
                    .as_deref()
                    .filter(|span_id| *span_id == execution_id)
                    .map(|_| span.clone()),
                SpanKind::Manuscript(meta) => meta
                    .execution_span
                    .as_deref()
                    .filter(|span_id| *span_id == execution_id)
                    .map(|_| span.clone()),
                SpanKind::Artifact(meta) => meta
                    .execution_span
                    .as_deref()
                    .filter(|span_id| *span_id == execution_id)
                    .map(|_| span.clone()),
                SpanKind::Execution(meta) => meta
                    .protocol_span
                    .as_deref()
                    .filter(|span_id| *span_id == execution_id)
                    .map(|_| span.clone()),
                _ => None,
            }
        })
        .collect()
}

fn build_twin_summary(spans: &[UniversalSpan], execution_id: &str) -> Option<TwinSummary> {
    let mut observations = Vec::new();
    let mut divergences = Vec::new();
    let mut cycles: HashMap<String, CyclePresence> = HashMap::new();
    let mut latest_observation: Option<DateTime<Utc>> = None;
    let mut latest_divergence: Option<DateTime<Utc>> = None;

    for span in spans {
        match mapping::classify_span(span) {
            SpanKind::TwinObservation(meta) => {
                if !span_links_execution(span, execution_id, meta.execution_span.as_ref()) {
                    continue;
                }

                let metric_count = meta
                    .metrics
                    .as_object()
                    .map(|m| m.len())
                    .or_else(|| meta.metrics.as_array().map(|arr| arr.len()))
                    .unwrap_or(0);

                observations.push(TwinObservationSummary {
                    span_id: span.id.0.clone(),
                    cycle_id: meta.cycle_id.clone(),
                    side: meta.side.clone(),
                    recorded_at: meta.recorded_at,
                    metric_count,
                    metrics: meta.metrics.clone(),
                });

                cycles
                    .entry(meta.cycle_id.clone())
                    .or_insert_with(CyclePresence::new)
                    .mark(&meta.side);

                if latest_observation
                    .map(|current| meta.recorded_at > current)
                    .unwrap_or(true)
                {
                    latest_observation = Some(meta.recorded_at);
                }
            }
            SpanKind::TwinDivergence(meta) => {
                if !span_links_execution(span, execution_id, meta.execution_span.as_ref()) {
                    continue;
                }

                divergences.push(TwinDivergenceSummary {
                    span_id: span.id.0.clone(),
                    cycle_id: meta.cycle_id.clone(),
                    metric: meta.metric.clone(),
                    severity: meta.severity.clone(),
                    absolute_delta: meta.absolute_delta,
                    percent_delta: meta.percent_delta,
                    detected_at: meta.detected_at,
                    physical_span: meta.physical_span.clone(),
                    digital_span: meta.digital_span.clone(),
                    payload: meta.payload.clone(),
                });

                cycles
                    .entry(meta.cycle_id.clone())
                    .or_insert_with(CyclePresence::new);

                if latest_divergence
                    .map(|current| meta.detected_at > current)
                    .unwrap_or(true)
                {
                    latest_divergence = Some(meta.detected_at);
                }
            }
            _ => {}
        }
    }

    if observations.is_empty() && divergences.is_empty() {
        return None;
    }

    observations.sort_by_key(|obs| obs.recorded_at);
    divergences.sort_by_key(|div| div.detected_at);

    let cycles_total = cycles.len();
    let paired_cycles = cycles
        .values()
        .filter(|presence| presence.physical && presence.digital)
        .count();

    Some(TwinSummary {
        execution_id: execution_id.to_string(),
        cycles: cycles_total,
        paired_cycles,
        observation_total: observations.len(),
        latest_observation,
        divergence_total: divergences.len(),
        latest_divergence,
        observations,
        divergences,
    })
}

fn span_links_execution(
    span: &UniversalSpan,
    execution_id: &str,
    execution_hint: Option<&String>,
) -> bool {
    if execution_hint
        .map(|candidate| candidate == execution_id)
        .unwrap_or(false)
    {
        return true;
    }

    if let Some(value) = span.payload.get("execution_span").and_then(|v| v.as_str()) {
        if value == execution_id {
            return true;
        }
    }

    if let Some(value) = span
        .payload
        .get("metadata")
        .and_then(|meta| meta.get("execution_span"))
        .and_then(|v| v.as_str())
    {
        if value == execution_id {
            return true;
        }
    }

    if span.id.0 == execution_id {
        return true;
    }

    if span
        .causal
        .parent_id
        .as_ref()
        .map(|parent| parent.0.as_str() == execution_id)
        .unwrap_or(false)
    {
        return true;
    }

    if span
        .causal
        .related_ids
        .iter()
        .any(|related| related.0 == execution_id)
    {
        return true;
    }

    false
}

#[derive(Serialize)]
struct ExecutionSummary {
    span_id: String,
    workflow: String,
    status: Option<String>,
    started_at: DateTime<Utc>,
    finished_at: Option<DateTime<Utc>>,
    subject_span: Option<String>,
    protocol_span: Option<String>,
}

#[derive(Serialize)]
struct TwinObservationSummary {
    span_id: String,
    cycle_id: String,
    side: String,
    recorded_at: DateTime<Utc>,
    metric_count: usize,
    metrics: Value,
}

#[derive(Serialize)]
struct TwinDivergenceSummary {
    span_id: String,
    cycle_id: String,
    metric: String,
    severity: String,
    absolute_delta: f64,
    percent_delta: f64,
    detected_at: DateTime<Utc>,
    physical_span: Option<String>,
    digital_span: Option<String>,
    payload: Value,
}

#[derive(Serialize)]
struct TwinSummary {
    execution_id: String,
    cycles: usize,
    paired_cycles: usize,
    observation_total: usize,
    latest_observation: Option<DateTime<Utc>>,
    divergence_total: usize,
    latest_divergence: Option<DateTime<Utc>>,
    observations: Vec<TwinObservationSummary>,
    divergences: Vec<TwinDivergenceSummary>,
}

struct CyclePresence {
    physical: bool,
    digital: bool,
}

impl CyclePresence {
    fn new() -> Self {
        Self {
            physical: false,
            digital: false,
        }
    }

    fn mark(&mut self, side: &str) {
        if side.eq_ignore_ascii_case("physical") {
            self.physical = true;
        }
        if side.eq_ignore_ascii_case("digital") {
            self.digital = true;
        }
    }
}

struct LedgerState {
    modified: Option<SystemTime>,
    spans: Vec<UniversalSpan>,
}

struct LedgerCache {
    path: PathBuf,
    state: RwLock<LedgerState>,
}

impl LedgerCache {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            state: RwLock::new(LedgerState {
                modified: None,
                spans: Vec::new(),
            }),
        }
    }

    async fn spans(&self) -> Result<Vec<UniversalSpan>> {
        if !tokio::fs::try_exists(&self.path).await? {
            let state = self.state.read().await;
            return Ok(state.spans.clone());
        }

        let metadata = tokio::fs::metadata(&self.path).await?;
        let modified = metadata.modified().unwrap_or(UNIX_EPOCH);

        {
            let state = self.state.read().await;
            if state
                .modified
                .map(|cached| cached == modified)
                .unwrap_or(false)
            {
                return Ok(state.spans.clone());
            }
        }

        let path = self.path.clone();
        let spans = tokio::task::spawn_blocking(move || sync_ledger(path))
            .await
            .map_err(|err| anyhow::anyhow!(err.to_string()))??;

        {
            let mut state = self.state.write().await;
            state.modified = Some(modified);
            state.spans = spans.clone();
        }

        Ok(spans)
    }
}

#[derive(Debug)]
enum AppError {
    NotFound(String),
    Internal(anyhow::Error),
}

impl AppError {
    fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound(message) => (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": message })),
            )
                .into_response(),
            AppError::Internal(err) => {
                warn!(error = %err, "runner_service_error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": "internal error" })),
                )
                    .into_response()
            }
        }
    }
}
