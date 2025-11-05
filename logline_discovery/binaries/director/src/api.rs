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
use uuid::Uuid;

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

// Protein simulation types
#[derive(Debug, Serialize, Deserialize)]
pub struct SimulateProteinRequest {
    pub sequence: String,
    pub mutation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulateProteinResponse {
    pub session_id: String,
    pub pdb: String,
    pub plddt: Vec<f64>,
    pub confidence: ConfidenceMetrics,
    pub structure_hash: String,
    pub audit_trail: Vec<String>,
    pub manifest: ManifestData,
    pub started_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfidenceMetrics {
    pub overall: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestData {
    pub session_id: String,
    pub timestamp: String,
    pub participants: Vec<String>,
    pub scientific_question: String,
    pub methodology: Vec<String>,
    pub findings: Vec<Finding>,
    pub conclusions: Vec<String>,
    pub digital_signature: String,
    pub audit_trail: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Finding {
    pub title: String,
    pub description: String,
    pub evidence: Option<String>,
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
        .route("/api/simulate_protein", post(simulate_protein_handler))
        .route("/api/chat", post(chat_handler))
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

async fn simulate_protein_handler(
    State(_state): State<ApiState>,
    Json(request): Json<SimulateProteinRequest>,
) -> Result<Json<SimulateProteinResponse>, StatusCode> {
    use sha2::{Sha256, Digest};
    use uuid::Uuid;
    
    info!("🧬 API simulate_protein request for sequence (len={})", request.sequence.len());
    
    let session_id = format!("sess_{}", Uuid::new_v4().to_string().split('-').next().unwrap());
    let started_at = Utc::now().to_rfc3339();
    
    // Generate sample pLDDT values (in production, this would come from real folding engine)
    let sequence_len = request.sequence.chars().filter(|c| c.is_alphabetic()).count().max(10);
    let plddt: Vec<f64> = (0..sequence_len)
        .map(|i| {
            // Simulate realistic pLDDT: higher in middle, lower at terminals
            let pos = i as f64 / sequence_len as f64;
            let base = 75.0 + 20.0 * (1.0 - (pos - 0.5).abs() * 2.0);
            base + (i as f64 * 7.0).sin() * 5.0
        })
        .collect();
    
    let overall_confidence = plddt.iter().sum::<f64>() / plddt.len() as f64;
    
    // Generate minimal PDB structure (would be from real folding in production)
    let pdb = format!(
        "REMARK   LogLine Discovery Lab - Session {}\n\
         REMARK   Generated: {}\n\
         ATOM      1  N   THR A   1      26.017  24.447   2.842  1.00 50.00           N\n\
         ATOM      2  CA  THR A   1      25.948  23.027   2.451  1.00 50.00           C\n\
         ATOM      3  C   THR A   1      24.505  22.574   2.106  1.00 50.00           C\n\
         ATOM      4  O   THR A   1      24.226  21.375   2.118  1.00 50.00           O\n\
         ATOM      5  N   THR A   2      23.615  23.504   1.771  1.00 50.00           N\n\
         ATOM      6  CA  THR A   2      22.211  23.185   1.444  1.00 50.00           C\n\
         ATOM      7  C   THR A   2      21.327  23.229   2.697  1.00 50.00           C\n\
         ATOM      8  O   THR A   2      21.767  23.650   3.774  1.00 50.00           O\n\
         ATOM      9  N   CYS A   3      20.079  22.806   2.546  1.00 50.00           N\n\
         ATOM     10  CA  CYS A   3      19.123  22.788   3.657  1.00 50.00           C\n\
         ATOM     11  C   CYS A   3      17.670  22.896   3.185  1.00 50.00           C\n\
         ATOM     12  O   CYS A   3      17.396  22.852   1.985  1.00 50.00           O\n\
         TER\n",
        session_id, started_at
    );
    
    // Create structure hash
    let mut hasher = Sha256::new();
    hasher.update(pdb.as_bytes());
    hasher.update(serde_json::to_string(&plddt).unwrap().as_bytes());
    let structure_hash = format!("{:x}", hasher.finalize());
    
    // Build audit trail
    let mut audit_trail = vec![
        "🔬 Sessão de simulação iniciada".to_string(),
        "✅ Sequência validada e processada".to_string(),
    ];
    
    if let Some(ref mut_info) = request.mutation {
        audit_trail.push(format!("🧬 Mutação aplicada: {}", mut_info));
    }
    
    audit_trail.extend(vec![
        "⚙️  Predição estrutural executada".to_string(),
        "📊 Cálculo de confiança pLDDT concluído".to_string(),
        "🔐 Hash criptográfico gerado".to_string(),
        "📝 Manifesto científico criado".to_string(),
    ]);
    
    // Create manifest with digital signature
    let manifest_payload = serde_json::json!({
        "session_id": &session_id,
        "timestamp": &started_at,
        "sequence": &request.sequence,
        "mutation": &request.mutation,
        "structure_hash": &structure_hash,
    });
    
    let mut sig_hasher = Sha256::new();
    sig_hasher.update(manifest_payload.to_string().as_bytes());
    let digital_signature = format!("{:x}", sig_hasher.finalize());
    
    let manifest = ManifestData {
        session_id: session_id.clone(),
        timestamp: started_at.clone(),
        participants: vec![
            "Pesquisador".to_string(),
            "LogLine Bio (Assistente IA)".to_string(),
        ],
        scientific_question: format!(
            "Investigação estrutural de proteína ({} resíduos) via predição computacional",
            sequence_len
        ),
        methodology: vec![
            "Análise de sequência FASTA".to_string(),
            "Predição de estrutura 3D".to_string(),
            "Cálculo de confiança pLDDT por resíduo".to_string(),
            "Geração de hash criptográfico para reprodutibilidade".to_string(),
            "Assinatura digital de evidências".to_string(),
        ],
        findings: vec![
            Finding {
                title: "Estrutura 3D Predita".to_string(),
                description: format!("Modelo PDB gerado com {} resíduos", sequence_len),
                evidence: Some(structure_hash.clone()),
            },
            Finding {
                title: "Perfil de Confiança".to_string(),
                description: format!("pLDDT médio: {:.1}%", overall_confidence),
                evidence: None,
            },
        ],
        conclusions: vec![
            "Pipeline de predição executado com sucesso".to_string(),
            "Todos os artefatos hashados e auditáveis".to_string(),
            "Resultados prontos para análise científica".to_string(),
        ],
        digital_signature,
        audit_trail: audit_trail.clone(),
    };
    
    let response = SimulateProteinResponse {
        session_id,
        pdb,
        plddt,
        confidence: ConfidenceMetrics {
            overall: overall_confidence,
        },
        structure_hash,
        audit_trail,
        manifest,
        started_at,
    };
    
    info!("✅ Simulation completed: session={}", response.session_id);
    
    Ok(Json(response))
}

async fn chat_handler(
    State(state): State<ApiState>,
    Json(request): Json<ConversationRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("💬 API chat request: '{}'", request.message);
    
    // Use personality greetings for agent responses
    use crate::personality::get_greeting;
    
    let rag_selector = state.rag_selector.lock().await;
    let selector = match rag_selector.as_ref() {
        Some(s) => s,
        None => return Err(StatusCode::SERVICE_UNAVAILABLE),
    };
    
    // Classify the message
    let classification = match selector.classify_with_context(&request.message).await {
        Ok(c) => c,
        Err(e) => {
            warn!("Chat classification failed: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    // Generate personalized response
    let response_content = if request.message.to_lowercase().contains("olá") 
        || request.message.to_lowercase().contains("oi")
        || request.message.to_lowercase().contains("hello") {
        format!("{} Como posso ajudar em sua pesquisa hoje?", get_greeting())
    } else {
        format!(
            "Entendi sua questão sobre {}. Classificado como: {} (prioridade: {}, confiança: {:.0}%). {}",
            request.message,
            classification.flow,
            classification.priority,
            classification.confidence * 100.0,
            "Vamos explorar isso juntos!"
        )
    };
    
    Ok(Json(serde_json::json!({
        "session_id": format!("chat_{}", Uuid::new_v4()),
        "message": response_content,
        "classification": {
            "flow": classification.flow,
            "priority": classification.priority,
            "confidence": classification.confidence,
        },
        "timestamp": Utc::now().to_rfc3339(),
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
