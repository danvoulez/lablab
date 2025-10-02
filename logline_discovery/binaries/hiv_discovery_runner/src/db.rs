use crate::mapping::{
    self, AcquisitionMetadata, AnalysisMetadata, ArtifactMetadata, ExecutionMetadata,
    FrameMetadata, ManuscriptMetadata, MetricMetadata, ProtocolMetadata, ReviewMetadata, SpanKind,
    SubjectMetadata, TwinDivergenceMetadata, TwinObservationMetadata,
};
use anyhow::Result;
use md5;
use spans_core::UniversalSpan;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::types::Json;
use tracing::warn;
use uuid::Uuid;

pub async fn init_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}

pub async fn insert_raw_span(pool: &PgPool, span: &UniversalSpan) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO discovery.raw_spans (span_id, flow, workflow, payload)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (span_id) DO NOTHING
        "#,
    )
    .bind(&span.id.0)
    .bind(&span.flow)
    .bind(&span.workflow)
    .bind(Json(span.payload.clone()))
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn apply_mapping(pool: &PgPool, span: &UniversalSpan, kind: &SpanKind) -> Result<()> {
    match kind {
        SpanKind::Subject(meta) => insert_subject(pool, span, meta).await?,
        SpanKind::Protocol(meta) => insert_protocol(pool, span, meta).await?,
        SpanKind::Execution(meta) => insert_execution(pool, span, meta).await?,
        SpanKind::Acquisition(meta) => insert_acquisition(pool, span, meta).await?,
        SpanKind::Frame(meta) => insert_frame(pool, span, meta).await?,
        SpanKind::Metric(meta) => insert_metric(pool, span, meta).await?,
        SpanKind::Analysis(meta) => insert_analysis(pool, span, meta).await?,
        SpanKind::Review(meta) => insert_review(pool, span, meta).await?,
        SpanKind::Manuscript(meta) => insert_manuscript(pool, span, meta).await?,
        SpanKind::Artifact(meta) => insert_artifact(pool, span, meta).await?,
        SpanKind::TwinObservation(meta) => insert_twin_observation(pool, span, meta).await?,
        SpanKind::TwinDivergence(meta) => insert_twin_divergence(pool, span, meta).await?,
        _ => {}
    }
    Ok(())
}

