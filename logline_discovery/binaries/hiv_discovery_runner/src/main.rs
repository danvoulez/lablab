mod commands;
mod config;
mod db;
mod ledger;
mod manuscript;
mod mapping;
mod pipeline;
mod service;
mod twin;
mod triage;

use anyhow::{self, Result};
use logline_common::{triage::make_plan_from_json, Error};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration as StdDuration, Instant, SystemTime};

use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use commands::{run_causal_analysis, sync_ledger};
use config::RunnerConfig;
use db::{apply_mapping, init_pool, insert_raw_span};
use discovery_agent::DiscoveryAgent;
use folding_core::{
    ContractInstruction, FoldingContract, FoldingEngineBuilder, MicroOscillator, PhysicsLevel,
    Ruleset,
};
use folding_molecule::{AminoAcid, EnergyModel, PeptideChain, Residue, ResidueId};
use folding_runtime::FoldingAnalysis;
use folding_time::RotationClock;
use ledger::append_span;
use mapping::classify_span;
use serde_json::{json, Value};
use span_ingestor::{ingest_fold_json, ingest_json, IngestOptions};
use spans_core::{span_from_json, UniversalSpan};
use sqlx::postgres::PgPool;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};
use uuid::Uuid;
use walkdir::WalkDir;
use warp_common::{new_audit_id, ContractEnvelope, ContractVersion};
use warp_ledger_vault::runtime::SpanLog;

#[derive(Parser, Debug)]
#[command(author, version, about = "LogLine HIV Discovery Runner", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Ingest a span payload into the ledger (NDJSON) and Postgres
    Ingest {
        /// Path to a JSON file containing a span payload
        path: PathBuf,
        /// Override flow if not present in payload
        #[arg(long)]
        flow: Option<String>,
        /// Override workflow if not present in payload
        #[arg(long)]
        workflow: Option<String>,
    },
    /// Analyze causal relationships within a span payload JSON file
    Causal {
        /// Path to JSON file containing an array of spans in UniversalSpan schema
        #[arg(long)]
        input: PathBuf,
        /// Optional path to write the chain output as JSON (stdout otherwise)
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Watch a directory and ingest new/updated span files on the fly
    Watch {
        /// Directory to monitor for span payloads
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Recursively watch subdirectories
        #[arg(long)]
        recursive: bool,
        /// Polling interval in seconds
        #[arg(long, default_value_t = 60)]
        interval: u64,
        /// Override flow for all files (otherwise auto-detected)
        #[arg(long)]
        flow: Option<String>,
        /// Override workflow for all files (otherwise auto-detected)
        #[arg(long)]
        workflow: Option<String>,
    },
    /// Generate a manuscript bundle from the latest (or specified) execution
    Manuscript {
        /// Execution span ID to materialize (defaults to most recent execution)
        #[arg(long)]
        execution_span: Option<String>,
        /// Output path for the generated manuscript bundle (`.md` for Markdown, `.json` to keep the serialized bundle)
        #[arg(long)]
        output: PathBuf,
    },
    /// Mirror existing NDJSON ledger entries into the local workspace
    SyncLedger {
        /// Path to NDJSON file (one span per line)
        #[arg(long)]
        path: PathBuf,
    },
    /// Execute an `.lll` folding contract and emit a report
    Fold {
        /// Path to folding contract file
        #[arg(long)]
        contract: PathBuf,
        /// Output path for the JSON report
        #[arg(long, default_value = "tmp/folding_report.json")]
        output: PathBuf,
    },
    /// Serve an HTTP API for dashboards and automation hooks
    Serve {
        /// Address to bind (e.g. 127.0.0.1:4040)
        #[arg(long, default_value = "127.0.0.1:4040")]
        address: SocketAddr,
    },
    /// Ledger utilities (rollback, checkpoint) backed by Warp ledger vault
    Ledger {
        #[command(subcommand)]
        command: LedgerCommand,
    },
    /// Diagnose environment readiness (database, ledger paths, scripts)
    Diagnose,
    /// Ingest demo spans and generate a manuscript bundle automatically
    Quickstart {
        /// Output path for the generated manuscript bundle
        #[arg(long, default_value = "manuscripts/quickstart.md")]
        output: PathBuf,
    },
    /// Run a toy folding engine demonstration and print the summary
    FoldingDemo,
    /// Run the demo notebook pipeline to produce a manuscript bundle
    Demo,
    /// Process recently modified spans in the ledger (for event-driven processing)
    ProcessRecent {
        /// Only process spans modified since this timestamp (ISO 8601)
        #[arg(long)]
        since: Option<String>,
        /// Maximum number of spans to process
        #[arg(long, default_value_t = 100)]
        limit: usize,
    },
}

