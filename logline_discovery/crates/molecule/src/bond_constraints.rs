use crate::aminoacid::ResidueId;

/// Represents a simple bond constraint between two residues.
#[derive(Clone, Debug)]
pub struct BondConstraint {
    pub left: ResidueId,
    pub right: ResidueId,
    pub max_distance: f64,
    pub min_distance: f64,
}

/// Collection of bond constraints applied to a peptide chain.
#[derive(Clone, Debug, Default)]
pub struct BondConstraintSet {
    constraints: Vec<BondConstraint>,
}

impl BondConstraintSet {
    pub fn new(constraints: Vec<BondConstraint>) -> Self {
        Self { constraints }
    }

    pub fn constraints(&self) -> &[BondConstraint] {
        &self.constraints
    }
}
