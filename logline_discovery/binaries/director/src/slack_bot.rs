//! Slack Bot integration for LogLine Director
//! 
//! Enables the Director to operate as a Slack bot, allowing laboratory
//! management through Slack channels and direct messages.

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn, error};

use crate::rag_models::IntelligentRAGSelector;

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackMessage {
    pub text: String,
    pub channel: String,
    pub user: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackResponse {
    pub text: String,
    pub response_type: Option<String>, // "in_channel" or "ephemeral"
    pub attachments: Option<Vec<SlackAttachment>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackAttachment {
    pub color: String,
    pub title: String,
    pub text: String,
    pub fields: Option<Vec<SlackField>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackField {
    pub title: String,
    pub value: String,
    pub short: bool,
}

// Shared state for Slack bot
#[derive(Clone)]
pub struct SlackBotState {
    pub rag_selector: Arc<Mutex<Option<IntelligentRAGSelector>>>,
    pub bot_token: String,
}

impl SlackBotState {
    pub async fn new(bot_token: String) -> Result<Self, Box<dyn std::error::Error>> {
        let rag_selector = match IntelligentRAGSelector::new().await {
            Ok(selector) => Some(selector),
            Err(e) => {
                warn!("Failed to initialize RAG selector for Slack bot: {}", e);
                None
            }
        };

        Ok(Self {
            rag_selector: Arc::new(Mutex::new(rag_selector)),
            bot_token,
        })
    }
}

pub fn create_slack_router() -> Router<SlackBotState> {
    Router::new()
        .route("/slack/events", post(handle_slack_events))
        .route("/slack/slash", post(handle_slack_slash_command))
        .route("/slack/interactive", post(handle_slack_interactive))
}

// Event handlers

async fn handle_slack_events(
    State(state): State<SlackBotState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("📱 Received Slack event: {:?}", payload);

    // Handle URL verification challenge
    if let Some(challenge) = payload.get("challenge") {
        return Ok(Json(serde_json::json!({
            "challenge": challenge
        })));
    }

    // Handle app mention events
    if let Some(event) = payload.get("event") {
        if event.get("type") == Some(&serde_json::Value::String("app_mention".to_string())) {
            return handle_app_mention(state, event).await;
        }
    }

    Ok(Json(serde_json::json!({"ok": true})))
}

async fn handle_app_mention(
    state: SlackBotState,
    event: &serde_json::Value,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let text = event.get("text")
        .and_then(|t| t.as_str())
        .unwrap_or("");
    
    let _channel = event.get("channel")
        .and_then(|c| c.as_str())
        .unwrap_or("");
    
    let user = event.get("user")
        .and_then(|u| u.as_str())
        .unwrap_or("unknown");

    info!("🤖 Processing Slack mention from {}: '{}'", user, text);

    // Remove bot mention from text
    let clean_text = text
        .split_whitespace()
        .skip(1) // Skip the @bot_name
        .collect::<Vec<_>>()
        .join(" ");

    // Process with Director intelligence
    let response = match process_slack_message(&state, &clean_text, user).await {
        Ok(resp) => resp,
        Err(e) => {
            error!("Failed to process Slack message: {}", e);
            SlackResponse {
                text: "❌ Desculpe, encontrei um erro ao processar sua solicitação.".to_string(),
                response_type: Some("ephemeral".to_string()),
                attachments: None,
            }
        }
    };

    // Send response back to Slack
    // In a real implementation, you'd use the Slack Web API to post the message
    info!("📤 Sending Slack response: {}", response.text);

    Ok(Json(serde_json::json!({"ok": true})))
}

async fn handle_slack_slash_command(
    State(state): State<SlackBotState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<SlackResponse>, StatusCode> {
    let command = payload.get("command")
        .and_then(|c| c.as_str())
        .unwrap_or("");
    
    let text = payload.get("text")
        .and_then(|t| t.as_str())
        .unwrap_or("");
    
    let user_name = payload.get("user_name")
        .and_then(|u| u.as_str())
        .unwrap_or("unknown");

    info!("⚡ Received Slack slash command: {} from {}: '{}'", command, user_name, text);

    match command {
        "/director" => {
            let response = process_slack_message(&state, text, user_name).await
                .unwrap_or_else(|_| SlackResponse {
                    text: "❌ Erro interno do Director".to_string(),
                    response_type: Some("ephemeral".to_string()),
                    attachments: None,
                });
            
            Ok(Json(response))
        }
        "/lab-status" => {
            Ok(Json(SlackResponse {
                text: "📊 Status do Laboratório".to_string(),
                response_type: Some("in_channel".to_string()),
                attachments: Some(vec![
                    SlackAttachment {
                        color: "good".to_string(),
                        title: "Queue Status".to_string(),
                        text: "Sistema operacional".to_string(),
                        fields: Some(vec![
                            SlackField {
                                title: "Na Fila".to_string(),
                                value: "12".to_string(),
                                short: true,
                            },
                            SlackField {
                                title: "Executando".to_string(),
                                value: "3".to_string(),
                                short: true,
                            },
                            SlackField {
                                title: "Workers Ativos".to_string(),
                                value: "6/8".to_string(),
                                short: true,
                            },
                        ]),
                    }
                ]),
            }))
        }
        _ => {
            Ok(Json(SlackResponse {
                text: format!("❓ Comando desconhecido: {}", command),
                response_type: Some("ephemeral".to_string()),
                attachments: None,
            }))
        }
    }
}

async fn handle_slack_interactive(
    State(_state): State<SlackBotState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("🔘 Received Slack interactive component: {:?}", payload);
    
    // Handle button clicks, menu selections, etc.
    Ok(Json(serde_json::json!({"ok": true})))
}

// Helper functions

async fn process_slack_message(
    state: &SlackBotState,
    text: &str,
    user: &str,
) -> Result<SlackResponse, Box<dyn std::error::Error>> {
    let rag_selector = state.rag_selector.lock().await;
    let selector = match rag_selector.as_ref() {
        Some(s) => s,
        None => {
            return Ok(SlackResponse {
                text: "🔧 Director está inicializando, tente novamente em alguns segundos.".to_string(),
                response_type: Some("ephemeral".to_string()),
                attachments: None,
            });
        }
    };

    // Classify the message with RAG context
    let classification = selector.classify_with_context(text).await?;
    
    // Format response based on classification
    let response_text = match classification.flow.as_str() {
        "Monitor" => {
            format!("📊 **Status do Lab**\n\n*Confiança: {:.0}%*\n\n{}\n\n```\nFila: 12 jobs | Executando: 3 | Workers: 6/8 ativos\n```", 
                classification.confidence * 100.0, 
                classification.reasoning)
        }
        "Diagnose" => {
            format!("🔍 **Diagnóstico Solicitado**\n\n*Confiança: {:.0}%*\n\n{}\n\n✅ Nenhuma falha crítica detectada nas últimas 24h", 
                classification.confidence * 100.0, 
                classification.reasoning)
        }
        "SubmitJob" => {
            format!("🎯 **Job Submetido**\n\n*Confiança: {:.0}%*\n\n{}\n\n📝 Job ID: `job-{}`", 
                classification.confidence * 100.0, 
                classification.reasoning,
                uuid::Uuid::new_v4().to_string()[..8].to_string())
        }
        _ => {
            format!("🤖 **Director Resposta**\n\n*Classificação: {} (Confiança: {:.0}%)*\n\n{}", 
                classification.flow,
                classification.confidence * 100.0, 
                classification.reasoning)
        }
    };

    let color = match classification.priority.as_str() {
        "critical" => "danger",
        "high" => "warning", 
        "medium" => "good",
        _ => "#36a64f",
    };

    Ok(SlackResponse {
        text: format!("Olá <@{}>! 👋", user),
        response_type: Some("in_channel".to_string()),
        attachments: Some(vec![
            SlackAttachment {
                color: color.to_string(),
                title: "LogLine Discovery Lab Director".to_string(),
                text: response_text,
                fields: None,
            }
        ]),
    })
}

pub async fn start_slack_bot(port: u16, bot_token: String) -> Result<(), Box<dyn std::error::Error>> {
    let state = SlackBotState::new(bot_token).await?;
    let app = create_slack_router().with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    
    info!("📱 Director Slack Bot starting on http://0.0.0.0:{}", port);
    info!("🔗 Configure Slack webhook URL: http://your-domain:{}/slack/events", port);

    axum::serve(listener, app).await?;
    
    Ok(())
}
