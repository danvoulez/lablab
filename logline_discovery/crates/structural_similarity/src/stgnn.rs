use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{anyhow, Context, Result};
use serde_json::{json, Value};
use spans_core::UniversalSpan;

/// Computes ST-GNN similarity between two spans.
///
/// If the environment variable `STGNN_INFER_PATH` is set, the runner will invoke the
/// given Python script (`python3 <script>`), sending a JSON payload (`{"span_a": ..., "span_b": ...}`)
/// on stdin and expecting a JSON response `{ "similarity": <float> }` on stdout.
/// Otherwise, a deterministic heuristic based on the span payload is used.
pub fn infer_stgnn_similarity(a: &UniversalSpan, b: &UniversalSpan) -> Result<f64> {
    if let Some(script) = std::env::var_os("STGNN_INFER_PATH") {
        let payload = json!({
            "span_a": a.payload,
            "span_b": b.payload,
        });

        let mut child = Command::new("python3")
            .arg(script)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .context("failed to spawn ST-GNN Python process")?;

        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(serde_json::to_string(&payload)?.as_bytes())
                .context("failed to send payload to ST-GNN process")?;
        }

        let output = child
            .wait_with_output()
            .context("failed to read ST-GNN process output")?;

        if !output.status.success() {
            return Err(anyhow!(
                "ST-GNN process exited with status {:?}: {}",
                output.status.code(),
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let response: Value = serde_json::from_slice(&output.stdout)
            .context("failed to parse ST-GNN response JSON")?;
        let similarity = response
            .get("similarity")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| anyhow!("ST-GNN response missing numeric 'similarity'"))?;
        return Ok(similarity.clamp(0.0, 1.0));
    }

    Ok(heuristic_similarity(a, b))
}

fn heuristic_similarity(a: &UniversalSpan, b: &UniversalSpan) -> f64 {
    let features_a = extract_features(a);
    let features_b = extract_features(b);

    let diff: f64 = features_a
        .iter()
        .zip(features_b.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();

    // Map L1 distance into (0,1] similarity.
    (1.0 / (1.0 + diff)).clamp(0.0, 1.0)
}

fn extract_features(span: &UniversalSpan) -> Vec<f64> {
    let payload = &span.payload;

    let mut features = Vec::with_capacity(6);

    features.push(get_number(payload, &["results", "final_rmsd_angstrom"]).unwrap_or(0.0));
    features.push(get_number(payload, &["results", "final_energy_kcal_mol"]).unwrap_or(0.0));
    features.push(get_number(payload, &["execution", "simulation_time_ns"]).unwrap_or(0.0));
    features.push(get_number(payload, &["execution", "performance_ns_per_day"]).unwrap_or(0.0));
    features.push(get_number(payload, &["analysis", "mean_energy"]).unwrap_or(0.0));
    features.push(get_number(payload, &["analysis", "max_rmsd"]).unwrap_or(0.0));

    features
}

fn get_number(payload: &Value, path: &[&str]) -> Option<f64> {
    let mut current = payload;
    for key in path {
        current = current.get(*key)?;
    }
    match current {
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
        UniversalSpan::new(
            id,
            "test span",
            "test_flow",
            "test_workflow",
            Utc::now(),
            payload,
        )
    }

    #[test]
    fn heuristic_similarity_invariant() {
        let payload = json!({
            "results": {
                "final_rmsd_angstrom": 0.5,
                "final_energy_kcal_mol": -100.0
            }
        });
        let span_a = make_span("span_a", payload.clone());
        let span_b = make_span("span_b", payload);

        let sim = infer_stgnn_similarity(&span_a, &span_b).unwrap();
        assert!((sim - 1.0).abs() < 1e-6);
    }

    #[test]
    fn heuristic_similarity_changes_with_features() {
        let span_a = make_span(
            "span_a",
            json!({
                "results": {
                    "final_rmsd_angstrom": 0.4,
                    "final_energy_kcal_mol": -95.0
                },
                "execution": {
                    "simulation_time_ns": 0.2
                }
            }),
        );
        let span_b = make_span(
            "span_b",
            json!({
                "results": {
                    "final_rmsd_angstrom": 1.5,
                    "final_energy_kcal_mol": -60.0
                },
                "execution": {
                    "simulation_time_ns": 1.0
                }
            }),
        );

        let sim = infer_stgnn_similarity(&span_a, &span_b).unwrap();
        assert!(sim < 0.5);
    }
}
