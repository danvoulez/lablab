use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use causal_engine::CausalEngine;
use chrono::{DateTime, Utc};
use folding_runtime::FoldingAnalysis;
use manuscript_generator::{EnhancedManuscript, ManuscriptBuilder, render_markdown_from};
use serde_json::{json, Value};
use spans_core::{SpanId, UniversalSpan};
use sqlx::postgres::PgPool;
use sqlx::{FromRow, Row};
use tracing::{info, warn};
use uuid::Uuid;

use crate::config::RunnerConfig;
use crate::db::{apply_mapping, init_pool, insert_raw_span};
use crate::ledger::append_span;
use crate::mapping;

pub async fn run(
    execution_span: Option<String>,
    output: PathBuf,
    cfg: &RunnerConfig,
) -> Result<()> {
    let database_url = cfg
        .database_url
        .as_ref()
        .ok_or_else(|| anyhow!("DATABASE_URL must be set for manuscript generation"))?
        .clone();

    let pool = init_pool(&database_url).await?;
    let ctx = ManuscriptContext::load(execution_span, &pool).await?;

    let mut engine = CausalEngine::new();
    engine.ingest(ctx.spans.clone());
    let causal = engine.infer();
    let causal_json = serde_json::to_value(&causal)?;

    let protocol_json = ctx.protocol_json();
    let analysis_json = ctx.analysis_json();

    let title = ctx.title();
    let abstract_text = ctx.abstract_text();
    let keywords = ctx.keywords();

    let mut builder = ManuscriptBuilder::new(title.clone(), ctx.execution.span_id.clone())
        .with_authors(vec!["LogLine Discovery Lab".to_string()])
        .with_abstract(abstract_text.clone())
        .with_keywords(keywords);

    builder.add_methods_section(&protocol_json);
    builder.add_results_section(&analysis_json, &causal_json);

    let manuscript = builder.build();
    let markdown = render_markdown_from(&manuscript);
    let json_blob = serde_json::to_vec_pretty(&manuscript)?;

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    let extension = output
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_lowercase);

    let mut markdown_path: Option<PathBuf> = None;
    let json_path: Option<PathBuf>;

    match extension.as_deref() {
        Some("json") => {
            fs::write(&output, &json_blob)?;
            info!(path = %output.display(), "manuscript_json_written");
            json_path = Some(output.clone());
        }
        Some("md") => {
            fs::write(&output, markdown.as_bytes())?;
            info!(path = %output.display(), "manuscript_markdown_written");
            markdown_path = Some(output.clone());
            let sidecar = replace_extension(&output, "json");
            fs::write(&sidecar, &json_blob)?;
            info!(path = %sidecar.display(), "manuscript_json_sidecar_written");
            json_path = Some(sidecar);
        }
        _ => {
            fs::write(&output, markdown.as_bytes())?;
            info!(path = %output.display(), "manuscript_markdown_written");
            markdown_path = Some(output.clone());
            let sidecar = replace_extension(&output, "json");
            fs::write(&sidecar, &json_blob)?;
            info!(path = %sidecar.display(), "manuscript_json_sidecar_written");
            json_path = Some(sidecar);
        }
    }

    persist_manuscript(
        &manuscript,
        markdown_path.as_ref().map(|p| p.as_path()),
        json_path.as_ref().map(|p| p.as_path()),
        &ctx,
        cfg,
        &pool,
    )
    .await?;

    Ok(())
}

fn replace_extension(path: &Path, ext: &str) -> PathBuf {
    let mut new_path = path.to_path_buf();
    new_path.set_extension(ext);
    new_path
}

struct ManuscriptContext {
    execution: ExecutionRow,
    subject: SubjectRow,
    protocol: Option<ProtocolRow>,
    analysis: FoldingAnalysis,
    metrics: Vec<MetricRow>,
    spans: Vec<UniversalSpan>,
}

