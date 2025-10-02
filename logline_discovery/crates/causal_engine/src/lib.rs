use chrono::{DateTime, Utc};
use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use spans_core::{SpanId, UniversalSpan};
use std::collections::{HashMap, HashSet};
use structural_similarity::aggregate_similarity;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CorrelationType {
    TemporalProximity,
    StructuralSimilarity,
    ResourceContention,
    CascadeFailure,
    PerformanceCoupling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLink {
    pub cause: SpanId,
    pub effect: SpanId,
    pub confidence: f64,
    pub lag_ms: i64,
    pub relation: CorrelationType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalChain {
    pub links: Vec<CausalLink>,
    pub strength: f64,
    pub hypothesis: String,
    pub narrative: String,
}

#[derive(Debug, Default)]
pub struct CausalEngine {
    graph: DiGraph<UniversalSpan, CausalLink>,
    index_map: HashMap<SpanId, NodeIndex>,
    inferences: Vec<CausalChain>,
    config: CausalConfig,
}

impl CausalEngine {
    pub fn new() -> Self {
        Self::with_config(CausalConfig::default())
    }

    pub fn with_config(config: CausalConfig) -> Self {
        Self {
            graph: DiGraph::new(),
            index_map: HashMap::new(),
            inferences: Vec::new(),
            config,
        }
    }

    pub fn ingest(&mut self, spans: Vec<UniversalSpan>) {
        for span in spans {
            let node = self.graph.add_node(span.clone());
            self.index_map.insert(span.id.clone(), node);
        }
        self.build_edges();
    }

    fn build_edges(&mut self) {
        let nodes: Vec<(SpanId, NodeIndex, DateTime<Utc>)> = self
            .graph
            .node_indices()
            .map(|idx| {
                let span = &self.graph[idx];
                (span.id.clone(), idx, span.started_at)
            })
            .collect();

        for (cause_id, cause_node, cause_start) in &nodes {
            let cause_span = self.graph.node_weight(*cause_node).cloned().unwrap();
            for (effect_id, effect_node, effect_start) in &nodes {
                if cause_id == effect_id {
                    continue;
                }
                if effect_start < cause_start {
                    continue;
                }
                let lag_ms = (*effect_start - *cause_start).num_milliseconds();
                if lag_ms < 0 {
                    continue;
                }

                let effect_span = self.graph.node_weight(*effect_node).cloned().unwrap();
                let links = evaluate_rules(&self.config, &cause_span, &effect_span, lag_ms);
                for link in links {
                    self.graph.add_edge(*cause_node, *effect_node, link);
                }
            }
        }
    }

    pub fn infer(mut self) -> Vec<CausalChain> {
        self.inferences = self
            .graph
            .edge_indices()
            .map(|edge_idx| {
                let link = self.graph[edge_idx].clone();
                let (source, target) = self.graph.edge_endpoints(edge_idx).expect("edge endpoints");
                let cause = &self.graph[source];
                let effect = &self.graph[target];
                let hypothesis = describe_hypothesis(&link, cause, effect);
                let narrative = describe_narrative(&link, cause, effect);
                CausalChain {
                    links: vec![link.clone()],
                    strength: link.confidence,
                    hypothesis,
                    narrative,
                }
            })
            .collect();
        self.inferences.clone()
    }
}

const RESOURCE_KEYWORDS: &[&str] = &["cpu", "gpu", "memory", "ram", "disk", "io", "network"];

#[derive(Debug, Clone)]
pub struct CausalConfig {
    pub temporal_window_ms: i64,
    pub structural_threshold: f64,
    pub performance_jitter: f64,
    pub cascade_window_ms: i64,
}

impl Default for CausalConfig {
    fn default() -> Self {
        Self {
            temporal_window_ms: 5_000,
            structural_threshold: 0.30,
            performance_jitter: 0.25,
            cascade_window_ms: 5_000,
        }
    }
}

fn evaluate_rules(
    config: &CausalConfig,
    cause: &UniversalSpan,
    effect: &UniversalSpan,
    lag_ms: i64,
) -> Vec<CausalLink> {
    let mut out = Vec::new();

    if let Some(confidence) = temporal_confidence(config, lag_ms) {
        out.push(make_link(
            cause,
            effect,
            CorrelationType::TemporalProximity,
            confidence,
            lag_ms,
        ));
    }

    if let Some(confidence) = resource_contention_confidence(cause, effect) {
        out.push(make_link(
            cause,
            effect,
            CorrelationType::ResourceContention,
            confidence,
            lag_ms,
        ));
    }

    if let Some(confidence) = structural_similarity_confidence(config, cause, effect) {
        out.push(make_link(
            cause,
            effect,
            CorrelationType::StructuralSimilarity,
            confidence,
            lag_ms,
        ));
    }

    if let Some(confidence) = performance_coupling_confidence(config, cause, effect) {
        out.push(make_link(
            cause,
            effect,
            CorrelationType::PerformanceCoupling,
            confidence,
            lag_ms,
        ));
    }

    if let Some(confidence) = cascade_failure_confidence(config, cause, effect, lag_ms) {
        out.push(make_link(
            cause,
            effect,
            CorrelationType::CascadeFailure,
            confidence,
            lag_ms,
        ));
    }

    out
}

fn make_link(
    cause: &UniversalSpan,
    effect: &UniversalSpan,
    relation: CorrelationType,
    confidence: f64,
    lag_ms: i64,
) -> CausalLink {
    CausalLink {
        cause: cause.id.clone(),
        effect: effect.id.clone(),
        confidence: confidence.clamp(0.0, 1.0),
        lag_ms,
        relation,
    }
}

fn temporal_confidence(config: &CausalConfig, lag_ms: i64) -> Option<f64> {
    if lag_ms <= config.temporal_window_ms {
        let ratio = lag_ms as f64 / config.temporal_window_ms as f64;
        Some(1.0 - ratio)
    } else {
        None
    }
}

fn resource_contention_confidence(cause: &UniversalSpan, effect: &UniversalSpan) -> Option<f64> {
    let cause_tokens = resource_tokens(cause);
    if cause_tokens.is_empty() {
        return None;
    }
    let effect_tokens = resource_tokens(effect);
    if effect_tokens.is_empty() {
        return None;
    }

    let shared: HashSet<_> = cause_tokens.intersection(&effect_tokens).collect();
    if shared.is_empty() {
        None
    } else {
        let base = 0.65;
        let bonus = (shared.len() as f64 * 0.05).min(0.25);
        Some((base + bonus).min(0.9))
    }
}

fn resource_tokens(span: &UniversalSpan) -> HashSet<String> {
    let mut tokens = HashSet::new();
    collect_resource_tokens(&span.payload, &mut tokens);
    tokens
}

fn collect_resource_tokens(value: &Value, tokens: &mut HashSet<String>) {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let key_lower = key.to_lowercase();
                if RESOURCE_KEYWORDS.iter().any(|kw| key_lower.contains(kw)) {
                    tokens.insert(key_lower.clone());
                }
                collect_resource_tokens(val, tokens);
            }
        }
        Value::String(s) => {
            let lower = s.to_lowercase();
            if RESOURCE_KEYWORDS.iter().any(|kw| lower.contains(kw)) {
                tokens.insert(lower);
            }
        }
        Value::Array(arr) => {
            for item in arr {
                collect_resource_tokens(item, tokens);
            }
        }
        _ => {}
    }
}

