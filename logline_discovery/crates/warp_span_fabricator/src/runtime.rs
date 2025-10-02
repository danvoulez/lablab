use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Synthetic {
    pub id: String,
    pub tag: String,
    pub weight: f64,
}

pub struct Runtime;

impl Runtime {
    pub fn new() -> Self {
        Self
    }

    pub fn delta_span_merge(&self, spans: &[Synthetic]) -> Vec<Synthetic> {
        use std::collections::BTreeMap;
        let mut acc: BTreeMap<String, f64> = BTreeMap::new();
        for s in spans {
            *acc.entry(s.tag.clone()).or_insert(0.0) += s.weight;
        }
        acc.into_iter()
            .map(|(tag, weight)| Synthetic {
                id: format!("merge-{tag}"),
                tag,
                weight,
            })
            .collect()
    }

    pub fn fabricate_demo(&self) -> Result<Vec<Synthetic>> {
        let spans = vec![
            Synthetic {
                id: "a1".into(),
                tag: "latency".into(),
                weight: 0.3,
            },
            Synthetic {
                id: "a2".into(),
                tag: "latency".into(),
                weight: 0.5,
            },
            Synthetic {
                id: "b1".into(),
                tag: "tick".into(),
                weight: 0.2,
            },
        ];
        Ok(self.delta_span_merge(&spans))
    }
}
