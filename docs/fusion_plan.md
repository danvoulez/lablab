# LogLine Fusion Plan

This document captures the path to merge LogLine Warp, LogLine Fold, and LogLine Discovery Lab into a single “LogLine Discovery Lab” workspace.

## Stage 0 · Inventory & Decision Making (in progress)

| Domain | Current Repo | Key Components | Must Keep? | Notes |
| --- | --- | --- | --- | --- |
| Runtime / Ledger | `LogLine Warp` | `packages/common`, `warp_controller`, `ledger_vault`, `span_fabricator`, `warp_motor` | ✓ | Provides span ledger semantics, rollback tools, REPL. `common` and `ledger_vault` contain types we should ingest. REPL binaries can be optional features. |
| Folding Engine | `LogLine Folding Engine v1` | `core/` Rust crates, `physics/` bridge, `app/` UI | ✓ | Keep core folding kernels + contract definitions; UI/assets become optional. |
| Discovery Orchestration | `LogLine Discovery Lab` | `crates/*`, `binaries/hiv_discovery_runner`, automation scripts | ✓ | Already the integration backbone. |
| Legacy UI / Assets | Warp SVG dashboards, Fold Next.js front-end, various scripts | △ | Retain only if we commit to supporting them; otherwise flag for archival. |

### Immediate Decisions
- **Span Schema** → single source in `crates/spans_core/` (already here). Warp/Fold any duplicates get removed after migration.
- **Ledger append utilities** → take from Warp’s `ledger_vault` to replace/adapt our local `ledger` helper once merged.
- **Contracts** → aggregate into `contracts/` with sub-folders (`runtime/`, `folding/`, `manuscripts/`).

## Stage 1 · Workspace Bring-up (in progress)
1. Unified workspace manifest now includes Warp crates (`warp_common`, `warp_ledger_vault`, `warp_span_fabricator`) and Fold primitives (`folding_time`, `folding_molecule`, `folding_core`).
2. Next: expose folding kernels through the orchestrator (new CLI hooks) and decide whether to vendor optional physics bridges.
3. Keep higher-level binaries (`warp_motor`, Fold UI) out until base crates compile under the new structure.

## Stage 2 · Tooling Harmonisation
- Consolidate `.env` configuration with namespaced defaults (runtime, folding, orchestration).
- Ensure `scripts/setup.sh`, `run_lab.sh`, `auto_backfill.sh` initialise ledger + runtime pieces.
- Extend `hiv_discovery_runner` CLI (eventually `logline`) to expose runtime/folding commands.

## Stage 3 · API & Automation Fusion
- Offer a single CLI entrypoint that can:
  - start the ledger runtime loop (Warp engine),
  - launch folding jobs (Fold kernels),
  - run ingestion/watchers/causal analysis.
- Provide profile bundles (e.g., `profiles/demo`, `profiles/gp41`) with spans + runtime configs.

## Stage 4 · Cleanup & Archival
- Drop duplicate schema definitions, redundant scripts, and unmaintained UI stubs.
- Archive old repos (or leave as mirrors) once the unified workspace is stable.

## Task Hooks
- [ ] Stage 0: finish mapping concrete crates/files to migrate or deprecate.
- [ ] Stage 1: import Warp `packages/common` into `logline_discovery/crates/`.
- [ ] Stage 1: import Warp `packages/ledger_vault` and adjust dependencies.
- [ ] Stage 1: integrate Fold core crates (starting with data model / contract definitions).
- [ ] Stage 2: harmonise `.env` + scripts.
- [ ] Stage 3: unify CLI under a `logline` binary.
- [ ] Stage 4: remove duplicated code and document archival plan.

This document will track progress; update it as stages complete or decisions change.
