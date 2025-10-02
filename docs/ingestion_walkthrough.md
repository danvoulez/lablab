# Ingestion Walkthrough

This walkthrough demonstrates how to bootstrap the Postgres schema, ingest sample spans (Fold payload + supporting metadata), and inspect the resulting tables.

## 1. Prepare Postgres
```bash
# Easiest path
./scripts/setup.sh

# Manual alternative
createdb logline_discovery
psql logline_discovery < db/migrations/0001_init.sql
```

## 2. Configure the Runner
Create `.env` inside `logline_discovery/`:
```
DATABASE_URL=postgres://localhost/logline_discovery
LEDGER_PATH=ledger/spans/discovery.ndjson
```

## 3. Ingest Sample Data
Two data sources are provided:
- **Fold payload** (`LogLine Fold/Backend/spans/chignolin_loglineforce/span.json`) — real simulation output auto-detected by the runner.
- **Structured spans** (`logline_discovery/samples/spans/*.json`) — subject/protocol/execution/metric/etc. to exercise the mapping pipeline.

Recommended flow:
```bash
# From repo root
./scripts/ingest_gp41.sh                      # seed example span
cargo run -p hiv_discovery_runner -- ingest "${HOME}/Library/Mobile Documents/com~apple~CloudDocs/LogLine Fold/Backend/spans/chignolin_loglineforce/span.json"
./scripts/ingest_chignolin_pipeline.sh        # subject → artifact structured spans

# Optional: keep polling the Fold spans directory for new files every 30 seconds
cargo run -p hiv_discovery_runner -- watch --path "${HOME}/Library/Mobile Documents/com~apple~CloudDocs/LogLine Fold/Backend/spans" --recursive --interval 30

# Optional: generate a manuscript bundle (suitable for cron/systemd jobs)
./scripts/run_manuscript_job.sh
# produces `manuscripts/latest.md` plus a JSON sidecar capturing the structured bundle
# ledger + Postgres automatically receive a new `manuscript` span for auditability
# a `.lll` contract is written to contracts/manuscripts/ for reproducibility

# Zero-touch demo
cargo run -p hiv_discovery_runner -- quickstart --output manuscripts/quickstart.md
```

## 4. Inspect the Tables
```sql
-- Raw mirror
SELECT span_id, flow, workflow FROM discovery.raw_spans ORDER BY ingested_at DESC;

-- Structured tables
SELECT span_id, subject_type, subject_identifier FROM discovery.runs_subjects;
SELECT span_id, recipe_name, checksum FROM discovery.runs_protocols;
SELECT span_id, status, started_at, finished_at FROM discovery.runs_executions;
SELECT span_id, metric_name, value, unit FROM discovery.runs_metrics;
SELECT span_id, analysis_type, summary FROM discovery.runs_analysis;
SELECT span_id, title, format, storage_path FROM discovery.runs_manuscripts;
SELECT span_id, artifact_kind, storage_path FROM discovery.runs_artifacts;
```

These queries should return the ingested chignolin records, proving the ingest pipeline populates both the ledger and domain-specific audit tables.

### Ledger Sync
To replay an existing NDJSON ledger into the Discovery workspace:

```bash
cargo run -p hiv_discovery_runner -- sync-ledger --path /absolute/path/to/spans.ndjson
```

Each line must be a JSON span in the UniversalSpan schema; the command reuses the ingestion pipeline so both NDJSON and Postgres mirrors are updated.
