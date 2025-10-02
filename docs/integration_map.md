# LogLine Discovery Lab — Capability Mapping

## Inputs We Already Own

### LogLine Warp (`../LogLine Warp`)
- Span ledger (`data/spans.ndjson`) + append-only rollback semantics.
- Modular runtime packages (`runtime_monitoring`, `span_fabricator`, `twin_sync_controller`, etc.) already aligned with LogLineOS packaging.
- Warp shell CLI exposing span inspection, reexecution, and rollback flows—ideal backbone for Lab ingestion/auditing.

### LogLine Fold (`../Library/.../LogLine Fold`)
- Rust folding engine with validated performance metrics and documented workflows.
- Frontend PWA (Tesla-grade UI) suitable for the Lab dashboard foundation.
- Existing spans/documentation pipelines compatible with the Lab’s scientific audit expectations.

### Chat Transcript (`Chat.md`)
- Detailed causal correlation engine blueprint (data structures, rules) ready to seed `causal_engine` crate implementation.

### Discovery Blueprint (`LogLine Discovery Lab.md`)
- Target workspace layout, HIV case study requirements, and module responsibilities across folding, causal inference, manuscript generation, and dashboards.

## Alignment Highlights
- Warp’s span format/logging maps directly to the Lab’s `ledger/spans/*` targets; we can reuse or wrap existing types.
- Fold’s workflows can emit spans that feed `span_ingestor` and `folding_runtime` without reinventing the physics layer.
- Transcript-driven causal logic lets us ship a meaningful `causal_engine` MVP early, feeding insights back as spans/contracts.

## Immediate Architecture Moves
1. Create a `logline_discovery` cargo workspace mirroring the blueprint crates.
2. Establish a shared `spans_core` module (or reuse Warp types) to standardize span ingestion across engines.
3. Plumb Fold outputs through the new span ingestor and persist into Warp-compatible ledger paths (`ledger/spans/folding`).
4. Implement `causal_engine` atop mapped span schema, then expose outputs to `manuscript_generator` and dashboards.