impl ManuscriptContext {
    async fn load(span_id: Option<String>, pool: &PgPool) -> Result<Self> {
        let execution = fetch_execution(span_id, pool).await?;
        let subject = fetch_subject(execution.subject_id, pool).await?;
        let protocol = match execution.protocol_id {
            Some(protocol_id) => Some(fetch_protocol(protocol_id, pool).await?),
            None => None,
        };

        let analysis = fetch_folding_analysis(&execution, pool).await?;
        let metrics = fetch_metrics(execution.id, pool).await?;
        let span_ids = collect_span_ids(&execution, &subject, protocol.as_ref(), pool).await?;
        let spans = fetch_spans(span_ids, pool).await?;

        Ok(Self {
            execution,
            subject,
            protocol,
            analysis,
            metrics,
            spans,
        })
    }

    fn title(&self) -> String {
        format!(
            "Discovery Report — {} {}",
            self.subject.subject_type, self.subject.subject_identifier
        )
    }

    fn abstract_text(&self) -> String {
        let duration = self
            .execution
            .finished_at
            .map(|finished| finished - self.execution.started_at)
            .map(|delta| format!("{} s", delta.num_seconds()))
            .unwrap_or_else(|| "ongoing".to_string());

        let protocol = self
            .protocol
            .as_ref()
            .map(|p| p.recipe_name.as_str())
            .unwrap_or("unspecified protocol");

        let intent = self.subject.intent.as_deref().unwrap_or("exploratory run");

        format!(
            "Execution {} ({}) evaluated {} {} using {}. Mean potential energy {:.2} kcal/mol, maximum RMSD {:.2} Å, unstable={}. Runtime: {}.",
            self.execution.span_id,
            intent,
            self.subject.subject_type,
            self.subject.subject_identifier,
            protocol,
            self.analysis.mean_energy,
            self.analysis.max_rmsd,
            self.analysis.unstable,
            duration,
        )
    }

    fn keywords(&self) -> Vec<String> {
        vec![
            self.subject.subject_type.clone(),
            "molecular dynamics".to_string(),
            "causal inference".to_string(),
        ]
    }

    fn protocol_json(&self) -> Value {
        json!({
            "recipe_name": self.protocol.as_ref().map(|p| p.recipe_name.clone()),
        })
    }

    fn analysis_json(&self) -> Value {
        let stability = if self.analysis.unstable {
            "unstable"
        } else {
            "stable"
        };

        let energy_series: Vec<Value> = self
            .metrics
            .iter()
            .filter(|metric| metric.metric_name.eq_ignore_ascii_case("potential_energy"))
            .filter_map(|metric| metric.value.map(Value::from))
            .collect();

        json!({
            "mean_energy": self.analysis.mean_energy,
            "max_rmsd": self.analysis.max_rmsd,
            "unstable": self.analysis.unstable,
            "stability": stability,
            "energy_trajectory": energy_series,
        })
    }
}

