# Postgres Setup for LogLine Discovery Lab

This document covers provisioning the Discovery Lab database, applying migrations, and wiring the HIV Discovery Runner to persist spans and artifacts.

## 1. Create Database
```bash
createdb logline_discovery
```

Alternatively, connect to your Postgres server and run:
```sql
CREATE DATABASE logline_discovery;
```

## 2. Apply Migrations
The SQL migrations live in `db/migrations/`. To apply the schema:
```bash
psql logline_discovery < db/migrations/0001_init.sql
```

This creates the `discovery` schema, append-only tables (subjects, protocols, executions, etc.), and the `raw_spans` mirror.

## 3. Configure Environment
Export the database URL so the runner mirrors spans into Postgres:
```bash
export DATABASE_URL="postgres://localhost/logline_discovery"
export LEDGER_PATH="ledger/spans/discovery.ndjson"   # optional override
```

You can also place these values into a `.env` file at the workspace root:
```
DATABASE_URL=postgres://localhost/logline_discovery
LEDGER_PATH=ledger/spans/discovery.ndjson
```

## 4. Run Ingest Command
With the environment configured, the runner will append spans to both NDJSON and Postgres:
```bash
cargo run -p hiv_discovery_runner -- ingest path/to/span.json --flow protein_folding --workflow hiv_reservoir_mapping
```

On success you should see:
- A new line appended to the NDJSON ledger at `LEDGER_PATH`.
- A row inserted into `discovery.raw_spans` with matching `span_id`.

## 5. Verify Data
Connect using `psql` and inspect the mirror table:
```sql
SELECT span_id, flow, workflow, ingested_at
FROM discovery.raw_spans
ORDER BY ingested_at DESC
LIMIT 10;
```

## 6. Next Steps
Future iterations will fan out spans into domain-specific tables (`runs_subjects`, `runs_protocols`, etc.). For now, the raw mirror ensures every payload ingested has an auditable database record alongside the append-only ledger.
