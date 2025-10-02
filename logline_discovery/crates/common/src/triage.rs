use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use crate::{Error, Result};

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
            return Ok(create_default_rules());
        }

        let content = std::fs::read_to_string(path)?;
        Self::load_rules_from_string(&content)
    }

    pub fn load_rules_from_string(content: &str) -> Result<Self> {
        // For now, return default rules since we don't have toml parsing
        Ok(create_default_rules())
    }

    pub fn make_plan(&self, manifest: &Manifest) -> Result<ExecutionPlan> {
        let mut stages = HashSet::new();
        let mut metadata = HashMap::new();

        for rule in &self.rules {
            if rule.matches(manifest)? {
                tracing::info!(rule_name = %rule.name, "rule_matched");

                for stage_str in &rule.then {
                    if let Some(stage) = stage_str.strip_prefix('!') {
                        stages.remove(&AnalysisStage::from(stage));
                        tracing::info!(stage = stage, "stage_removed");
                    } else {
                        let stage = AnalysisStage::from(stage_str.as_str());
                        stages.insert(stage.clone());
                        tracing::info!(stage = %format!("{:?}", stage), "stage_added");
                    }
                }
            }
        }

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

    pub fn parse_manifest_from_json(value: &Value) -> Result<Manifest> {
        let manifest: Manifest = serde_json::from_value(value.clone())?;
        Ok(manifest)
    }
}

impl TriageRule {
    pub fn matches(&self, manifest: &Manifest) -> Result<bool> {
        if let Some(env_var) = &self.env {
            if std::env::var(env_var).is_err() {
                return Ok(false);
            }
        }

        for (key, expected) in &self.when {
            let actual = self.get_manifest_value(manifest, key)?;

            if !self.compare_values(&expected, &actual)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn get_manifest_value(&self, manifest: &Manifest, key: &str) -> Result<String> {
        match key {
            "kind" => Ok(manifest.kind.clone()),
            "qsar_score" => Ok(manifest.qsar_score.map(|v| v.to_string()).unwrap_or_default()),
            "docking_affinity" => Ok(manifest.docking_affinity.map(|v| v.to_string()).unwrap_or_default()),
            "force_field" => Ok(manifest.force_field.clone().unwrap_or_default()),
            "docked" => Ok(manifest.docked.map(|v| v.to_string()).unwrap_or_default()),
            "file_xtc" => Ok(manifest.file_xtc.clone().unwrap_or_default()),
            _ => {
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

pub fn make_plan_from_json(value: &Value) -> Result<ExecutionPlan> {
    let manifest = TriageEngine::parse_manifest_from_json(value)?;

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
