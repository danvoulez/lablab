# 🧬 LogLine Discovery Lab

**Laboratório computacional de descoberta de medicamentos com IA conversacional**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-1.0.0--partial-green.svg)](releases/)

## 🎯 Visão Geral

O LogLine Discovery Lab é um **laboratório farmacêutico computacional** focado em descoberta de medicamentos para **HIV**, com ênfase em:

- 🤖 **Agente conversacional IA** em português brasileiro
- 🧬 **Simulações moleculares** (folding, energia, RMSD)
- 🔗 **Inferência causal** entre eventos científicos
- 📊 **Interface web moderna** para visualização
- 📱 **Integração Slack** profissional
- 🧠 **RAG System** com conhecimento contextual

## ✨ Funcionalidades Principais

### 🤖 Director (Agente Conversacional)
- **RAG System**: Base de conhecimento com contexto histórico HIV
- **Function Calling**: 5 ferramentas dinâmicas para monitoramento
- **Classificação inteligente**: Processamento de linguagem natural
- **Múltiplas interfaces**: CLI, API REST, Slack Bot

### 🧬 Motor Científico
- **Folding Runtime**: Análise de proteínas (gp41, gp120, Rev, Tat)
- **Causal Engine**: Inferência de relações causais
- **Discovery Agent**: Geração automática de hipóteses
- **Manuscript Generator**: Artigos científicos automáticos

### 🌐 Integrações
- **API REST**: 7 endpoints documentados
- **Slack Bot**: 4 slash commands + mentions
- **Dashboard Web**: Interface científica moderna
- **PostgreSQL**: Persistência estruturada

## 🚀 Instalação Rápida

### Pré-requisitos
```bash
# macOS
brew install postgresql@15 redis ollama

# Configurar Ollama
ollama pull mistral:instruct
ollama pull llama3.2
```

### Download e Instalação
```bash
# Download do release
wget https://github.com/seu-usuario/logline-discovery-lab/releases/download/v1.0.0/logline-discovery-lab-v1.0.0-partial.tar.gz

# Extrair e instalar
tar -xzf logline-discovery-lab-v1.0.0-partial.tar.gz
cd logline-discovery-lab-v1.0.0-partial
./install.sh
```

## 🎯 Uso

### 🤖 Director (Agente Conversacional)
```bash
# Modo interativo
./bin/director chat

# API REST
./bin/director serve --port 3000

# Bot Slack
export SLACK_BOT_TOKEN=xoxb-seu-token
./bin/director slack --port 3002

# Comando único
./bin/director exec "status do laboratório"
```

### 📊 Dashboard Web
```bash
# Iniciar dashboard
./bin/discovery_dashboard

# Acessar: http://127.0.0.1:4600
```

### 📱 Slack Integration
1. Use o manifest em `slack-app-manifest.json`
2. Configure sua URL (ngrok ou produção)
3. Instale o app no workspace
4. Use comandos:
   - `/director como está a fila?`
   - `/lab-status`
   - `@Director preciso saber sobre HIV`

## 🏗️ Arquitetura

```
🧬 LOGLINE DISCOVERY LAB
├─ 🤖 Director (Interface IA)
├─ 🔬 Scientific Engines (Análises)
├─ 📊 Discovery Dashboard (Web UI)
├─ 💾 Data Layer (Spans + DB)
└─ 🔌 Integrations (Slack + API)
```

## 📊 Status do Projeto

### ✅ Funcional (V1.0-Partial)
- 🤖 **Director**: Agente conversacional completo
- 📊 **Dashboard**: Interface web científica
- 🧠 **RAG System**: Knowledge base contextual
- 📱 **Slack Bot**: Integração profissional
- 🌐 **API REST**: 7 endpoints funcionais

### 🔄 Em Desenvolvimento (V1.1)
- 🧬 **HIV Discovery Runner**: Pipeline científico completo
- ⚡ **Job System**: Processamento distribuído
- 📊 **Observabilidade**: OpenTelemetry + métricas
- ⚡ **Cache**: Redis + Moka otimizado

## 🧬 Foco Científico: HIV

### Proteínas Estudadas
- **gp41**: Proteína de fusão viral
- **gp120**: Proteína de superfície
- **Rev**: Regulação de RNA
- **Tat**: Transativação

### Métricas Científicas
- **RMSD > 5.0 Å**: Instabilidade estrutural
- **Energia < -120 kcal/mol**: Estabilidade
- **Redução reservatório > 2 logs**: Sucesso terapêutico

## 🤝 Contribuição

```bash
# Clonar repositório
git clone https://github.com/seu-usuario/logline-discovery-lab.git
cd logline-discovery-lab

# Build desenvolvimento
cd logline_discovery
cargo build --release

# Executar testes
cargo test

# Corrigir warnings
cargo clippy --fix
```

## 📚 Documentação

- [📋 Changelog](CHANGELOG.md)
- [🔧 Setup Guide](docs/setup.md)
- [📡 API Documentation](docs/api.md)
- [📱 Slack Integration](SLACK_SETUP.md)

## 📄 Licença

Este projeto está licenciado sob MIT OR Apache-2.0 - veja os arquivos [LICENSE-MIT](LICENSE-MIT) e [LICENSE-APACHE](LICENSE-APACHE) para detalhes.

## 🎯 Roadmap

### V1.1 (Próxima)
- [ ] Corrigir erros de compilação restantes
- [ ] Sistema de jobs distribuído completo
- [ ] Observabilidade com OpenTelemetry
- [ ] Cache inteligente Redis/Moka

### V1.2 (Futuro)
- [ ] Expansão para outras doenças (Malária, COVID-19)
- [ ] ST-GNN para análise espaciotemporal
- [ ] Deploy automatizado Railway/Docker
- [ ] Interface mobile

## 🏆 Reconhecimentos

Desenvolvido com foco em descoberta de medicamentos para HIV, utilizando as melhores práticas de:
- **Rust** para performance e segurança
- **IA Conversacional** para acessibilidade
- **Arquitetura científica** para precisão
- **Integração moderna** para colaboração

---

**🧬 LogLine Discovery Lab - Acelerando a descoberta de medicamentos com IA** 🤖🇧🇷
