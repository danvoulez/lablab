//! Policy engine for permissions and approvals

use crate::{Contract, Flow};

pub struct Policy;

impl Policy {
    pub fn requires_approval(c: &Contract) -> bool {
        matches!(
            c.flow,
            Flow::VacuumDb | Flow::ScaleWorkers | Flow::HotReload | Flow::BackupSnap
        ) && (c.severity == "high" || c.severity == "critical")
    }

    pub fn allowed(actor_role: &str, c: &Contract) -> bool {
        // Operators can do everything except VACUUM/HotReload without approval
        match (actor_role, &c.flow) {
            ("admin", _) => true,
            ("operator", Flow::VacuumDb | Flow::HotReload) => false,
            ("operator", _) => true,
            ("viewer", Flow::Monitor | Flow::Diagnose) => true,
            _ => false,
        }
    }
}
