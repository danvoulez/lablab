//! RAG (Retrieval-Augmented Generation) system for LogLine Director
//! 
//! This module provides contextual knowledge by indexing and retrieving
//! relevant historical data from the laboratory operations.

use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::{info, warn, debug};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    pub id: String,
    pub content: String,
    pub metadata: EntryMetadata,
    pub embedding: Option<Vec<f32>>, // Will be populated by embedding model
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMetadata {
    pub timestamp: String,
    pub source: String,        // "span", "log", "result"
    pub category: String,      // "error", "success", "analysis", "simulation"
    pub subject: Option<String>, // "HIV", "dengue", "malaria", etc.
    pub priority: String,      // "low", "medium", "high", "critical"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub entry: KnowledgeEntry,
    pub similarity: f32,
}

pub struct RAGSystem {
    knowledge_base: HashMap<String, KnowledgeEntry>,
    index_path: String,
}

impl RAGSystem {
    pub fn new(index_path: &str) -> Self {
        Self {
            knowledge_base: HashMap::new(),
            index_path: index_path.to_string(),
        }
    }

    /// Initialize RAG system and load existing knowledge base
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🧠 Initializing RAG system...");
        
        // Create index directory if it doesn't exist
        if let Some(parent) = Path::new(&self.index_path).parent() {
            fs::create_dir_all(parent).await?;
        }

        // Load existing knowledge base
        self.load_knowledge_base().await?;
        
        info!("✅ RAG system initialized with {} entries", self.knowledge_base.len());
        Ok(())
    }

    /// Add new knowledge entry to the system
    pub async fn add_knowledge(&mut self, content: &str, metadata: EntryMetadata) -> Result<String, Box<dyn std::error::Error>> {
        let id = self.generate_entry_id(content, &metadata);
        
        let entry = KnowledgeEntry {
            id: id.clone(),
            content: content.to_string(),
            metadata,
            embedding: None, // TODO: Generate embedding using local model
        };

        self.knowledge_base.insert(id.clone(), entry);
        self.save_knowledge_base().await?;
        
        debug!("Added knowledge entry: {}", id);
        Ok(id)
    }

    /// Search for relevant knowledge entries
    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        info!("🔍 Searching knowledge base for: '{}'", query);
        
        // For now, use simple keyword matching
        // TODO: Replace with semantic similarity using embeddings
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        for entry in self.knowledge_base.values() {
            let content_lower = entry.content.to_lowercase();
            let similarity = self.calculate_keyword_similarity(&query_lower, &content_lower);
            
            if similarity > 0.1 { // Threshold for relevance
                results.push(SearchResult {
                    entry: entry.clone(),
                    similarity,
                });
            }
        }

        // Sort by similarity (highest first)
        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
        results.truncate(limit);

        info!("Found {} relevant entries", results.len());
        Ok(results)
    }

    /// Get contextual information for a query
    pub async fn get_context(&self, query: &str) -> Result<String, Box<dyn std::error::Error>> {
        let results = self.search(query, 3).await?;
        
        if results.is_empty() {
            return Ok("Nenhum contexto histórico relevante encontrado.".to_string());
        }

        let mut context = String::from("CONTEXTO HISTÓRICO RELEVANTE:\n\n");
        
        for (i, result) in results.iter().enumerate() {
            context.push_str(&format!(
                "{}. [{}] {} ({})\n{}\n\n",
                i + 1,
                result.entry.metadata.timestamp,
                result.entry.metadata.category.to_uppercase(),
                result.entry.metadata.source,
                result.entry.content
            ));
        }

        Ok(context)
    }

    /// Populate knowledge base with sample laboratory data
    pub async fn populate_sample_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("📚 Populating knowledge base with sample laboratory data...");

        let sample_entries = vec![
            (
                "Simulação de docking para compostos anti-HIV falhou devido a recursos insuficientes de memória. Recomenda-se aumentar alocação de RAM para 32GB.",
                EntryMetadata {
                    timestamp: "2025-09-30T14:30:00Z".to_string(),
                    source: "span".to_string(),
                    category: "error".to_string(),
                    subject: Some("HIV".to_string()),
                    priority: "high".to_string(),
                }
            ),
            (
                "Análise QSAR para compostos anti-malária completada com sucesso. 15 compostos promissores identificados com IC50 < 1μM.",
                EntryMetadata {
                    timestamp: "2025-09-30T16:45:00Z".to_string(),
                    source: "result".to_string(),
                    category: "success".to_string(),
                    subject: Some("malaria".to_string()),
                    priority: "medium".to_string(),
                }
            ),
            (
                "Pipeline de machine learning para predição de toxicidade apresentou overfitting. Ajustando regularização L2.",
                EntryMetadata {
                    timestamp: "2025-10-01T09:15:00Z".to_string(),
                    source: "log".to_string(),
                    category: "analysis".to_string(),
                    subject: Some("toxicity".to_string()),
                    priority: "medium".to_string(),
                }
            ),
            (
                "Simulação de dinâmica molecular para proteína spike do SARS-CoV-2 convergiu após 100ns. Identificadas 3 cavidades de ligação potenciais.",
                EntryMetadata {
                    timestamp: "2025-10-01T11:20:00Z".to_string(),
                    source: "result".to_string(),
                    category: "success".to_string(),
                    subject: Some("COVID-19".to_string()),
                    priority: "high".to_string(),
                }
            ),
            (
                "Backup automático do banco de dados molecular completado. 2.3TB de dados de compostos arquivados com sucesso.",
                EntryMetadata {
                    timestamp: "2025-10-01T02:00:00Z".to_string(),
                    source: "system".to_string(),
                    category: "success".to_string(),
                    subject: None,
                    priority: "low".to_string(),
                }
            ),
        ];

        for (content, metadata) in sample_entries {
            self.add_knowledge(content, metadata).await?;
        }

        info!("✅ Sample data populated successfully");
        Ok(())
    }

    // Private helper methods
    
    fn generate_entry_id(&self, content: &str, metadata: &EntryMetadata) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        hasher.update(metadata.timestamp.as_bytes());
        hasher.update(metadata.source.as_bytes());
        hex::encode(hasher.finalize())[..16].to_string()
    }

    fn calculate_keyword_similarity(&self, query: &str, content: &str) -> f32 {
        let query_words: Vec<&str> = query.split_whitespace().collect();
        let content_words: Vec<&str> = content.split_whitespace().collect();
        
        if query_words.is_empty() || content_words.is_empty() {
            return 0.0;
        }

        let mut matches = 0;
        for query_word in &query_words {
            if content_words.iter().any(|&content_word| {
                content_word.contains(query_word) || query_word.contains(content_word)
            }) {
                matches += 1;
            }
        }

        matches as f32 / query_words.len() as f32
    }

    async fn load_knowledge_base(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match fs::read_to_string(&self.index_path).await {
            Ok(content) => {
                self.knowledge_base = serde_json::from_str(&content)?;
                info!("Loaded {} entries from knowledge base", self.knowledge_base.len());
            }
            Err(_) => {
                info!("No existing knowledge base found, starting fresh");
            }
        }
        Ok(())
    }

    async fn save_knowledge_base(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(&self.knowledge_base)?;
        fs::write(&self.index_path, content).await?;
        Ok(())
    }
}
