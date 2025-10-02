use graphlet::compute_graphlet_score;
use mapper::compute_mapper_overlap;
use serde::{Deserialize, Serialize};
use spans_core::UniversalSpan;
use stgnn::infer_stgnn_similarity;

pub mod graphlet;
pub mod mapper;
pub mod stgnn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityScores {
    pub graphlet: Option<f64>,
    pub mapper_overlap: Option<f64>,
    pub stgnn: Option<f64>,
}

impl SimilarityScores {
    pub fn new() -> Self {
        Self {
            graphlet: None,
            mapper_overlap: None,
            stgnn: None,
        }
    }

    pub fn composite(&self) -> Option<f64> {
        let mut acc = 0.0;
        let mut count = 0.0;

        if let Some(score) = self.graphlet {
            acc += score;
            count += 1.0;
        }
        if let Some(score) = self.mapper_overlap {
            acc += score;
            count += 1.0;
        }
        if let Some(score) = self.stgnn {
            acc += score;
            count += 1.0;
        }

        if count > 0.0 {
            Some(acc / count)
        } else {
            None
        }
    }
}

/// Placeholder: wiring for future graphlet computation.
pub fn graphlet_similarity(a: &UniversalSpan, b: &UniversalSpan) -> anyhow::Result<f64> {
    compute_graphlet_score(a, b)
}

/// Placeholder: wiring for future Mapper computation.
pub fn mapper_similarity(a: &UniversalSpan, b: &UniversalSpan) -> anyhow::Result<f64> {
    compute_mapper_overlap(a, b)
}

/// Placeholder: wiring for ST-GNN inference.
pub fn stgnn_similarity(a: &UniversalSpan, b: &UniversalSpan) -> anyhow::Result<f64> {
    infer_stgnn_similarity(a, b)
}

pub fn aggregate_similarity(
    a: &UniversalSpan,
    b: &UniversalSpan,
) -> anyhow::Result<SimilarityScores> {
    let mut scores = SimilarityScores::new();
    scores.graphlet = Some(graphlet_similarity(a, b)?);
    scores.mapper_overlap = Some(mapper_similarity(a, b)?);
    scores.stgnn = Some(stgnn_similarity(a, b)?);
    Ok(scores)
}
