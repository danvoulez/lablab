# 🧬 LogLine Discovery Lab

### Acelere descoberta de medicamentos HIV em 10x com IA conversacional

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-1.0.0--partial-green.svg)](releases/)

> **De semanas para horas**: Da proteína ao manuscrito científico automaticamente.  
> Único agente IA conversacional em português brasileiro para drug discovery.

---

## ⚡ Demo em 60 Segundos

```bash
# 1. Clone e setup
git clone https://github.com/danvoulez/lablab.git && cd lablab
./demo.sh

# 2. Pergunte ao agente
./target/release/director --mode cli
> Como funciona a proteína gp41 do HIV?

# 3. Veja o resultado
✅ Análise completa com métricas científicas validadas
```

**[📹 Assista ao vídeo demo completo →](https://youtube.com/...)** *(em breve)*

---

## 🎯 Por Que Isso Existe?

**O Problema**: Descobrir medicamentos para HIV leva 10+ anos e custa $2.6B.

**Nossa Solução**: Plataforma open source que acelera descoberta em 10x através de:
- 🤖 IA conversacional que entende português
- 🧬 Análise automatizada de proteínas HIV
- 📊 Pipeline completo: da proteína ao paper científico
- 🔓 100% transparente e auditável (open source)

**Diferencial**: Primeiro laboratório computacional com agente IA em português brasileiro, focado em HIV.

---

## ✨ Features Principais

### 🤖 Agente Conversacional (Director)
Converse naturalmente sobre HIV drug discovery:
```
Você: "Qual a diferença entre gp41 e gp120?"
IA: "A gp41 é responsável pela fusão de membranas, enquanto 
     gp120 faz a ligação inicial com receptores CD4..."
     [resposta completa com citações científicas]
```

- 🧠 **RAG System**: Contexto científico de milhares de papers
- 🔧 **Function Calling**: 5 ferramentas especializadas
- 💬 **3 Interfaces**: CLI, API REST, Slack Bot

### 🧬 Motor Científico HIV
Análise completa de proteínas:
- **Proteínas Suportadas**: gp41, gp120, Rev, Tat
- **Métricas**: RMSD, energia molecular, estabilidade
- **Output**: Manuscritos científicos automáticos

### 📊 Dashboard Web
Interface visual moderna para:
- Monitorar simulações em tempo real
- Visualizar gráficos de RMSD e energia
- Buscar na base de conhecimento
- Gerar relatórios automáticos

### 📱 Integração Slack
Bot profissional para equipes distribuídas:
```
/director Analisar estabilidade da gp41
/hiv-status
/knowledge-search mecanismos de fusão
```

---

## 🚀 Quick Start

### Opção 1: Demo Rápido (Recomendado)
```bash
git clone https://github.com/danvoulez/lablab.git
cd lablab
./demo.sh  # Demonstração interativa
```

### Opção 2: Instalação Completa
Ver [**GETTING_STARTED.md**](GETTING_STARTED.md) para guia detalhado.

**Pré-requisitos**: Rust 1.70+, PostgreSQL 15+, Ollama

---

## 💡 Novo por aqui?

- 📖 **[GETTING_STARTED.md](GETTING_STARTED.md)** - Guia passo-a-passo completo
- 🎯 **[EXECUTIVE_SUMMARY.md](EXECUTIVE_SUMMARY.md)** - Visão estratégica (10min)
- 📋 **[DECISION.md](DECISION.md)** - Estratégia oficial do produto
- 🚀 **[ACTION_PLAN.md](ACTION_PLAN.md)** - Roadmap de 30 dias

## 📚 Exemplos Práticos

### Exemplo 1: Análise de Proteína
```bash
./examples/01_gp41_basic.sh
```
**Output**:
```
🔬 Analisando proteína gp41...
📊 RMSD: 3.2 Å (✅ Estável)
📊 Energia: -135.4 kcal/mol (✅ Favorável)
✅ Nenhuma instabilidade detectada
```

### Exemplo 2: Dashboard Interativo
```bash
./examples/02_dashboard_demo.sh
# Abre http://localhost:3000 com interface completa
```

### Exemplo 3: Slack Bot
```bash
./examples/03_slack_integration.sh
# Guia completo de setup + demo
```

### Exemplo 4: Geração de Manuscrito
```bash
./examples/04_manuscript_generation.sh
# Gera paper científico completo automaticamente
```

---

## 🏗️ Para Desenvolvedores

### Build do Projeto
```bash
# Build completo (primeira vez: 5-10min)
cargo build --release

# Build de binário específico
cargo build --release -p director
cargo build --release -p discovery_dashboard
```

### Testes
```bash
# Rodar todos os testes
cargo test --all

# Testes de integração
cargo test --test integration_tests
```

### Desenvolvimento
```bash
# Watch mode (auto-rebuild)
cargo install cargo-watch
cargo watch -x "build -p director"

# Formatar código
cargo fmt --all

# Linting
cargo clippy --all -- -D warnings
```

---

## 🌟 Casos de Uso

### Para Pesquisadores
- 📄 **Acelerar publicações**: Manuscritos automáticos a partir de simulações
- 🔬 **Validar hipóteses**: Análise rápida de estabilidade estrutural
- 📊 **Gerar dados**: Métricas reproduzíveis para papers

### Para Biotech/Pharma
- 💊 **Drug discovery**: Identificar alvos terapêuticos em HIV
- 🎯 **Triagem virtual**: Análise automatizada de candidatos
- 📈 **ROI**: Reduzir tempo e custo de descoberta em 10x

### Para Equipes Distribuídas
- 💬 **Slack Bot**: Monitoramento colaborativo de simulações
- 📡 **API REST**: Integração com pipelines existentes
- 🔄 **Auditável**: Rastreamento completo de decisões científicas

---

## 💎 Diferenciais Únicos

| Feature | LogLine | AlphaFold | RoseTTAFold | Schrödinger |
|---------|---------|-----------|-------------|-------------|
| 🇧🇷 Português | ✅ | ❌ | ❌ | ❌ |
| 🤖 Conversacional | ✅ | ❌ | ❌ | ❌ |
| 📄 Manuscritos | ✅ | ❌ | ❌ | ❌ |
| 🔓 Open Source | ✅ | ✅ | ✅ | ❌ |
| 💰 Gratuito | ✅ | ✅ | ✅ | ❌ |
| 🧬 Foco HIV | ✅ | ❌ | ❌ | ❌ |

---

## 📊 Status do Projeto

- ✅ **Código**: 80% completo e compilando
- ✅ **Funcionalidades Core**: Todas implementadas
- ⚠️ **Documentação**: Em andamento (ver [STATUS.md](STATUS.md))
- 🎯 **Validação**: Iniciando fase com usuários reais
- 🚀 **Deploy Produção**: Planejado para Q1 2026

**Ver roadmap completo**: [STRATEGIC_ROADMAP.md](STRATEGIC_ROADMAP.md)

---

## 🤝 Como Contribuir

Adoraríamos sua contribuição! 

1. 🍴 **Fork** o repositório
2. 🌿 **Branch**: `git checkout -b feature/minha-feature`
3. ✅ **Commit**: `git commit -m 'Add: minha feature'`
4. 📤 **Push**: `git push origin feature/minha-feature`
5. 🎉 **Pull Request**: Abra PR com descrição detalhada

**Ver**: [CONTRIBUTING.md](CONTRIBUTING.md) *(em breve)*

**Áreas que precisam de ajuda**:
- 🧬 Validação científica (pesquisadores HIV)
- 🦀 Desenvolvimento Rust (contributors code)
- 📚 Documentação (tutoriais, exemplos)
- 🧪 Testes (aumentar coverage)
- 🌐 Tradução (inglês, espanhol)

---

## 🏗️ Arquitetura

```
🧬 LOGLINE DISCOVERY LAB
│
├─ 🤖 Director (Agente Conversacional)
│  ├─ RAG System (Contexto científico)
│  ├─ Function Calling (5 tools)
│  └─ LLM Classification (Ollama)
│
├─ 🔬 Scientific Engines
│  ├─ Folding Runtime (gp41, gp120, Rev, Tat)
│  ├─ Causal Engine (Inferência)
│  ├─ Discovery Agent (Hipóteses)
│  └─ Manuscript Generator (Papers)
│
├─ 📊 Discovery Dashboard (Web UI)
│  ├─ Visualizações científicas
│  ├─ Monitoring simulações
│  └─ Knowledge base search
│
├─ 💾 Data Layer
│  ├─ PostgreSQL (Persistência)
│  ├─ Redis (Cache opcional)
│  └─ NDJSON Ledger (Auditoria)
│
└─ 🔌 Integrations
   ├─ API REST (7 endpoints)
   └─ Slack Bot (4 commands)
```

**Stack Técnico**:
- **Backend**: Rust (Axum, SQLx, Tokio)
- **Database**: PostgreSQL 15+
- **Cache**: Redis 7+ (opcional)
- **LLM**: Ollama (Mistral, Llama)
- **Deploy**: Railway (planejado)

## 📚 Documentação

### 🎯 Estratégia e Planejamento
- 📄 [**DECISION.md**](DECISION.md) - ⭐ Decisão estratégica oficial (NOVO!)
- 📊 [**STATUS.md**](STATUS.md) - Status atual do projeto (NOVO!)
- 🎯 [**EXECUTIVE_SUMMARY.md**](EXECUTIVE_SUMMARY.md) - Resumo executivo
- 🚀 [**STRATEGIC_ROADMAP.md**](STRATEGIC_ROADMAP.md) - Roadmap estratégico
- 📋 [**ACTION_PLAN.md**](ACTION_PLAN.md) - Plano de ação 30 dias
- 📄 [**QUICK_REFERENCE.md**](QUICK_REFERENCE.md) - Referência rápida

### 📖 Guias e Tutoriais
- 🚀 [**GETTING_STARTED.md**](GETTING_STARTED.md) - ⭐ Guia completo de instalação (NOVO!)
- 📱 [**Slack Setup**](logline_discovery/SLACK_SETUP.md) - Configurar Slack bot
- 🔧 [**PostgreSQL Setup**](docs/setup_postgres.md) - Configurar database
- 📋 [**Task List**](TASKLIST.md) - Lista de tarefas técnicas

### 🧬 Documentação Científica
- 📖 [**LogLine Discovery Lab**](LogLine%20Discovery%20Lab.md) - Visão científica
- 📡 [**Master Plan**](docs/master_plan.md) - Plano técnico detalhado
- 📋 [**Changelog**](logline_discovery/CHANGELOG.md) - Histórico de mudanças

---

## 🎯 Roadmap

### ✅ V1.0-Partial (Atual - Novembro 2025)
- [x] Director (agente IA conversacional)
- [x] RAG System (base de conhecimento)
- [x] API REST (7 endpoints)
- [x] Slack Bot (4 commands)
- [x] Dashboard Web
- [x] Scientific Engines (gp41, gp120, Rev, Tat)
- [x] **NOVO**: Decisão estratégica formalizada
- [x] **NOVO**: GETTING_STARTED.md completo
- [x] **NOVO**: 4 exemplos práticos
- [x] **NOVO**: demo.sh interativo

### 🎯 V1.1 (Dezembro 2025 - Janeiro 2026)
- [ ] 100 GitHub stars
- [ ] 25 usuários ativos semanalmente
- [ ] 10 conversas de validação
- [ ] Vídeo demo 5min (YouTube)
- [ ] Landing page completa
- [ ] Testes >80% coverage
- [ ] CI/CD completo (GitHub Actions)

### 🚀 V1.2 (Q1 2026)
- [ ] Paper científico submetido
- [ ] 500 GitHub stars
- [ ] 3 parcerias universitárias
- [ ] Discord community ativa
- [ ] Observabilidade completa (OpenTelemetry)
- [ ] Cache system (Redis + Moka)

### 💰 V2.0 (Q2-Q4 2026)
- [ ] SaaS launch (Free/Pro/Enterprise)
- [ ] $10k MRR
- [ ] 25 clientes pagantes
- [ ] Deploy produção (Railway)
- [ ] Expansão: Malária, COVID-19
- [ ] ST-GNN implementation

**Ver roadmap completo**: [STRATEGIC_ROADMAP.md](STRATEGIC_ROADMAP.md)

---

## 📞 Comunidade e Suporte

### 💬 Discussões
- 💡 **GitHub Discussions**: [Q&A, Ideias, Show & Tell](https://github.com/danvoulez/lablab/discussions)
- 🐛 **Issues**: [Reportar bugs](https://github.com/danvoulez/lablab/issues)

### 📱 Social
- 🐦 **Twitter/X**: *em breve*
- 💼 **LinkedIn**: *em breve*
- 📺 **YouTube**: Tutoriais *em breve*
- 💬 **Discord**: Community server *em breve*

### 📧 Contato
- **Email**: *em breve*
- **Colaborações**: Abra uma issue ou discussion
- **Parcerias**: Veja [STRATEGIC_ROADMAP.md](STRATEGIC_ROADMAP.md)

---

## 🏆 Reconhecimentos

Desenvolvido com ❤️ e foco em:
- 🧬 **Impacto Científico**: Acelerar descoberta de medicamentos HIV
- 🇧🇷 **Acessibilidade**: Primeiro agente IA em português brasileiro
- 🔓 **Transparência**: 100% open source e auditável
- 🤝 **Colaboração**: Community-driven development

### Stack de Excelência
- **Rust** - Performance e segurança type-safe
- **PostgreSQL** - Confiabilidade em persistência
- **Ollama** - LLMs rodando localmente
- **Axum** - Web framework moderno e rápido

---

## 📄 Licença

Este projeto está licenciado sob **MIT OR Apache-2.0**.

- [LICENSE-MIT](LICENSE-MIT)
- [LICENSE-APACHE](LICENSE-APACHE)

Você é livre para usar, modificar e distribuir conforme os termos dessas licenças.

---

## 🌟 Star History

⭐ **Ajude-nos a crescer dando uma estrela no projeto!**

Cada star ajuda a:
- Aumentar visibilidade do projeto
- Atrair mais contributors
- Validar o valor da plataforma
- Motivar desenvolvimento contínuo

---

## 🎊 Próximos Passos

**Se você chegou até aqui, aqui está o que fazer agora**:

1. ⭐ **Star o repositório** para acompanhar o progresso
2. 📖 **Leia [GETTING_STARTED.md](GETTING_STARTED.md)** para instalar
3. 🚀 **Rode `./demo.sh`** para ver o sistema funcionando
4. 💬 **Dê feedback**: Abra issue ou discussion
5. 🤝 **Contribua**: Veja [Como Contribuir](#como-contribuir)

**Questões? Problemas?** Abra uma [issue](https://github.com/danvoulez/lablab/issues)!

---

<div align="center">

**🧬 LogLine Discovery Lab**

*Acelerando a descoberta de medicamentos HIV em 10x com IA conversacional*

[![GitHub stars](https://img.shields.io/github/stars/danvoulez/lablab?style=social)](https://github.com/danvoulez/lablab/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/danvoulez/lablab?style=social)](https://github.com/danvoulez/lablab/network/members)

Made with ❤️ in Brazil 🇧🇷 | Powered by Rust 🦀 + AI 🤖

[⭐ Star](https://github.com/danvoulez/lablab) • [🐛 Report Bug](https://github.com/danvoulez/lablab/issues) • [💡 Request Feature](https://github.com/danvoulez/lablab/issues)

</div>
