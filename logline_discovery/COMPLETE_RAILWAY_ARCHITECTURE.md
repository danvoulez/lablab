# 🚂 LogLine Discovery Lab - Complete Railway Architecture

## 🏗️ Full Lab Architecture

### 🌐 Railway Services (Always Online)
```
logline-discovery-lab/
├─ director/           # 🤖 Director API & Slack Bot
├─ dashboard/          # 📊 Next.js Web Dashboard  
├─ database/           # 🗄️ PostgreSQL Database
├─ queue/              # 📋 Job Queue System
├─ notifications/      # 🔔 Alerts & Webhooks
└─ metrics/            # 📈 Observability
```

### 💪 Mac Mini Workers (Heavy Computing)
```
mac-mini-workers/
├─ molecular-sim/      # 🧬 QSAR, Docking, MD
├─ ollama-models/      # 🤖 Local LLMs
├─ jupyter-lab/        # 🔬 Analysis Notebooks
├─ data-storage/       # 💾 Large Datasets
└─ gpu-compute/        # 🖥️ CUDA/Metal Tasks
```

## 🚀 Railway Deployment Strategy

### 1. **Multi-Service Deployment**
```bash
# Each component as separate Railway service
railway init director
railway init dashboard  
railway init queue
railway init notifications
```

### 2. **Database Setup**
```sql
-- Railway PostgreSQL
CREATE DATABASE logline_discovery;
CREATE TABLE experiments (...);
CREATE TABLE results (...);
CREATE TABLE knowledge_base (...);
```

### 3. **Environment Variables**
```env
# Shared across services
DATABASE_URL=postgresql://...
SLACK_BOT_TOKEN=xoxb-...
MAC_MINI_ENDPOINT=https://your-mac.ngrok.io
OPENAI_API_KEY=sk-...
GEMINI_API_KEY=AIza...
```

## 🔄 Job Flow Architecture

```
User Request (Slack/Web)
       ↓
Railway Director (classify & queue)
       ↓
Railway Queue System (job management)
       ↓
Mac Mini Worker (heavy compute)
       ↓
Railway Database (store results)
       ↓
Railway Dashboard (display results)
       ↓
User Notification (Slack/Email)
```

## 📊 Benefits of This Architecture

✅ **24/7 Availability**: Railway keeps interface online
✅ **Cost Optimization**: Heavy compute stays local
✅ **Scalability**: Railway auto-scales interface
✅ **Data Security**: Sensitive data on Mac Mini
✅ **Performance**: Best of both worlds
✅ **Monitoring**: Full observability in cloud
✅ **Collaboration**: Team access via web/Slack

## 🎯 Implementation Plan

### Phase 1: Core Services
- [ ] Director API on Railway
- [ ] PostgreSQL Database
- [ ] Basic Web Dashboard
- [ ] Mac Mini connection

### Phase 2: Advanced Features  
- [ ] Job Queue System
- [ ] Real-time notifications
- [ ] Advanced analytics
- [ ] Multi-worker support

### Phase 3: Production Ready
- [ ] Full observability
- [ ] Auto-scaling
- [ ] Backup systems
- [ ] Security hardening
