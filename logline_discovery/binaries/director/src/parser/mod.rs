//! Natural language intent parser with LLM fallback

use std::collections::HashMap;
use serde_json::json;
use crate::{Contract, Workflow, Flow};
use crate::rag_models::IntelligentRAGSelector;
use tracing::{info, warn, debug};

pub struct Parsed {
    pub intent: String,
    pub params: HashMap<String, String>,
    pub original: String,
    pub severity: String, // default: "low"
}

pub struct Intent;

impl Intent {
    pub async fn parse(text: &str, actor: &str) -> Contract {
        info!("🔍 Processing: '{}'", text);
        
        // Try rule-based parsing first (fast, reliable)
        if let Some(contract) = Self::parse_rules(text, actor) {
            info!("✅ Intent classified by rules: {:?}", contract.flow);
            return contract;
        }

        // Fallback to LLM if rules don't match
        info!("🧠 Rules didn't match, trying LLM classification...");
        Self::parse_with_llm(text, actor).await
            .unwrap_or_else(|e| {
                warn!("❌ LLM classification failed: {}, using Unknown", e);
                Self::create_unknown_contract(text, actor)
            })
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

    /// LLM-powered parsing for complex natural language with RAG context
    async fn parse_with_llm(text: &str, actor: &str) -> Result<Contract, Box<dyn std::error::Error>> {
        // Use RAG-enhanced intelligent model selection
        let selector = IntelligentRAGSelector::new().await?;
        
        match selector.classify_with_context(text).await {
            Ok(classification) => {
                info!("✅ RAG-enhanced model selection successful");
                
                debug!("RAG-enhanced LLM classification: {:?}", classification);

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
                    "original_text": text,
                    "rag_enhanced": true,
                    "contextual_knowledge": true
                });

                info!("RAG-enhanced LLM classified as {:?} with confidence {:.2}", flow, classification.confidence);

                Ok(Self::create_contract(flow, workflow, payload, &classification.priority, actor, "rag_llm"))
            }
            Err(e) => {
                warn!("❌ All RAG-enhanced models failed: {}", e);
                Err("All LLM providers failed".into())
            }
        }
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
            json!({ "note": text, "fallback_reason": "llm_failed" }),
            "low",
            actor,
            "fallback"
        )
    }
}
