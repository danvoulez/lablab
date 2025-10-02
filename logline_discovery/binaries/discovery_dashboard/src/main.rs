use axum::{extract::Path, http::StatusCode, response::Html, routing::get, Json, Router};
use once_cell::sync::Lazy;
use serde_json::Value;
use std::{env, net::SocketAddr};
use tokio::sync::OnceCell;
use tracing::{info, Level};

static CLIENT: OnceCell<reqwest::Client> = OnceCell::const_new();
static SERVICE_URL: Lazy<String> = Lazy::new(|| {
    env::var("DISCOVERY_SERVICE_URL").unwrap_or_else(|_| "http://127.0.0.1:4040".to_string())
});

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    CLIENT
        .set(reqwest::Client::builder().build().expect("client"))
        .ok();

    let app = Router::new()
        .route("/", get(index))
        .route("/api/executions", get(list_executions))
        .route("/api/executions/:id/causal", get(execution_causal))
        .route("/api/executions/:id/spans", get(execution_spans));

    let addr: SocketAddr = env::var("DISCOVERY_DASHBOARD_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:4600".to_string())
        .parse()
        .expect("valid socket address");
    info!(%addr, service = %SERVICE_URL.clone(), "dashboard_listening");
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.expect("bind"),
        app,
    )
    .await
    .expect("server");
}

async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn list_executions() -> Result<Json<Value>, DashboardError> {
    proxy_json("/executions").await
}

async fn execution_causal(Path(id): Path<String>) -> Result<Json<Value>, DashboardError> {
    let path = format!("/executions/{}/causal", id);
    proxy_json(&path).await
}

async fn execution_spans(Path(id): Path<String>) -> Result<Json<Value>, DashboardError> {
    let path = format!("/executions/{}/spans", id);
    proxy_json(&path).await
}

async fn proxy_json(path: &str) -> Result<Json<Value>, DashboardError> {
    let url = format!("{}{}", SERVICE_URL.as_str(), path);
    let client = CLIENT
        .get()
        .expect("client not initialised")
        .get(&url)
        .send()
        .await?;

    if client.status().is_success() {
        let json = client.json::<Value>().await?;
        Ok(Json(json))
    } else {
        Err(DashboardError::Upstream(client.status().as_u16()))
    }
}

#[derive(Debug)]
enum DashboardError {
    Upstream(u16),
    Request(reqwest::Error),
}

impl From<reqwest::Error> for DashboardError {
    fn from(err: reqwest::Error) -> Self {
        DashboardError::Request(err)
    }
}

impl axum::response::IntoResponse for DashboardError {
    fn into_response(self) -> axum::response::Response {
        match self {
            DashboardError::Upstream(status) => (
                StatusCode::from_u16(status).unwrap_or(StatusCode::BAD_GATEWAY),
                format!("Upstream service returned status {status}"),
            )
                .into_response(),
            DashboardError::Request(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Request error: {err}"),
            )
                .into_response(),
        }
    }
}

const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\" />
  <title>Causal Chain Explorer</title>
  <style>
    body { font-family: sans-serif; margin: 2rem; background: #0c1021; color: #f6f7fb; }
    h1 { margin-bottom: 1rem; }
    select, button { padding: 0.5rem; margin-right: 0.5rem; }
    .card { background: #161b32; padding: 1rem; border-radius: 8px; margin-top: 1rem; }
    .links { margin-left: 1rem; }
    pre { white-space: pre-wrap; word-break: break-word; background: #11152a; padding: 1rem; border-radius: 6px; }
  </style>
</head>
<body>
  <h1>Causal Chain Explorer</h1>
  <p>Point the service API at <code>$DISCOVERY_SERVICE_URL</code> (default <code>http://127.0.0.1:4040</code>). Select an execution to inspect its spans and causal hypotheses.</p>
  <div>
    <select id=\"executionSelect\"></select>
    <button onclick=\"loadExecution()\">Load</button>
    <span id=\"status\"></span>
  </div>
  <div id=\"executionDetails\" class=\"card\" style=\"display:none\">
    <h2 id=\"executionTitle\"></h2>
    <h3>Spans</h3>
    <pre id=\"spans\"></pre>
    <h3>Causal Chains</h3>
    <div id=\"chains\"></div>
  </div>
  <script>
    async function fetchJSON(path) {
      const res = await fetch(path);
      if (!res.ok) throw new Error(`Request failed: ${res.status}`);
      return res.json();
    }

    async function loadExecutions() {
      try {
        const executions = await fetchJSON('/api/executions');
        const select = document.getElementById('executionSelect');
        select.innerHTML = '';
        executions.forEach(item => {
          const option = document.createElement('option');
          option.value = item.span_id;
          option.textContent = `${item.span_id} — ${item.workflow}`;
          select.appendChild(option);
        });
        if (executions.length > 0) {
          loadExecution();
        }
      } catch (err) {
        document.getElementById('status').textContent = err.message;
      }
    }

    async function loadExecution() {
      const select = document.getElementById('executionSelect');
      if (!select.value) return;
      const id = encodeURIComponent(select.value);
      try {
        document.getElementById('status').textContent = 'Loading...';
        const [spans, chains] = await Promise.all([
          fetchJSON(`/api/executions/${id}/spans`),
          fetchJSON(`/api/executions/${id}/causal`)
        ]);
        document.getElementById('status').textContent = '';
        document.getElementById('executionDetails').style.display = 'block';
        document.getElementById('executionTitle').textContent = select.value;
        document.getElementById('spans').textContent = JSON.stringify(spans, null, 2);
        const chainContainer = document.getElementById('chains');
        chainContainer.innerHTML = '';
        if (chains.length === 0) {
          chainContainer.textContent = 'No causal links detected.';
          return;
        }
        chains.forEach((chain, idx) => {
          const div = document.createElement('div');
          div.className = 'card';
          const linkList = chain.links.map(link => `${link.cause} → ${link.effect} (${link.relation}, confidence ${link.confidence.toFixed(2)})`).join('\n');
          div.innerHTML = `<h4>Hypothesis ${idx + 1}</h4><p>${chain.narrative || chain.hypothesis}</p><pre class='links'>${linkList}</pre>`;
          chainContainer.appendChild(div);
        });
      } catch (err) {
        document.getElementById('status').textContent = err.message;
      }
    }

    loadExecutions();
  </script>
</body>
</html>"#;
