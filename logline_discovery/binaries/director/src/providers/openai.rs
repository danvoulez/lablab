//! OpenAI provider implementation

use super::{UnifiedLLMProvider, IntentClassification, ProviderMetrics, ProviderType};
use async_trait::async_trait;
use async_openai::{
    Client as OpenAIClient,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestUserMessage, CreateChatCompletionRequestArgs,
    },
};
use std::env;
use tracing::{debug, info, warn};

pub struct OpenAIProvider {
    client: OpenAIClient<async_openai::config::OpenAIConfig>,
    model: String,
    provider_id: String,
}

impl OpenAIProvider {
    pub fn new(model: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| "OPENAI_API_KEY environment variable not set")?;

        let client = OpenAIClient::new().with_api_key(api_key);

        Ok(Self {
            client,
            model: model.to_string(),
            provider_id: format!("openai-{}", model),
        })
    }
    
    fn build_openai_prompt(user_input: &str) -> String {
        format!(r#"Classify this LogLine Discovery Lab request into one operation:

OPERATIONS: Monitor, SubmitJob, Diagnose, HealthCheck, BackupSnap, RequeueStuck, ScaleWorkers, RotateLogs, VacuumDb, DatasetRegister, HotReload, Unknown

PRIORITY: low, medium, high, critical

USER: "{}"

Respond ONLY with JSON:
{{"flow": "Monitor", "priority": "low", "confidence": 0.95, "reasoning": "User asking about status"}}"#, user_input)
    }
    
    fn parse_openai_response(content: &str) -> Result<IntentClassification, Box<dyn std::error::Error>> {
        // Try to extract JSON from the response
        let json_start = content.find('{');
        let json_end = content.rfind('}');
        
        if let (Some(start), Some(end)) = (json_start, json_end) {
            let json_str = &content[start..=end];
            debug!("Extracted JSON from OpenAI: {}", json_str);
            
            match serde_json::from_str::<serde_json::Value>(json_str) {
                Ok(parsed) => {
                    let classification = IntentClassification {
                        flow: parsed["flow"].as_str().unwrap_or("Unknown").to_string(),
                        priority: parsed["priority"].as_str().unwrap_or("low").to_string(),
                        confidence: parsed["confidence"].as_f64().unwrap_or(0.0) as f32,
                        reasoning: parsed["reasoning"].as_str().unwrap_or("").to_string(),
                        provider_used: "openai".to_string(), // Will be overridden
                        response_time_ms: 0,
                    };
                    
                    info!("OpenAI classification successful: {:?}", classification.flow);
                    Ok(classification)
                }
                Err(e) => {
                    warn!("Failed to parse OpenAI JSON: {}", e);
                    Err(e.into())
                }
            }
        } else {
            warn!("No JSON found in OpenAI response: {}", content);
            Err("No JSON in OpenAI response".into())
        }
    }
}

#[async_trait]
impl UnifiedLLMProvider for OpenAIProvider {
    async fn classify_intent(&self, input: &str) -> Result<IntentClassification, Box<dyn std::error::Error>> {
        let system_message = ChatCompletionRequestSystemMessage {
            content: "You are a LogLine Discovery Lab Director assistant. Classify user requests into operations and respond with JSON only.".to_string(),
            role: "system".to_string(),
            name: None,
        };

        let user_message = ChatCompletionRequestUserMessage {
            content: Self::build_openai_prompt(input),
            role: "user".to_string(),
            name: None,
        };

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages(vec![
                ChatCompletionRequestMessage::System(system_message),
                ChatCompletionRequestMessage::User(user_message),
            ])
            .temperature(0.1)
            .max_tokens(200u16)
            .build()?;

        info!("Sending request to OpenAI {}", self.model);
        let response = self.client.chat().completions().create(request).await?;

        if let Some(choice) = response.choices.first() {
            if let Some(content) = &choice.message.content {
                debug!("OpenAI response: {}", content);
                return Self::parse_openai_response(content);
            }
        }

        Err("No response from OpenAI".into())
    }
    
    async fn health_check(&self) -> Result<bool, Box<dyn std::error::Error>> {
        // Simple health check by listing models
        match self.client.models().list().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    fn get_metrics(&self) -> ProviderMetrics {
        let (cost, latency) = match self.model.as_str() {
            "gpt-4o" => (0.005, 1000),      // $0.005/1K tokens, ~1s
            "gpt-4o-mini" => (0.0005, 500), // $0.0005/1K tokens, ~0.5s
            _ => (0.002, 800),               // Default values
        };
        
        ProviderMetrics {
            cost_per_token: cost / 1000.0,
            avg_latency_ms: latency,
            success_rate: 0.99,
            supports_streaming: true,
            supports_function_calling: true,
        }
    }
    
    fn provider_id(&self) -> &str {
        &self.provider_id
    }
    
    fn provider_type(&self) -> ProviderType {
        ProviderType::Cloud { 
            api_key_env: "OPENAI_API_KEY".to_string() 
        }
    }
}
