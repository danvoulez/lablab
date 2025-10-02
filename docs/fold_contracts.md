# Folding Contracts in the Unified Lab

The `fold` subcommand of `hiv_discovery_runner` accepts `.lll` contracts and executes them using the fused `folding_core` engine.

## Running a Contract
```bash
cargo run -p hiv_discovery_runner -- fold \
  --contract contracts/folding/demo_fold.lll \
  --output tmp/demo_fold.json
```

The command prints the resulting JSON path and writes a structured summary containing:
- final energy (potential + kinetic)
- rotations applied/ghosted
- rule violations encountered
- per-span trajectory deltas

## Contract Format
Contracts follow the simplified instruction set re-exported from `folding_core::folding_parser`:
- `rotate residue=<id> angle=<deg> duration=<ms>`
- `clash_check`, `commit`, `rollback`
- `span_alias`, `define_domain`, `require_chaperone`, `add_modification`
- `set_physics_level`, `physics_span`

See `contracts/folding/demo_fold.lll` for a minimal example.

Each execution also writes a `folding_report` span to the ledger and appends a matching contract, so results are auditable alongside other lab events.

