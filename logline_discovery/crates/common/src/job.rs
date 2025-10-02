use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "job_status", rename_all = "lowercase")]
pub enum JobStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "job_priority", rename_all = "lowercase")]
pub enum JobPriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub status: JobStatus,
    pub priority: JobPriority,
    pub payload: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub worker_id: Option<String>,
    pub error_message: Option<String>,
    pub retry_count: i32,
    pub max_retries: i32,
}

impl Job {
    pub fn new(name: String, payload: serde_json::Value, priority: JobPriority) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            status: JobStatus::Queued,
            priority,
            payload,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            worker_id: None,
            error_message: None,
            retry_count: 0,
            max_retries: 3,
        }
    }

    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries && matches!(self.status, JobStatus::Failed)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct JobExecution {
    pub id: Uuid,
    pub job_id: Uuid,
    pub worker_id: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub success: bool,
    pub error_message: Option<String>,
    pub metrics: serde_json::Value,
}
