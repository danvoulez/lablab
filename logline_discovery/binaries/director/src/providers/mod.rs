//! Unified LLM Provider Interface
//! 
//! This module defines a common interface for all LLM providers,
//! enabling seamless switching between local and cloud models.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod ollama;
pub mod openai;
pub mod gemini;

pub use ollama::OllamaProvider;
pub use openai::OpenAIProvider;
pub use gemini::GeminiProvider;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentClassification {
    pub flow: String,
    pub priority: String,
    pub confidence: f32,
    pub reasoning: String,
    pub provider_used: String,
    pub response_time_ms: u64,
}

#[derive(Debug, Clone)]
pub struct ProviderMetrics {
    pub cost_per_token: f64,
    pub avg_latency_ms: u64,
    pub success_rate: f32,
    pub supports_streaming: bool,
    pub supports_function_calling: bool,
}

#[derive(Debug, Clone)]
pub enum ProviderType {
    Local { endpoint: String },
    Cloud { api_key_env: String },
    Hybrid { primary: String, fallback: String },
}

/// Unified interface for all LLM providers
#[async_trait]
pub trait UnifiedLLMProvider: Send + Sync {
    /// Classify user intent into LogLine operations
    async fn classify_intent(&self, input: &str) -> Result<IntentClassification, Box<dyn std::error::Error>>;
    
    /// Check if provider is available and healthy
    async fn health_check(&self) -> Result<bool, Box<dyn std::error::Error>>;
    
    /// Get provider metadata and capabilities
    fn get_metrics(&self) -> ProviderMetrics;
    
    /// Get provider identifier
    fn provider_id(&self) -> &str;
    
    /// Get provider type
    fn provider_type(&self) -> ProviderType;
}

/// Provider selection strategy
#[derive(Debug, Clone)]
pub enum SelectionStrategy {
    /// Always try local first, then cloud
    LocalFirst,
    /// Select based on cost optimization
    CostOptimized { budget_per_hour: f64 },
    /// Select based on latency requirements
    LatencyOptimized { max_latency_ms: u64 },
    /// Select based on quality requirements
    QualityOptimized,
    /// Adaptive selection based on context
    Adaptive,
}

/// Provider orchestrator that manages multiple LLM providers
pub struct ProviderOrchestrator {
    providers: HashMap<String, Box<dyn UnifiedLLMProvider>>,
    selection_strategy: SelectionStrategy,
    circuit_breaker: CircuitBreaker,
}

/// Simple circuit breaker implementation
#[derive(Debug)]
pub struct CircuitBreaker {
    failure_counts: HashMap<String, u32>,
    failure_threshold: u32,
    recovery_timeout_ms: u64,
    last_failure: HashMap<String, std::time::Instant>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, recovery_timeout_ms: u64) -> Self {
        Self {
            failure_counts: HashMap::new(),
            failure_threshold,
            recovery_timeout_ms,
            last_failure: HashMap::new(),
        }
    }
    
    pub fn is_open(&self, provider_id: &str) -> bool {
        if let Some(&count) = self.failure_counts.get(provider_id) {
            if count >= self.failure_threshold {
                if let Some(&last_failure) = self.last_failure.get(provider_id) {
                    let elapsed = last_failure.elapsed().as_millis() as u64;
                    return elapsed < self.recovery_timeout_ms;
                }
            }
        }
        false
    }
    
    pub fn record_success(&mut self, provider_id: &str) {
        self.failure_counts.insert(provider_id.to_string(), 0);
        self.last_failure.remove(provider_id);
    }
    
    pub fn record_failure(&mut self, provider_id: &str) {
        let count = self.failure_counts.get(provider_id).unwrap_or(&0) + 1;
        self.failure_counts.insert(provider_id.to_string(), count);
        self.last_failure.insert(provider_id.to_string(), std::time::Instant::now());
    }
}

impl ProviderOrchestrator {
    pub fn new(strategy: SelectionStrategy) -> Self {
        Self {
            providers: HashMap::new(),
            selection_strategy: strategy,
            circuit_breaker: CircuitBreaker::new(3, 60000), // 3 failures, 60s recovery
        }
    }
    
    pub fn register_provider(&mut self, id: String, provider: Box<dyn UnifiedLLMProvider>) {
        self.providers.insert(id, provider);
    }
    
    pub async fn classify_with_orchestration(&mut self, input: &str) -> Result<IntentClassification, Box<dyn std::error::Error>> {
        let provider_order = self.get_provider_order().await;
        
        for provider_id in provider_order {
            if self.circuit_breaker.is_open(&provider_id) {
                tracing::warn!("Skipping provider {} (circuit breaker open)", provider_id);
                continue;
            }
            
            if let Some(provider) = self.providers.get(&provider_id) {
                let start_time = std::time::Instant::now();
                
                match provider.classify_intent(input).await {
                    Ok(mut classification) => {
                        classification.provider_used = provider_id.clone();
                        classification.response_time_ms = start_time.elapsed().as_millis() as u64;
                        
                        self.circuit_breaker.record_success(&provider_id);
                        tracing::info!("✅ Provider {} succeeded in {}ms", provider_id, classification.response_time_ms);
                        
                        return Ok(classification);
                    }
                    Err(e) => {
                        self.circuit_breaker.record_failure(&provider_id);
                        tracing::warn!("❌ Provider {} failed: {}", provider_id, e);
                    }
                }
            }
        }
        
        Err("All providers failed".into())
    }
    
    async fn get_provider_order(&self) -> Vec<String> {
        match &self.selection_strategy {
            SelectionStrategy::LocalFirst => {
                let mut local = Vec::new();
                let mut cloud = Vec::new();
                
                for (id, provider) in &self.providers {
                    match provider.provider_type() {
                        ProviderType::Local { .. } => local.push(id.clone()),
                        ProviderType::Cloud { .. } => cloud.push(id.clone()),
                        ProviderType::Hybrid { .. } => local.push(id.clone()),
                    }
                }
                
                local.extend(cloud);
                local
            }
            SelectionStrategy::CostOptimized { .. } => {
                let mut providers: Vec<_> = self.providers.iter().collect();
                providers.sort_by(|a, b| {
                    a.1.get_metrics().cost_per_token.partial_cmp(&b.1.get_metrics().cost_per_token).unwrap()
                });
                providers.into_iter().map(|(id, _)| id.clone()).collect()
            }
            SelectionStrategy::LatencyOptimized { .. } => {
                let mut providers: Vec<_> = self.providers.iter().collect();
                providers.sort_by(|a, b| {
                    a.1.get_metrics().avg_latency_ms.cmp(&b.1.get_metrics().avg_latency_ms)
                });
                providers.into_iter().map(|(id, _)| id.clone()).collect()
            }
            SelectionStrategy::QualityOptimized => {
                // For now, prioritize by success rate
                let mut providers: Vec<_> = self.providers.iter().collect();
                providers.sort_by(|a, b| {
                    b.1.get_metrics().success_rate.partial_cmp(&a.1.get_metrics().success_rate).unwrap()
                });
                providers.into_iter().map(|(id, _)| id.clone()).collect()
            }
            SelectionStrategy::Adaptive => {
                // Simple adaptive: local first, then by success rate
                self.get_provider_order().await // Recursive call with LocalFirst for now
            }
        }
    }
}