#[derive(Subcommand, Debug)]
enum LedgerCommand {
    /// Count entries in the ledger
    Status,
    /// Roll back to a given span ID
    Rollback {
        #[arg(long)]
        span_id: String,
    },
    /// Create a checkpoint with the provided label
    Checkpoint {
        #[arg(long)]
        label: String,
    },
    /// Roll back to a previously created checkpoint label
    RollbackCheckpoint {
        #[arg(long)]
        label: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    let cfg = RunnerConfig::load()?;

    match cli.command {
        Command::Ingest {
            path,
            flow,
            workflow,
        } => {
            handle_ingest(path, flow, workflow, &cfg).await?;
        }
        Command::Watch {
            path,
            recursive,
            interval,
            flow,
            workflow,
        } => {
            handle_watch(path, recursive, interval, flow, workflow, &cfg).await?;
        }
        Command::Manuscript {
            execution_span,
            output,
        } => {
            manuscript::run(execution_span, output, &cfg).await?;
        }
        Command::Causal { input, output } => {
            handle_causal(input, output).await?;
        }
        Command::SyncLedger { path } => {
            handle_sync_ledger(path, &cfg).await?;
        }
        Command::Serve { address } => {
            handle_serve(address, cfg.clone()).await?;
        }
        Command::Fold { contract, output } => {
            handle_fold_contract(contract, output, &cfg).await?;
        }
        Command::Ledger { command } => {
            handle_ledger_command(command, &cfg).await?;
        }
        Command::Diagnose => {
            handle_diagnose(&cfg).await?;
        }
        Command::Quickstart { output } => {
            handle_quickstart(output, &cfg).await?;
        }
        Command::FoldingDemo => {
            handle_folding_demo()?;
        }
        Command::Demo => {
            run_demo().await?;
        }
        Command::ProcessRecent { since, limit } => {
            handle_process_recent(since, limit, &cfg).await?;
        }
    }

    Ok(())
}

async fn handle_ingest(
    path: PathBuf,
    flow: Option<String>,
    workflow: Option<String>,
    cfg: &RunnerConfig,
) -> Result<()> {
    info!(?path, "ingest_start");
    let pool = if let Some(db_url) = &cfg.database_url {
        Some(Arc::new(init_pool(db_url).await?))
    } else {
        None
    };

    let span_id = ingest_path(&path, flow, workflow, cfg, pool).await?;
    info!(span = %span_id, "ingest_complete");

    Ok(())
}

async fn handle_watch(
    directory: PathBuf,
    recursive: bool,
    interval_secs: u64,
    flow: Option<String>,
    workflow: Option<String>,
    cfg: &RunnerConfig,
) -> Result<()> {
    let watch_root = directory.canonicalize().unwrap_or(directory);
    if !watch_root.exists() {
        anyhow::bail!("watch path does not exist: {}", watch_root.display());
    }

    info!(?watch_root, recursive, interval_secs, "watch_start");

    let pool = if let Some(db_url) = &cfg.database_url {
        Some(Arc::new(init_pool(db_url).await?))
    } else {
        None
    };

    let mut seen: HashMap<PathBuf, SystemTime> = HashMap::new();
    let max_depth = if recursive { std::usize::MAX } else { 1 };
    let interval = Duration::from_secs(interval_secs.max(1));

    loop {
        let cycle_start = Instant::now();
        let mut processed = 0u32;
        let mut errors = 0u32;
        for entry in WalkDir::new(&watch_root)
            .max_depth(max_depth)
            .min_depth(0)
            .follow_links(false)
        {
            let entry = match entry {
                Ok(e) => e,
                Err(err) => {
                    warn!(error = %err, "watch_walk_error");
                    continue;
                }
            };

            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            if !is_supported_file(path) {
                continue;
            }

            let metadata = match std::fs::metadata(path) {
                Ok(m) => m,
                Err(err) => {
                    warn!(?path, error = %err, "watch_metadata_error");
                    continue;
                }
            };

            let modified = metadata.modified().ok();
            let should_process = match seen.get(path) {
                Some(prev) => modified.map(|m| m > *prev).unwrap_or(false),
                None => true,
            };

            if !should_process {
                continue;
            }

            match ingest_path(path, flow.clone(), workflow.clone(), cfg, pool.clone()).await {
                Ok(span_id) => {
                    info!(?path, span = %span_id, "watch_ingest_success");
                    if let Some(m) = modified {
                        seen.insert(path.to_path_buf(), m);
                    }
                    processed += 1;
                }
                Err(err) => {
                    warn!(?path, error = %err, "watch_ingest_failed");
                    errors += 1;
                }
            }
        }

        let duration = cycle_start.elapsed();
        if let Err(err) =
            emit_cycle_metrics(cfg, pool.clone(), processed, errors, duration, &watch_root).await
        {
            warn!(error = %err, "emit_cycle_metrics_failed");
        }

        info!(
            processed,
            errors,
            duration_ms = duration.as_millis() as u64,
            "watch_cycle_complete"
        );
        sleep(interval).await;
    }
}

async fn run_demo() -> Result<()> {
    info!("runner_demo_start" = %Utc::now());

    let sample_payload = serde_json::json!({
        "span_id": "span::folding::gp41::001",
        "name": "gp41 folding",
        "flow": "protein_folding",
        "workflow": "hiv_reservoir_mapping",
        "protein": "gp41",
        "timestamp": "2025-09-29T20:00:00Z"
    });

    let span = ingest_json(
        sample_payload,
        &IngestOptions::new("protein_folding", "hiv_reservoir_mapping", Utc::now()),
    )?;

    let analysis = FoldingAnalysis {
        span_id: span.id.clone(),
        protein: "gp41".into(),
        mean_energy: -123.0,
        max_rmsd: 6.1,
        unstable: true,
        window: (span.started_at, span.finished_at),
    };

    let manuscript = DiscoveryAgent::orchestrate(vec![span], vec![analysis])?;
    println!("Generated manuscript bundle: {}", manuscript.title);

    Ok(())
}

async fn ingest_path(
    path: &Path,
    flow: Option<String>,
    workflow: Option<String>,
    cfg: &RunnerConfig,
    pool: Option<Arc<PgPool>>,
) -> Result<String> {
    let raw = std::fs::read_to_string(path)?;
    let json_payload: Value = serde_json::from_str(&raw)?;
    let is_fold_payload = json_payload.get("protein_metadata").is_some();

    let effective_flow = flow.or_else(|| {
        if is_fold_payload {
            Some("protein_folding".to_string())
        } else {
            None
        }
    });

    let flow_value = effective_flow.unwrap_or_else(|| "unknown_flow".to_string());

    let effective_workflow = workflow.or_else(|| {
        if is_fold_payload {
            Some("fold_pipeline".to_string())
        } else {
            None
        }
    });

    let workflow_value = effective_workflow.unwrap_or_else(|| "unspecified".to_string());

    let ingest_opts = IngestOptions::new(flow_value, workflow_value, Utc::now());
    let span = if is_fold_payload {
        ingest_fold_json(json_payload, &ingest_opts)?
    } else {
        ingest_json(json_payload, &ingest_opts)?
    };

    process_span(span, cfg, pool).await
}

async fn handle_sync_ledger(path: PathBuf, cfg: &RunnerConfig) -> Result<()> {
    let spans = sync_ledger(path.clone())?;
    let pool = if let Some(db_url) = &cfg.database_url {
        Some(Arc::new(init_pool(db_url).await?))
    } else {
        None
    };

    let mut count = 0usize;
    for span in spans {
        process_span(span, cfg, pool.clone()).await?;
        count += 1;
    }
    info!(?path, count, "sync_ledger_complete");
    println!("Ingested {} spans", count);
    Ok(())
}

async fn handle_serve(address: SocketAddr, cfg: RunnerConfig) -> Result<()> {
    service::serve(cfg, address).await
}

async fn handle_causal(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    let chains = run_causal_analysis(input)?;
    if let Some(path) = output {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let file = File::create(&path)?;
        serde_json::to_writer_pretty(file, &chains)?;
        println!("Causal chains written to {}", path.display());
    } else {
        println!("{}", serde_json::to_string_pretty(&chains)?);
    }
    Ok(())
}

async fn handle_fold_contract(
    contract_path: PathBuf,
    output_path: PathBuf,
    cfg: &RunnerConfig,
) -> Result<()> {
    if !contract_path.exists() {
        return Err(Error::Validation(format!(
            "Contract file not found: {}",
            contract_path.display()
        ))
        .into());
    }

    let contract_text = fs::read_to_string(&contract_path)?;
    let contract = parse_contract(&contract_text);
    let mut engine = build_demo_engine();
    let report = engine.execute_contract(&contract);

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let summary = summarize_execution_report(&report);
    fs::write(
        &output_path,
        serde_json::to_string_pretty(&summary)?,
    )?;

    println!(
        "Executed folding contract {} → {}",
        contract_path.display(),
        output_path.display()
    );

    if cfg.database_url.is_none() {
        warn!("DATABASE_URL not set; skipping Postgres persistence");
    }
    let pool = if let Some(db_url) = &cfg.database_url {
        Some(Arc::new(init_pool(db_url).await?))
    } else {
        None
    };

    persist_folding_report(&report, &contract_path, &output_path, cfg, pool).await?;

    Ok(())
}

async fn handle_diagnose(cfg: &RunnerConfig) -> Result<()> {
    println!("LogLine Discovery Lab — Diagnose");

    let ledger_path = cfg
        .ledger_path
        .canonicalize()
        .unwrap_or_else(|_| cfg.ledger_path.clone());
    if ledger_path.exists() {
        println!("✓ Ledger path reachable: {}", ledger_path.display());
    } else {
        println!(
            "✗ Ledger path missing ({}). Run scripts/setup.sh",
            ledger_path.display()
        );
    }

    if let Some(db_url) = &cfg.database_url {
        match init_pool(db_url).await {
            Ok(pool) => {
                println!("✓ Database reachable: {}", db_url);
                let exists: Option<String> =
                    sqlx::query_scalar("SELECT to_regclass('discovery.raw_spans')::text")
                        .fetch_optional(&pool)
                        .await?;
                if exists.is_some() {
                    println!("✓ Migration applied (discovery.raw_spans present)");
                } else {
                    println!(
                        "✗ Discovery schema missing. Re-run scripts/setup.sh or psql migration"
                    );
                }
            }
            Err(err) => {
                println!("✗ Database check failed: {err}");
            }
        }
    } else {
        println!("! DATABASE_URL not set. Add it to .env or run scripts/setup.sh");
    }

    for script in [
        "scripts/setup.sh",
        "scripts/run_lab.sh",
        "scripts/auto_backfill.sh",
    ] {
        if PathBuf::from(script).exists() {
            println!("✓ Found {script}");
        } else {
            println!("✗ Missing {script}");
        }
    }

    Ok(())
}

async fn handle_quickstart(output: PathBuf, cfg: &RunnerConfig) -> Result<()> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let samples_dir = manifest_dir.join("../../samples/spans");
    if !samples_dir.exists() {
        return Err(Error::Validation(format!(
            "Samples directory missing: {}",
            samples_dir.display()
        ))
        .into());
    }