#[derive(Clone, Debug, FromRow)]
struct ExecutionRow {
    id: Uuid,
    span_id: String,
    subject_id: Uuid,
    protocol_id: Option<Uuid>,
    _status: String,
    started_at: DateTime<Utc>,
    finished_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
struct SubjectRow {
    _id: Uuid,
    span_id: String,
    subject_type: String,
    subject_identifier: String,
    intent: Option<String>,
}

#[derive(Clone, Debug, FromRow)]
struct ProtocolRow {
    _id: Uuid,
    span_id: String,
    recipe_name: String,
}

#[derive(Clone, Debug, FromRow)]
struct MetricRow {
    id: Uuid,
    span_id: String,
    metric_name: String,
    value: Option<f64>,
    unit: Option<String>,
    recorded_at: DateTime<Utc>,
}

async fn fetch_execution(span: Option<String>, pool: &PgPool) -> Result<ExecutionRow> {
    let stmt = if let Some(span_id) = span {
        sqlx::query_as::<_, ExecutionRow>("SELECT id, span_id, subject_id, protocol_id, status, started_at, finished_at FROM discovery.runs_executions WHERE span_id = $1 LIMIT 1")
            .bind(span_id)
    } else {
        sqlx::query_as::<_, ExecutionRow>("SELECT id, span_id, subject_id, protocol_id, status, started_at, finished_at FROM discovery.runs_executions ORDER BY created_at DESC LIMIT 1")
    };

    stmt.fetch_optional(pool)
        .await?
        .ok_or_else(|| anyhow!("no executions found"))
}

async fn fetch_subject(subject_id: Uuid, pool: &PgPool) -> Result<SubjectRow> {
    sqlx::query_as::<_, SubjectRow>(
        "SELECT id, span_id, subject_type, subject_identifier, intent FROM discovery.runs_subjects WHERE id = $1",
    )
    .bind(subject_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("subject missing for execution"))
}

async fn fetch_protocol(protocol_id: Uuid, pool: &PgPool) -> Result<ProtocolRow> {
    sqlx::query_as::<_, ProtocolRow>(
        "SELECT id, span_id, recipe_name FROM discovery.runs_protocols WHERE id = $1",
    )
    .bind(protocol_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("protocol missing for execution"))
}

async fn fetch_folding_analysis(
    execution: &ExecutionRow,
    pool: &PgPool,
) -> Result<FoldingAnalysis> {
    let summary: Option<sqlx::types::Json<Value>> = sqlx::query_scalar(
        "SELECT summary FROM discovery.runs_analysis WHERE execution_id = $1 ORDER BY created_at DESC LIMIT 1",
    )
    .bind(execution.id)
    .fetch_optional(pool)
    .await?;

    let summary = summary.map(|json| json.0).ok_or_else(|| {
        anyhow!(
            "analysis summary missing for execution {}",
            execution.span_id
        )
    })?;

    let mean_energy = summary
        .get("mean_energy")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| anyhow!("analysis summary missing mean_energy"))?;

    let max_rmsd = summary
        .get("max_rmsd")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| anyhow!("analysis summary missing max_rmsd"))?;

    let unstable = summary
        .get("unstable")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let protein = summary
        .get("protein")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    Ok(FoldingAnalysis {
        span_id: SpanId::new(execution.span_id.clone()),
        protein,
        mean_energy,
        max_rmsd,
        unstable,
        window: (execution.started_at, execution.finished_at),
    })
}

async fn fetch_metrics(execution_id: Uuid, pool: &PgPool) -> Result<Vec<MetricRow>> {
    let rows = sqlx::query_as::<_, MetricRow>("SELECT span_id, metric_name, value, unit, recorded_at FROM discovery.runs_metrics WHERE execution_id = $1 ORDER BY recorded_at")
        .bind(execution_id)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

async fn collect_span_ids(
    execution: &ExecutionRow,
    subject: &SubjectRow,
    protocol: Option<&ProtocolRow>,
    pool: &PgPool,
) -> Result<HashSet<String>> {
    let mut ids: HashSet<String> = HashSet::new();
    ids.insert(execution.span_id.clone());
    ids.insert(subject.span_id.clone());
    if let Some(protocol) = protocol {
        ids.insert(protocol.span_id.clone());
    }

    ids.extend(fetch_child_span_ids("runs_metrics", execution.id, pool).await?);
    ids.extend(fetch_child_span_ids("runs_analysis", execution.id, pool).await?);
    ids.extend(fetch_child_span_ids("runs_acquisitions", execution.id, pool).await?);
    ids.extend(fetch_child_span_ids("runs_frames", execution.id, pool).await?);
    ids.extend(fetch_child_span_ids("runs_reviews", execution.id, pool).await?);
    ids.extend(fetch_child_span_ids("runs_manuscripts", execution.id, pool).await?);
    ids.extend(fetch_child_span_ids("runs_artifacts", execution.id, pool).await?);

    Ok(ids)
}

async fn fetch_child_span_ids(
    table: &str,
    execution_id: Uuid,
    pool: &PgPool,
) -> Result<HashSet<String>> {
    let query = format!(
        "SELECT span_id FROM discovery.{} WHERE execution_id = $1",
        table
    );
    let rows = sqlx::query(&query)
        .bind(execution_id)
        .fetch_all(pool)
        .await?;

    let mut set = HashSet::new();
    for row in rows {
        let span_id: String = row.try_get("span_id")?;
        set.insert(span_id);
    }
    Ok(set)
}

async fn fetch_spans(ids: HashSet<String>, pool: &PgPool) -> Result<Vec<UniversalSpan>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let id_list: Vec<String> = ids.into_iter().collect();
    let rows = sqlx::query_as::<_, (String, sqlx::types::Json<Value>)>(
        "SELECT span_id, payload FROM discovery.raw_spans WHERE span_id = ANY($1)",
    )
    .bind(&id_list)
    .fetch_all(pool)
    .await?;

    let mut spans = Vec::new();
    for (span_id, payload) in rows {
        match spans_core::span_from_json(payload.0.clone()) {
            Ok(span) => spans.push(span),
            Err(err) => warn!(span_id, error = %err, "failed_to_parse_span"),
        }
    }

    spans.sort_by_key(|span| span.started_at);
    Ok(spans)
}

