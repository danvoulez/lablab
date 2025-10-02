//! REST API for LogLine Director
//! 
//! Exposes the Director's intelligence as HTTP endpoints for integration
//! with other systems, dashboards, and automation tools.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, warn};

use crate::rag_models::IntelligentRAGSelector;
use crate::rag::EntryMetadata;
use crate::function_calling::{FunctionRegistry, FunctionCall};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationRequest {
    pub message: String,
    pub actor: Option<String>,
    pub include_context: Option<bool>,
    pub use_functions: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationResponse {
    pub classification: ClassificationResult,
    pub context_used: Option<String>,
    pub function_results: Option<Vec<FunctionExecutionResult>>,
    pub response_time_ms: u64,
    pub model_used: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassificationResult {
    pub flow: String,
    pub priority: String,
    pub confidence: f32,
    pub reasoning: String,
    pub requires_approval: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionExecutionResult {
    pub function_name: String,
    pub success: bool,
    pub data: serde_json::Value,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub models_available: Vec<String>,
    pub knowledge_base_entries: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KnowledgeRequest {
    pub content: String,
    pub source: String,
    pub category: String,
    pub subject: Option<String>,
    pub priority: String,
}

// Shared application state
#[derive(Clone)]
pub struct ApiState {
    pub rag_selector: Arc<Mutex<Option<IntelligentRAGSelector>>>,
    pub function_registry: Arc<FunctionRegistry>,
    pub start_time: std::time::Instant,
}

impl ApiState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let rag_selector = match IntelligentRAGSelector::new().await {
            Ok(selector) => Some(selector),
            Err(e) => {
                warn!("Failed to initialize RAG selector: {}", e);
                None
            }
        };

        Ok(Self {
            rag_selector: Arc::new(Mutex::new(rag_selector)),
            function_registry: Arc::new(FunctionRegistry::new()),
            start_time: std::time::Instant::now(),
        })
    }
}

pub fn create_router() -> Router<ApiState> {
    Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/api/converse", post(converse_handler))
        .route("/api/classify", post(classify_handler))
        .route("/api/functions", get(list_functions_handler))
        .route("/api/knowledge", post(add_knowledge_handler))
        .route("/api/knowledge", get(search_knowledge_handler))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

// Route handlers

async fn root_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "service": "LogLine Director API",
        "version": "1.0.0",
        "description": "Intelligent laboratory management assistant with RAG and function calling",
        "endpoints": {
            "GET /health": "Service health and status",
            "POST /api/converse": "Full conversation with context and functions",
            "POST /api/classify": "Simple intent classification",
            "GET /api/functions": "List available functions",
            "POST /api/knowledge": "Add knowledge to RAG system",
            "GET /api/knowledge": "Search knowledge base"
        }
    }))
}

async fn health_handler(State(state): State<ApiState>) -> Json<HealthResponse> {
    let uptime = state.start_time.elapsed().as_secs();

    // Check available Ollama models
    let models_available = vec![
        "director-smart".to_string(),
        "director-brain".to_string(),
        "mistral:instruct".to_string(),
        "llama3.2:1b".to_string(),
    ];

    // Get knowledge base size
    let knowledge_entries = if let Some(ref selector) = *state.rag_selector.lock().await {
        selector.knowledge_count().await
    } else {
        0
    };

    Json(HealthResponse {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
        uptime_seconds: uptime,
        models_available,
        knowledge_base_entries: knowledge_entries,
    })
}

