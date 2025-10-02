use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntheticSpanProfile {
    pub tag: String,
    pub weight: f64,
}