fn structural_similarity_confidence(
    config: &CausalConfig,
    cause: &UniversalSpan,
    effect: &UniversalSpan,
) -> Option<f64> {
    match aggregate_similarity(cause, effect) {
        Ok(scores) => {
            let composite = scores.composite().unwrap_or(0.0);
            if composite >= config.structural_threshold {
                Some(((composite - config.structural_threshold) * 0.5 + composite).min(0.95))
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn performance_coupling_confidence(
    config: &CausalConfig,
    cause: &UniversalSpan,
    effect: &UniversalSpan,
) -> Option<f64> {
    let latency_a = extract_metric(cause, &["latency_ms", "latency", "duration_ms"]);
    let latency_b = extract_metric(effect, &["latency_ms", "latency", "duration_ms"]);
    if let (Some(a), Some(b)) = (latency_a, latency_b) {
        return metric_similarity(a, b, config.performance_jitter);
    }

    let throughput_a = extract_metric(cause, &["throughput", "requests_per_sec", "rps"]);
    let throughput_b = extract_metric(effect, &["throughput", "requests_per_sec", "rps"]);
    if let (Some(a), Some(b)) = (throughput_a, throughput_b) {
        return metric_similarity(a, b, config.performance_jitter);
    }

    let performance_a = extract_metric(cause, &["performance_ns_per_day", "performance"]);
    let performance_b = extract_metric(effect, &["performance_ns_per_day", "performance"]);
    if let (Some(a), Some(b)) = (performance_a, performance_b) {
        return metric_similarity(a, b, config.performance_jitter);
    }

    None
}

fn metric_similarity(a: f64, b: f64, tolerance: f64) -> Option<f64> {
    let denom = a.abs().max(b.abs()).max(1.0);
    let delta = (a - b).abs() / denom;
    if delta <= tolerance {
        Some((1.0 - delta).clamp(0.0, 0.85))
    } else {
        None
    }
}

fn extract_metric(span: &UniversalSpan, keys: &[&str]) -> Option<f64> {
    for key in keys {
        if let Some(value) = span.payload.get(*key) {
            if let Some(num) = value.as_f64() {
                return Some(num);
            }
            if let Some(s) = value.as_str() {
                if let Ok(n) = s.parse::<f64>() {
                    return Some(n);
                }
            }
        }
    }
    None
}

fn cascade_failure_confidence(
    config: &CausalConfig,
    cause: &UniversalSpan,
    effect: &UniversalSpan,
    lag_ms: i64,
) -> Option<f64> {
    if !is_failure_span(cause) {
        return None;
    }
    if lag_ms > config.cascade_window_ms {
        return None;
    }
    if is_failure_span(effect) {
        Some(0.9)
    } else {
        Some(0.65)
    }
}

fn is_failure_span(span: &UniversalSpan) -> bool {
    detects_failure(&span.payload)
}

fn detects_failure(value: &Value) -> bool {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let key_lower = key.to_lowercase();
                if key_lower.contains("error") || key_lower.contains("fail") {
                    return match val {
                        Value::Bool(b) => *b,
                        Value::String(s) => {
                            let ls = s.to_lowercase();
                            ls.contains("error") || ls.contains("fail") || ls.contains("panic")
                        }
                        _ => true,
                    };
                }
                if key_lower.contains("status") {
                    if let Some(s) = val.as_str() {
                        let ls = s.to_lowercase();
                        if ls.contains("error") || ls.contains("fail") || ls.contains("panic") {
                            return true;
                        }
                    }
                }
                if detects_failure(val) {
                    return true;
                }
            }
            false
        }
        Value::String(s) => {
            let ls = s.to_lowercase();
            ls.contains("error") || ls.contains("failure") || ls.contains("panic")
        }
        Value::Array(arr) => arr.iter().any(detects_failure),
        _ => false,
    }
}

fn describe_hypothesis(link: &CausalLink, cause: &UniversalSpan, effect: &UniversalSpan) -> String {
    match link.relation {
        CorrelationType::TemporalProximity => format!(
            "{} may influence {} due to tight temporal coupling",
            cause.name, effect.name
        ),
        CorrelationType::ResourceContention => format!(
            "Resource contention between {} and {} detected",
            cause.name, effect.name
        ),
        CorrelationType::StructuralSimilarity => format!(
            "{} shares structural behaviour with {}",
            cause.name, effect.name
        ),
        CorrelationType::CascadeFailure => format!(
            "{} failure likely cascades into {}",
            cause.name, effect.name
        ),
        CorrelationType::PerformanceCoupling => format!(
            "Performance metrics of {} and {} appear coupled",
            cause.name, effect.name
        ),
    }
}

fn describe_narrative(link: &CausalLink, cause: &UniversalSpan, effect: &UniversalSpan) -> String {
    match link.relation {
        CorrelationType::TemporalProximity => format!(
            "{} led {} by {} ms (confidence {:.2})",
            cause.name, effect.name, link.lag_ms, link.confidence
        ),
        CorrelationType::ResourceContention => format!(
            "Shared resource indicators between spans (confidence {:.2})",
            link.confidence
        ),
        CorrelationType::StructuralSimilarity => format!(
            "Structural similarity score {:.2} between spans",
            link.confidence
        ),
        CorrelationType::CascadeFailure => format!(
            "Failure signature in {} observed before {} (confidence {:.2})",
            cause.name, effect.name, link.confidence
        ),
        CorrelationType::PerformanceCoupling => format!(
            "Aligned latency/throughput metrics between spans (confidence {:.2})",
            link.confidence
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, TimeZone};
    use serde_json::json;

    fn make_span(id: &str, start_ms: i64, payload: Value) -> UniversalSpan {
        let start = Utc.timestamp_millis_opt(start_ms).unwrap();
        let mut span = UniversalSpan::new(id, id, "flow", "workflow", start, payload);
        span.finished_at = Some(start + Duration::milliseconds(100));
        span
    }

    fn run_engine(spans: Vec<UniversalSpan>) -> Vec<CausalChain> {
        let mut engine = CausalEngine::new();
        engine.ingest(spans);
        engine.infer()
    }

    #[test]
    fn temporal_rule_creates_link() {
        let a = make_span("a", 0, json!({"latency_ms": 120}));
        let b = make_span("b", 2000, json!({"latency_ms": 140}));
        let chains = run_engine(vec![a, b]);
        assert!(chains
            .iter()
            .any(|c| matches!(c.links[0].relation, CorrelationType::TemporalProximity)));
    }

    #[test]
    fn tightened_temporal_window_blocks_link() {
        let config = CausalConfig {
            temporal_window_ms: 500,
            ..Default::default()
        };
        let mut engine = CausalEngine::with_config(config);
        let a = make_span("a", 0, json!({"latency_ms": 100}));
        let b = make_span("b", 2000, json!({"latency_ms": 110}));
        engine.ingest(vec![a, b]);
        let chains = engine.infer();
        assert!(chains
            .iter()
            .all(|c| c.links[0].relation != CorrelationType::TemporalProximity));
    }

    #[test]
    fn resource_rule_detects_overlap() {
        let a = make_span(
            "a",
            0,
            json!({"cpu_usage": 0.8, "metadata": {"resource": "GPU"}}),
        );
        let b = make_span(
            "b",
            1000,
            json!({"metadata": {"resource": "gpu", "notes": "memory intensive"}}),
        );
        let chains = run_engine(vec![a, b]);
        assert!(chains
            .iter()
            .any(|c| matches!(c.links[0].relation, CorrelationType::ResourceContention)));
    }

    #[test]
    fn structural_rule_activates_for_similar_spans() {
        let payload = json!({
            "results": {"final_rmsd_angstrom": 0.6, "final_energy_kcal_mol": -110.0},
            "execution": {"simulation_time_ns": 0.5}
        });
        let a = make_span("a", 0, payload.clone());
        let b = make_span("b", 1200, payload);
        let chains = run_engine(vec![a, b]);
        assert!(chains
            .iter()
            .any(|c| matches!(c.links[0].relation, CorrelationType::StructuralSimilarity)));
    }

    #[test]
    fn performance_coupling_detects_close_latency() {
        let a = make_span("a", 0, json!({"latency_ms": 150.0}));
        let b = make_span("b", 1500, json!({"latency_ms": 165.0}));
        let chains = run_engine(vec![a, b]);
        assert!(chains
            .iter()
            .any(|c| matches!(c.links[0].relation, CorrelationType::PerformanceCoupling)));
    }

    #[test]
    fn cascade_failure_detects_error_propagation() {
        let a = make_span("a", 0, json!({"status": "error", "message": "timeout"}));
        let b = make_span("b", 800, json!({"status": "degraded"}));
        let chains = run_engine(vec![a, b]);
        assert!(chains
            .iter()
            .any(|c| matches!(c.links[0].relation, CorrelationType::CascadeFailure)));
    }
}
