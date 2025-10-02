use std::collections::HashMap;

use crate::{aminoacid::ResidueId, chain::PeptideChain};

/// Graph representation of a peptide chain where edges indicate potential fold interactions.
#[derive(Debug, Default)]
pub struct FoldableGraph {
    neighbors: HashMap<ResidueId, Vec<ResidueId>>,
}

impl FoldableGraph {
    pub fn from_chain(chain: &PeptideChain) -> Self {
        let mut neighbors: HashMap<ResidueId, Vec<ResidueId>> = HashMap::new();
        let residues = chain.residues();
        for window in residues.windows(2) {
            if let [left, right] = window {
                neighbors.entry(left.id).or_default().push(right.id);
                neighbors.entry(right.id).or_default().push(left.id);
            }
        }
        Self { neighbors }
    }

    pub fn neighbors(&self, id: &ResidueId) -> &[ResidueId] {
        self.neighbors.get(id).map(Vec::as_slice).unwrap_or(&[])
    }
}