    println!(
        "Quickstart — ingesting demo spans from {}",
        samples_dir.display()
    );

    let pool = if let Some(db_url) = &cfg.database_url {
        Some(Arc::new(init_pool(db_url).await?))
    } else {
        None
    };

    let files = [
        "subject_chignolin.json",
        "protocol_chignolin.json",
        "execution_chignolin.json",
        "metric_chignolin.json",
        "analysis_chignolin.json",
        "manuscript_chignolin.json",
        "artifact_bundle.json",
    ];

    for file in files {
        let path = samples_dir.join(file);
        ingest_path(&path, None, None, cfg, pool.clone()).await?;
        println!("✓ Ingested {}", file);
    }

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    println!("Generating manuscript bundle at {}", output.display());
    manuscript::run(None, output.clone(), cfg).await?;
    println!("✓ Manuscript bundle generated at {}", output.display());

    Ok(())
}

async fn handle_process_recent(
    since: Option<String>,
    limit: usize,
    cfg: &RunnerConfig,
) -> Result<()> {
    if limit == 0 {
        println!("No spans processed (limit = 0)");
        return Ok(());
    }

    let since_filter = if let Some(since) = since {
        Some(
            since
                .parse::<DateTime<Utc>>()
                .map_err(|err| Error::Validation(format!("Invalid --since timestamp: {err}")))?,
        )
    } else {
        None
    };

    if !cfg.ledger_path.exists() {
        return Err(Error::Validation(format!(
            "Ledger path missing: {}",
            cfg.ledger_path.display()
        ))
        .into());
    }

    let pool = if let Some(db_url) = &cfg.database_url {
        Some(Arc::new(init_pool(db_url).await?))
    } else {
        None
    };

    let file = File::open(&cfg.ledger_path)?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<Value>(&line) {
            Ok(value) => entries.push(value),
            Err(err) => {
                warn!(error = %err, "ledger_line_parse_failed");
            }
        }
    }

    let mut processed = 0usize;
    let mut duplicates = 0usize;
    let mut seen = HashSet::new();

    for entry in entries.into_iter().rev() {
        if processed >= limit {
            break;
        }

        let span_json = match entry.get("type").and_then(|v| v.as_str()) {
            Some("span") => entry.get("span").cloned(),
            Some(_) => None,
            None => Some(entry.clone()),
        };

        let Some(span_json) = span_json else {
            continue;
        };

        let span = match span_from_json(span_json) {
            Ok(span) => span,
            Err(err) => {
                warn!(error = %err, "ledger_span_parse_failed");
                continue;
            }
        };

        if let Some(threshold) = since_filter {
            if span.started_at < threshold {
                continue;
            }
        }

        if !seen.insert(span.id.0.clone()) {
            duplicates += 1;
            continue;
        }

        process_span(span, cfg, pool.clone()).await?;
        processed += 1;
    }

    println!(
        "Processed {} spans, skipped {} (already processed)",
        processed, duplicates
    );

    Ok(())
}

