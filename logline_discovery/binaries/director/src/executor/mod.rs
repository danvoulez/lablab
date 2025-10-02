//! Integration with existing job queue system

use crate::Contract;
use logline_common::Result;
use uuid::Uuid;

pub mod job_submit {
    use super::*;

    pub async fn submit_job_from_contract(contract: &Contract) -> Result<Uuid> {
        // Extract job details from contract payload
        let job_type = contract.payload["job_type"].as_str().unwrap_or("span_processing");
        let priority_str = contract.payload["priority"].as_str().unwrap_or("normal");

        let priority = match priority_str {
            "low" => logline_common::job::JobPriority::Low,
            "normal" => logline_common::job::JobPriority::Normal,
            "high" => logline_common::job::JobPriority::High,
            "critical" => logline_common::job::JobPriority::Critical,
            _ => logline_common::job::JobPriority::Normal,
        };

        let payload = serde_json::json!({
            "type": job_type,
            "source": "director",
            "contract_id": contract.name,
        });

        // Use our existing job submission mechanism
        let job = logline_common::job::Job::new(
            format!("director::{}", contract.name),
            payload,
            priority,
        );

        // TODO: Integrate with actual job_client submission
        // For now, return the job ID
        Ok(job.id)
    }
}
