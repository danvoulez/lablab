use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationMode {
    Reversible,
    LiveSync,
}

impl std::str::FromStr for OperationMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "reversible" => Ok(OperationMode::Reversible),
            "live-sync" | "live_sync" | "livesync" => Ok(OperationMode::LiveSync),
            _ => Err(format!("unknown mode: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractVersion(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanRecord {
    pub id: String,
    pub who: String,
    pub did: String,
    pub what: String,
    pub when_epoch_ms: u128,
    pub tags: Vec<String>,
    pub delta_s: f64,
    pub effort: f64,
    pub metadata: serde_json::Value,
}

impl SpanRecord {
    pub fn now() -> u128 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractEnvelope {
    pub version: ContractVersion,
    pub audit_id: AuditId,
    pub kind: String,
    pub payload: serde_json::Value,
}

pub fn new_audit_id() -> AuditId {
    use rand::{distributions::Alphanumeric, Rng};
    let id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    AuditId(format!("AUDIT-{id}"))
}
