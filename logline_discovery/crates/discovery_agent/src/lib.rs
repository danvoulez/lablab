use anyhow::Result;
use chrono::Utc;
use folding_runtime::FoldingAnalysis;
use manuscript_generator::{EnhancedManuscript, ManuscriptBuilder, Section, render_markdown_from};
use serde_json::Value;
use spans_core::{SpanId, UniversalSpan};
use tracing::{info, warn};

pub mod triage;

pub struct DiscoveryAgent;

impl DiscoveryAgent {
    pub fn orchestrate(
        spans: Vec<UniversalSpan>,
        folding: Vec<FoldingAnalysis>,
    ) -> Result<EnhancedManuscript> {
        info!("total_spans" = spans.len(), "Generating hypothesis manuscript");
        let title = "HIV Reservoir Instability — First Look";
        let execution_id = spans
            .iter()
            .map(|s| s.id.0.clone())
            .find(|_| true)
            .unwrap_or_else(|| "exec_demo".to_string());

        let mut builder = ManuscriptBuilder::new(title.to_string(), execution_id.clone())
            .with_authors(vec!["LogLine Discovery Lab".to_string()])
            .with_abstract(format!(
                "Generated on {} with {} spans",
                Utc::now().to_rfc3339(),
                spans.len()
            ));

        let overview = Section {
            id: "overview".to_string(),
            title: "Overview".to_string(),
            content: format!("Ingested {} spans dated {}.", spans.len(), Utc::now().date_naive()),
            subsections: vec![],
            figure_refs: vec![],
            citation_refs: vec![],
        };
        builder.add_section(overview);

        if let Some(analysis) = folding.first() {
            let folding_section = Section {
                id: "folding".to_string(),
                title: "Folding Analysis".to_string(),
                content: format!(
                    "Mean potential energy {:.2} kcal/mol, max RMSD {:.2} Å, unstable={}",
                    analysis.mean_energy, analysis.max_rmsd, analysis.unstable
                ),
                subsections: vec![],
                figure_refs: vec![],
                citation_refs: vec![],
            };
            builder.add_section(folding_section);
        }

        let manuscript = builder.build();
        let _markdown = render_markdown_from(&manuscript); // ensure checksum computed paths maintained
        Ok(manuscript)
    }

    pub fn highlight_targets(spans: &[UniversalSpan]) -> Vec<SpanId> {
        spans
            .iter()
            .filter(|span| span.flow.contains("folding"))
            .map(|span| span.id.clone())
            .collect()
    }

    /// Intelligently triage which analyses to run on which spans
    pub fn triage_analyses(&self, spans: &[UniversalSpan]) -> Vec<triage::TriageDecision> {
        let mut triage_system = triage::AnalysisTriage::new();
        let mut decisions = Vec::new();

        for span in spans {
            let decision = triage_system.triage_span(span);
            decisions.push(decision.clone());

            // Log triage decisions for monitoring
            info!(
                span_id = %span.id.0,
                analyses = ?decision.recommended_analyses,
                priority = %decision.priority_score,
                "analysis_triage_decision"
            );

            if !decision.skip_reasons.is_empty() {
                warn!(
                    span_id = %span.id.0,
                    skip_reasons = ?decision.skip_reasons,
                    "analyses_skipped"
                );
            }
        }

        decisions
    }

    /// Execute analyses based on triage decisions
    pub fn execute_analyses(&self, decisions: &[triage::TriageDecision], spans: &[UniversalSpan]) -> AnalysisResults {
        let results = AnalysisResults::new();

        for decision in decisions {
            let span = spans.iter().find(|s| s.id.0 == decision.span_id);

            if let Some(span) = span {
                for analysis_type in &decision.recommended_analyses {
                    match analysis_type {
                        triage::AnalysisType::FoldingAnalysis => {
                            // Run folding analysis
                            info!(span_id = %span.id.0, "running_folding_analysis");
                        }
                        triage::AnalysisType::CausalInference => {
                            // Run causal inference
                            info!(span_id = %span.id.0, "running_causal_inference");
                        }
                        triage::AnalysisType::StructuralSimilarity => {
                            // Run structural similarity
                            info!(span_id = %span.id.0, "running_structural_similarity");
                        }
                        triage::AnalysisType::TwinComparison => {
                            // Run twin comparison
                            info!(span_id = %span.id.0, "running_twin_comparison");
                        }
                        triage::AnalysisType::QualityAssessment => {
                            // Run quality assessment
                            info!(span_id = %span.id.0, "running_quality_assessment");
                        }
                        triage::AnalysisType::ManuscriptGeneration => {
                            // Generate manuscript
                            info!(span_id = %span.id.0, "generating_manuscript");
                        }
                    }
                }
            }
        }

        results
    }
}

pub struct AnalysisResults {
    pub folding_analyses: Vec<FoldingAnalysis>,
    pub causal_chains: Vec<Value>, // Using Value instead of causal_engine::CausalChain
    pub similarity_scores: Vec<Value>, // Using Value instead of structural_similarity::SimilarityResult
    pub manuscripts: Vec<EnhancedManuscript>,
}

impl AnalysisResults {
    fn new() -> Self {
        Self {
            folding_analyses: Vec::new(),
            causal_chains: Vec::new(),
            similarity_scores: Vec::new(),
            manuscripts: Vec::new(),
        }
    }
}
