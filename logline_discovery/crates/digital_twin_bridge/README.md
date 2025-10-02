# Digital Twin Bridge — Quick Reference

This crate manages bidirectional digital-twin synchronization cycles and now ships a divergence analyzer that compares physical and simulated telemetry before persisting reconciliation spans.

## Key Types

- `SyncConfig` — controls storage limits, reconciliation thresholds, and metric tolerances (global + per-metric).
- `TwinObservation` — captures a physical/digital snapshot with metrics.
- `TwinComparison` — result of `BidirectionalTwinBridge::analyze_cycle`, containing aligned metrics and detected divergences.
- `TwinDivergence` — per-metric delta with severity, ready to materialize as a span.
- `BidirectionalTwinBridge` — orchestrates comparisons, cycle recording, and divergence span emission.

## Example

```rust
use chrono::Utc;
use digital_twin_bridge::{
    BidirectionalTwinBridge, DivergenceSeverity, SyncConfig, TwinObservation, TwinSide,
};
use spans_core::{SpanId, UniversalSpan};
use serde_json::json;
use std::collections::HashMap;

fn observation(span: &str, side: TwinSide, temp: f64) -> TwinObservation {
    let mut metrics = HashMap::new();
    metrics.insert("temperature".to_string(), temp);
    TwinObservation {
        span_id: SpanId::new(span),
        side,
        recorded_at: Utc::now(),
        metrics,
    }
}

let mut bridge = BidirectionalTwinBridge::with_config(SyncConfig {
    default_metric_tolerance: 0.05,
    ..SyncConfig::default()
});

let physical = observation("span::phys", TwinSide::Physical, 300.0);
let digital = observation("span::dig", TwinSide::Digital, 360.0);

let comparison = bridge.analyze_cycle(physical.clone(), digital.clone());
assert!(comparison.has_divergences());
assert_eq!(comparison.divergences[0].severity, DivergenceSeverity::Critical);

// Persist cycle bookkeeping
let cycle = bridge.record_comparison(&comparison);
assert_eq!(cycle.divergences.len(), 1);

// Emit a normalized span for ledger/ledger+DB writes
let base = UniversalSpan::new(
    "span::twin_base",
    "Twin mirror",
    "digital_twin",
    "twin_workflow",
    Utc::now(),
    json!({}),
);
let divergence_span = bridge
    .emit_divergence_span(&base, &comparison.divergences[0])
    .expect("span emission");
```

The bridge pairs with the ingestion automation and causal engine so digital-twin drifts surface as first-class spans, ready for reconciliation workflows.
