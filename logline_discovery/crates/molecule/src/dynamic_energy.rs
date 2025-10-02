use crate::{
    aminoacid::ResidueId,
    chain::PeptideChain,
    parameters::{self, ResidueClass},
};

/// Snapshot of energy for a residue during simulation.
#[derive(Clone, Debug)]
pub struct EnergySample {
    pub residue: ResidueId,
    pub potential_energy: f64,
    pub kinetic_energy: f64,
}

/// Summary of global energy contributions.
#[derive(Clone, Debug, Default)]
pub struct EnergySummary {
    pub bond: f64,
    pub angle: f64,
    pub dihedral: f64,
    pub van_der_waals: f64,
    pub electrostatic: f64,
    pub hydrogen_bond: f64,
}

impl EnergySummary {
    pub fn total(&self) -> f64 {
        self.bond
            + self.angle
            + self.dihedral
            + self.van_der_waals
            + self.electrostatic
            + self.hydrogen_bond
    }
}

#[derive(Clone, Debug, Default)]
pub struct FoldStepEnergy {
    pub bond: f64,
    pub angle: f64,
    pub dihedral: f64,
    pub van_der_waals: f64,
    pub electrostatic: f64,
    pub hydrogen_bond: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct EnvironmentPreset {
    pub name: &'static str,
    pub dielectric: f64,
    pub hydrogen_strength: f64,
    pub default_temperature: f64,
}

impl EnvironmentPreset {
    pub fn aqueous() -> Self {
        Self {
            name: "aqueous",
            dielectric: 78.5,
            hydrogen_strength: 0.6,
            default_temperature: 310.0,
        }
    }

    pub fn vacuum() -> Self {
        Self {
            name: "vacuum",
            dielectric: 1.0,
            hydrogen_strength: 0.0,
            default_temperature: 298.0,
        }
    }

    pub fn membrane() -> Self {
        Self {
            name: "membrane",
            dielectric: 10.0,
            hydrogen_strength: 0.4,
            default_temperature: 305.0,
        }
    }

    pub fn by_name(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().as_str() {
            "aqueous" | "water" => Some(Self::aqueous()),
            "vacuum" => Some(Self::vacuum()),
            "membrane" | "lipid" => Some(Self::membrane()),
            _ => None,
        }
    }
}

/// Simplified dynamic energy model for the folding simulation.
#[derive(Clone, Debug)]
pub struct EnergyModel {
    pub scaling_factor: f64,
    pub bond_k: f64,
    pub angle_k: f64,
    pub dihedral_k: f64,
    pub dihedral_multiplicity: f64,
    pub dihedral_phase: f64,
    pub dielectric: f64,
    pub hydrogen_strength: f64,
    pub environment: EnvironmentPreset,
}

impl Default for EnergyModel {
    fn default() -> Self {
        let env = EnvironmentPreset::aqueous();
        Self {
            scaling_factor: 1.0,
            bond_k: 80.0,
            angle_k: 12.0,
            dihedral_k: 2.5,
            dihedral_multiplicity: 3.0,
            dihedral_phase: 0.0,
            dielectric: env.dielectric,
            hydrogen_strength: env.hydrogen_strength,
            environment: env,
        }
    }
}

impl EnergyModel {
    pub fn new(scaling_factor: f64) -> Self {
        Self {
            scaling_factor,
            ..Self::default()
        }
    }

    pub fn with_environment(mut self, environment: EnvironmentPreset) -> Self {
        self.dielectric = environment.dielectric;
        self.hydrogen_strength = environment.hydrogen_strength;
        self.environment = environment;
        self
    }

    pub fn environment(&self) -> EnvironmentPreset {
        self.environment
    }

