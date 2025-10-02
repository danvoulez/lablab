# Discovery Lab Task Backlog

## Phase 1 — Data Plumbing (Complete)
- [x] Define Postgres schema (subjects, protocols, acquisitions, frames, metrics, analyses, reviews, manuscripts).
- [x] Write Diesel/sqlx migrations that enforce insert-only semantics via triggers or permissions.
- [x] Implement `span_ingestor` adapter for LogLine Fold outputs (read sample simulation JSON → emit `UniversalSpan`).
- [x] Provide sample ingestion scripts and structured spans for local verification.
- [x] Create CLI command `hiv_discovery_runner ingest <path>` to append spans to NDJSON and Postgres in lockstep.
- [ ] Mirror existing Warp `data/spans.ndjson` records into the new ledger directory for regression tests.
- [x] Add configuration module for database DSN, ledger paths, and environment overrides.
- [x] Classify spans (`subject_init`, `protocol_config`, `execution_run`, etc.) and populate domain tables.

## Phase 2 — Automation Backbone
- [x] Build a directory watcher/service that tails Fold span outputs and enqueues ingestion automatically.
- [x] Provide runnable manuscript job (CLI + script) for scheduled publishing.
- [x] Add scheduling harness (cron/systemd templates) to run analytics/manuscript jobs on cadence.
- [x] Emit operational metrics/logging for 24/7 monitoring and alerting.
- [ ] Mirror historical Warp NDJSON into the new ledger for regression tests. (ready to run via `cargo run -p hiv_discovery_runner -- sync-ledger --path /path/to/warp/spans.ndjson` once the dataset is mounted)
- [x] Enhance digital twin bridge with configurable cycles and summaries to feed automation loops.

## Phase 3 — Analytics Core
- [x] Hook real folding trajectories into `folding_runtime::FoldingAnalysis::derives` and add unit tests with fixtures.
- [x] Port causal rule definitions from transcript (temporal, resource contention, structural similarity, cascade failure).
- [x] Integrate structural similarity engines: graphlet kernel (petgraph), Mapper (TDA pipeline), ST-GNN (PyTorch subprocess).
- [x] Calibrate confidence scores and expose parameters via config.
- [ ] Add benchmarks for causal inference graph construction with realistic span counts.
- [x] Implement `cargo` tests covering ingestion → analysis pipeline.

## Phase 4 — Orchestration & Contracts
- [x] Expand `hiv_discovery_runner` into service mode (REST/gRPC) with span ingestion endpoints.
- [x] Implement contract emission (`.lll`) per run and store hashes in Postgres + NDJSON.
- [x] Integrate Warp rollback/checkpoint commands into runner (CLI subcommands).
- [ ] Add manuscript templates (Markdown + LaTeX) with placeholders for analytics tables and figures. _(Markdown generator shipped; LaTeX renderer pending.)_
- [x] Persist generated manuscripts into `runs_manuscripts` / ledger spans (contract written to contracts/manuscripts/).
- [ ] Add citation + attachment support (BibTeX, figures, data links) to manuscript bundles.
- [ ] Automate generation of `artifact_manifest` spans after each run.
- [ ] Build pipeline scripts to package NDJSON + DB snapshot + contracts for reviewers.
- [x] Wire `digital_twin_bridge::emit_divergence_span` outputs into the runner so twin drift spans persist automatically (handled via twin coordinator in `hiv_discovery_runner`).
- [x] Expose `/executions/{id}/twin` service endpoint summarising twin observations/divergences.
- [x] Ship turnkey scripts (`setup.sh`, `run_lab.sh`, `auto_backfill.sh`) that configure env vars, launch watcher/service/manuscript jobs, and backfill ledgers.
- [x] Add CLI commands `diagnose` and `quickstart` for environment checks and demo ingestion.
- [x] Integrate Warp ledger utilities (`ledger_vault`, `span_fabricator`) into the unified workspace.
- [x] Import Fold core crates (contracts + folding kernels) and expose them via the shared CLI.
- [ ] Integrate Fold physics bridge (OpenMM) and advanced ruleset validation into the unified CLI.
- [ ] Evaluate and port remaining Fold tooling (benchmarks, UI assets) as optional modules.

## Phase 5 — Experience Layer
- [ ] Scaffold `discovery_dashboard` with Vite/Vue based on Fold’s UI kit. (current Axum prototype serves the Causal Chain Explorer; next step is porting Fold’s Vue shell)
- [x] Build "Causal Chain Explorer" view backed by the service API.
- [ ] Implement Playground mode views: span timeline, digital twin 3D, live metrics.
- [ ] Implement Publication mode views: condensed tables, confidence intervals, export buttons.
- [ ] Wire SSE/WebSocket feeds from orchestrator to dashboard components.
- [ ] Surface manuscript previews/downloads inside the dashboard (Markdown + JSON sidecar).
- [ ] Add audit trail overlays linking UI elements to span IDs and raw data downloads.
- [ ] Conduct initial usability review with scientists; track feedback.

## Phase 6 — Validation & Release
- [ ] Run full gp41 scenario end-to-end; capture gaps and remediation tasks.
- [ ] Establish regression suite (unit + integration + property tests for spans).
- [ ] Document reproducibility playbook (commands, environment setup, data provenance).
- [ ] Prepare release notes + roadmap retrospective.
- [ ] Finalize bootstrap scripts for new lab deployments.
- [ ] Submit compliance package (NDJSON, SQL export, contracts, manuscript hash) for archival.

## Cross-Cutting
- [ ] Set up CI pipeline (GitHub Actions) running fmt, clippy, tests, custom validators.
- [ ] Create local crates.io mirror or vendor dependencies for offline builds.
- [ ] Implement telemetry/tracing sinks and dashboards for orchestrator & scheduled jobs.
- [ ] Maintain HIV research notebooks with citation tracking and sync updates to spans.
- [ ] Define security/access policies for Postgres and ledger assets.
- [ ] Schedule recurring updates to `docs/master_plan.md` and this backlog.
- [ ] Add integration tests covering the service API + manuscript pipeline end-to-end.
- [ ] Document Stage 0 inventory results and migrate remaining Warp/Fold tasks into this backlog.
