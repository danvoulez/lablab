//! Unified contract system for all Director operations

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Workflow {
    JobQueue,
    LabOps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Flow {
    SubmitJob,
    Diagnose,
    Monitor,
    HotReload,
    RequeueStuck,
    ScaleWorkers,
    RotateLogs,
    VacuumDb,
    BackupSnap,
    DatasetRegister,
    LabHealthcheck,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub name: String,
    pub workflow: Workflow,
    pub flow: Flow,
    pub payload: Value,
    pub tags: Vec<String>,
    pub requested_by: String,   // LogLine ID / actor
    pub requires_approval: bool,
    pub severity: String,       // low|medium|high|critical
}
