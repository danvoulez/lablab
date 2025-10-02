use anyhow::Result;
use serde_json::Value;
use spans_core::UniversalSpan;

const GRAPHLET_PATHS: &[&str] = &[
    "analysis.graphlet_histogram",
    "analysis.graphlet_distribution",
    "graphlet_histogram",
    "graphlet_distribution",
    "metadata.graphlet_histogram",
    "metadata.graphlets",
    "graphlets",
];

const FALLBACK_KEYS: &[&str] = &[
    "results.final_rmsd_angstrom",
    "results.final_energy_kcal_mol",
    "execution.simulation_time_ns",
    "execution.performance_ns_per_day",
];

pub fn compute_graphlet_score(a: &UniversalSpan, b: &UniversalSpan) -> Result<f64> {
    let hist_a = extract_histogram(a);
    let hist_b = extract_histogram(b);

    if !hist_a.is_empty() || !hist_b.is_empty() {
        return Ok(compare_histograms(&hist_a, &hist_b));
    }

    let fallback_a = extract_fallback(a);
    let fallback_b = extract_fallback(b);
    if fallback_a.is_empty() || fallback_b.is_empty() {
        return Ok(0.0);
    }

    Ok(compare_histograms(&fallback_a, &fallback_b))
}

fn extract_histogram(span: &UniversalSpan) -> Vec<f64> {
    for path in GRAPHLET_PATHS {
        if let Some(values) = resolve_array(&span.payload, path) {
            let series: Vec<f64> = values
                .into_iter()
                .filter_map(|value| value_to_number(&value))
                .collect();
            if !series.is_empty() {
                return series;
            }
        }
    }
    Vec::new()
}

fn extract_fallback(span: &UniversalSpan) -> Vec<f64> {
    FALLBACK_KEYS
        .iter()
        .filter_map(|path| resolve_scalar(&span.payload, path))
        .collect()
}

fn compare_histograms(a: &[f64], b: &[f64]) -> f64 {
    let norm_a = normalize(a);
    let norm_b = normalize(b);
    let len = norm_a.len().max(norm_b.len());
    if len == 0 {
        return 0.0;
    }

    let mut l1 = 0.0;
    for idx in 0..len {
        let va = norm_a.get(idx).copied().unwrap_or(0.0);
        let vb = norm_b.get(idx).copied().unwrap_or(0.0);
        l1 += (va - vb).abs();
    }
    (1.0 - 0.5 * l1).clamp(0.0, 1.0)
}

fn normalize(values: &[f64]) -> Vec<f64> {
    let sum: f64 = values.iter().map(|v| v.max(0.0)).sum();
    if sum <= f64::EPSILON {
        return values.iter().map(|_| 0.0).collect();
    }
    values.iter().map(|v| v.max(0.0) / sum).collect()
}

fn resolve_array<'a>(root: &'a Value, path: &str) -> Option<Vec<Value>> {
    let mut current = root;
    for key in path.split('.') {
        current = current.get(key)?;
    }
    current.as_array().map(|arr| arr.to_vec())
}

fn resolve_scalar(root: &Value, path: &str) -> Option<f64> {
    let mut current = root;
    for key in path.split('.') {
        current = current.get(key)?;
    }
    value_to_number(current)
}

fn value_to_number(value: &Value) -> Option<f64> {
    match value {
        Value::Number(num) => num.as_f64(),
        Value::String(s) => s.parse::<f64>().ok(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;

    fn make_span(id: &str, payload: Value) -> UniversalSpan {
        UniversalSpan::new(id, id, "flow", "workflow", Utc::now(), payload)
    }

    #[test]
    fn identical_histograms_yield_one() {
        let payload = json!({"analysis": {"graphlet_histogram": [1.0, 2.0, 3.0]}});
        let span_a = make_span("a", payload.clone());
        let span_b = make_span("b", payload);
        let score = compute_graphlet_score(&span_a, &span_b).unwrap();
        assert!((score - 1.0).abs() < 1e-6);
    }

    #[test]
    fn differing_histograms_reduce_score() {
        let span_a = make_span(
            "a",
            json!({"analysis": {"graphlet_histogram": [5.0, 1.0, 0.0]}}),
        );
        let span_b = make_span(
            "b",
            json!({"analysis": {"graphlet_histogram": [1.0, 3.0, 2.0]}}),
        );
        let score = compute_graphlet_score(&span_a, &span_b).unwrap();
        assert!(score < 0.8);
    }

    #[test]
    fn fallback_uses_structural_features() {
        let span_a = make_span(
            "a",
            json!({
                "results": {"final_rmsd_angstrom": 0.6, "final_energy_kcal_mol": -120.0},
                "execution": {"simulation_time_ns": 0.5}
            }),
        );
        let span_b = make_span(
            "b",
            json!({
                "results": {"final_rmsd_angstrom": 0.6, "final_energy_kcal_mol": -120.0},
                "execution": {"simulation_time_ns": 0.5}
            }),
        );
        let score = compute_graphlet_score(&span_a, &span_b).unwrap();
        assert!(score > 0.9);
    }
}
