use crate::trajectory::Trajectory;

/// Tracks rolling entropy and information metrics for the active trajectory.
#[derive(Debug, Default)]
pub struct EntropyMeter {
    entropy_budget: f64,
    information_budget: f64,
}

impl EntropyMeter {
    pub fn new(entropy_budget: f64, information_budget: f64) -> Self {
        Self {
            entropy_budget,
            information_budget,
        }
    }

    pub fn entropy_budget(&self) -> f64 {
        self.entropy_budget
    }

    pub fn information_budget(&self) -> f64 {
        self.information_budget
    }

    pub fn remaining_entropy(&self, trajectory: &Trajectory) -> f64 {
        (self.entropy_budget - trajectory.total_entropy()).max(0.0)
    }

    pub fn remaining_information(&self, trajectory: &Trajectory) -> f64 {
        (self.information_budget - trajectory.total_information()).max(0.0)
    }
}
