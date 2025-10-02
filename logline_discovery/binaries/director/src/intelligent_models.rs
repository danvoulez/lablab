//! Intelligent model selection for optimal performance and quality

use crate::simple_llm::SimpleOllamaClient;
use tracing::{info, warn, debug};

pub struct IntelligentModelSelector {
    available_models: Vec<ModelInfo>,
}

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub intelligence_level: u8,  // 1-10 scale
    pub speed_ms: u64,          // Average response time
    pub specialization: ModelSpecialization,
    pub fallback_priority: u8,   // Lower = higher priority
}

#[derive(Debug, Clone)]
pub enum ModelSpecialization {
    Classification,  // Best for intent classification
    Reasoning,      // Best for complex reasoning
    Portuguese,     // Best for Portuguese language
    General,        // General purpose
}

impl Default for IntelligentModelSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl IntelligentModelSelector {
    pub fn new() -> Self {
        let available_models = vec![
            ModelInfo {
                name: "director-smart".to_string(),
                intelligence_level: 9,
                speed_ms: 800,
                specialization: ModelSpecialization::Classification,
                fallback_priority: 1,
            },
            ModelInfo {
                name: "director-brain".to_string(),
                intelligence_level: 8,
                speed_ms: 600,
                specialization: ModelSpecialization::Portuguese,
                fallback_priority: 2,
            },
            ModelInfo {
                name: "qwen2.5:7b-instruct".to_string(),
                intelligence_level: 9,
                speed_ms: 1000,
                specialization: ModelSpecialization::Reasoning,
                fallback_priority: 3,
            },
            ModelInfo {
                name: "llama3.2:3b-instruct".to_string(),
                intelligence_level: 7,
                speed_ms: 400,
                specialization: ModelSpecialization::General,
                fallback_priority: 4,
            },
            ModelInfo {
                name: "mistral-director".to_string(),
                intelligence_level: 7,
                speed_ms: 500,
                specialization: ModelSpecialization::General,
                fallback_priority: 5,
            },
            ModelInfo {
                name: "llama3.2:1b".to_string(),
                intelligence_level: 5,
                speed_ms: 200,
                specialization: ModelSpecialization::General,
                fallback_priority: 6,
            },
        ];

        Self { available_models }
    }

    pub async fn classify_with_best_model(&self, user_input: &str) -> Result<crate::simple_llm::SimpleClassification, Box<dyn std::error::Error>> {
        let complexity = self.assess_complexity(user_input);
        let selected_models = self.select_models_for_complexity(&complexity);

        info!("🧠 Input complexity: {:?}, trying {} models", complexity, selected_models.len());

        for model_info in selected_models {
            debug!("Trying model: {} (intelligence: {}, speed: {}ms)", 
                   model_info.name, model_info.intelligence_level, model_info.speed_ms);

            let client = SimpleOllamaClient::new(&model_info.name);
            
            match client.classify_intent(user_input).await {
                Ok(result) => {
                    info!("✅ Model {} succeeded with confidence {:.2}", 
                          model_info.name, result.confidence);
                    return Ok(result);
                }
                Err(e) => {
                    warn!("❌ Model {} failed: {}", model_info.name, e);
                    continue;
                }
            }
        }

        Err("All intelligent models failed".into())
    }

    fn assess_complexity(&self, input: &str) -> InputComplexity {
        let word_count = input.split_whitespace().count();
        let has_technical_terms = input.to_lowercase().contains("análise") || 
                                 input.to_lowercase().contains("simulação") ||
                                 input.to_lowercase().contains("molecular") ||
                                 input.to_lowercase().contains("pipeline");
        let has_ambiguity = input.contains("?") && word_count > 8;
        let has_context_dependency = input.to_lowercase().contains("hoje") ||
                                   input.to_lowercase().contains("agora") ||
                                   input.to_lowercase().contains("atual");

        match (word_count, has_technical_terms, has_ambiguity, has_context_dependency) {
            (w, true, true, _) if w > 10 => InputComplexity::VeryHigh,
            (w, true, _, true) if w > 8 => InputComplexity::High,
            (w, _, true, _) if w > 6 => InputComplexity::Medium,
            (w, true, _, _) if w > 4 => InputComplexity::Medium,
            _ => InputComplexity::Low,
        }
    }

    fn select_models_for_complexity(&self, complexity: &InputComplexity) -> Vec<&ModelInfo> {
        let mut models: Vec<&ModelInfo> = match complexity {
            InputComplexity::VeryHigh => {
                // Use most intelligent models first
                self.available_models.iter()
                    .filter(|m| m.intelligence_level >= 8)
                    .collect()
            }
            InputComplexity::High => {
                // Use smart models, prioritize reasoning
                self.available_models.iter()
                    .filter(|m| m.intelligence_level >= 7)
                    .collect()
            }
            InputComplexity::Medium => {
                // Use balanced approach
                self.available_models.iter()
                    .filter(|m| m.intelligence_level >= 6)
                    .collect()
            }
            InputComplexity::Low => {
                // Use fastest models first
                self.available_models.iter()
                    .filter(|m| m.speed_ms <= 600)
                    .collect()
            }
        };

        // Sort by fallback priority
        models.sort_by_key(|m| m.fallback_priority);
        models
    }
}

#[derive(Debug, Clone)]
enum InputComplexity {
    Low,      // Simple commands, clear intent
    Medium,   // Some ambiguity or technical terms
    High,     // Complex language or context dependency
    VeryHigh, // Highly ambiguous or very technical
}
