use serde::{Deserialize, Serialize};
use serde_json::Value;
use spans_core::UniversalSpan;
use std::collections::HashMap;

/// Intelligent triage system for the Discovery Lab
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisTriage {
    /// Thresholds for different analysis types
    pub thresholds: AnalysisThresholds,
    /// Execution history for learning
    pub execution_cache: HashMap<String, ExecutionProfile>,
}

/// Configurable thresholds for triage decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisThresholds {
    /// Minimum data quality score (0.0-1.0) for any analysis
    pub min_quality_score: f64,
    /// Energy convergence threshold for folding analysis
    pub energy_convergence_threshold: f64,
    /// RMSD threshold for structural analysis
    pub rmsd_threshold: f64,
    /// Minimum observation count for causal analysis
    pub min_observations_for_causal: usize,
    /// Maximum execution time before skipping intensive analysis
    pub max_execution_time_seconds: u64,
}

/// Profile of previous executions for learning-based decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionProfile {
    pub execution_id: String,
    pub subject_type: String,
    pub analysis_types_run: Vec<String>,
    pub outcomes: HashMap<String, f64>,
    pub execution_time: u64,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriageDecision {
    pub span_id: String,
    pub recommended_analyses: Vec<AnalysisType>,
    pub skip_reasons: Vec<String>,
    pub priority_score: f64,
    pub estimated_cost: AnalysisCost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisType {
    FoldingAnalysis,
    CausalInference,
    StructuralSimilarity,
    TwinComparison,
    ManuscriptGeneration,
    QualityAssessment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisCost {
    pub cpu_time_seconds: u64,
    pub memory_mb: u64,
    pub storage_mb: u64,
}

impl AnalysisTriage {
    pub fn new() -> Self {
        Self {
            thresholds: AnalysisThresholds::default(),
            execution_cache: HashMap::new(),
        }
    }

    /// Decide which analyses to run on a given span
    pub fn triage_span(&mut self, span: &UniversalSpan) -> TriageDecision {
        let mut recommended = Vec::new();
        let mut skip_reasons = Vec::new();
        let mut priority_score = 0.0;

        // Quality assessment (always run first)
        if self.should_run_quality_assessment(span) {
            recommended.push(AnalysisType::QualityAssessment);
            priority_score += 0.5;
        } else {
            skip_reasons.push("Quality too low for analysis".to_string());
        }

        // Folding analysis triage
        if self.should_run_folding_analysis(span) {
            recommended.push(AnalysisType::FoldingAnalysis);
            priority_score += 0.3;
        } else {
            skip_reasons.push("Folding analysis not applicable".to_string());
        }

        // Causal inference triage
        if self.should_run_causal_inference(span) {
            recommended.push(AnalysisType::CausalInference);
            priority_score += 0.4;
        } else {
            skip_reasons.push("Insufficient data for causal analysis".to_string());
        }

        // Structural similarity triage
        if self.should_run_structural_similarity(span) {
            recommended.push(AnalysisType::StructuralSimilarity);
            priority_score += 0.2;
        } else {
            skip_reasons.push("Structural analysis not needed".to_string());
        }

        // Twin comparison triage
        if self.should_run_twin_comparison(span) {
            recommended.push(AnalysisType::TwinComparison);
            priority_score += 0.3;
        } else {
            skip_reasons.push("No twin data available".to_string());
        }

        // Manuscript generation (only for completed workflows)
        if self.should_generate_manuscript(span) {
            recommended.push(AnalysisType::ManuscriptGeneration);
            priority_score += 0.1;
        }

        TriageDecision {
            span_id: span.id.0.clone(),
            recommended_analyses: recommended.clone(),
            skip_reasons,
            priority_score,
            estimated_cost: self.estimate_cost(&recommended),
        }
    }

    fn should_run_quality_assessment(&self, span: &UniversalSpan) -> bool {
        // Always run quality assessment on new data
        matches!(span.flow.as_str(), "execution_run" | "metric" | "frame")
    }

    fn should_run_folding_analysis(&self, span: &UniversalSpan) -> bool {
        if !matches!(span.flow.as_str(), "execution_run" | "analysis") {
            return false;
        }

        // Check if execution completed successfully
        if let Some(metadata) = span.payload.get("metadata") {
            if let Some(status) = metadata.get("status").and_then(|s| s.as_str()) {
                if status != "completed" {
                    return false;
                }
            }
        }

        // Check energy convergence if available
        if let Some(energy) = extract_energy_from_span(span) {
            if energy.abs() > self.thresholds.energy_convergence_threshold {
                return false;
            }
        }

        true
    }

    fn should_run_causal_inference(&self, span: &UniversalSpan) -> bool {
        // Need sufficient observation data
        if !has_sufficient_observations(span) {
            return false;
        }

        // Check execution history for similar spans
        if let Some(history) = self.get_execution_history(span) {
            if history.success_rate < 0.7 {
                return false; // Poor success rate on similar data
            }
        }

        matches!(span.flow.as_str(), "execution_run" | "metric" | "frame")
    }

    fn should_run_structural_similarity(&self, span: &UniversalSpan) -> bool {
        // Only for protein folding results
        if !is_protein_folding_span(span) {
            return false;
        }

        // Check RMSD threshold
        if let Some(rmsd) = extract_rmsd_from_span(span) {
            if rmsd > self.thresholds.rmsd_threshold {
                return false; // Structure too different
            }
        }

        true
    }

    fn should_run_twin_comparison(&self, span: &UniversalSpan) -> bool {
        // Only for spans that have twin data
        matches!(span.flow.as_str(), "twin_observation")
            || span.payload.get("metadata")
                .and_then(|m| m.get("twin_type"))
                .is_some()
    }

    fn should_generate_manuscript(&self, span: &UniversalSpan) -> bool {
        // Only for completed workflows with sufficient results
        if !matches!(span.flow.as_str(), "execution_run") {
            return false;
        }

        // Check if workflow is complete
        if let Some(metadata) = span.payload.get("metadata") {
            if let Some(status) = metadata.get("status").and_then(|s| s.as_str()) {
                if status == "completed" && self.has_sufficient_results(span) {
                    return true;
                }
            }
        }

        false
    }

    fn estimate_cost(&self, analyses: &[AnalysisType]) -> AnalysisCost {
        let mut total_cpu = 0u64;
        let mut total_memory = 0u64;
        let mut total_storage = 0u64;

        for analysis in analyses {
            match analysis {
                AnalysisType::QualityAssessment => {
                    total_cpu += 10;
                    total_memory += 100;
                }
                AnalysisType::FoldingAnalysis => {
                    total_cpu += 300;
                    total_memory += 2000;
                    total_storage += 50;
                }
                AnalysisType::CausalInference => {
                    total_cpu += 600;
                    total_memory += 4000;
                    total_storage += 100;
                }
                AnalysisType::StructuralSimilarity => {
                    total_cpu += 200;
                    total_memory += 1500;
                    total_storage += 25;
                }
                AnalysisType::TwinComparison => {
                    total_cpu += 150;
                    total_memory += 800;
                    total_storage += 10;
                }
                AnalysisType::ManuscriptGeneration => {
                    total_cpu += 60;
                    total_memory += 500;
                    total_storage += 200;
                }
            }
        }

        AnalysisCost {
            cpu_time_seconds: total_cpu,
            memory_mb: total_memory,
            storage_mb: total_storage,
        }
    }

    fn get_execution_history(&self, span: &UniversalSpan) -> Option<&ExecutionProfile> {
        // Simple subject-based lookup for now
        if let Some(metadata) = span.payload.get("metadata") {
            if let Some(subject_type) = metadata.get("subject_type").and_then(|s| s.as_str()) {
                // Look for similar execution profiles
                for profile in self.execution_cache.values() {
                    if profile.subject_type == subject_type {
                        return Some(profile);
                    }
                }
            }
        }
        None
    }

    fn has_sufficient_results(&self, span: &UniversalSpan) -> bool {
        // Check if span has meaningful results
        span.payload.get("results").is_some()
            || span.payload.get("summary").is_some()
            || span.payload.get("metrics").is_some()
    }
}

impl Default for AnalysisThresholds {
    fn default() -> Self {
        Self {
            min_quality_score: 0.6,
            energy_convergence_threshold: 10.0,
            rmsd_threshold: 5.0,
            min_observations_for_causal: 10,
            max_execution_time_seconds: 3600, // 1 hour
        }
    }
}

// Helper functions
fn extract_energy_from_span(span: &UniversalSpan) -> Option<f64> {
    span.payload
        .get("results")
        .and_then(|r| r.get("final_energy_kcal_mol"))
        .or_else(|| span.payload.get("energy"))
        .and_then(|e| e.as_f64())
}

fn extract_rmsd_from_span(span: &UniversalSpan) -> Option<f64> {
    span.payload
        .get("results")
        .and_then(|r| r.get("final_rmsd_angstrom"))
        .or_else(|| span.payload.get("rmsd"))
        .and_then(|r| r.as_f64())
}

fn has_sufficient_observations(span: &UniversalSpan) -> bool {
    // Check if span has multiple related observations
    !span.causal.related_ids.is_empty()
        || span.payload.get("observations").is_some()
        || span.payload.get("frames").is_some()
}

fn is_protein_folding_span(span: &UniversalSpan) -> bool {
    span.payload.get("protein_metadata").is_some()
        || span.flow.contains("folding")
        || span.payload.get("sequence").is_some()
}
