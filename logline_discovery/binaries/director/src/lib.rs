//! LogLine Discovery Lab Director
//!
//! Unified agent that combines front-office (user interaction) and back-office (lab operations)
//! functionality into a single conversational interface.

pub mod agents;
pub mod api;               // REST API server
pub mod chat;
pub mod contracts;
pub mod executor;
pub mod fast_models;       // Fast and reliable model selection
pub mod function_calling;  // Dynamic function execution
pub mod intelligent_models;  // Intelligent model selection
pub mod labops;
// pub mod llm;  // Temporarily disabled - async-openai compatibility issues
pub mod observability;
pub mod parser;
pub mod personality;       // Unified personality system
pub mod policy;
// pub mod providers;  // Temporarily disabled - compilation issues
pub mod rag;               // RAG system for contextual knowledge
pub mod rag_models;        // RAG-enhanced model selection
pub mod simple_llm;  // Working Ollama integration
pub mod slack_bot;         // Slack bot integration
pub mod timeline;
pub mod tools;

// Reexports p/ manter os imports existentes nos módulos:
pub use contracts::{Contract, Flow, Workflow};
pub use policy::Policy;
pub use parser::Intent;
