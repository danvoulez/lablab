//! Ollama provider implementation

use super::{UnifiedLLMProvider, IntentClassification, ProviderMetrics, ProviderType};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, warn};

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

pub struct OllamaProvider {
    client: Client,
    base_url: String,
    model: String,
    provider_id: String,
}

impl OllamaProvider {
    pub fn new(model: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: "http://localhost:11434".to_string(),
            model: model.to_string(),
            provider_id: format!("ollama-{}", model),
        }
    }
    
    pub fn with_endpoint(model: &str, endpoint: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: endpoint.to_string(),
            model: model.to_string(),
            provider_id: format!("ollama-{}", model),
        }
    }
    
    fn build_classification_prompt(&self, user_input: &str) -> String {
        format!(r#"You are a LogLine Discovery Lab Director assistant. Classify the user's request into one of these operations:

AVAILABLE OPERATIONS:
- Monitor: Check queue status, system status ("como está a fila", "status", "queue")
- SubmitJob: Process data, submit jobs ("processa", "submete", "analisa", "roda")
- Diagnose: Check for failures, errors ("diagnóstico", "falhas", "erros", "problemas")
- HealthCheck: System health ("saúde", "health", "sistema")
- BackupSnap: Database backup ("backup", "cópia", "snapshot")
- RequeueStuck: Requeue stuck jobs ("reenvia", "reenfileira", "travados")
- ScaleWorkers: Scale workers up/down ("escala", "workers", "sobe", "desce")
- RotateLogs: Rotate logs ("rotaciona logs", "limpa logs")
- VacuumDb: Database maintenance ("vacuum", "otimiza", "índices")
- DatasetRegister: Register datasets ("registra dataset", "dataset")
- HotReload: Reload logic ("hot reload", "atualiza lógica")
- Unknown: Cannot classify

PRIORITY LEVELS: low, medium, high, critical

USER REQUEST: "{}"

Respond ONLY with valid JSON in this exact format:
{{"flow": "Monitor", "priority": "low", "confidence": 0.95, "reasoning": "User is asking about queue status"}}

JSON:"#, user_input)
    }
    
    fn parse_classification_response(&self, response: &str) -> Result<IntentClassification, Box<dyn std::error::Error>> {
        // Try to extract JSON from the response
        let json_start = response.find('{');
        let json_end = response.rfind('}');
        
        if let (Some(start), Some(end)) = (json_start, json_end) {
            let json_str = &response[start..=end];
            debug!("Extracted JSON from Ollama: {}", json_str);
            
            match serde_json::from_str::<serde_json::Value>(json_str) {
                Ok(parsed) => {
                    let classification = IntentClassification {
                        flow: parsed["flow"].as_str().unwrap_or("Unknown").to_string(),
                        priority: parsed["priority"].as_str().unwrap_or("low").to_string(),
                        confidence: parsed["confidence"].as_f64().unwrap_or(0.0) as f32,
                        reasoning: parsed["reasoning"].as_str().unwrap_or("").to_string(),
                        provider_used: self.provider_id.clone(),
                        response_time_ms: 0, // Will be set by orchestrator
                    };
                    
                    debug!("Successfully parsed Ollama classification: {:?}", classification);
                    Ok(classification)
                }
                Err(e) => {
                    warn!("Failed to parse Ollama JSON: {}, response: {}", e, json_str);
                    // Fallback to Unknown
                    Ok(IntentClassification {
                        flow: "Unknown".to_string(),
                        priority: "low".to_string(),
                        confidence: 0.0,
                        reasoning: "Failed to parse Ollama response".to_string(),
                        provider_used: self.provider_id.clone(),
                        response_time_ms: 0,
                    })
                }
            }
        } else {
            warn!("No JSON found in Ollama response: {}", response);
            Ok(IntentClassification {
                flow: "Unknown".to_string(),
                priority: "low".to_string(),
                confidence: 0.0,
                reasoning: "No JSON in Ollama response".to_string(),
                provider_used: self.provider_id.clone(),
                response_time_ms: 0,
            })
        }
    }
}

#[async_trait]
impl UnifiedLLMProvider for OllamaProvider {
    async fn classify_intent(&self, input: &str) -> Result<IntentClassification, Box<dyn std::error::Error>> {
        let prompt = self.build_classification_prompt(input);
        
        debug!("Sending prompt to Ollama ({}): {}", self.model, prompt);
        
        let request = OllamaRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
            options: Some(OllamaOptions {
                temperature: 0.1, // Low temperature for consistent classification
                top_p: 0.9,
                num_ctx: 4096,
            }),
        };

        let response = self.client
            .post(&format!("{}/api/generate", self.base_url))
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
    
    async fn health_check(&self) -> Result<bool, Box<dyn std::error::Error>> {
        match self.client
            .get(&format!("{}/api/tags", self.base_url))
            .send()
            .await
        {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
    
    fn get_metrics(&self) -> ProviderMetrics {
        ProviderMetrics {
            cost_per_token: 0.0, // Local model, no cost
            avg_latency_ms: if self.model.contains("mistral") { 500 } else { 200 },
            success_rate: 0.95,
            supports_streaming: true,
            supports_function_calling: false,
        }
    }
    
    fn provider_id(&self) -> &str {
        &self.provider_id
    }
    
    fn provider_type(&self) -> ProviderType {
        ProviderType::Local { 
            endpoint: self.base_url.clone() 
        }
    }
}
