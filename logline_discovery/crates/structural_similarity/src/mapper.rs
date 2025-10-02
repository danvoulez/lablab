use anyhow::Result;
use serde_json::Value;
use spans_core::UniversalSpan;

const MAPPER_PATHS: &[&str] = &[
    "analysis.mapper.nodes",
    "mapper.nodes",
    "analysis.mapper_signature",
];

pub fn compute_mapper_overlap(a: &UniversalSpan, b: &UniversalSpan) -> Result<f64> {
    let profile_a = extract_mapper_profile(a);
    let profile_b = extract_mapper_profile(b);

    if profile_a.is_empty() && profile_b.is_empty() {
        return Ok(0.0);
    }

    Ok(compare_profiles(&profile_a, &profile_b))
}

fn extract_mapper_profile(span: &UniversalSpan) -> Vec<f64> {
    for path in MAPPER_PATHS {
        if let Some(values) = resolve_array(&span.payload, path) {
            let mut profile = Vec::new();
            for value in values.into_iter() {
                match value {
                    Value::Number(_) | Value::String(_) => {
                        if let Some(num) = value_to_number(&value) {
                            profile.push(num);
                        }
                    }
                    Value::Object(obj) => {
                        if let Some(size) = obj.get("size").and_then(value_to_number) {
                            profile.push(size);
                        }
                        if let Some(weight) = obj.get("weight").and_then(value_to_number) {
                            profile.push(weight);
                        }
                        if let Some(density) = obj.get("density").and_then(value_to_number) {
                            profile.push(density);
                        }
                    }
                    _ => {}
                }
            }
            if !profile.is_empty() {
                return profile;
            }
        }
    }

    Vec::new()
}

fn compare_profiles(a: &[f64], b: &[f64]) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }

    let norm_a = normalize_positive(a);
    let norm_b = normalize_positive(b);

    let len = norm_a.len().max(norm_b.len());
    let mut l1 = 0.0;
    for i in 0..len {
        let va = norm_a.get(i).copied().unwrap_or(0.0);
        let vb = norm_b.get(i).copied().unwrap_or(0.0);
        l1 += (va - vb).abs();
    }
    let distribution_score = (1.0 - 0.5 * l1).clamp(0.0, 1.0);

    let sum_a: f64 = a.iter().map(|v| v.max(0.0)).sum();
    let sum_b: f64 = b.iter().map(|v| v.max(0.0)).sum();
    let magnitude_penalty = if sum_a + sum_b > 0.0 {
        ((sum_a - sum_b).abs() / (sum_a + sum_b)).clamp(0.0, 1.0)
    } else {
        0.0
    };

    (distribution_score * (1.0 - magnitude_penalty)).clamp(0.0, 1.0)
}

fn resolve_array<'a>(root: &'a Value, path: &str) -> Option<Vec<Value>> {
    let mut current = root;
    for key in path.split('.') {
        current = current.get(key)?;
    }
    current.as_array().map(|arr| arr.to_vec())
}

fn value_to_number(value: &Value) -> Option<f64> {
    match value {
        Value::Number(num) => num.as_f64(),
        Value::String(s) => s.parse::<f64>().ok(),
        _ => None,
    }
}

fn normalize_positive(values: &[f64]) -> Vec<f64> {
    let sum: f64 = values.iter().map(|v| v.max(0.0)).sum();
    if sum <= f64::EPSILON {
        return values.iter().map(|_| 0.0).collect();
    }
    values.iter().map(|v| v.max(0.0) / sum).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;

    fn span(payload: Value) -> UniversalSpan {
        UniversalSpan::new("span", "span", "flow", "workflow", Utc::now(), payload)
    }

    #[test]
    fn identical_profiles_score_high() {
        let payload = json!({"analysis": {"mapper": {"nodes": [
            {"size": 10.0, "density": 0.2},
            {"size": 12.0, "density": 0.25}
        ]}}});
        let a = span(payload.clone());
        let b = span(payload);
        let score = compute_mapper_overlap(&a, &b).unwrap();
        assert!(score > 0.99);
    }

    #[test]
    fn divergent_profiles_reduce_score() {
        let a = span(json!({"analysis": {"mapper": {"nodes": [1.0, 2.0, 3.0]}}}));
        let b = span(json!({"analysis": {"mapper": {"nodes": [10.0, 20.0, 30.0]}}}));
        let score = compute_mapper_overlap(&a, &b).unwrap();
        assert!(score < 0.5);
    }
}
