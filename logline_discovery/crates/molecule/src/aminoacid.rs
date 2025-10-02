/// Identifier for a residue in a peptide chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ResidueId(pub usize);

/// Simplified representation of amino acids; extend as needed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AminoAcid {
    Alanine,
    Cysteine,
    Aspartate,
    Glutamate,
    Phenylalanine,
    Glycine,
    Histidine,
    Isoleucine,
    Lysine,
    Leucine,
    Methionine,
    Asparagine,
    Proline,
    Glutamine,
    Arginine,
    Serine,
    Threonine,
    Valine,
    Tryptophan,
    Tyrosine,
}

impl AminoAcid {
    pub fn from_char(symbol: char) -> Option<Self> {
        match symbol.to_ascii_uppercase() {
            'A' => Some(Self::Alanine),
            'C' => Some(Self::Cysteine),
            'D' => Some(Self::Aspartate),
            'E' => Some(Self::Glutamate),
            'F' => Some(Self::Phenylalanine),
            'G' => Some(Self::Glycine),
            'H' => Some(Self::Histidine),
            'I' => Some(Self::Isoleucine),
            'K' => Some(Self::Lysine),
            'L' => Some(Self::Leucine),
            'M' => Some(Self::Methionine),
            'N' => Some(Self::Asparagine),
            'P' => Some(Self::Proline),
            'Q' => Some(Self::Glutamine),
            'R' => Some(Self::Arginine),
            'S' => Some(Self::Serine),
            'T' => Some(Self::Threonine),
            'V' => Some(Self::Valine),
            'W' => Some(Self::Tryptophan),
            'Y' => Some(Self::Tyrosine),
            _ => None,
        }
    }

    pub fn partial_charge(self) -> f64 {
        match self {
            Self::Aspartate | Self::Glutamate => -1.0,
            Self::Lysine | Self::Arginine | Self::Histidine => 1.0,
            Self::Tyrosine
            | Self::Threonine
            | Self::Serine
            | Self::Asparagine
            | Self::Glutamine => 0.2,
            Self::Cysteine => -0.2,
            _ => 0.0,
        }
    }

    pub fn is_polar(self) -> bool {
        matches!(
            self,
            Self::Serine
                | Self::Threonine
                | Self::Tyrosine
                | Self::Asparagine
                | Self::Glutamine
                | Self::Histidine
                | Self::Lysine
                | Self::Arginine
                | Self::Aspartate
                | Self::Glutamate
        )
    }

    pub fn default_torsion_limit(self) -> f64 {
        match self {
            Self::Glycine => 180.0,
            Self::Proline => 70.0,
            _ => 120.0,
        }
    }

    pub fn mass(self) -> f64 {
        match self {
            Self::Glycine => 75.07,
            Self::Alanine => 89.09,
            Self::Serine => 105.09,
            Self::Valine => 117.15,
            Self::Leucine | Self::Isoleucine => 131.17,
            Self::Proline => 115.13,
            Self::Phenylalanine => 165.19,
            Self::Tyrosine => 181.19,
            Self::Tryptophan => 204.23,
            Self::Cysteine => 121.16,
            Self::Methionine => 149.21,
            Self::Threonine => 119.12,
            Self::Aspartate | Self::Asparagine => 132.12,
            Self::Glutamate | Self::Glutamine => 146.15,
            Self::Histidine => 155.16,
            Self::Lysine => 146.19,
            Self::Arginine => 174.20,
        }
    }
}