async fn persist_manuscript(
    manuscript: &EnhancedManuscript,
    markdown_path: Option<&Path>,
    json_path: Option<&Path>,
    ctx: &ManuscriptContext,
    cfg: &RunnerConfig,
    pool: &PgPool,
) -> Result<()> {
    let storage_path = markdown_path
        .or(json_path)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "manuscripts/latest".to_string());

    let contract_path = contract_destination(Path::new(&storage_path));
    let checksum = &manuscript.metadata.checksum;

    let artifacts = json!({
        "markdown": markdown_path.map(|p| p.to_string_lossy().to_string()),
        "json": json_path.map(|p| p.to_string_lossy().to_string()),
        "contract": contract_path.to_string_lossy().to_string(),
    });

    let span_id = format!("span::manuscript::{}", Uuid::new_v4());
    let timestamp = manuscript.metadata.generated_at;

    let payload = json!({
        "span_id": &span_id,
        "name": &manuscript.title,
        "flow": "manuscript",
        "workflow": &ctx.execution.span_id,
        "parent_id": &ctx.execution.span_id,
        "started_at": timestamp.to_rfc3339(),
        "finished_at": timestamp.to_rfc3339(),
        "metadata": {
            "execution_span": &ctx.execution.span_id,
            "title": &manuscript.title,
            "storage_path": storage_path,
            "checksum": checksum,
            "artifacts": artifacts,
        }
    });

    let span = spans_core::span_from_json(payload)?;

    append_span(&cfg.ledger_path, &span)?;
    insert_raw_span(pool, &span).await?;
    let kind = mapping::classify_span(&span);
    apply_mapping(pool, &span, &kind).await?;
    info!(span = %span_id, "manuscript_persisted");

    write_contract(
        manuscript,
        &ctx.execution.span_id,
        &span_id,
        &storage_path,
        contract_path,
        checksum,
    )?;
    Ok(())
}

fn contract_destination(output: &Path) -> PathBuf {
    let file_name = output
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("manuscript");
    let mut sanitized = file_name.replace(':', "_");
    if sanitized.is_empty() {
        sanitized = "manuscript".into();
    }
    PathBuf::from("contracts/manuscripts").join(format!("{}.lll", sanitized))
}

fn write_contract(
    manuscript: &EnhancedManuscript,
    execution_span: &str,
    manuscript_span: &str,
    storage_path: &str,
    contract_path: PathBuf,
    checksum: &str,
) -> Result<()> {
    let directory = contract_path
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("contracts/manuscripts"));
    fs::create_dir_all(&directory)?;
    let content = format!(
        "contract manuscript_bundle {{\n  version: \"1.0\"\n  manuscript_span: \"{}\"\n  execution_span: \"{}\"\n  title: \"{}\"\n  storage_path: \"{}\"\n  checksum_md5: \"{}\"\n  generated_at: \"{}\"\n}}\n",
        manuscript_span,
        execution_span,
        manuscript.title.replace('"', "'"),
        storage_path,
        checksum,
        manuscript.metadata.generated_at.to_rfc3339()
    );
    fs::write(&contract_path, content)?;
    info!(path = %contract_path.display(), "contract_emitted");
    Ok(())
}

// The rest of the helper functions (collect_span_ids, etc.) remain unchanged above.
