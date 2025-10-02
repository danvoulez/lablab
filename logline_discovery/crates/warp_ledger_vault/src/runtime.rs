use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use warp_common::{new_audit_id, ContractEnvelope, ContractVersion, SpanRecord};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LogEntry {
    #[serde(rename = "span")]
    Span {
        span: SpanRecord,
        #[serde(default)]
        reversed: bool,
    },
    #[serde(rename = "contract")]
    Contract { contract: ContractEnvelope },
}

#[derive(Debug, Clone)]
pub struct SpanLog {
    pub path: PathBuf,
}

impl SpanLog {
    pub fn new<P: AsRef<Path>>(p: P) -> Result<Self> {
        let path = p.as_ref().to_path_buf();
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        if !path.exists() {
            File::create(&path)?;
        }
        Ok(Self { path })
    }

    pub fn append_span(&self, span: &SpanRecord) -> Result<()> {
        let mut f = fs::OpenOptions::new().append(true).open(&self.path)?;
        let entry = LogEntry::Span {
            span: span.clone(),
            reversed: false,
        };
        let line = serde_json::to_string(&entry)?;
        writeln!(f, "{}", line)?;
        Ok(())
    }

    pub fn append_contract(&self, contract: &ContractEnvelope) -> Result<()> {
        let mut f = fs::OpenOptions::new().append(true).open(&self.path)?;
        let entry = LogEntry::Contract {
            contract: contract.clone(),
        };
        let line = serde_json::to_string(&entry)?;
        writeln!(f, "{}", line)?;
        Ok(())
    }

    pub fn load(&self) -> Result<Vec<LogEntry>> {
        let f = File::open(&self.path)?;
        let reader = BufReader::new(f);
        let mut v = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let entry: LogEntry = serde_json::from_str(&line)?;
            v.push(entry);
        }
        Ok(v)
    }

    pub fn count(&self) -> Result<usize> {
        Ok(self.load()?.len())
    }

    pub fn find_span(&self, id: &str) -> Result<Option<SpanRecord>> {
        for e in self.load()? {
            if let LogEntry::Span { span, .. } = e {
                if span.id == id {
                    return Ok(Some(span));
                }
            }
        }
        Ok(None)
    }

    pub fn rollback_to(&self, target_span_id: &str) -> Result<(usize, ContractEnvelope)> {
        let mut entries = self.load()?;
        let mut target_idx: Option<usize> = None;
        for (i, e) in entries.iter().enumerate() {
            if let LogEntry::Span { span, .. } = e {
                if span.id == target_span_id {
                    target_idx = Some(i);
                }
            }
        }
        let idx = match target_idx {
            Some(i) => i,
            None => bail!("span id not found: {}", target_span_id),
        };

        let mut reversed_count = 0usize;
        for i in (idx + 1)..entries.len() {
            if let LogEntry::Span { reversed, .. } = &mut entries[i] {
                if !*reversed {
                    *reversed = true;
                    reversed_count += 1;
                }
            }
        }

        let tmp = self.path.with_extension("tmp");
        {
            let mut f = File::create(&tmp)?;
            for e in &entries {
                let s = serde_json::to_string(e)?;
                writeln!(f, "{}", s)?;
            }
        }
        fs::rename(&tmp, &self.path)?;

        let contract = ContractEnvelope {
            version: ContractVersion("1.x".into()),
            audit_id: new_audit_id(),
            kind: "rollback_performed".into(),
            payload: serde_json::json!({
                "target_span_id": target_span_id,
                "reversed_count": reversed_count,
            }),
        };
        self.append_contract(&contract)?;

        Ok((reversed_count, contract))
    }

    pub fn checkpoint_create(&self, label: &str) -> Result<ContractEnvelope> {
        let ck = Checkpoint {
            id: warp_common::new_audit_id().0,
            label: label.to_string(),
            created_ms: warp_common::SpanRecord::now(),
        };
        let contract = ContractEnvelope {
            version: ContractVersion("1.x".into()),
            audit_id: new_audit_id(),
            kind: "checkpoint".into(),
            payload: serde_json::to_value(&ck)?,
        };
        self.append_contract(&contract)?;
        Ok(contract)
    }

    pub fn rollback_to_checkpoint(&self, label: &str) -> Result<(usize, ContractEnvelope)> {
        let mut entries = self.load()?;
        let mut ck_idx: Option<usize> = None;
        for (i, e) in entries.iter().enumerate() {
            if let LogEntry::Contract { contract } = e {
                if contract.kind == "checkpoint" {
                    if let Some(lab) = contract.payload.get("label").and_then(|x| x.as_str()) {
                        if lab == label {
                            ck_idx = Some(i);
                        }
                    }
                }
            }
        }
        let idx = match ck_idx {
            Some(i) => i,
            None => bail!("checkpoint not found: {}", label),
        };

        let mut reversed_count = 0usize;
        for i in (idx + 1)..entries.len() {
            if let LogEntry::Span { reversed, .. } = &mut entries[i] {
                if !*reversed {
                    *reversed = true;
                    reversed_count += 1;
                }
            }
        }

        let tmp = self.path.with_extension("tmp");
        {
            let mut f = File::create(&tmp)?;
            for e in &entries {
                let s = serde_json::to_string(e)?;
                writeln!(f, "{}", s)?;
            }
        }
        fs::rename(&tmp, &self.path)?;

        let contract = ContractEnvelope {
            version: ContractVersion("1.x".into()),
            audit_id: new_audit_id(),
            kind: "checkpoint_rollback_performed".into(),
            payload: serde_json::json!({
                "checkpoint_label": label,
                "reversed_count": reversed_count,
            }),
        };
        self.append_contract(&contract)?;

        Ok((reversed_count, contract))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub id: String,
    pub label: String,
    pub created_ms: u128,
}
