use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use serde_json::to_string;
use spans_core::UniversalSpan;

pub fn append_span(path: &Path, span: &UniversalSpan) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    let serialized = to_string(span)?;
    writeln!(file, "{}", serialized)?;
    file.flush()?;

    Ok(())
}
