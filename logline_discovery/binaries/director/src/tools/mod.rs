//! Tool implementations for Director operations

use async_trait::async_trait;
use serde_json::Value;
use crate::{Contract, Flow};
use logline_common::{Error, Result};

pub struct ToolResult {
    pub ok: bool,
    pub msg: String,
    pub extra: Option<Value>,
}

#[async_trait]
pub trait Tool {
    async fn run(&self, c: &Contract) -> ToolResult;
}

// === Tool Implementations ===

pub struct SubmitJob;

#[async_trait]
impl Tool for SubmitJob {
    async fn run(&self, c: &Contract) -> ToolResult {
        // Call our existing job submission system
        match crate::executor::job_submit::submit_job_from_contract(c).await {
            Ok(job_id) => ToolResult {
                ok: true,
                msg: format!("🎯 Job submitted: {}", job_id),
                extra: None,
            },
            Err(e) => ToolResult {
                ok: false,
                msg: format!("❌ Failed to submit job: {}", e),
                extra: None,
            },
        }
    }
}

pub struct Diagnose;

#[async_trait]
impl Tool for Diagnose {
    async fn run(&self, _c: &Contract) -> ToolResult {
        match crate::observability::metrics::generate_diagnostic_report().await {
            Ok(report) => ToolResult {
                ok: true,
                msg: report,
                extra: None,
            },
            Err(e) => ToolResult {
                ok: false,
                msg: format!("❌ Diagnostic failed: {}", e),
                extra: None,
            },
        }
    }
}

pub struct Monitor;

#[async_trait]
impl Tool for Monitor {
    async fn run(&self, _c: &Contract) -> ToolResult {
        match crate::observability::metrics::queue_status_summary().await {
            Ok(status) => ToolResult {
                ok: true,
                msg: status,
                extra: None,
            },
            Err(e) => ToolResult {
                ok: false,
                msg: format!("❌ Status check failed: {}", e),
                extra: None,
            },
        }
    }
}

// Back-office tools
pub struct RequeueStuck;

#[async_trait]
impl Tool for RequeueStuck {
    async fn run(&self, c: &Contract) -> ToolResult {
        let older_than = c.payload["older_than_minutes"].as_i64().unwrap_or(60);
        match crate::labops::requeue_stuck::execute(older_than).await {
            Ok(count) => ToolResult {
                ok: true,
                msg: format!("🔄 Requeued {} stuck jobs.", count),
                extra: None,
            },
            Err(e) => ToolResult {
                ok: false,
                msg: format!("❌ Failed to requeue stuck jobs: {}", e),
                extra: None,
            },
        }
    }
}

pub struct ScaleWorkers;

#[async_trait]
impl Tool for ScaleWorkers {
    async fn run(&self, c: &Contract) -> ToolResult {
        let delta = c.payload["delta_workers"].as_i64().unwrap_or(1);
        match crate::labops::scale_workers::adjust(delta).await {
            Ok(_) => ToolResult {
                ok: true,
                msg: format!("📈 Worker adjustment: delta={}", delta),
                extra: None,
            },
            Err(e) => ToolResult {
                ok: false,
                msg: format!("❌ Failed to scale workers: {}", e),
                extra: None,
            },
        }
    }
}

pub struct RotateLogs;

#[async_trait]
impl Tool for RotateLogs {
    async fn run(&self, _c: &Contract) -> ToolResult {
        match crate::labops::logs::rotate().await {
            Ok(_) => ToolResult {
                ok: true,
                msg: "🧹 Logs rotated.".into(),
                extra: None,
            },
            Err(e) => ToolResult {
                ok: false,
                msg: format!("❌ Failed to rotate logs: {}", e),
                extra: None,
            },
        }
    }
}

pub struct VacuumDb;

#[async_trait]
impl Tool for VacuumDb {
    async fn run(&self, _c: &Contract) -> ToolResult {
        match crate::labops::db::vacuum().await {
            Ok(_) => ToolResult {
                ok: true,
                msg: "🧽 VACUUM/REINDEX completed.".into(),
                extra: None,
            },
            Err(e) => ToolResult {
                ok: false,
                msg: format!("❌ VACUUM failed: {}", e),
                extra: None,
            },
        }
    }
}

pub struct BackupSnap;

#[async_trait]
impl Tool for BackupSnap {
    async fn run(&self, _c: &Contract) -> ToolResult {
        match crate::labops::backup::snapshot().await {
            Ok(info) => ToolResult {
                ok: true,
                msg: "💾 Backup executed.".into(),
                extra: Some(info),
            },
            Err(e) => ToolResult {
                ok: false,
                msg: format!("❌ Backup failed: {}", e),
                extra: None,
            },
        }
    }
}

pub struct DatasetRegister;

#[async_trait]
impl Tool for DatasetRegister {
    async fn run(&self, c: &Contract) -> ToolResult {
        match crate::labops::dataset::register(c.payload.clone()).await {
            Ok(_) => ToolResult {
                ok: true,
                msg: "📚 Dataset registered.".into(),
                extra: None,
            },
            Err(e) => ToolResult {
                ok: false,
                msg: format!("❌ Dataset registration failed: {}", e),
                extra: None,
            },
        }
    }
}

pub struct LabHealthcheck;

#[async_trait]
impl Tool for LabHealthcheck {
    async fn run(&self, _c: &Contract) -> ToolResult {
        match crate::labops::health::check().await {
            Ok(health) => ToolResult {
                ok: true,
                msg: format!("🩺 {}", health),
                extra: None,
            },
            Err(e) => ToolResult {
                ok: false,
                msg: format!("❌ Health check failed: {}", e),
                extra: None,
            },
        }
    }
}

pub struct HotReload;

#[async_trait]
impl Tool for HotReload {
    async fn run(&self, c: &Contract) -> ToolResult {
        let version = c.payload["version"].as_str().unwrap_or("latest");
        match crate::labops::hotreload::publish_signal(version).await {
            Ok(_) => ToolResult {
                ok: true,
                msg: format!("♻️ Hot-reload signaled (version={}).", version),
                extra: None,
            },
            Err(e) => ToolResult {
                ok: false,
                msg: format!("❌ Hot-reload failed: {}", e),
                extra: None,
            },
        }
    }
}
