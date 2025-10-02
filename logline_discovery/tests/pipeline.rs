use chrono::Utc;
use causal_engine::{CausalEngine, CorrelationType};
use folding_runtime::FoldingAnalysis;
use serde_json::json;
use span_ingestor::{ingest_json, IngestOptions};
use spans_core::UniversalSpan;

fn ingest(value: serde_json::Value, flow: &str, workflow: &str) -> UniversalSpan {
    ingest_json(value, &IngestOptions::new(flow, workflow, Utc::now()))
        .expect("ingest span")
}

#[test]
fn pipeline_ingest_analysis_and_causal_link() {
    let system_payload = json!({
        "span_id": "span::system::monitor",
        "name": "system monitor",
        "timestamp": "2025-09-29T10:00:00Z",
        "metadata": {
            "cpu_usage": 0.9,
            "latency_ms": 180,
            "status": "degraded"
        }
    });
    let mut system_span = ingest(system_payload, "system_monitoring", "ops");
    system_span.finished_at = Some(Utc::now());

    let fold_payload = json!({
        "span_id": "span::fold::cycle",
        "name": "fold cycle",
        "timestamp": "2025-09-29T10:00:01Z",
        "protein": "chignolin",
        "results": {
            "final_energy_kcal_mol": -95.0,
            "final_rmsd_angstrom": 0.7,
            "rmsd_series": [0.8, 0.7, 0.65]
        },
        "execution": {
            "simulation_time_ns": 0.3,
            "performance_ns_per_day": 68000
        }
    });
    let mut fold_span = ingest(fold_payload, "protein_folding", "fold_pipeline");
    fold_span.finished_at = Some(Utc::now());

    let analysis = FoldingAnalysis::from_span(&fold_span).expect("analysis");
    assert!((analysis.mean_energy + 95.0).abs() < 1.0);
    assert!(analysis.max_rmsd < 1.0);

    let mut engine = CausalEngine::new();
    engine.ingest(vec![system_span.clone(), fold_span.clone()]);
    let chains = engine.infer();
    assert!(!chains.is_empty());
    assert!(chains
        .iter()
        .any(|chain| matches!(chain.links[0].relation, CorrelationType::TemporalProximity)));
}
