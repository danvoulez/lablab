use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::Result;
use causal_engine::{CausalChain, CausalEngine};
use serde_json::Value;
use spans_core::{span_from_json, UniversalSpan};

pub fn run_causal_analysis(input: PathBuf) -> Result<Vec<CausalChain>> {
    let file = File::open(&input)?;
    let reader = BufReader::new(file);
    let payload: Value = serde_json::from_reader(reader)?;
    let spans = payload
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("expected array of spans"))?
        .iter()
        .map(|value| span_from_json(value.clone()))
        .collect::<Result<Vec<_>, _>>()?;

    let mut engine = CausalEngine::new();
    engine.ingest(spans);
    Ok(engine.infer())
}

pub fn sync_ledger(ndjson_path: PathBuf) -> Result<Vec<UniversalSpan>> {
    let file = File::open(&ndjson_path)?;
    let reader = BufReader::new(file);
    let mut spans = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let value: Value = serde_json::from_str(&line)?;
        let span = span_from_json(value)?;
        spans.push(span);
    }
    Ok(spans)
}