    pub fn energy_summary(&self, chain: &PeptideChain) -> EnergySummary {
        let residues = chain.residues();
        if residues.len() < 2 {
            return EnergySummary::default();
        }

        let mut summary = EnergySummary::default();

        // Bonds
        for window in residues.windows(2) {
            if let [left, right] = window {
                let class_left = parameters::classify(left.amino_acid);
                let class_right = parameters::classify(right.amino_acid);
                let equilibrium = parameters::bond_equilibrium_distance(class_left, class_right);
                let distance = distance(left.position(), right.position());
                let diff = distance - equilibrium;
                summary.bond += 0.5 * self.bond_k * diff * diff;
            }
        }

        // Angles
        for window in residues.windows(3) {
            let [p1, p2, p3] = match window {
                [a, b, c] => [a, b, c],
                _ => continue,
            };
            let class_middle = parameters::classify(p2.amino_acid);
            let equilibrium = parameters::angle_equilibrium(class_middle);
            let angle = bond_angle(p1.position(), p2.position(), p3.position());
            let diff = angle - equilibrium;
            summary.angle += 0.5 * self.angle_k * diff * diff;
        }

        // Dihedral
        for window in residues.windows(4) {
            let [p1, p2, p3, p4] = match window {
                [a, b, c, d] => [a, b, c, d],
                _ => continue,
            };
            let dihedral =
                dihedral_angle(p1.position(), p2.position(), p3.position(), p4.position());
            let energy = self.dihedral_k
                * (1.0 + (self.dihedral_multiplicity * dihedral - self.dihedral_phase).cos());
            summary.dihedral += energy;
        }

        // Pairwise interactions
        let mut vdw = 0.0;
        let mut electro = 0.0;
        let mut hydrogen = 0.0;
        for (i, left) in residues.iter().enumerate() {
            let class_left = parameters::classify(left.amino_acid);
            for right in residues.iter().skip(i + 1) {
                let class_right = parameters::classify(right.amino_acid);
                let distance = distance(left.position(), right.position()).max(0.1);

                let (sigma, epsilon) = parameters::lennard_jones_params(class_left, class_right);
                let sr = sigma / distance;
                let sr6 = sr.powi(6);
                vdw += 4.0 * epsilon * (sr6 * sr6 - sr6);

                let charge_product =
                    left.amino_acid.partial_charge() * right.amino_acid.partial_charge();
                if charge_product.abs() > f64::EPSILON {
                    electro += 332.0636 * charge_product / (self.dielectric * distance);
                }

                if is_hbond_candidate(class_left, class_right) && distance < 3.5 {
                    let r10 = distance.powi(10);
                    let r12 = distance.powi(12);
                    hydrogen += -self.hydrogen_strength * ((1.0 / r12) - (1.0 / r10));
                }
            }
        }

        summary.bond *= self.scaling_factor;
        summary.angle *= self.scaling_factor;
        summary.dihedral *= self.scaling_factor;
        summary.van_der_waals = vdw * self.scaling_factor;
        summary.electrostatic = electro * self.scaling_factor;
        summary.hydrogen_bond = hydrogen * self.scaling_factor;
        summary
    }

    pub fn total_energy(&self, chain: &PeptideChain) -> f64 {
        self.energy_summary(chain).total()
    }

    pub fn evaluate_chain(&self, chain: &PeptideChain) -> Vec<EnergySample> {
        let residues = chain.residues();
        let summary = self.energy_summary(chain);
        let per_residue = if residues.is_empty() {
            0.0
        } else {
            summary.total() / residues.len() as f64
        };

        residues
            .iter()
            .map(|residue| EnergySample {
                residue: residue.id,
                potential_energy: per_residue,
                kinetic_energy: residue.phi.abs() + residue.psi.abs(),
            })
            .collect()
    }
}

fn distance(left: [f64; 3], right: [f64; 3]) -> f64 {
    ((left[0] - right[0]).powi(2) + (left[1] - right[1]).powi(2) + (left[2] - right[2]).powi(2))
        .sqrt()
}

fn normalize(v: [f64; 3]) -> [f64; 3] {
    let norm = (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).sqrt();
    if norm < 1e-8 {
        [0.0, 0.0, 0.0]
    } else {
        [v[0] / norm, v[1] / norm, v[2] / norm]
    }
}

fn subtract(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn bond_angle(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> f64 {
    let v1 = normalize(subtract(a, b));
    let v2 = normalize(subtract(c, b));
    let dot = dot(v1, v2).clamp(-1.0, 1.0);
    dot.acos()
}

fn dihedral_angle(p0: [f64; 3], p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> f64 {
    let b0 = normalize(subtract(p1, p0));
    let b1 = normalize(subtract(p2, p1));
    let b2 = normalize(subtract(p3, p2));

    let n1 = normalize(cross(b0, b1));
    let n2 = normalize(cross(b1, b2));
    let m1 = cross(n1, normalize(b1));

    let x = dot(n1, n2);
    let y = dot(m1, n2);
    y.atan2(x)
}

fn is_hbond_candidate(left: ResidueClass, right: ResidueClass) -> bool {
    matches!(
        (left, right),
        (ResidueClass::Polar, ResidueClass::Polar)
            | (ResidueClass::Polar, ResidueClass::Positive)
            | (ResidueClass::Polar, ResidueClass::Negative)
            | (ResidueClass::Positive, ResidueClass::Polar)
            | (ResidueClass::Negative, ResidueClass::Polar)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AminoAcid, PeptideChain, Residue, ResidueId};

    #[test]
    fn environment_changes_dielectric() {
        let model_default = EnergyModel::default();
        let vacuum = model_default
            .clone()
            .with_environment(EnvironmentPreset::vacuum());
        assert!(model_default.dielectric > vacuum.dielectric);
        assert!(model_default.hydrogen_strength > vacuum.hydrogen_strength);
    }

    #[test]
    fn bond_energy_uses_class_specific_distance() {
        let residues = vec![
            Residue::new(ResidueId(0), AminoAcid::Lysine).with_position([0.0, 0.0, 0.0]),
            Residue::new(ResidueId(1), AminoAcid::Glutamate).with_position([1.3, 0.0, 0.0]),
        ];
        let chain = PeptideChain::new(residues);
        let model = EnergyModel::default();
        let summary = model.energy_summary(&chain);
        let class_left = parameters::classify(AminoAcid::Lysine);
        let class_right = parameters::classify(AminoAcid::Glutamate);
        let eq = parameters::bond_equilibrium_distance(class_left, class_right);
        let expected = 0.5 * model.bond_k * (1.3 - eq).powi(2);
        assert!((summary.bond - expected).abs() < 1e-6);
    }
}
