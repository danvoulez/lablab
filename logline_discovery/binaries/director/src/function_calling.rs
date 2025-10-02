//! Function calling system for dynamic laboratory data access
//! 
//! This module provides tools that the LLM can call to get real-time
//! information about the laboratory state.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionResult {
    pub success: bool,
    pub data: serde_json::Value,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStatus {
    pub queued: u32,
    pub running: u32,
    pub failed: u32,
    pub succeeded: u32,
    pub total_workers: u32,
    pub active_workers: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionSummary {
    pub id: String,
    pub job_type: String,
    pub status: String,
    pub subject: Option<String>,
    pub error_message: Option<String>,
    pub started_at: String,
    pub duration_seconds: Option<u64>,
}

pub struct FunctionRegistry {
    functions: HashMap<String, String>, // function_name -> description
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionRegistry {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        
        functions.insert(
            "get_current_queue_status".to_string(),
            "Get the current status of job queues including counts of queued, running, failed, and succeeded jobs".to_string()
        );
        
        functions.insert(
            "find_failed_simulations".to_string(),
            "Find recent failed simulations, optionally filtered by subject (e.g., 'HIV', 'malaria')".to_string()
        );
        
        functions.insert(
            "get_recent_successes".to_string(),
            "Get recent successful analyses and simulations, optionally filtered by subject".to_string()
        );
        
        functions.insert(
            "check_worker_health".to_string(),
            "Check the health status of computational workers".to_string()
        );
        
        functions.insert(
            "get_system_resources".to_string(),
            "Get current system resource usage (CPU, memory, disk)".to_string()
        );

        Self { functions }
    }

    pub fn get_function_descriptions(&self) -> String {
        let mut descriptions = String::from("FERRAMENTAS DISPONÍVEIS:\n\n");
        
        for (name, desc) in &self.functions {
            descriptions.push_str(&format!("• {}: {}\n", name, desc));
        }
        
        descriptions.push_str("\nPara usar uma ferramenta, inclua no seu JSON de resposta:\n");
        descriptions.push_str("\"function_call\": {\"name\": \"get_current_queue_status\", \"arguments\": {}}\n");
        
        descriptions
    }

    pub async fn execute_function(&self, call: &FunctionCall) -> FunctionResult {
        info!("🔧 Executing function: {} with args: {:?}", call.name, call.arguments);
        
        match call.name.as_str() {
            "get_current_queue_status" => self.get_current_queue_status().await,
            "find_failed_simulations" => {
                let subject = call.arguments.get("subject")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                self.find_failed_simulations(subject).await
            }
            "get_recent_successes" => {
                let subject = call.arguments.get("subject")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                self.get_recent_successes(subject).await
            }
            "check_worker_health" => self.check_worker_health().await,
            "get_system_resources" => self.get_system_resources().await,
            _ => FunctionResult {
                success: false,
                data: serde_json::Value::Null,
                error: Some(format!("Unknown function: {}", call.name)),
            }
        }
    }

    // Function implementations (mock data for now - would connect to real systems)
    
    async fn get_current_queue_status(&self) -> FunctionResult {
        // Mock data - in real implementation, would query actual queue system
        let status = QueueStatus {
            queued: 12,
            running: 3,
            failed: 1,
            succeeded: 247,
            total_workers: 8,
            active_workers: 6,
        };

        FunctionResult {
            success: true,
            data: serde_json::to_value(status).unwrap(),
            error: None,
        }
    }

    async fn find_failed_simulations(&self, subject: Option<String>) -> FunctionResult {
        // Mock data - would query actual execution logs
        let mut failures = vec![
            ExecutionSummary {
                id: "sim_hiv_001".to_string(),
                job_type: "molecular_docking".to_string(),
                status: "failed".to_string(),
                subject: Some("HIV".to_string()),
                error_message: Some("Insufficient memory: required 32GB, available 16GB".to_string()),
                started_at: "2025-10-01T10:30:00Z".to_string(),
                duration_seconds: Some(1800),
            },
            ExecutionSummary {
                id: "qsar_malaria_003".to_string(),
                job_type: "qsar_analysis".to_string(),
                status: "failed".to_string(),
                subject: Some("malaria".to_string()),
                error_message: Some("Dataset corruption detected in molecular descriptors".to_string()),
                started_at: "2025-10-01T14:15:00Z".to_string(),
                duration_seconds: Some(300),
            },
        ];

        // Filter by subject if provided
        if let Some(filter_subject) = subject {
            failures.retain(|f| {
                f.subject.as_ref()
                    .map(|s| s.to_lowercase().contains(&filter_subject.to_lowercase()))
                    .unwrap_or(false)
            });
        }

        FunctionResult {
            success: true,
            data: serde_json::to_value(failures).unwrap(),
            error: None,
        }
    }

    async fn get_recent_successes(&self, subject: Option<String>) -> FunctionResult {
        // Mock data - would query actual results database
        let mut successes = vec![
            ExecutionSummary {
                id: "md_covid_spike".to_string(),
                job_type: "molecular_dynamics".to_string(),
                status: "completed".to_string(),
                subject: Some("COVID-19".to_string()),
                error_message: None,
                started_at: "2025-10-01T09:00:00Z".to_string(),
                duration_seconds: Some(7200),
            },
            ExecutionSummary {
                id: "qsar_malaria_002".to_string(),
                job_type: "qsar_analysis".to_string(),
                status: "completed".to_string(),
                subject: Some("malaria".to_string()),
                error_message: None,
                started_at: "2025-10-01T11:30:00Z".to_string(),
                duration_seconds: Some(1200),
            },
        ];

        if let Some(filter_subject) = subject {
            successes.retain(|s| {
                s.subject.as_ref()
                    .map(|subj| subj.to_lowercase().contains(&filter_subject.to_lowercase()))
                    .unwrap_or(false)
            });
        }

        FunctionResult {
            success: true,
            data: serde_json::to_value(successes).unwrap(),
            error: None,
        }
    }

    async fn check_worker_health(&self) -> FunctionResult {
        // Mock data - would check actual worker nodes
        let health_data = serde_json::json!({
            "total_workers": 8,
            "healthy_workers": 6,
            "unhealthy_workers": 2,
            "workers": [
                {"id": "worker-001", "status": "healthy", "cpu_usage": 45.2, "memory_usage": 78.1},
                {"id": "worker-002", "status": "healthy", "cpu_usage": 67.8, "memory_usage": 82.3},
                {"id": "worker-003", "status": "unhealthy", "cpu_usage": 0.0, "memory_usage": 0.0, "error": "Connection timeout"},
                {"id": "worker-004", "status": "healthy", "cpu_usage": 23.1, "memory_usage": 45.7}
            ]
        });

        FunctionResult {
            success: true,
            data: health_data,
            error: None,
        }
    }

    async fn get_system_resources(&self) -> FunctionResult {
        // Mock data - would use sysinfo or similar
        let resources = serde_json::json!({
            "cpu_usage_percent": 34.5,
            "memory_total_gb": 128,
            "memory_used_gb": 89.2,
            "memory_usage_percent": 69.7,
            "disk_total_tb": 10.0,
            "disk_used_tb": 6.8,
            "disk_usage_percent": 68.0,
            "gpu_count": 4,
            "gpu_usage_percent": [23.1, 45.6, 78.9, 12.3]
        });

        FunctionResult {
            success: true,
            data: resources,
            error: None,
        }
    }
}
