//! Intelligent RAG-powered model selector with contextual knowledge

use crate::simple_llm::SimpleOllamaClient;
use crate::rag::RAGSystem;
use crate::personality::DirectorPersonality;
use tracing::{info, warn, debug};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct IntelligentRAGSelector {
    rag_system: Arc<Mutex<RAGSystem>>,
}

impl IntelligentRAGSelector {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut rag_system = RAGSystem::new("data/knowledge_base.json");
        rag_system.initialize().await?;
        
        // Populate with sample data if empty
        if rag_system.search("", 1).await?.is_empty() {
            rag_system.populate_sample_data().await?;
        }

        Ok(Self {
            rag_system: Arc::new(Mutex::new(rag_system)),
        })
    }

    pub async fn classify_with_context(&self, user_input: &str) -> Result<crate::simple_llm::SimpleClassification, Box<dyn std::error::Error>> {
        // Get contextual knowledge from RAG
        let context = {
            let rag = self.rag_system.lock().await;
            rag.get_context(user_input).await.unwrap_or_else(|_| String::new())
        };

        let context_to_use = if context.trim().is_empty() {
            None
        } else {
            Some(context.as_str())
        };

        info!("🧠 RAG context length: {} chars", context.len());

        // Try models in order of reliability with RAG context
        let models = vec![
            "director-smart",    // Most optimized
            "director-brain",    // Backup optimized  
            "mistral:instruct",  // Standard instruct
            "mistral-director",  // Original
            "llama3.2:1b",      // Fast fallback
        ];

        for (i, model_name) in models.iter().enumerate() {
            info!("🧠 Trying RAG-enhanced model {} ({}/{})", model_name, i + 1, models.len());
            
            let client = SimpleOllamaClient::new(model_name);
            
            match self.classify_with_rag_context(&client, user_input, context_to_use).await {
                Ok(result) => {
                    info!("✅ RAG-enhanced model {} succeeded with confidence {:.2}", model_name, result.confidence);
                    return Ok(result);
                }
                Err(e) => {
                    warn!("❌ RAG-enhanced model {} failed: {}", model_name, e);
                    continue;
                }
            }
        }

        Err("All RAG-enhanced models failed".into())
    }

    async fn classify_with_rag_context(
        &self,
        client: &SimpleOllamaClient,
        user_input: &str,
        context: Option<&str>
    ) -> Result<crate::simple_llm::SimpleClassification, Box<dyn std::error::Error>> {
        // Use the enhanced prompt with RAG context
        let prompt = DirectorPersonality::get_classification_prompt_with_context(user_input, context);
        let params = DirectorPersonality::get_optimal_parameters();
        
        debug!("Sending RAG-enhanced prompt to Ollama");
        
        let request = crate::simple_llm::OllamaRequest {
            model: client.model.clone(),
            prompt,
            stream: false,
            options: Some(crate::simple_llm::OllamaOptions {
                temperature: params.temperature,
                top_p: params.top_p,
                num_ctx: params.num_ctx,
            }),
        };

        let response = client.client
            .post(&format!("{}/api/generate", client.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            warn!("Ollama request failed: {}", response.status());
            return Err(format!("Ollama request failed: {}", response.status()).into());
        }

        let ollama_response: crate::simple_llm::OllamaResponse = response.json().await?;
        debug!("RAG-enhanced Ollama response: {}", ollama_response.response);

        self.parse_classification_response(&ollama_response.response)
    }

    fn parse_classification_response(&self, response: &str) -> Result<crate::simple_llm::SimpleClassification, Box<dyn std::error::Error>> {
        // Try to extract JSON from the response
        let json_start = response.find('{');
        let json_end = response.rfind('}');
        
        if let (Some(start), Some(end)) = (json_start, json_end) {
            let json_str = &response[start..=end];
            debug!("Extracted JSON from RAG-enhanced Ollama: {}", json_str);
            
            match serde_json::from_str::<serde_json::Value>(json_str) {
                Ok(parsed) => {
                    let classification = crate::simple_llm::SimpleClassification {
                        flow: parsed["flow"].as_str().unwrap_or("Unknown").to_string(),
                        priority: parsed["priority"].as_str().unwrap_or("low").to_string(),
                        confidence: parsed["confidence"].as_f64().unwrap_or(0.0) as f32,
                        reasoning: parsed["reasoning"].as_str().unwrap_or("").to_string(),
                    };
                    
                    info!("✅ RAG-enhanced classification successful: {:?} (confidence: {:.2})", 
                          classification.flow, classification.confidence);
                    Ok(classification)
                }
                Err(e) => {
                    warn!("Failed to parse RAG-enhanced JSON: {}, response: {}", e, json_str);
                    // Fallback to Unknown
                    Ok(crate::simple_llm::SimpleClassification {
                        flow: "Unknown".to_string(),
                        priority: "low".to_string(),
                        confidence: 0.0,
                        reasoning: "Failed to parse RAG-enhanced response".to_string(),
                    })
                }
            }
        } else {
            warn!("No JSON found in RAG-enhanced response: {}", response);
            Ok(crate::simple_llm::SimpleClassification {
                flow: "Unknown".to_string(),
                priority: "low".to_string(),
                confidence: 0.0,
                reasoning: "No JSON in RAG-enhanced response".to_string(),
            })
        }
    }

    pub async fn add_knowledge(&self, content: &str, metadata: crate::rag::EntryMetadata) -> Result<String, Box<dyn std::error::Error>> {
        let mut rag = self.rag_system.lock().await;
        rag.add_knowledge(content, metadata).await
    }

    pub async fn knowledge_count(&self) -> usize {
        let rag = self.rag_system.lock().await;
        rag.knowledge_count()
    }

    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<crate::rag::SearchResult>, Box<dyn std::error::Error>> {
        let rag = self.rag_system.lock().await;
        rag.search(query, limit).await
    }
}
