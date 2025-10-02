# LogLine Discovery Lab

LogLine Discovery Lab is the integration workspace that fuses LogLine Warp's runtime spine and LogLine Fold's scientific engine into a computable HIV discovery pipeline.

## Workspace Layout

- `crates/`
  - `spans_core` — shared span schema compatible with Warp append-only ledgers.
  - `span_ingestor` — adapters that normalize external payloads into `UniversalSpan` records.
  - `folding_runtime` — local analyses over protein-folding spans (mean energy, RMSD, instability flag).
  - `structural_similarity` — placeholders for graphlet, Mapper, and ST-GNN similarity engines.
  - `causal_engine` — cross-domain causal inference graph (temporal MVP shipped, extended rules planned).
  - `digital_twin_bridge` — bidirectional twin cycle recorder, ready to tie into Warp's twin controllers.
  - `manuscript_generator` — bundles analyses and causal insights into manuscript-ready payloads.
  - `discovery_agent` — orchestrates spans + analyses into hypotheses and manuscript drafts.
- `binaries/`
  - `hiv_discovery_runner` — Rust CLI that ingests a folding span, runs the discovery agent, and emits a manuscript bundle stub.
  - `discovery_dashboard/` — placeholder for the Vue/PWA UI (to reuse LogLine Fold visual components).
- `ledger/spans/folding/` — seed spans (`gp41_folding.json`) ready for ingestion tests.
- `contracts/` — mission contracts aligned with Warp (`hiv_cure_mission.lll`, `reservoir_map_experiment.lll`).
- `notebooks/` — research context and hypothesis graph notebooks (placeholders for now).
- `docs/` — cross-repo capability mapping (`docs/integration_map.md`).

## How Warp & Fold Plug In

- **Span Lifecycle**: `span_ingestor` converts Fold outputs (or Warp spans) into the shared schema. These records can be appended to Warp's ledger verbatim.
- **Folding Analytics**: `folding_runtime` consumes spans emitted via Fold simulations. Its outputs feed the agent and manuscript generator.
- **Causal Correlation**: `causal_engine` implements the temporal rule-set captured in the Chat dump; structural similarity hooks are wired for future integration with Fold's embeddings.
- **Digital Twin**: `digital_twin_bridge` mirrors Warp's twin controllers, records cycles via `TwinSyncCycle`, produces `TwinSummary` snapshots, and is configurable through `SyncConfig` (max divergences, thresholds, auto-reconcile toggles). The runner now pairs physical/digital `twin_observation` spans and emits normalized `twin_divergence` spans when drift exceeds tolerance, persisting them alongside the source observations.
- **Orchestration**: `hiv_discovery_runner` is the first CLI proving that spans → analysis → manuscript loop executes end-to-end.

## Immediate Next Steps

1. **Wire Real Fold Data**: implement adapters that read `LogLine Fold/Backend` outputs and emit spans via `span_ingestor`.
2. **Ledger Integration**: point the runner at Warp's `data/spans.ndjson` (or expose the same format) for persistence.
3. **Enrich Causal Rules**: port the remaining rule system from the transcript (resource contention, structural similarity joins).
4. **Dashboard Bootstrap**: clone Fold's frontend scaffold into `binaries/discovery_dashboard` and point it at the runner or a new HTTP service.
5. **Testing/CI**: once network access is available, add unit tests for span parsing, folding analytics, and causal inference.

## Running Locally

```bash
cd logline_discovery
cargo run -p hiv_discovery_runner
```

### One-Button Flow

```bash
# from repo root
./scripts/setup.sh           # creates .env, ledger, and applies migrations (if psql available)
./scripts/run_lab.sh         # launches watcher + service API with sensible defaults
```

`./scripts/run_lab.sh` tails the watcher/service inside the same terminal; press `Ctrl+C` to stop. Use `./scripts/auto_backfill.sh /path/to/spans.ndjson` to replay an existing Warp ledger before or after the watcher runs.

### Ingest Fold Simulation Output

```bash
# assumes DATABASE_URL/LEDGER_PATH configured or .env present
cargo run -p hiv_discovery_runner -- ingest ../Library/Mobile\ Documents/com~apple~CloudDocs/LogLine\ Fold/Backend/spans/chignolin_loglineforce/span.json

# optional: ingest structured subject/protocol/execution spans
../scripts/ingest_chignolin_pipeline.sh
```

The runner auto-detects LogLine Fold payloads, sets the correct `flow`/`workflow`, extracts execution timestamps, and writes both the append-only NDJSON ledger and normalized Postgres rows (`runs_subjects`, `runs_protocols`, `runs_executions`).

### Continuous Watcher

Keep the lab running 24/7 by polling a directory for new spans:

```bash
cargo run -p hiv_discovery_runner -- watch --path ../Library/Mobile\ Documents/com~apple~CloudDocs/LogLine\ Fold/Backend/spans --recursive --interval 30
```

