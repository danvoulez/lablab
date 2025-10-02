use serde::{Deserialize, Serialize};
use serde_json::Value;
use spans_core::UniversalSpan;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use anyhow::Result;

/// Intelligent triage system for the Discovery Lab
/// Based on configurable rules that determine which analysis stages to run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriageEngine {
    rules: Vec<TriageRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriageRule {
    pub name: String,
    pub when: HashMap<String, String>,
    pub then: Vec<String>,
    pub env: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub kind: String,
    pub qsar_score: Option<f64>,
    pub docking_affinity: Option<f64>,
    pub force_field: Option<String>,
    pub docked: Option<bool>,
    pub file_xtc: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnalysisStage {
    QualityAssessment,
    QSAR,
    Docking,
    Simulate,
    Validate,
    Manuscript,
    TwinComparison,
    CausalAnalysis,
    StructuralSimilarity,
}

impl From<&str> for AnalysisStage {
    fn from(s: &str) -> Self {
        match s {
            "QualityAssessment" | "QA" => AnalysisStage::QualityAssessment,
            "QSAR" => AnalysisStage::QSAR,
            "Docking" => AnalysisStage::Docking,
            "Simulate" => AnalysisStage::Simulate,
            "Validate" => AnalysisStage::Validate,
            "Manuscript" => AnalysisStage::Manuscript,
            "TwinComparison" => AnalysisStage::TwinComparison,
            "CausalAnalysis" => AnalysisStage::CausalAnalysis,
            "StructuralSimilarity" => AnalysisStage::StructuralSimilarity,
            _ => panic!("Unknown stage: {}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub stages: Vec<AnalysisStage>,
    pub metadata: HashMap<String, Value>,
}

impl TriageEngine {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }

    pub fn load_rules<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path: PathBuf = path.into();
        if !path.exists() {
            // Return default rules if file doesn't exist
            return Ok(create_default_rules());
        }

        let content = std::fs::read_to_string(path)?;
        Self::load_rules_from_string(&content)
    }

    pub fn load_rules_from_string(content: &str) -> Result<Self> {
        let rules: Vec<TriageRule> = toml::from_str(content)?;
        Ok(Self { rules })
    }

    pub fn make_plan(&self, manifest: &Manifest) -> Result<ExecutionPlan> {
        let mut stages = HashSet::new();
        let mut metadata = HashMap::new();

        for rule in &self.rules {
            if rule.matches(manifest)? {
                tracing::info!(rule_name = %rule.name, "rule_matched");

                for stage_str in &rule.then {
                    if let Some(stage) = stage_str.strip_prefix('!') {
                        // Remove stage
                        stages.remove(&AnalysisStage::from(stage));
                        tracing::info!(stage = stage, "stage_removed");
                    } else {
                        // Add stage
                        let stage = AnalysisStage::from(stage_str.as_str());
                        stages.insert(stage);
                        tracing::info!(stage = %format!("{:?}", stage), "stage_added");
                    }
                }
            }
        }

        // Canonical ordering for deterministic execution
        let ordered_stages = self.order_stages(&stages);

        Ok(ExecutionPlan {
            stages: ordered_stages,
            metadata,
        })
    }

    fn order_stages(&self, stages: &HashSet<AnalysisStage>) -> Vec<AnalysisStage> {
        let canonical_order = vec![
            AnalysisStage::QualityAssessment,
            AnalysisStage::QSAR,
            AnalysisStage::Docking,
            AnalysisStage::Simulate,
            AnalysisStage::TwinComparison,
            AnalysisStage::Validate,
            AnalysisStage::CausalAnalysis,
            AnalysisStage::StructuralSimilarity,
            AnalysisStage::Manuscript,
        ];

        canonical_order
            .into_iter()
            .filter(|stage| stages.contains(stage))
            .collect()
    }

    pub fn parse_manifest_from_span(span: &UniversalSpan) -> Result<Manifest> {
        // Extract manifest data from span metadata
        let metadata = span.payload.get("metadata")
            .or_else(|| span.payload.get("manifest"))
            .ok_or_else(|| anyhow::anyhow!("No manifest data in span"))?;

        // Try to parse as TOML first, then JSON
        let manifest_str = if let Some(toml_str) = metadata.get("toml_content").and_then(|v| v.as_str()) {
            toml_str.to_string()
        } else {
            serde_json::to_string(metadata)?
        };

        let manifest: Manifest = if manifest_str.trim().starts_with('{') {
            serde_json::from_str(&manifest_str)?
        } else {
            toml::from_str(&manifest_str)?
        };

        Ok(manifest)
    }
}

impl TriageRule {
    pub fn matches(&self, manifest: &Manifest) -> Result<bool> {
        // Check environment condition
        if let Some(env_var) = &self.env {
            if std::env::var(env_var).is_err() {
                return Ok(false);
            }
        }

        // Check all conditions in 'when' clause
        for (key, expected) in &self.when {
            let actual = self.get_manifest_value(manifest, key)?;

            if !self.compare_values(&expected, &actual)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn get_manifest_value(&self, manifest: &Manifest, key: &str) -> Result<String> {
        match key.as_str() {
            "kind" => Ok(manifest.kind.clone()),
            "qsar_score" => Ok(manifest.qsar_score.map(|v| v.to_string()).unwrap_or_default()),
            "docking_affinity" => Ok(manifest.docking_affinity.map(|v| v.to_string()).unwrap_or_default()),
            "force_field" => Ok(manifest.force_field.clone().unwrap_or_default()),
            "docked" => Ok(manifest.docked.map(|v| v.to_string()).unwrap_or_default()),
            "file_xtc" => Ok(manifest.file_xtc.clone().unwrap_or_default()),
            _ => {
                // Check extra fields
                if let Some(value) = manifest.extra.get(key) {
                    Ok(serde_json::to_string(value)?)
                } else {
                    Ok("".to_string())
                }
            }
        }
    }

    fn compare_values(&self, expected: &str, actual: &str) -> Result<bool> {
        if expected.contains('<') {
            let num: f64 = actual.parse()?;
            let threshold: f64 = expected.strip_prefix('<').unwrap().trim().parse()?;
            Ok(num < threshold)
        } else if expected.contains('>') {
            let num: f64 = actual.parse()?;
            let threshold: f64 = expected.strip_prefix('>').unwrap().trim().parse()?;
            Ok(num > threshold)
        } else {
            Ok(expected == actual)
        }
    }
}

pub fn make_plan_from_span(span: &UniversalSpan) -> Result<ExecutionPlan> {
    let manifest = TriageEngine::parse_manifest_from_span(span)?;

    // Load triage rules from file or use defaults
    let rules_path = std::env::var("TRIAGE_RULES_PATH")
        .unwrap_or_else(|_| "triage_rules.toml".to_string());

    let engine = if PathBuf::from(&rules_path).exists() {
        TriageEngine::load_rules(rules_path)?
    } else {
        create_default_rules()
    };

    engine.make_plan(&manifest)
}

fn create_default_rules() -> TriageEngine {
    let default_toml = r#"
# Default triage rules for LogLine Discovery Lab

[[rule]]
name = "Raw sequence pipeline"
when.kind = "raw_sequence"
then = ["QualityAssessment", "QSAR", "Simulate", "Validate", "Manuscript"]

[[rule]]
name = "QSAR candidate pipeline"
when.kind = "qsar_candidate"
when.qsar_score = "< 0.7"
then = ["QSAR"]

[[rule]]
name = "Docked structure pipeline"
when.docked = "true"
when.docking_affinity = "< -8"
then = ["Validate", "Manuscript"]

[[rule]]
name = "Skip simulation for Amber"
when.force_field = "Amber99SB"
then = ["!Simulate"]

[[rule]]
name = "CI mode - skip manuscript"
env = "CI"
then = ["!Manuscript"]

[[rule]]
name = "Twin comparison pipeline"
when.kind = "twin_observation"
then = ["TwinComparison", "Validate"]

[[rule]]
name = "High quality execution"
when.kind = "execution_run"
when.status = "completed"
then = ["CausalAnalysis", "StructuralSimilarity", "Manuscript"]
"#;

    TriageEngine::load_rules_from_string(default_toml).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_sequence_pipeline() {
        let manifest = Manifest {
            kind: "raw_sequence".to_string(),
            qsar_score: None,
            docking_affinity: None,
            force_field: None,
            docked: None,
            file_xtc: None,
            extra: HashMap::new(),
        };

        let engine = create_default_rules();
        let plan = engine.make_plan(&manifest).unwrap();

        assert!(plan.stages.contains(&AnalysisStage::QualityAssessment));
        assert!(plan.stages.contains(&AnalysisStage::QSAR));
        assert!(plan.stages.contains(&AnalysisStage::Simulate));
    }

    #[test]
    fn test_ci_mode_skips_manuscript() {
        std::env::set_var("CI", "true");

        let manifest = Manifest {
            kind: "raw_sequence".to_string(),
            qsar_score: None,
            docking_affinity: None,
            force_field: None,
            docked: None,
            file_xtc: None,
            extra: HashMap::new(),
        };

        let engine = create_default_rules();
        let plan = engine.make_plan(&manifest).unwrap();

        assert!(!plan.stages.contains(&AnalysisStage::Manuscript));

        std::env::remove_var("CI");
    }
}
