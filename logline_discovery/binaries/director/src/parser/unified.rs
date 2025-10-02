//! Unified parser that integrates the new provider orchestration system

use std::collections::HashMap;
use serde_json::json;
use crate::{Contract, Workflow, Flow};
use crate::providers::{
    ProviderOrchestrator, SelectionStrategy, UnifiedLLMProvider,
    OllamaProvider, OpenAIProvider, GeminiProvider
};
use tracing::{info, warn, debug, error};

pub struct Parsed {
    pub intent: String,
    pub params: HashMap<String, String>,
    pub original: String,
    pub severity: String,
}

pub struct UnifiedIntent {
    orchestrator: ProviderOrchestrator,
}

impl UnifiedIntent {
    pub fn new() -> Self {
        let mut orchestrator = ProviderOrchestrator::new(SelectionStrategy::LocalFirst);
        
        // Register Ollama providers (local, zero cost)
        orchestrator.register_provider(
            "ollama-mistral".to_string(),
            Box::new(OllamaProvider::new("mistral-director"))
        );
        orchestrator.register_provider(
            "ollama-llama".to_string(),
            Box::new(OllamaProvider::new("llama3.2:1b"))
        );
        
        // TODO: Add cloud providers when API issues are resolved
        // if let Ok(openai_provider) = OpenAIProvider::new("gpt-4o-mini") {
        //     orchestrator.register_provider(
        //         "openai-gpt4-mini".to_string(),
        //         Box::new(openai_provider)
        //     );
        // }
        
        info!("🧠 Unified Intent Parser initialized with Ollama providers");
        
        Self { orchestrator }
    }
    
    pub async fn parse(mut self, text: &str, actor: &str) -> Contract {
        info!("🔍 Processing: '{}'", text);
        
        // Try rule-based parsing first (fast, reliable)
        if let Some(contract) = Self::parse_rules(text, actor) {
            info!("✅ Intent classified by rules: {:?}", contract.flow);
            return contract;
        }

        // Fallback to LLM orchestration
        info!("🧠 Rules didn't match, trying LLM orchestration...");
        match self.orchestrator.classify_with_orchestration(text).await {
            Ok(classification) => {
                info!("✅ LLM classification successful via {}: {:?} (confidence: {:.2})", 
                      classification.provider_used, classification.flow, classification.confidence);
                
                Self::create_contract_from_llm(classification, actor)
            }
            Err(e) => {
                warn!("❌ All LLM providers failed: {}, using Unknown", e);
                Self::create_unknown_contract(text, actor)
            }
        }
    }
    
    /// Fast rule-based parsing for known patterns
    fn parse_rules(text: &str, actor: &str) -> Option<Contract> {
        let t = text.to_lowercase();
        
        let (flow, workflow, payload, severity) = if t.contains("reenvia") || t.contains("reenfileira") {
            (Flow::RequeueStuck, Workflow::LabOps, json!({ "older_than_minutes": 60 }), "medium")
        } else if t.contains("hot reload") || t.contains("atualiza lógica") {
            (Flow::HotReload, Workflow::LabOps, json!({ "channel": "logic_reload", "version": "v2" }), "high")
        } else if t.contains("backup") {
            (Flow::BackupSnap, Workflow::LabOps, json!({ "scope": "full" }), "high")
        } else if t.contains("vacuum") || t.contains("otimiza índice") {
            (Flow::VacuumDb, Workflow::LabOps, json!({ "mode": "full" }), "high")
        } else if t.contains("escala") || t.contains("sobe worker") || t.contains("desce worker") {
            (Flow::ScaleWorkers, Workflow::LabOps, json!({ "delta_workers": 3 }), "medium")
        } else if t.contains("rotaciona log") {
            (Flow::RotateLogs, Workflow::LabOps, json!({ "units": ["logline-discovery-*"] }), "low")
        } else if t.contains("diagnóstico") || t.contains("falhas") {
            (Flow::Diagnose, Workflow::LabOps, json!({ "window": "yesterday" }), "low")
        } else if t.contains("como está a fila") || t.contains("fila agora") || t.contains("status") {
            (Flow::Monitor, Workflow::LabOps, json!({}), "low")
        } else if t.contains("registra dataset") || t.contains("dataset") {
            (Flow::DatasetRegister, Workflow::LabOps, json!({ "name": "dengue_2025" }), "low")
        } else if t.contains("saúde") || t.contains("health") {
            (Flow::LabHealthcheck, Workflow::LabOps, json!({}), "low")
        } else if t.contains("processa") || t.contains("submete") {
            (Flow::SubmitJob, Workflow::JobQueue, json!({ "job_type": "span_processing", "priority": "high" }), "medium")
        } else {
            return None; // No rule matched
        };

        Some(Self::create_contract(flow, workflow, payload, severity, actor, "rules"))
    }
    
    fn create_contract_from_llm(classification: crate::providers::IntentClassification, actor: &str) -> Contract {
        // Convert LLM response to our Flow enum
        let flow = match classification.flow.as_str() {
            "Monitor" => Flow::Monitor,
            "SubmitJob" => Flow::SubmitJob,
            "Diagnose" => Flow::Diagnose,
            "HealthCheck" => Flow::LabHealthcheck,
            "BackupSnap" => Flow::BackupSnap,
            "RequeueStuck" => Flow::RequeueStuck,
            "ScaleWorkers" => Flow::ScaleWorkers,
            "RotateLogs" => Flow::RotateLogs,
            "VacuumDb" => Flow::VacuumDb,
            "DatasetRegister" => Flow::DatasetRegister,
            "HotReload" => Flow::HotReload,
            _ => Flow::Unknown,
        };

        let workflow = match flow {
            Flow::SubmitJob => Workflow::JobQueue,
            _ => Workflow::LabOps,
        };

        let payload = json!({
            "llm_classification": true,
            "confidence": classification.confidence,
            "reasoning": classification.reasoning,
            "provider_used": classification.provider_used,
            "response_time_ms": classification.response_time_ms,
        });

        Self::create_contract(flow, workflow, payload, &classification.priority, actor, &classification.provider_used)
    }
    
    fn create_contract(flow: Flow, workflow: Workflow, payload: serde_json::Value, severity: &str, actor: &str, source: &str) -> Contract {
        let mut contract = Contract {
            name: format!("auto::{}", match flow {
                Flow::SubmitJob => "submit_job",
                _ => "lab_op"
            }),
            workflow,
            flow,
            payload,
            tags: vec!["director".into(), source.into()],
            requested_by: actor.into(),
            requires_approval: false,
            severity: severity.into(),
        };
        
        contract.requires_approval = crate::Policy::requires_approval(&contract);
        contract
    }

    fn create_unknown_contract(text: &str, actor: &str) -> Contract {
        Self::create_contract(
            Flow::Unknown,
            Workflow::LabOps,
            json!({ "note": text, "fallback_reason": "all_providers_failed" }),
            "low",
            actor,
            "fallback"
        )
    }
}

// Legacy Intent struct for backward compatibility
pub struct Intent;

impl Intent {
    pub async fn parse(text: &str, actor: &str) -> Contract {
        let unified_intent = UnifiedIntent::new();
        unified_intent.parse(text, actor).await
    }
}