async fn insert_subject(pool: &PgPool, span: &UniversalSpan, meta: &SubjectMetadata) -> Result<()> {
    let metadata = mapping::metadata_payload(span);

    sqlx::query(
        r#"
        INSERT INTO discovery.runs_subjects (span_id, subject_type, subject_identifier, intent, metadata)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (span_id) DO NOTHING
        "#,
    )
    .bind(&span.id.0)
    .bind(&meta.subject_type)
    .bind(&meta.subject_identifier)
    .bind(&meta.intent)
    .bind(Json(metadata))
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_protocol(
    pool: &PgPool,
    span: &UniversalSpan,
    meta: &ProtocolMetadata,
) -> Result<()> {
    let subject_span_id = mapping::resolve_protocol_subject(meta, span);
    let Some(subject_span_id) = subject_span_id else {
        warn!(span = %span.id.0, "protocol_subject_missing");
        return Ok(());
    };

    let subject_id = match lookup_subject_id(pool, &subject_span_id).await? {
        Some(id) => id,
        None => {
            warn!(span = %span.id.0, subject_span = %subject_span_id, "protocol_subject_not_found");
            return Ok(());
        }
    };

    let checksum = if let Some(existing) = &meta.checksum {
        existing.clone()
    } else {
        let bytes = serde_json::to_vec(&meta.parameters)?;
        let digest = md5::compute(bytes);
        format!("{:x}", digest)
    };

    let metadata = mapping::metadata_payload(span);

    sqlx::query(
        r#"
        INSERT INTO discovery.runs_protocols (span_id, subject_id, recipe_name, parameters, checksum, metadata)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (span_id) DO NOTHING
        "#,
    )
    .bind(&span.id.0)
    .bind(subject_id)
    .bind(&meta.recipe_name)
    .bind(Json(meta.parameters.clone()))
    .bind(&checksum)
    .bind(Json(metadata))
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_execution(
    pool: &PgPool,
    span: &UniversalSpan,
    meta: &ExecutionMetadata,
) -> Result<()> {
    let subject_span_id = mapping::resolve_subject_span(span, meta)?;
    let subject_id = match lookup_subject_id(pool, &subject_span_id).await? {
        Some(id) => id,
        None => {
            warn!(span = %span.id.0, subject_span = %subject_span_id, "execution_subject_not_found");
            return Ok(());
        }
    };

    let protocol_id = match mapping::resolve_protocol_span(span, meta) {
        Some(protocol_span_id) => lookup_protocol_id(pool, &protocol_span_id).await?,
        None => None,
    };

    let status = meta.status.clone().unwrap_or_else(|| "pending".to_string());
    let started_at = meta.started_at.unwrap_or(span.started_at);
    let finished_at = meta.finished_at.or(span.finished_at);
    let metadata = mapping::metadata_payload(span);

    sqlx::query(
        r#"
        INSERT INTO discovery.runs_executions (span_id, subject_id, protocol_id, status, started_at, finished_at, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (span_id) DO NOTHING
        "#,
    )
    .bind(&span.id.0)
    .bind(subject_id)
    .bind(protocol_id)
    .bind(&status)
    .bind(started_at)
    .bind(finished_at)
    .bind(Json(metadata))
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_acquisition(
    pool: &PgPool,
    span: &UniversalSpan,
    meta: &AcquisitionMetadata,
) -> Result<()> {
    let Some(execution_id) =
        resolve_execution_id(pool, span, &meta.execution_span, "acquisition").await?
    else {
        return Ok(());
    };

    sqlx::query(
        r#"
        INSERT INTO discovery.runs_acquisitions (span_id, execution_id, source, artifact_type, artifact_reference, metadata)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (span_id) DO NOTHING
        "#,
    )
    .bind(&span.id.0)
    .bind(execution_id)
    .bind(&meta.source)
    .bind(&meta.artifact_type)
    .bind(&meta.artifact_reference)
    .bind(Json(mapping::metadata_payload(span)))
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_frame(pool: &PgPool, span: &UniversalSpan, meta: &FrameMetadata) -> Result<()> {
    let Some(execution_id) =
        resolve_execution_id(pool, span, &meta.execution_span, "frame").await?
    else {
        return Ok(());
    };

    let recorded_at = meta.recorded_at.unwrap_or(span.started_at);

    sqlx::query(
        r#"
        INSERT INTO discovery.runs_frames (span_id, execution_id, frame_index, recorded_at, payload)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (span_id) DO NOTHING
        "#,
    )
    .bind(&span.id.0)
    .bind(execution_id)
    .bind(meta.frame_index)
    .bind(recorded_at)
    .bind(Json(meta.payload.clone()))
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_metric(pool: &PgPool, span: &UniversalSpan, meta: &MetricMetadata) -> Result<()> {
    let Some(execution_id) =
        resolve_execution_id(pool, span, &meta.execution_span, "metric").await?
    else {
        return Ok(());
    };

    let recorded_at = meta.recorded_at.unwrap_or(span.started_at);

    sqlx::query(
        r#"
        INSERT INTO discovery.runs_metrics (span_id, execution_id, metric_name, value, unit, recorded_at, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (span_id) DO NOTHING
        "#,
    )
    .bind(&span.id.0)
    .bind(execution_id)
    .bind(&meta.metric_name)
    .bind(meta.value)
    .bind(&meta.unit)
    .bind(recorded_at)
    .bind(Json(meta.metadata.clone()))
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_analysis(
    pool: &PgPool,
    span: &UniversalSpan,
    meta: &AnalysisMetadata,
) -> Result<()> {
    let Some(execution_id) =
        resolve_execution_id(pool, span, &meta.execution_span, "analysis").await?
    else {
        return Ok(());
    };

    sqlx::query(
        r#"
        INSERT INTO discovery.runs_analysis (span_id, execution_id, analysis_type, summary)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (span_id) DO NOTHING
        "#,
    )
    .bind(&span.id.0)
    .bind(execution_id)
    .bind(&meta.analysis_type)
    .bind(Json(meta.summary.clone()))
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_review(pool: &PgPool, span: &UniversalSpan, meta: &ReviewMetadata) -> Result<()> {
    let Some(execution_id) =
        resolve_execution_id(pool, span, &meta.execution_span, "review").await?
    else {
        return Ok(());
    };

    sqlx::query(
        r#"
        INSERT INTO discovery.runs_reviews (span_id, execution_id, reviewer, verdict, notes)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (span_id) DO NOTHING
        "#,
    )
    .bind(&span.id.0)
    .bind(execution_id)
    .bind(&meta.reviewer)
    .bind(&meta.verdict)
    .bind(&meta.notes)
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_manuscript(
    pool: &PgPool,
    span: &UniversalSpan,
    meta: &ManuscriptMetadata,
) -> Result<()> {
    let Some(execution_id) =
        resolve_execution_id(pool, span, &meta.execution_span, "manuscript").await?
    else {
        return Ok(());
    };

    sqlx::query(
        r#"
        INSERT INTO discovery.runs_manuscripts (span_id, execution_id, title, format, storage_path, checksum, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (span_id) DO NOTHING
        "#,
    )
    .bind(&span.id.0)
    .bind(execution_id)
    .bind(&meta.title)
    .bind(&meta.format)
    .bind(&meta.storage_path)
    .bind(&meta.checksum)
    .bind(Json(meta.metadata.clone()))
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_artifact(
    pool: &PgPool,
    span: &UniversalSpan,
    meta: &ArtifactMetadata,
) -> Result<()> {
    let Some(execution_id) =
        resolve_execution_id(pool, span, &meta.execution_span, "artifact").await?
    else {
        return Ok(());
    };

    sqlx::query(
        r#"
        INSERT INTO discovery.runs_artifacts (span_id, execution_id, artifact_kind, storage_path, checksum)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (span_id) DO NOTHING
        "#,
    )
    .bind(&span.id.0)
    .bind(execution_id)
    .bind(&meta.artifact_kind)
    .bind(&meta.storage_path)
    .bind(&meta.checksum)
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_twin_observation(
    pool: &PgPool,
    span: &UniversalSpan,
    meta: &TwinObservationMetadata,
) -> Result<()> {
    let execution_id =
        resolve_execution_id(pool, span, &meta.execution_span, "twin_observation").await?;

    sqlx::query(
        r#"
        INSERT INTO discovery.twin_observations (
            span_id, execution_id, cycle_id, side, recorded_at, metrics, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (span_id) DO NOTHING
        "#,
    )
    .bind(&span.id.0)
    .bind(execution_id)
    .bind(&meta.cycle_id)
    .bind(&meta.side)
    .bind(meta.recorded_at)
    .bind(Json(meta.metrics.clone()))
    .bind(Json(span.payload.clone()))
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_twin_divergence(
    pool: &PgPool,
    span: &UniversalSpan,
    meta: &TwinDivergenceMetadata,
) -> Result<()> {
    let execution_id =
        resolve_execution_id(pool, span, &meta.execution_span, "twin_divergence").await?;

    sqlx::query(
        r#"
        INSERT INTO discovery.twin_divergences (
            span_id, execution_id, cycle_id, metric, severity, absolute_delta,
            percent_delta, detected_at, physical_span, digital_span, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ON CONFLICT (span_id) DO NOTHING
        "#,
    )
    .bind(&span.id.0)
    .bind(execution_id)
    .bind(&meta.cycle_id)
    .bind(&meta.metric)
    .bind(&meta.severity)
    .bind(meta.absolute_delta)
    .bind(meta.percent_delta)
    .bind(meta.detected_at)
    .bind(&meta.physical_span)
    .bind(&meta.digital_span)
    .bind(Json(meta.payload.clone()))
    .execute(pool)
    .await?;

    Ok(())
}

async fn lookup_subject_id(pool: &PgPool, span_id: &str) -> Result<Option<Uuid>> {
    let id =
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM discovery.runs_subjects WHERE span_id = $1")
            .bind(span_id)
            .fetch_optional(pool)
            .await?;
    Ok(id)
}

async fn lookup_protocol_id(pool: &PgPool, span_id: &str) -> Result<Option<Uuid>> {
    let id =
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM discovery.runs_protocols WHERE span_id = $1")
            .bind(span_id)
            .fetch_optional(pool)
            .await?;
    Ok(id)
}

async fn lookup_execution_id(pool: &PgPool, span_id: &str) -> Result<Option<Uuid>> {
    let id = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM discovery.runs_executions WHERE span_id = $1",
    )
    .bind(span_id)
    .fetch_optional(pool)
    .await?;
    Ok(id)
}

async fn resolve_execution_id(
    pool: &PgPool,
    span: &UniversalSpan,
    execution_span: &Option<String>,
    context: &str,
) -> Result<Option<Uuid>> {
    let Some(reference_span) = mapping::resolve_execution_reference(span, execution_span) else {
        warn!(span = %span.id.0, context = context, "execution_reference_missing");
        return Ok(None);
    };

    let execution_id = lookup_execution_id(pool, &reference_span).await?;
    if execution_id.is_none() {
        warn!(span = %span.id.0, ref_span = %reference_span, context = context, "execution_reference_not_found");
    }

    Ok(execution_id)
}
