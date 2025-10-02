# Ledger Operations in the Unified LogLine Lab

The `warp_ledger_vault` crate provides rollback and checkpoint helpers that mirror the original Warp runtime. These utilities wrap the local ledger (`ledger/spans/discovery.ndjson`) and append contract entries when corrective actions occur.

## Rollback to Span
```bash
cargo run -p hiv_discovery_runner -- ledger rollback --span-id span::execution::demo
```
- Marks every span after the specified ID as `reversed=true` in the NDJSON log.
- Appends a `rollback_performed` contract recording the target span and the count of reversed entries.

## Checkpoint Lifecycle
```bash
# Create a checkpoint label
cargo run -p hiv_discovery_runner -- ledger checkpoint --label demo-save

# Roll back to that checkpoint later
cargo run -p hiv_discovery_runner -- ledger rollback-checkpoint --label demo-save
```
- Checkpoint creation appends a `checkpoint` contract with metadata (`label`, timestamp).
- Rollback replays the ledger, flips spans after the checkpoint to reversed, and appends a `checkpoint_rollback_performed` contract.

All commands reuse the `warp_ledger_vault::SpanLog` helpers introduced during the fusion, ensuring the ledger remains append-only and auditable.