The watcher scans for `.json` payloads on the given interval (seconds), auto-detects Fold output, and pipes everything through the ingestion pipeline.
Each cycle emits an `ops_monitoring` span recording processed count, error count, and duration so the ledger/Postgres mirror expose operational health.

### Scheduled Manuscript Job

Generate manuscript bundles (for cron/systemd jobs) using the latest execution data:

```bash
./scripts/run_manuscript_job.sh

# or manually
cargo run -p hiv_discovery_runner -- manuscript --output manuscripts/latest.md
```

By default the command selects the most recent execution span; pass `--execution-span <span_id>` to target a specific run.
When targeting a Markdown path (`.md`), the runner writes a publication-ready report and drops a JSON sidecar with the structured bundle. Supplying a `.json` output path preserves the previous behaviour.
Every invocation also appends a `manuscript` span to the ledger and mirrors it into Postgres (`runs_manuscripts`), so artifacts stay traceable without extra steps.

### Quickstart Demo

```bash
cargo run -p hiv_discovery_runner -- diagnose
cargo run -p hiv_discovery_runner -- quickstart --output manuscripts/quickstart.md

# Toy folding engine demo
cargo run -p hiv_discovery_runner -- folding-demo

# Execute a custom folding contract
cargo run -p hiv_discovery_runner -- fold --contract contracts/folding/demo_fold.lll --output tmp/demo_fold.json

# Ledger operations (rollback/checkpoint)
cargo run -p hiv_discovery_runner -- ledger status
cargo run -p hiv_discovery_runner -- ledger rollback --span-id span::execution::demo
cargo run -p hiv_discovery_runner -- ledger checkpoint --label demo
cargo run -p hiv_discovery_runner -- ledger rollback-checkpoint --label demo
```

`diagnose` verifies your `.env`, ledger path, and database connectivity. `quickstart` ingests the bundled chignolin spans, generates a manuscript, and persists the results so you can inspect the end-to-end loop instantly.
`folding-demo` spins up the simplified folding engine from the fused LogLine Folding stack, executes a baked-in demo contract, and prints the resulting trajectory/energy summary.
`fold` lets you point at any `.lll` contract; the command runs the engine and writes a JSON report (see `docs/fold_contracts.md`).
Each run also records a `folding_report` span + contract in the ledger for auditability.
Ledger subcommands reuse the imported Warp ledger vault utilities; see `docs/ledger_workflow.md`.

Each manuscript run also emits a `.lll` contract under `contracts/manuscripts/`, capturing the title, checksum, and execution span for reproducibility proofs.

> **Note:** Building the workspace currently requires crates.io access. Network calls are blocked in this environment; run the command once you have connectivity or vendor the dependencies.
### Causal CLI Demo
Analyze causal relationships in span payloads:

```bash
cargo run -p hiv_discovery_runner -- causal \
  --input samples/payloads/causal_demo.json \
  --output tmp/causal_output.json
```

This runs the upgraded `causal_engine` over a span array, applying temporal, resource, structural, and failure rules. Review the JSON output for hypotheses and narratives.

You can tune causal thresholds programmatically via `CausalEngine::with_config` (e.g., adjust temporal windows or structural similarity cutoffs) before wiring the engine into other pipelines.

### Ledger Sync
Backfill legacy NDJSON ledgers directly into the Discovery workspace:

```bash
cargo run -p hiv_discovery_runner -- sync-ledger --path /path/to/warp/spans.ndjson
```

Each line must be a JSON span in the UniversalSpan schema; the command replays spans through the same ingestion pipeline, updating both NDJSON and Postgres mirrors.

### Service Mode API
Expose an HTTP API that surfaces executions, their spans, and causal chains—perfect for dashboards or automation hooks:

```bash
cargo run -p hiv_discovery_runner -- serve --address 127.0.0.1:4040
```

Endpoints:

- `GET /health` — simple readiness probe.
- `GET /executions` — lists execution spans with workflow, status, and timing metadata.
- `GET /executions/{span_id}/spans` — returns all spans associated with the selected execution (metrics, analyses, manuscripts, etc.).
- `GET /executions/{span_id}/causal` — runs the causal engine over the execution’s spans and returns confidence-weighted hypotheses.

The server reads directly from `LEDGER_PATH`, reloading spans when the underlying NDJSON changes.

### Causal Chain Explorer

```bash
cargo run -p discovery_dashboard
```

The dashboard starts on `127.0.0.1:4600` (override via `DISCOVERY_DASHBOARD_ADDR`). It proxies to the service API—ensure `hiv_discovery_runner -- serve` is running—and renders execution lists, spans, and causal hypotheses in the browser. Point it at a different service host with `DISCOVERY_SERVICE_URL=http://host:port cargo run -p discovery_dashboard`.
