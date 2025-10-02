use causal_engine::CausalEngine;
use chrono::{Duration, Utc};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::json;
use spans_core::{SpanId, UniversalSpan};

fn generate_span(index: usize, start_ms: i64) -> UniversalSpan {
    let payload = json!({
        "span_id": format!("span::bench::{}", index),
        "name": format!("bench span {}", index),
        "flow": "bench_flow",
        "workflow": "bench_workflow",
        "timestamp": Utc.timestamp_millis_opt(start_ms).unwrap().to_rfc3339(),
        "metadata": {
            "cpu_usage": (index % 100) as f64 / 100.0,
            "latency_ms": 100 + (index % 50) as i64,
            "status": if index % 25 == 0 { "error" } else { "ok" }
        },
        "results": {
            "final_energy_kcal_mol": -100.0 + (index % 30) as f64,
            "final_rmsd_angstrom": 0.5 + (index % 10) as f64 / 10.0
        }
    });

    let mut span = UniversalSpan::new(
        SpanId::new(format!("span::bench::{}", index)),
        format!("bench span {}", index),
        "bench_flow",
        "bench_workflow",
        Utc.timestamp_millis_opt(start_ms).unwrap(),
        payload,
    );
    span.finished_at = Some(span.started_at + Duration::milliseconds(250));
    span
}

fn generate_spans(count: usize) -> Vec<UniversalSpan> {
    (0..count)
        .map(|i| generate_span(i, 1_700_000_000_000 + (i as i64) * 200))
        .collect()
}

fn bench_ingest_infer(c: &mut Criterion) {
    let spans_1k = generate_spans(1_000);
    let spans_5k = generate_spans(5_000);

    c.bench_function("causal_engine_ingest_infer_1k", |b| {
        b.iter(|| {
            let mut engine = CausalEngine::new();
            engine.ingest(spans_1k.clone());
            let chains = engine.infer();
            black_box(chains);
        })
    });

    c.bench_function("causal_engine_ingest_infer_5k", |b| {
        b.iter(|| {
            let mut engine = CausalEngine::new();
            engine.ingest(spans_5k.clone());
            let chains = engine.infer();
            black_box(chains);
        })
    });
}

criterion_group!(benches, bench_ingest_infer);
criterion_main!(benches);
