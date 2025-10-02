//! Timeline recording for audit trail

use crate::Contract;
use chrono::Utc;
use uuid::Uuid;

pub fn record_span(event: &str, contract: &Contract) {
    println!(
        "🧾 [span] {} :: {} -> {:?}",
        Utc::now().to_rfc3339(),
        event,
        contract.name
    );
}
