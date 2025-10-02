use crate::AminoAcid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResidueClass {
    Hydrophobic,
    Polar,
    Positive,
    Negative,
}

pub fn classify(amino_acid: AminoAcid) -> ResidueClass {
    match amino_acid {
        AminoAcid::Alanine
        | AminoAcid::Valine
        | AminoAcid::Leucine
        | AminoAcid::Isoleucine
        | AminoAcid::Phenylalanine
        | AminoAcid::Methionine
        | AminoAcid::Tryptophan
        | AminoAcid::Proline => ResidueClass::Hydrophobic,
        AminoAcid::Serine
        | AminoAcid::Threonine
        | AminoAcid::Tyrosine
        | AminoAcid::Asparagine
        | AminoAcid::Glutamine
        | AminoAcid::Histidine
        | AminoAcid::Cysteine
        | AminoAcid::Glycine => ResidueClass::Polar,
        AminoAcid::Lysine | AminoAcid::Arginine => ResidueClass::Positive,
        AminoAcid::Aspartate | AminoAcid::Glutamate => ResidueClass::Negative,
    }
}

pub fn lennard_jones_params(a: ResidueClass, b: ResidueClass) -> (f64, f64) {
    let sigma_a = match a {
        ResidueClass::Hydrophobic => 3.8,
        ResidueClass::Polar => 3.6,
        ResidueClass::Positive | ResidueClass::Negative => 3.4,
    };
    let sigma_b = match b {
        ResidueClass::Hydrophobic => 3.8,
        ResidueClass::Polar => 3.6,
        ResidueClass::Positive | ResidueClass::Negative => 3.4,
    };
    let epsilon_a: f64 = match a {
        ResidueClass::Hydrophobic => 0.20_f64,
        ResidueClass::Polar => 0.15_f64,
        ResidueClass::Positive | ResidueClass::Negative => 0.12_f64,
    };
    let epsilon_b: f64 = match b {
        ResidueClass::Hydrophobic => 0.20_f64,
        ResidueClass::Polar => 0.15_f64,
        ResidueClass::Positive | ResidueClass::Negative => 0.12_f64,
    };
    let sigma = (sigma_a + sigma_b) * 0.5;
    let epsilon = (epsilon_a * epsilon_b).sqrt();
    (sigma, epsilon)
}

pub fn bond_equilibrium_distance(a: ResidueClass, b: ResidueClass) -> f64 {
    let base = 1.45;
    match (a, b) {
        (ResidueClass::Hydrophobic, ResidueClass::Hydrophobic) => base + 0.03,
        (ResidueClass::Polar, ResidueClass::Polar) => base,
        (ResidueClass::Positive, ResidueClass::Negative)
        | (ResidueClass::Negative, ResidueClass::Positive) => base - 0.05,
        _ => base + 0.01,
    }
}

pub fn angle_equilibrium(class_middle: ResidueClass) -> f64 {
    match class_middle {
        ResidueClass::Hydrophobic => 1.95,                       // ~112°
        ResidueClass::Polar => 1.92,                             // ~110°
        ResidueClass::Positive | ResidueClass::Negative => 1.90, // ~109°
    }
}
