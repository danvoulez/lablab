use crate::aminoacid::{AminoAcid, ResidueId};

/// Residue entry in a peptide chain with simplified spatial metadata.
#[derive(Clone, Debug)]
pub struct Residue {
    pub id: ResidueId,
    pub amino_acid: AminoAcid,
    pub phi: f64,
    pub psi: f64,
    position: [f64; 3],
}

/// Ordered peptide chain used by the folding engine.
#[derive(Clone, Debug, Default)]
pub struct PeptideChain {
    residues: Vec<Residue>,
}

impl PeptideChain {
    pub fn new(residues: Vec<Residue>) -> Self {
        Self { residues }
    }

    pub fn len(&self) -> usize {
        self.residues.len()
    }

    pub fn is_empty(&self) -> bool {
        self.residues.is_empty()
    }

    pub fn residues(&self) -> &[Residue] {
        &self.residues
    }

    pub fn residue_mut(&mut self, id: ResidueId) -> Option<&mut Residue> {
        self.residues.iter_mut().find(|res| res.id == id)
    }
}

impl Residue {
    pub fn new(id: ResidueId, amino_acid: AminoAcid) -> Self {
        Self {
            id,
            amino_acid,
            phi: 0.0,
            psi: 0.0,
            position: [0.0, 0.0, 0.0],
        }
    }

    pub fn with_position(mut self, position: [f64; 3]) -> Self {
        self.position = position;
        self
    }

    pub fn position(&self) -> [f64; 3] {
        self.position
    }

    pub fn set_position(&mut self, position: [f64; 3]) {
        self.position = position;
    }
}
