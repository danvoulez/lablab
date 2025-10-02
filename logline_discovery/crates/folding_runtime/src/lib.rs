use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use spans_core::{SpanId, UniversalSpan};

const RMSD_UNSTABLE_THRESHOLD: f64 = 5.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoldingFrame {
    pub time_ps: f64,
    pub potential_energy: f64,
    pub rmsd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoldingSimulation {
    pub span_id: SpanId,
    pub protein: String,
    pub frames: Vec<FoldingFrame>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoldingAnalysis {
    pub span_id: SpanId,
    pub protein: String,
    pub mean_energy: f64,
    pub max_rmsd: f64,
    pub unstable: bool,
    pub window: (DateTime<Utc>, Option<DateTime<Utc>>),
}

impl FoldingAnalysis {
    pub fn derives(span: &UniversalSpan, frames: &[FoldingFrame]) -> anyhow::Result<Self> {
        let energy_sum: f64 = frames.iter().map(|f| f.potential_energy).sum();
        let mean_energy = if frames.is_empty() {
            0.0
        } else {
            energy_sum / frames.len() as f64
        };

        let max_rmsd = frames.iter().map(|f| f.rmsd).fold(0.0_f64, f64::max);

        Ok(Self {
            span_id: span.id.clone(),
            protein: span
                .payload
                .get("protein")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .into(),
            mean_energy,
            max_rmsd,
            unstable: max_rmsd > RMSD_UNSTABLE_THRESHOLD,
            window: (span.started_at, span.finished_at),
        })
    }

    pub fn from_span(span: &UniversalSpan) -> anyhow::Result<Self> {
        let frames = extract_frames(span);
        Self::derives(span, &frames)
    }
}

fn extract_frames(span: &UniversalSpan) -> Vec<FoldingFrame> {
    let payload = &span.payload;

    let energy_series = pick_series(
        payload,
        &[
            "energy_series",
            "potential_energy_series",
            "results.energy_series",
        ],
    );
    let rmsd_series = pick_series(
        payload,
        &["rmsd_series", "results.rmsd_series", "analysis.rmsd_series"],
    );
    let time_series = pick_series(payload, &["time_ps", "times_ps", "timeline_ps"]);

    let len = energy_series
        .len()
        .max(rmsd_series.len())
        .max(time_series.len());

    if len == 0 {
        let energy = pick_scalar(
            payload,
            &["results.final_energy_kcal_mol", "final_energy", "energy"],
        )
        .unwrap_or(0.0);
        let rmsd = pick_scalar(
            payload,
            &["results.final_rmsd_angstrom", "final_rmsd", "rmsd"],
        )
        .unwrap_or(0.0);
        return vec![FoldingFrame {
            time_ps: 0.0,
            potential_energy: energy,
            rmsd,
        }];
    }

    let total_time_ps = pick_scalar(
        payload,
        &[
            "execution.simulation_time_ps",
            "execution.simulation_time_ns",
        ],
    )
    .map(|v| {
        if has_key(payload, &["execution", "simulation_time_ns"]) {
            v * 1000.0
        } else {
            v
        }
    })
    .unwrap_or(len as f64);

    let mut frames = Vec::with_capacity(len);
    for idx in 0..len {
        let time_ps = if !time_series.is_empty() {
            *time_series
                .get(idx)
                .unwrap_or_else(|| time_series.last().unwrap())
        } else if len > 1 {
            total_time_ps * idx as f64 / (len - 1) as f64
        } else {
            0.0
        };

        let energy = *energy_series
            .get(idx)
            .or_else(|| energy_series.last())
            .unwrap_or(&0.0);

        let rmsd = *rmsd_series
            .get(idx)
            .or_else(|| rmsd_series.last())
            .unwrap_or(
                &pick_scalar(
                    payload,
                    &["results.final_rmsd_angstrom", "final_rmsd", "rmsd"],
                )
                .unwrap_or(0.0),
            );

        frames.push(FoldingFrame {
            time_ps,
            potential_energy: energy,
            rmsd,
        });
    }

    frames
}

fn pick_series(payload: &Value, paths: &[&str]) -> Vec<f64> {
    for raw_path in paths {
        if let Some(values) = resolve_array(payload, raw_path) {
            let series: Vec<f64> = values
                .into_iter()
                .filter_map(|v| match v {
                    Value::Number(n) => n.as_f64(),
                    Value::String(s) => s.parse::<f64>().ok(),
                    _ => None,
                })
                .collect();
            if !series.is_empty() {
                return series;
            }
        }
    }
    Vec::new()
}

fn resolve_array(payload: &Value, path: &str) -> Option<Vec<Value>> {
    let mut current = payload;
    for key in path.split('.') {
        current = current.get(key)?;
    }
    current.as_array().cloned()
}

fn pick_scalar(payload: &Value, paths: &[&str]) -> Option<f64> {
    for raw_path in paths {
        let mut current = payload;
        let mut found = true;
        for key in raw_path.split('.') {
            match current.get(key) {
                Some(v) => current = v,
                None => {
                    found = false;
                    break;
                }
            }
        }
        if !found {
            continue;
        }
        if let Some(num) = current.as_f64() {
            return Some(num);
        }
        if let Some(s) = current.as_str() {
            if let Ok(v) = s.parse::<f64>() {
                return Some(v);
            }
        }
    }
    None
}

fn has_key(payload: &Value, path: &[&str]) -> bool {
    let mut current = payload;
    for key in path {
        match current.get(*key) {
            Some(v) => current = v,
            None => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use serde_json::json;

    fn span_from_value(value: Value) -> UniversalSpan {
        let mut span = UniversalSpan::new(
            "span::test",
            "test span",
            "protein_folding",
            "workflow",
            Utc.timestamp_opt(1_700_000_000, 0).unwrap(),
            value,
        );
        span.finished_at = Some(Utc.timestamp_opt(1_700_000_500, 0).unwrap());
        span
    }

    #[test]
    fn frames_from_series() {
        let span = span_from_value(json!({
            "protein": "chignolin",
            "energy_series": [-120.0, -118.0, -122.0],
            "rmsd_series": [1.0, 2.0, 3.0],
            "execution": {
                "simulation_time_ns": 0.3
            }
        }));

        let analysis = FoldingAnalysis::from_span(&span).unwrap();
        assert_eq!(analysis.protein, "chignolin");
        assert!((analysis.mean_energy - (-120.0 - 118.0 - 122.0) / 3.0).abs() < 1e-6);
        assert_eq!(analysis.max_rmsd, 3.0);
        assert!(!analysis.unstable);
    }

    #[test]
    fn frames_from_single_values() {
        let span = span_from_value(json!({
            "results": {
                "final_energy_kcal_mol": -90.0,
                "final_rmsd_angstrom": 6.0
            }
        }));

        let analysis = FoldingAnalysis::from_span(&span).unwrap();
        assert_eq!(analysis.mean_energy, -90.0);
        assert_eq!(analysis.max_rmsd, 6.0);
        assert!(analysis.unstable);
    }
}