fn is_supported_file(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case("json") => true,
        Some(ext) if ext.eq_ignore_ascii_case("ndjson") => true,
        _ => false,
    }
}

async fn handle_ledger_command(command: LedgerCommand, cfg: &RunnerConfig) -> Result<()> {
    let log = SpanLog::new(&cfg.ledger_path)?;
    match command {
        LedgerCommand::Status => {
            let count = log.count()?;
            println!("Ledger entries: {count}");
        }
        LedgerCommand::Rollback { span_id } => {
            let (reversed, contract) = log.rollback_to(&span_id)?;
            println!(
                "Rolled back to {span_id}; {reversed} spans marked reversed (contract {})",
                contract.audit_id.0
            );
        }
        LedgerCommand::Checkpoint { label } => {
            let contract = log.checkpoint_create(&label)?;
            println!("Checkpoint '{label}' created ({})", contract.audit_id.0);
        }
        LedgerCommand::RollbackCheckpoint { label } => {
            let (reversed, contract) = log.rollback_to_checkpoint(&label)?;
            println!(
                "Rolled back to checkpoint '{label}'; {reversed} spans marked reversed (contract {})",
                contract.audit_id.0
            );
        }
    }
    Ok(())
}

fn parse_contract(text: &str) -> FoldingContract {
    let instructions: Vec<_> = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    FoldingContract::from_lines(&instructions)
}

