//! LLM integration with Ollama and OpenAI for natural language understanding

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, warn, info};
use async_openai::{
    Client as OpenAIClient,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestUserMessage, CreateChatCompletionRequestArgs,
    },
};
use std::env;

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
    pub max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaResponse {
    pub response: String,
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentClassification {
    pub flow: String,
    pub priority: String,
    pub confidence: f32,
    pub reasoning: String,
}

pub struct LlmClient {
    client: Client,
    base_url: String,
    model: String,
}

impl LlmClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "http://localhost:11434".to_string(),
            model: "mistral-director".to_string(), // Use Mistral 7B for better understanding
        }
    }

    pub fn with_fallback() -> Self {
        Self {
            client: Client::new(),
            base_url: "http://localhost:11434".to_string(),
            model: "llama3.2:1b".to_string(), // Fallback to smaller model
        }
    }

    pub async fn classify_intent(&self, user_input: &str) -> Result<IntentClassification, Box<dyn std::error::Error>> {
        let prompt = self.build_classification_prompt(user_input);
        
        debug!("Sending prompt to Ollama: {}", prompt);
        
        let request = OllamaRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
            options: Some(OllamaOptions {
                temperature: 0.1, // Low temperature for consistent classification
                top_p: 0.9,
                max_tokens: 200,
            }),
        };

        let response = self.client
            .post(&format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            warn!("Ollama request failed: {}", response.status());
            return Err("Ollama request failed".into());
        }

        let ollama_response: OllamaResponse = response.json().await?;
        debug!("Ollama response: {}", ollama_response.response);

        self.parse_classification_response(&ollama_response.response)
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
            debug!("Extracted JSON: {}", json_str);
            
            match serde_json::from_str::<IntentClassification>(json_str) {
                Ok(classification) => {
                    debug!("Successfully parsed classification: {:?}", classification);
                    Ok(classification)
                }
                Err(e) => {
                    warn!("Failed to parse JSON: {}, response: {}", e, json_str);
                    // Fallback to Unknown
                    Ok(IntentClassification {
                        flow: "Unknown".to_string(),
                        priority: "low".to_string(),
                        confidence: 0.0,
                        reasoning: "Failed to parse LLM response".to_string(),
                    })
                }
            }
        } else {
            warn!("No JSON found in response: {}", response);
            Ok(IntentClassification {
                flow: "Unknown".to_string(),
                priority: "low".to_string(),
                confidence: 0.0,
                reasoning: "No JSON in LLM response".to_string(),
            })
        }
    }

    pub async fn is_available(&self) -> bool {
        match self.client
            .get(&format!("{}/api/tags", self.base_url))
            .send()
            .await
        {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    /// OpenAI GPT fallback for when local models fail
    pub async fn classify_with_openai(user_input: &str) -> Result<IntentClassification, Box<dyn std::error::Error>> {
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| "OPENAI_API_KEY environment variable not set")?;

        let client = OpenAIClient::new().with_api_key(api_key);

        let system_message = ChatCompletionRequestSystemMessage {
            content: "You are a LogLine Discovery Lab Director assistant. Classify user requests into operations and respond with JSON only.".to_string(),
            role: "system".to_string(),
            name: None,
        };

        let user_message = ChatCompletionRequestUserMessage {
            content: Self::build_openai_prompt(user_input),
            role: "user".to_string(),
            name: None,
        };

        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4o") // Use GPT-4o for best results
            .messages(vec![
                ChatCompletionRequestMessage::System(system_message),
                ChatCompletionRequestMessage::User(user_message),
            ])
            .temperature(0.1)
            .max_tokens(200u16)
            .build()?;

        info!("Sending request to OpenAI GPT-4o");
        let response = client.chat().completions().create(request).await?;

        if let Some(choice) = response.choices.first() {
            if let Some(content) = &choice.message.content {
                debug!("OpenAI response: {}", content);
                return Self::parse_openai_response(content);
            }
        }

        Err("No response from OpenAI".into())
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
            
            match serde_json::from_str::<IntentClassification>(json_str) {
                Ok(classification) => {
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
