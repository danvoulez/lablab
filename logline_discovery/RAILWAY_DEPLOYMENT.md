# 🚂 LogLine Discovery Lab - Railway Deployment Guide

## 🏗️ Arquitetura Híbrida

### Railway (Cloud Interface)
- **Director API/Slack Bot** - Interface sempre online
- **RAG Knowledge Base** - Contexto histórico
- **Web Dashboard** - Monitoramento em tempo real
- **Webhooks & Notifications** - Integrações externas

### Mac Mini (Heavy Computing)
- **Ollama Models** - LLMs locais (director-smart, mistral, etc.)
- **Molecular Simulations** - QSAR, Docking, MD
- **Large Datasets** - Compostos, resultados, modelos ML
- **GPU/CPU Tasks** - Processamento intensivo

## 🚀 Deploy Steps

### 1. **Preparar Railway Version**

```bash
# Criar versão cloud do Director (sem Ollama)
cp -r binaries/director binaries/director-cloud
```

### 2. **Railway Configuration**

```toml
# railway.toml
[build]
builder = "NIXPACKS"

[deploy]
healthcheckPath = "/health"
healthcheckTimeout = 300
restartPolicyType = "ON_FAILURE"

[env]
PORT = { default = "3000" }
RUST_LOG = { default = "info" }
SLACK_BOT_TOKEN = { from = "SLACK_BOT_TOKEN" }
MAC_MINI_URL = { from = "MAC_MINI_URL" }
```

### 3. **Environment Variables**

```bash
# No Railway Dashboard, configure:
SLACK_BOT_TOKEN=xoxb-seu-token-slack
MAC_MINI_URL=https://seu-mac-mini.ngrok.io
RUST_LOG=info
```

### 4. **Deploy Commands**

```bash
# Install Railway CLI
npm install -g @railway/cli

# Login and deploy
railway login
railway init
railway up
```

### 5. **Mac Mini Setup**

```bash
# No Mac Mini, rode:
./target/debug/director serve --port 8080  # API local
ngrok http 8080  # Expose para Railway
```

## 🔗 Communication Flow

```
User → Slack → Railway Director → Mac Mini Worker → Results → Railway → User
```

## 📊 Benefits

✅ **Always Online**: Railway garante uptime 24/7
✅ **Cost Effective**: Processamento pesado local
✅ **Scalable**: Railway auto-scale, Mac Mini dedicado
✅ **Secure**: Dados sensíveis ficam locais
✅ **Fast**: Interface rápida na cloud, compute local