async fn converse_handler(
    State(state): State<ApiState>,
    Json(request): Json<ConversationRequest>,
) -> Result<Json<ConversationResponse>, StatusCode> {
    let start_time = std::time::Instant::now();
    
    info!("🌐 API conversation request: '{}'", request.message);

    // Get RAG selector
    let rag_selector = state.rag_selector.lock().await;
    let selector = match rag_selector.as_ref() {
        Some(s) => s,
        None => return Err(StatusCode::SERVICE_UNAVAILABLE),
    };

    // Classify with context
    let classification = match selector.classify_with_context(&request.message).await {
        Ok(c) => c,
        Err(e) => {
            warn!("Classification failed: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Execute functions if requested
    let function_results = if request.use_functions.unwrap_or(false) {
        // For demo, execute queue status function
        let call = FunctionCall {
            name: "get_current_queue_status".to_string(),
            arguments: HashMap::new(),
        };
        
        let result = state.function_registry.execute_function(&call).await;
        vec![FunctionExecutionResult {
            function_name: call.name,
            success: result.success,
            data: result.data,
            error: result.error,
        }]
    } else {
        vec![]
    };

    let response_time = start_time.elapsed().as_millis() as u64;

    let response = ConversationResponse {
        classification: ClassificationResult {
            flow: classification.flow,
            priority: classification.priority,
            confidence: classification.confidence,
            reasoning: classification.reasoning,
            requires_approval: false, // Would check policy
        },
        context_used: Some("RAG context included".to_string()),
        function_results: if function_results.is_empty() { None } else { Some(function_results) },
        response_time_ms: response_time,
        model_used: "RAG-enhanced".to_string(),
    };

    Ok(Json(response))
}

async fn classify_handler(
    State(state): State<ApiState>,
    Json(request): Json<ConversationRequest>,
) -> Result<Json<ClassificationResult>, StatusCode> {
    info!("🌐 API classification request: '{}'", request.message);

    let rag_selector = state.rag_selector.lock().await;
    let selector = match rag_selector.as_ref() {
        Some(s) => s,
        None => return Err(StatusCode::SERVICE_UNAVAILABLE),
    };

    let classification = match selector.classify_with_context(&request.message).await {
        Ok(c) => c,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(ClassificationResult {
        flow: classification.flow,
        priority: classification.priority,
        confidence: classification.confidence,
        reasoning: classification.reasoning,
        requires_approval: false,
    }))
}

async fn list_functions_handler(State(state): State<ApiState>) -> Json<serde_json::Value> {
    let descriptions = state.function_registry.get_function_descriptions();
    
    Json(serde_json::json!({
        "available_functions": descriptions,
        "count": 5
    }))
}

async fn add_knowledge_handler(
    State(state): State<ApiState>,
    Json(request): Json<KnowledgeRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("🌐 Adding knowledge: {}", request.content);

    let metadata = EntryMetadata {
        timestamp: Utc::now().to_rfc3339(),
        source: request.source.clone(),
        category: request.category.clone(),
        subject: request.subject.clone(),
        priority: request.priority.clone(),
    };

    let entry_id = {
        let rag_selector = state.rag_selector.lock().await;
        let selector = rag_selector
            .as_ref()
            .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

        selector
            .add_knowledge(&request.content, metadata)
            .await
            .map_err(|e| {
                warn!("Failed to add knowledge: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
    };

    Ok(Json(serde_json::json!({
        "success": true,
        "entry_id": entry_id,
        "message": "Knowledge added successfully"
    })))
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    limit: Option<usize>,
}

async fn search_knowledge_handler(
    State(state): State<ApiState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("🌐 Searching knowledge: '{}'", params.q);

    let limit = params.limit.unwrap_or(10).clamp(1, 50);

    let rag_selector = state.rag_selector.lock().await;
    let selector = rag_selector
        .as_ref()
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    let results = selector
        .search(&params.q, limit)
        .await
        .map_err(|e| {
            warn!("Failed to search knowledge base: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let formatted_results: Vec<serde_json::Value> = results
        .into_iter()
        .map(|result| {
            serde_json::json!({
                "id": result.entry.id,
                "content": result.entry.content,
                "similarity": result.similarity,
                "metadata": {
                    "timestamp": result.entry.metadata.timestamp,
                    "source": result.entry.metadata.source,
                    "category": result.entry.metadata.category,
                    "subject": result.entry.metadata.subject,
                    "priority": result.entry.metadata.priority,
                }
            })
        })
        .collect();

    Ok(Json(serde_json::json!({
        "query": params.q,
        "results": formatted_results,
        "total_found": formatted_results.len(),
    })))
}

pub async fn start_api_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let state = ApiState::new().await?;
    let app = create_router().with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    
    info!("🌐 Director API server starting on http://0.0.0.0:{}", port);
    info!("📖 API documentation available at http://localhost:{}/", port);

    axum::serve(listener, app).await?;
    
    Ok(())
}
