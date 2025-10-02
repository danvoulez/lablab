//! Simple Ollama integration for LogLine Director with unified personality

use super::personality::DirectorPersonality;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub options: Option<OllamaOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaOptions {
    pub temperature: f32,
    pub top_p: f32,
    pub num_ctx: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaResponse {
    pub response: String,
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleClassification {
    pub flow: String,
    pub priority: String,
    pub confidence: f32,
    pub reasoning: String,
}

pub struct SimpleOllamaClient {
    pub client: Client,
    pub base_url: String,
    pub model: String,
}

impl SimpleOllamaClient {
    pub fn new(model: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: "http://localhost:11434".to_string(),
            model: model.to_string(),
        }
    }
    
    pub async fn is_available(&self) -> bool {
        match self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
        {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
    
    pub async fn classify_intent(&self, user_input: &str) -> Result<SimpleClassification, Box<dyn std::error::Error>> {
        // Use unified personality prompt for ALL models
        let prompt = DirectorPersonality::get_classification_prompt_with_context(user_input, None);
        let params = DirectorPersonality::get_optimal_parameters();
        
        debug!("Sending unified personality prompt to Ollama ({})", self.model);
        
        let request = OllamaRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
            options: Some(OllamaOptions {
                temperature: params.temperature,
                top_p: params.top_p,
                num_ctx: params.num_ctx,
            }),
        };

        let response = self.client
            .post(format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            warn!("Ollama request failed: {}", response.status());
            return Err(format!("Ollama request failed: {}", response.status()).into());
        }

        let ollama_response: OllamaResponse = response.json().await?;
        debug!("Ollama response: {}", ollama_response.response);

        self.parse_classification_response(&ollama_response.response)
    }
    
    fn parse_classification_response(&self, response: &str) -> Result<SimpleClassification, Box<dyn std::error::Error>> {
        // Try to extract JSON from the response
        let json_start = response.find('{');
        let json_end = response.rfind('}');
        
        if let (Some(start), Some(end)) = (json_start, json_end) {
            let json_str = &response[start..=end];
            debug!("Extracted JSON from Ollama: {}", json_str);
            
            match serde_json::from_str::<serde_json::Value>(json_str) {
                Ok(parsed) => {
                    let classification = SimpleClassification {
                        flow: parsed["flow"].as_str().unwrap_or("Unknown").to_string(),
                        priority: parsed["priority"].as_str().unwrap_or("low").to_string(),
                        confidence: parsed["confidence"].as_f64().unwrap_or(0.0) as f32,
                        reasoning: parsed["reasoning"].as_str().unwrap_or("").to_string(),
                    };
                    
                    info!("✅ Ollama classification successful: {:?} (confidence: {:.2})", 
                          classification.flow, classification.confidence);
                    Ok(classification)
                }
                Err(e) => {
                    warn!("Failed to parse Ollama JSON: {}, response: {}", e, json_str);
                    // Fallback to Unknown
                    Ok(SimpleClassification {
                        flow: "Unknown".to_string(),
                        priority: "low".to_string(),
                        confidence: 0.0,
                        reasoning: "Failed to parse Ollama response".to_string(),
                    })
                }
            }
        } else {
            warn!("No JSON found in Ollama response: {}", response);
            Ok(SimpleClassification {
                flow: "Unknown".to_string(),
                priority: "low".to_string(),
                confidence: 0.0,
                reasoning: "No JSON in Ollama response".to_string(),
            })
        }
    }
}
