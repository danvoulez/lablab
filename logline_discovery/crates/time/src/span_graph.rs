use std::collections::{HashMap, HashSet};

use crate::trajectory::SpanId;

/// Lightweight adjacency representation of rotation spans.
#[derive(Default, Debug)]
pub struct SpanGraph {
    edges: HashMap<SpanId, HashSet<SpanId>>,
}

impl SpanGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connect(&mut self, from: SpanId, to: SpanId) {
        self.edges.entry(from).or_default().insert(to);
    }

    pub fn successors(&self, span: &SpanId) -> impl Iterator<Item = &SpanId> {
        self.edges.get(span).into_iter().flatten()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.values().map(HashSet::len).sum()
    }
}
