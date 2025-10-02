//! Google Gemini provider using OpenAI compatibility layer

use super::{UnifiedLLMProvider, IntentClassification, ProviderMetrics, ProviderType};
use async_trait::async_trait;
use async_openai::{
    Client as OpenAIClient,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestUserMessage, CreateChatCompletionRequestArgs,
    },
    config::OpenAIConfig,
};
use std::env;
use tracing::{debug, info, warn};

pub struct GeminiProvider {
    client: OpenAIClient<OpenAIConfig>,
    model: String,
    provider_id: String,
}

impl GeminiProvider {
    pub fn new(model: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let api_key = env::var("GEMINI_API_KEY")
            .map_err(|_| "GEMINI_API_KEY environment variable not set")?;

        // Use OpenAI client but point to Gemini endpoint
        let config = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base("https://generativelanguage.googleapis.com/v1beta/openai");
            
        let client = OpenAIClient::with_config(config);

        Ok(Self {
            client,
            model: model.to_string(),
            provider_id: format!("gemini-{}", model),
        })
    }
    
    fn build_gemini_prompt(user_input: &str) -> String {
        format!(r#"You are a LogLine Discovery Lab Director assistant. Classify user requests precisely.

OPERATIONS: Monitor, SubmitJob, Diagnose, HealthCheck, BackupSnap, RequeueStuck, ScaleWorkers, RotateLogs, VacuumDb, DatasetRegister, HotReload, Unknown

PRIORITY: low, medium, high, critical

USER REQUEST: "{}"

Respond with JSON only:
{{"flow": "Monitor", "priority": "low", "confidence": 0.95, "reasoning": "User asking about queue status"}}"#, user_input)
    }
    
    fn parse_gemini_response(content: &str) -> Result<IntentClassification, Box<dyn std::error::Error>> {
        // Try to extract JSON from the response
        let json_start = content.find('{');
        let json_end = content.rfind('}');
        
        if let (Some(start), Some(end)) = (json_start, json_end) {
            let json_str = &content[start..=end];
            debug!("Extracted JSON from Gemini: {}", json_str);
            
            match serde_json::from_str::<serde_json::Value>(json_str) {
                Ok(parsed) => {
                    let classification = IntentClassification {
                        flow: parsed["flow"].as_str().unwrap_or("Unknown").to_string(),
                        priority: parsed["priority"].as_str().unwrap_or("low").to_string(),
                        confidence: parsed["confidence"].as_f64().unwrap_or(0.0) as f32,
                        reasoning: parsed["reasoning"].as_str().unwrap_or("").to_string(),
                        provider_used: "gemini".to_string(), // Will be overridden
                        response_time_ms: 0,
                    };
                    
                    info!("Gemini classification successful: {:?}", classification.flow);
                    Ok(classification)
                }
                Err(e) => {
                    warn!("Failed to parse Gemini JSON: {}", e);
                    Err(e.into())
                }
            }
        } else {
            warn!("No JSON found in Gemini response: {}", content);
            Err("No JSON in Gemini response".into())
        }
    }
}

#[async_trait]
impl UnifiedLLMProvider for GeminiProvider {
    async fn classify_intent(&self, input: &str) -> Result<IntentClassification, Box<dyn std::error::Error>> {
        let system_message = ChatCompletionRequestSystemMessage {
            content: "You are a LogLine Discovery Lab Director assistant. You classify user requests into specific operations and respond with structured JSON.".to_string(),
            role: "system".to_string(),
            name: None,
        };

        let user_message = ChatCompletionRequestUserMessage {
            content: Self::build_gemini_prompt(input),
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

        info!("Sending request to Gemini {}", self.model);
        let response = self.client.chat().completions().create(request).await?;

        if let Some(choice) = response.choices.first() {
            if let Some(content) = &choice.message.content {
                debug!("Gemini response: {}", content);
                return Self::parse_gemini_response(content);
            }
        }

        Err("No response from Gemini".into())
    }
    
    async fn health_check(&self) -> Result<bool, Box<dyn std::error::Error>> {
        // Simple health check - try a minimal request
        let test_request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages(vec![
                ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                    content: "Hello".to_string(),
                    role: "user".to_string(),
                    name: None,
                })
            ])
            .max_tokens(1u16)
            .build()?;
            
        match self.client.chat().completions().create(test_request).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    fn get_metrics(&self) -> ProviderMetrics {
        let (cost, latency) = match self.model.as_str() {
            "gemini-2.0-flash" => (0.001, 600),           // Fast and cheap
            "gemini-2.0-flash-thinking" => (0.002, 800),  // Thinking model
            "gemini-2.5-flash" => (0.0015, 700),          // Balanced
            _ => (0.001, 600),                             // Default
        };
        
        ProviderMetrics {
            cost_per_token: cost / 1000.0,
            avg_latency_ms: latency,
            success_rate: 0.97,
            supports_streaming: true,
            supports_function_calling: true,
        }
    }
    
    fn provider_id(&self) -> &str {
        &self.provider_id
    }
    
    fn provider_type(&self) -> ProviderType {
        ProviderType::Hybrid { 
            primary: "gemini".to_string(),
            fallback: "openai".to_string(),
        }
    }
}
