# LogLine Discovery Lab — Master Plan

## North Star
Deliver a joyful, Tesla-grade discovery lab that logs every scientific action as reproducible evidence, impressing reviewers while delighting day-to-day explorers. The lab operates continuously—scheduled jobs ingest new simulations, run analytics, and publish manuscripts around the clock—so every cycle ends with a manuscript-ready artifact backed by append-only ledgers and institutional-grade audit trails.

## Pillars of Excellence
- **Scientific Credibility**: All spans, protocols, and artifacts are persisted in both Postgres (insert-only tables) and append-only NDJSON. Contracts (`.lll`) notarize missions, and every dashboard view links back to raw data and provenance hashes.
- **Playful Mastery**: The dashboard reuses LogLine Fold’s cinematic UI, layering animated timelines, manipulable embeddings, and digital twin scenes without sacrificing rigor.
- **Integrated Intelligence**: Warp’s runtime, Fold’s physics engine, causal inference, and manuscript generation cooperate via the shared `UniversalSpan` schema (`logline_discovery/crates/spans_core/src/lib.rs`).

## System Architecture Snapshot
1. **Runtime Spine (Warp)** — Provides span ledger semantics, rollback, and compliance tooling. Discovery Lab components append to Warp-compatible NDJSON and optionally submit contracts.
2. **Scientific Engine (Fold)** — Supplies protein folding simulations, PWA visual components, and verified pipelines. Outputs stream into `span_ingestor` (manually or via watchers) for standardization.
3. **Discovery Workspace** — Cargo workspace with crates for ingestion, analytics, causal inference, manuscript compilation, scheduled orchestration, and CLI/service binaries (`logline_discovery/binaries/hiv_discovery_runner`).
4. **Automation Layer** — Watchers and schedulers (cron/systemd/k8s) monitor Fold output, queue ingestion tasks, trigger analytics, emit manuscripts/contracts, and keep twin cycles analysed via the `digital_twin_bridge` config/summary interface.
5. **Data Stores**
   - `ledger/spans/*.ndjson`: immutable audit log for every event (subject selection → manuscript).
   - Postgres schema (`runs.*` tables, to be provisioned): insert-only mirror enabling high-speed queries and joins.
5. **Experience Layer** — `discovery_dashboard` (Vue/PWA) delivers dual modes: Playground (animated, exploratory) and Publication (serious reviewer layout with statistical context).

## End-to-End Workflow (Subject → Manuscript)
1. **Subject Initiation** — Scientist chooses protein/sample and intent. Emit `subject_init` span + Postgres record (`runs.subjects`).
2. **Protocol Design** — Select simulation recipe, parameters, and checks. Capture `protocol_config` span (hashed payload). Store in `runs.protocols`.
3. **Input Acquisition** — Fetch structural data via Fold pipelines; log `acquisition` spans, record provenance metadata. Integrity checks appended as `qc` spans.
4. **Simulation Execution** — Run folding/twin cycles. Runtime metrics stream into NDJSON ledger and Postgres (`runs.frames`, `runs.metrics`). Automation catches new runs and schedules downstream analytics.
5. **Analytics Pass** — `folding_runtime`, `structural_similarity`, and `causal_engine` produce computed spans (`analysis` type) plus summary tables (`runs.analysis`). Jobs can run immediately (event-driven) or on schedule for batch comparisons.
6. **Review & Sign-Off** — Investigators or scheduled QC agents record annotations and approvals; results in `review` spans and Postgres `runs.reviews` rows.
7. **Manuscript Assembly** — `manuscript_generator` crafts Markdown/PDF/`.lll`, capturing hash + storage path in `runs.manuscripts`. Continuous jobs can publish updates when thresholds are met.
8. **Archive & Release** — Package NDJSON log, DB export, manuscripts, and contracts for replication; optional Warp submission for notarization. Automated bundlers run nightly/weekly.

## Roadmap (9-Week Horizon)
- **Phase 0 · Baseline (Complete)**
  - Cargo workspace scaffolded with core crates and CLI runner (`cargo check` passes).
  - Integration map + README published.
- **Phase 1 · Data Plumbing (Complete)**
  - Fold adapters, ingestion scripts, schema, and mapping to Postgres tables delivered.
  - Continuous ingestion ready via CLI/scripts (`ingest`, `watch` subcommands).
- **Phase 2 · Automation Backbone (Week 2)**
  - Directory watcher/queue to auto-ingest Fold output (shipped via `hiv_discovery_runner watch`).
  - Add scheduler hooks (cron/systemd/k8s job manifests) for recurring analytics/manuscript generation.
  - Provide status dashboards/logging for 24/7 operations (watcher emits `ops_monitoring` spans with processed/errors/duration).
- **Phase 3 · Analytics Core (Weeks 3–4)**
  - Finalize folding analysis using actual trajectories.
  - Implement complete causal rule system (temporal, resource contention, structural similarity) with confidence calibration.
  - Connect ST-GNN runner and Mapper graph pipelines.
- **Phase 4 · Orchestration & Contracts (Weeks 5–6)**
  - Expand `hiv_discovery_runner` into service/API for programmatic ingestion.
  - Auto-generate contracts for each run; integrate rollback/checkpoint flows from Warp.
  - Add manuscript templates tuned for HIV case study.
- **Phase 5 · Experience Layer (Weeks 7–8)**
  - Port Fold UI to `discovery_dashboard` with Playground/Publication modes.
  - Visualize spans, twin cycles, causal chains, and statistics with direct data links.
  - Implement live SSE/WebSocket feeds for twin divergence alerts.
- **Phase 6 · Validation & Release (Weeks 9–10)**
  - Full gp41 scenario dry run from subject selection to manuscript.
  - Regression + property-based tests for ingestion, analytics, causal inference.
  - Package bootstrap scripts, documentation, and reproducibility bundles.

## Cross-Cutting Tracks
- **Infrastructure & Quality**: CI with `cargo fmt`, `clippy`, tests; optional local crates mirror; observability via tracing/log aggregation for scheduled jobs.
- **Research Knowledge Base**: Maintain `notebooks/` with HIV landscape, hypothesis graphs, literature references. Sync updates to spans.
- **Governance & Security**: Enforce schema versioning, cryptographic hashes, and access controls for Postgres and ledger artifacts. Provide reviewer-mode exports (LaTeX, NDJSON, SQL snapshot). Monitor automated jobs for compliance/anomaly detection.
- **Operational Readiness**: Define job scheduling strategy (cron/systemd/k8s), set up failure alerting, and document recovery playbooks for 24/7 operations.

## Immediate Actions
1. Backfill historical Warp NDJSON via the new `sync-ledger` command so the automation layer has realistic depth.
2. Stand up the "Causal Chain Explorer" UI slice atop the service API, validating the joyful workflow.
3. Stress-test the manuscript pipeline against real executions and extend it with citation/attachment hooks (LaTeX remains future work).
4. Continue Playground/Publication design prototypes leveraging Fold’s UI assets.

This master plan stays living in `docs/master_plan.md`; update it as milestones complete or priorities shift.
