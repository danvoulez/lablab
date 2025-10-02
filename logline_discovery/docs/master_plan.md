# LogLine Discovery Lab - Master Plan

## North Star Vision

The LogLine Discovery Lab is a 24/7 automated scientific discovery pipeline that fuses LogLine Warp's span ledger with LogLine Fold's molecular simulation engine to continuously discover, analyze, and publish scientific insights about molecular systems, particularly focusing on HIV research and protein folding dynamics.

### Core Principles

**Joyful Flow** - Scientists experience kinetic dashboards with rich motion states, interactive molecular viewers, and playful microcopy that respects scientific vernacular.

**Scientific Gravitas** - Every visualization hyperlinks to underlying spans, contracts, and parameter logs; includes uncertainty intervals, model provenance, and citations; enables one-click export to LaTeX/NDJSON for reproducible results.

**Narrative Cohesion** - Threads the scientific storyline across the experience: hypothesis graphs as subway maps, digital twin pulses that sync to real experiments, manuscript drafts that update live as analytics run.

## Architecture Snapshot

### Core Systems

- **Cargo Workspace**: Multi-crate Rust project with shared span schema (`spans_core`)
- **Database Layer**: Append-only Postgres for structured queries + NDJSON ledger for audit trail
- **Ingestion Engine**: Auto-detects Fold/Warp payloads, classifies spans, mirrors to both Postgres and ledger
- **Analytics Core**: Folding analysis, causal inference engine, structural similarity with ST-GNN bridge
- **Digital Twin Bridge**: Compares physical/digital observations, detects drift, emits divergence spans
- **Manuscript Generator**: Automated report generation with citations, figures, and metadata
- **Dashboard**: Vue.js PWA with causal chain explorer, real-time feeds, and publication mode

### Data Flow

```
Fold Simulations → Span Ingestion → Postgres + Ledger → Analytics → Manuscripts → Dashboard
     ↓              ↓                    ↓              ↓           ↓            ↓
  Raw NDJSON     Classification      Structured      Causal      Markdown     Live UI
  Outputs        & Mapping           Queries         Inference   + Contracts  Updates
```

## Subject → Manuscript Workflow

### 1. Initiate Subject
Scientist selects protein/mutation/patient sample; metadata logged as UniversalSpan in Postgres (`runs.subjects`) and NDJSON ledger.

### 2. Design Protocol
Choose simulation recipe from curated library; parameters become protocol_config span with cryptographic hash.

### 3. Acquire Inputs
Fetch structures via Fold pipeline; integrity verified with checksums; acquisition events flagged in spans.

### 4. Execute Simulation
Launch folding engine; runtime metrics stream into Warp ledger and Postgres (`runs.frames`, `runs.metrics`).

### 5. Post-Process Analytics
Run folding analysis, structural similarity, causal inference; results persisted as analysis spans.

### 6. Review & Sign-Off
Investigator reviews in Publication mode; annotations create review spans tied to user identities.

### 7. Manuscript Assembly
Generator compiles spans into Markdown/PDF; artifacts hashed and stored with metadata.

### 8. Archive & Publish
Bundle NDJSON log, database export, contracts, manuscript hash into immutable release folder.

## 9-Week Roadmap

### Phase 0 · Baseline (✅ Complete)
- Cargo workspace scaffolded with shared span schema
- Causal/folding/manuscript stubs compile
- gp41 seed data + mission contracts checked in

### Phase 1 · Data Plumbing (✅ Complete)
- Fold outputs stream through span_ingestor
- Warp NDJSON ledger structure mirrored
- CLI commands for ingest ↔ ledger sync

### Phase 2 · Analytics Core (✅ Complete)
- Folding analysis over real trajectories
- Full causal ruleset (resource contention, structural joins)
- Structural similarity wired to Fold embeddings + ST-GNN

### Phase 3 · Orchestration & Automation (✅ Complete)
- hiv_discovery_runner as orchestrator service
- Spans/contracts emitted for every analysis step
- Warp rollback/checkpoint integration
- Manuscript generator templates for case studies

### Phase 4 · Experience Layer (In Progress)
- Fold UI ported into discovery_dashboard
- Live spans, causal chains, instability markers
- Twin bridge views and SSE/websocket feeds

### Phase 5 · Validation & Release (Weeks 8–9)
- End-to-end HIV scenario dry run
- Regression + property tests for span ingestion and causal inference
- Bootstrap scripts and docs
- Roadmap-driven release notes

## Cross-Cutting Tracks

### Infrastructure
- CI with cargo fmt/clippy/test
- Optional local registry mirror for offline builds

### Research Assets
- Notebooks with HIV landscape + hypothesis graphs
- Contracts library for new experiments

### Risk Mitigation
- Version shared schemas
- Validate against sample Warp logs
- Keep runner working with both simulated and real spans

## Immediate Next Actions

1. **Backfill Historical Data**: Replay existing Warp NDJSON through sync-ledger command
2. **Vue Dashboard Migration**: Lift HTTP dashboard into full Vue.js PWA stack
3. **Twin Bridge Integration**: Feed real twin observations through watcher to populate divergence tables
4. **End-to-End Testing**: Validate complete pipeline from Fold span → manuscript generation

## Success Metrics

- **Scientific Output**: Manuscripts generated with proper citations and reproducible results
- **Operational Uptime**: 24/7 ingestion and analysis with <5% error rate
- **User Experience**: Scientists can go from "which protein?" to manuscript in <30 minutes
- **Audit Compliance**: Every result traceable to source spans and contracts

## Risk Register

**High Risk**: ST-GNN integration requires external Python model - fallback to deterministic heuristics implemented.

**Medium Risk**: Vue.js migration could delay dashboard delivery - HTTP fallback available for core functionality.

**Low Risk**: Historical Warp ledger backfill may reveal schema incompatibilities - validation scripts in place.
