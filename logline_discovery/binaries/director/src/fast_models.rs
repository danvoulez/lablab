//! Fast and reliable model selector

use crate::simple_llm::SimpleOllamaClient;
use tracing::{info, warn};

pub struct FastModelSelector;

impl FastModelSelector {
    pub async fn classify_with_fallback(user_input: &str) -> Result<crate::simple_llm::SimpleClassification, Box<dyn std::error::Error>> {
        // Try models in order of reliability and speed
        let models = vec![
            "director-smart",    // Most optimized
            "director-brain",    // Backup optimized
            "mistral:instruct",  // Standard instruct
            "mistral-director",  // Original
            "llama3.2:1b",      // Fast fallback
        ];

        for (i, model_name) in models.iter().enumerate() {
            info!("🧠 Trying model {} ({}/{})", model_name, i + 1, models.len());
            
            let client = SimpleOllamaClient::new(model_name);
            
            match client.classify_intent(user_input).await {
                Ok(result) => {
                    info!("✅ Model {} succeeded with confidence {:.2}", model_name, result.confidence);
                    return Ok(result);
                }
                Err(e) => {
                    warn!("❌ Model {} failed: {}", model_name, e);
                    continue;
                }
            }
        }

        Err("All models failed".into())
    }
}