fn build_demo_engine() -> folding_core::FoldingEngine {
    let chain = PeptideChain::new(vec![
        Residue::new(ResidueId(1), AminoAcid::Alanine).with_position([0.0, 0.0, 0.0]),
        Residue::new(ResidueId(2), AminoAcid::Serine).with_position([1.6, 0.0, 0.1]),
        Residue::new(ResidueId(3), AminoAcid::Valine).with_position([3.2, 0.5, -0.1]),
    ]);

    FoldingEngineBuilder::new()
        .with_chain(chain)
        .with_energy_model(EnergyModel::default())
        .with_oscillator(MicroOscillator::new(6.0, 0.4))
        .with_clock(RotationClock::new(5))
        .with_ruleset(Ruleset::default())
        .with_rng_seed(42)
        .with_physics_level(PhysicsLevel::Toy)
        .build()
}

fn handle_folding_demo() -> Result<()> {
    println!("Running folding demo with synthetic contract ...");

    let chain = PeptideChain::new(vec![
        Residue::new(ResidueId(1), AminoAcid::Alanine).with_position([0.0, 0.0, 0.0]),
        Residue::new(ResidueId(2), AminoAcid::Serine).with_position([1.6, 0.0, 0.1]),
        Residue::new(ResidueId(3), AminoAcid::Valine).with_position([3.2, 0.5, -0.1]),
    ]);

    let energy_model = EnergyModel::default();
    let contract = FoldingContract::new(vec![
        ContractInstruction::SpanAlias("tilt_alpha".into()),
        ContractInstruction::Rotate {
            residue: ResidueId(2),
            angle_degrees: 28.0,
            duration_ms: 6,
        },
        ContractInstruction::ClashCheck,
        ContractInstruction::Commit,
        ContractInstruction::SpanAlias("tilt_beta".into()),
        ContractInstruction::Rotate {
            residue: ResidueId(3),
            angle_degrees: -18.0,
            duration_ms: 8,
        },
        ContractInstruction::Commit,
    ]);

    let mut engine = FoldingEngineBuilder::new()
        .with_chain(chain)
        .with_energy_model(energy_model)
        .with_oscillator(MicroOscillator::new(6.0, 0.4))
        .with_clock(RotationClock::new(5))
        .with_ruleset(Ruleset::default())
        .with_rng_seed(42)
        .with_physics_level(PhysicsLevel::Toy)
        .build();

    let report = engine.execute_contract(&contract);

    let summary = summarize_execution_report(&report);
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

fn summarize_execution_report(report: &folding_core::ExecutionReport) -> Value {
    let applied: Vec<_> = report
        .applied_rotations
        .iter()
        .map(|rotation| {
            json!({
                "angle_degrees": rotation.applied_angle,
                "span_id": rotation.span_record.id.as_str(),
                "duration_ms": rotation.span_record.duration.as_millis(),
                "delta_entropy": rotation.span_record.delta_entropy,
                "delta_information": rotation.span_record.delta_information,
            })
        })
        .collect();

    let trajectory: Vec<_> = report
        .trajectory
        .spans()
        .iter()
        .map(|span| {
            json!({
                "span_id": span.id.as_str(),
                "delta_entropy": span.delta_entropy,
                "delta_information": span.delta_information,
                "delta_theta": span.delta_theta,
                "duration_ms": span.duration.as_millis(),
            })
        })
        .collect();

    let physics_spans: Vec<_> = report
        .physics_span_metrics
        .iter()
        .map(|entry| {
            json!({
                "span_id": entry.span_id,
                "rmsd": entry.metrics.rmsd,
                "radius_of_gyration": entry.metrics.radius_of_gyration,
                "potential_energy": entry.metrics.potential_energy,
                "kinetic_energy": entry.metrics.kinetic_energy,
                "temperature": entry.metrics.temperature,
                "simulation_time_ps": entry.metrics.simulation_time_ps,
                "trajectory_path": entry.metrics.trajectory_path,
            })
        })
        .collect();

    let rejections: Vec<String> = report
        .rejections
        .iter()
        .map(|rej| format!("{:?}", rej))
        .collect();

    json!({
        "final_energy": {
            "potential": report.final_energy.total_potential,
            "kinetic": report.final_energy.total_kinetic,
        },
        "metropolis": {
            "accepted": report.metropolis_stats.accepted,
            "rejected": report.metropolis_stats.rejected,
            "acceptance_rate": report.metropolis_stats.acceptance_rate(),
        },
        "physics_level": format!("{:?}", report.physics_level),
        "applied_rotations": applied,
        "ghost_rotations": report.ghost_rotations.len(),
        "rejections": rejections,
        "trajectory_spans": trajectory,
        "physics_spans": physics_spans,
        "domains": report.domains.len(),
        "chaperones": report.chaperone_requirements.len(),
        "modifications": report.modifications.len(),
    })
}

async fn process_span(
    span: UniversalSpan,
    cfg: &RunnerConfig,
    pool: Option<Arc<PgPool>>,
) -> Result<String> {
    let span_kind = classify_span(&span);

    // 🧠 INTELLIGENT TRIAGE: Decide which analyses to run based on configurable rules
    match make_plan_from_json(&span.payload) {
        Ok(plan) => {
            info!(
                span_id = %span.id.0,
                stages = ?plan.stages,
                "triage_plan_generated"
            );

            // 🚀 EXECUTE PIPELINE: Run the planned stages
            let orchestrator = pipeline::PipelineOrchestrator::new()?;
            orchestrator.orchestrate_span(&span).await?;
        }
        Err(err) => {
            warn!(
                span_id = %span.id.0,
                error = %err,
                "triage_failed_fallback_to_basic_processing"
            );

            // Fallback to basic processing if triage fails
            append_span(&cfg.ledger_path, &span)?;
            info!(ledger = ?cfg.ledger_path, span = %span.id.0, "ledger_append_success");

            if let Some(pool_arc) = &pool {
                insert_raw_span(pool_arc, &span).await?;
                apply_mapping(pool_arc, &span, &span_kind).await?;
                info!(span = %span.id.0, "db_ingest_success");
            } else {
                warn!(span = %span.id.0, "DATABASE_URL not set; skipping Postgres mirror");
            }
        }
    }

    let extra_spans = twin::handle_twin_observation(&span).await?;

    for extra in extra_spans {
        let extra_kind = classify_span(&extra);
        append_span(&cfg.ledger_path, &extra)?;
        if let Some(pool_arc) = &pool {
            insert_raw_span(pool_arc, &extra).await?;
            apply_mapping(pool_arc, &extra, &extra_kind).await?;
            info!(span = %extra.id.0, "twin_divergence_db_ingest_success");
        } else {
            warn!(span = %extra.id.0, "DATABASE_URL not set; skipping Postgres mirror");
        }
        info!(span = %extra.id.0, "twin_divergence_persisted");
    }

    Ok(span.id.0.clone())
}

async fn emit_cycle_metrics(
    cfg: &RunnerConfig,
    pool: Option<Arc<PgPool>>,
    processed: u32,
    errors: u32,
    duration: StdDuration,
    root: &Path,
) -> Result<()> {
    let span = UniversalSpan::new(
        format!("span::ops::watcher::{}", Uuid::new_v4()),
        "watcher cycle",
        "ops_monitoring",
        "ingest_watcher",
        Utc::now(),
        json!({
            "processed": processed,
            "errors": errors,
            "duration_ms": duration.as_millis() as u64,
            "root": root.display().to_string(),
        }),
    );

    process_span(span, cfg, pool).await.map(|_| ())
}

async fn persist_folding_report(
    report: &folding_core::ExecutionReport,
    contract_path: &Path,
    output_path: &Path,
    cfg: &RunnerConfig,
    pool: Option<Arc<PgPool>>,
) -> Result<()> {
    let span_id = format!("span::folding_report::{}", Uuid::new_v4());
    let payload = json!({
        "contract_path": contract_path.display().to_string(),
        "output_path": output_path.display().to_string(),
        "final_energy": {
            "potential": report.final_energy.total_potential,
            "kinetic": report.final_energy.total_kinetic,
        },
        "applied_rotations": report.applied_rotations.len(),
        "ghost_rotations": report.ghost_rotations.len(),
        "rejections": report
            .rejections
            .iter()
            .map(|r| format!("{:?}", r))
            .collect::<Vec<_>>(),
        "metropolis": {
            "accepted": report.metropolis_stats.accepted,
            "rejected": report.metropolis_stats.rejected,
            "acceptance_rate": report.metropolis_stats.acceptance_rate(),
        },
    });

    let span = UniversalSpan::new(
        span_id.clone(),
        "folding report",
        "folding_report",
        "fold_contract",
        Utc::now(),
        payload,
    );

    process_span(span, cfg, pool.clone()).await?;

    let contract = ContractEnvelope {
        version: ContractVersion("1.0".into()),
        audit_id: new_audit_id(),
        kind: "folding_report".into(),
        payload: serde_json::json!({
            "contract_path": contract_path.display().to_string(),
            "output_path": output_path.display().to_string(),
            "generated_at": Utc::now().to_rfc3339(),
            "span_id": span_id,
        }),
    };

    let log = SpanLog::new(&cfg.ledger_path)?;
    log.append_contract(&contract)?;
    Ok(())
}
