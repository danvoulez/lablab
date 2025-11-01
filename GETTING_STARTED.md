# 🚀 GETTING STARTED - LogLine Discovery Lab

**Guia completo para começar a usar o LogLine Discovery Lab em minutos**

---

## 📋 Pré-requisitos

### Sistema Operacional
- **macOS** (recomendado para desenvolvimento)
- **Linux** (Ubuntu 20.04+ ou similar)
- **Windows** (via WSL2)

### Ferramentas Necessárias

#### 1. Rust (versão 1.70+)
```bash
# Instalar Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verificar instalação
rustc --version
cargo --version
```

#### 2. PostgreSQL (versão 15+)
```bash
# macOS
brew install postgresql@15
brew services start postgresql@15

# Ubuntu/Debian
sudo apt update
sudo apt install postgresql-15 postgresql-contrib
sudo systemctl start postgresql
```

#### 3. Redis (versão 7+)
```bash
# macOS
brew install redis
brew services start redis

# Ubuntu/Debian
sudo apt install redis-server
sudo systemctl start redis
```

#### 4. Ollama (para LLM local)
```bash
# macOS/Linux
curl https://ollama.ai/install.sh | sh

# Baixar modelo necessário
ollama pull mistral:instruct
```

---

## 📥 Instalação

### Passo 1: Clone o Repositório
```bash
git clone https://github.com/danvoulez/lablab.git
cd lablab
```

### Passo 2: Configure o Banco de Dados
```bash
# Criar database PostgreSQL
createdb logline_discovery

# Criar usuário (opcional)
psql -d postgres -c "CREATE USER logline WITH PASSWORD 'your_password';"
psql -d postgres -c "GRANT ALL PRIVILEGES ON DATABASE logline_discovery TO logline;"
```

### Passo 3: Configure Variáveis de Ambiente
```bash
# Copiar arquivo de exemplo
cp .env.example .env

# Editar .env com suas configurações
nano .env
```

**Exemplo de `.env`**:
```env
# Database
DATABASE_URL=postgresql://logline:your_password@localhost/logline_discovery

# Redis (opcional)
REDIS_URL=redis://localhost:6379

# Ollama
OLLAMA_URL=http://localhost:11434
OLLAMA_MODEL=mistral:instruct

# API Configuration
API_PORT=8080
API_HOST=0.0.0.0

# Slack (opcional)
SLACK_BOT_TOKEN=xoxb-your-token-here
SLACK_SIGNING_SECRET=your-signing-secret

# Log Level
RUST_LOG=info,logline=debug
```

### Passo 4: Inicialize o Banco de Dados
```bash
# Rodar migrations (se existirem)
cd crates/spans_core
sqlx database create
sqlx migrate run
cd ../..

# Ou usar script de setup
./scripts/setup_postgres.sh
```

### Passo 5: Build do Projeto
```bash
# Build de todos os binários (primeira vez pode demorar 5-10min)
cargo build --release

# Ou build de binário específico
cargo build --release -p director
```

### Passo 6: Popular Base de Conhecimento
```bash
# Importar conhecimento HIV inicial
./scripts/import_hiv_knowledge.sh

# Ou manualmente via director CLI
./target/release/director --mode cli
> add_knowledge path/to/hiv_knowledge.json
```

---

## 🎯 Primeiro Uso

### Opção 1: Interface CLI (Mais Simples)

```bash
# Iniciar director em modo CLI
./target/release/director --mode cli

# Exemplos de comandos:
> Como funciona a proteína gp41 no HIV?
> Analise a estabilidade da gp120
> Qual o status dos reservatórios latentes?
> Gerar manuscrito sobre gp41
```

**Output esperado**:
```
🧬 LogLine Director v1.0.0
Modo: CLI Interativo
Modelo: mistral:instruct

> Como funciona a proteína gp41 no HIV?

[RAG] Buscando conhecimento relevante...
[Encontrado] 3 documentos relevantes

🤖 Resposta:
A proteína gp41 é uma proteína de fusão crucial do HIV...
[detalhes científicos]

Fontes:
- HIV-1 Envelope Protein Structure (PubMed:12345)
- gp41 Fusion Mechanism (Nature, 2023)
```

---

### Opção 2: API REST

```bash
# Iniciar servidor API (terminal 1)
./target/release/director --mode api --port 8080

# Em outro terminal (terminal 2), fazer requests
curl -X POST http://localhost:8080/rag/query \
  -H "Content-Type: application/json" \
  -d '{
    "query": "Como funciona a proteína gp41?",
    "max_results": 5
  }'
```

**Response esperado**:
```json
{
  "success": true,
  "answer": "A proteína gp41 é uma proteína de fusão...",
  "sources": [
    {
      "id": "hiv_gp41_structure_2023",
      "title": "HIV-1 gp41 Structure",
      "relevance": 0.95
    }
  ],
  "processing_time_ms": 234
}
```

---

### Opção 3: Dashboard Web

```bash
# Iniciar dashboard (terminal 1)
./target/release/discovery_dashboard --port 3000

# Abrir navegador
open http://localhost:3000
```

**O que você verá**:
- 📊 Dashboard de simulações HIV
- 🧬 Status de análise de proteínas
- 📈 Gráficos de métricas científicas (RMSD, energia)
- 🔍 Interface de busca na base de conhecimento

---

### Opção 4: Slack Bot

**Pré-requisito**: Configurar Slack App (ver [Slack Setup Guide](docs/slack_setup.md))

```bash
# Iniciar Slack bot
./target/release/director --mode slack

# No Slack, usar comandos:
/director Como funciona a proteína gp41?
/hiv-status
/knowledge-search fusion mechanisms
/analyze-protein gp41
```

---

## 🧬 Exemplos Práticos

### Exemplo 1: Análise Básica de gp41

```bash
# Via CLI
./target/release/director --mode cli

> Analisar proteína gp41
```

**O que acontece**:
1. Director classifica a query como "folding analysis"
2. Chama function `analyze_protein_folding`
3. Executa simulação molecular
4. Calcula RMSD e energia
5. Retorna análise com métricas

**Output**:
```
🔬 Analisando proteína gp41...

📊 Resultados:
- RMSD médio: 3.2 Å (✅ Estável - threshold: 5.0 Å)
- Energia: -135.4 kcal/mol (✅ Favorável - threshold: -120)
- Instabilidade: Não detectada
- Simulação: 100 frames, 10ns

💡 Conclusão:
A proteína gp41 apresenta alta estabilidade estrutural
nas condições simuladas.
```

---

### Exemplo 2: Busca na Base de Conhecimento

```bash
./target/release/director --mode cli

> Buscar conhecimento sobre "mecanismos de fusão HIV"
```

**Output**:
```
📚 Buscando: "mecanismos de fusão HIV"

Encontrados 5 documentos relevantes:

1. [0.95] HIV-1 Fusion Mechanism via gp41 (2023)
   → gp41 medeia fusão de membranas através de...

2. [0.89] Structural Basis of gp120-gp41 Complex (2022)
   → Conformational changes in gp41 triggered by...

3. [0.87] Inhibitors of HIV Fusion (2021)
   → Targeting gp41 fusion intermediate state...

Digite o número para ler completo, ou 'q' para voltar.
```

---

### Exemplo 3: Geração de Manuscrito

```bash
./target/release/director --mode cli

> Gerar manuscrito sobre análise de gp41
```

**Output**:
```
📝 Gerando manuscrito científico...

✅ Manuscrito gerado: manuscripts/gp41_analysis_2025_11_01.md

Conteúdo:
- Título: "Structural Analysis of HIV-1 gp41 Fusion Protein"
- Abstract: 250 palavras
- Introduction: Contexto e motivação
- Methods: Metodologia de simulação
- Results: Métricas e análises
- Discussion: Interpretação científica
- References: 15 citações automáticas

Próximo passo: Revisar e editar manualmente
```

---

### Exemplo 4: Monitoramento de Simulação

```bash
./target/release/director --mode cli

> Monitorar simulação gp120
```

**Output**:
```
📡 Monitorando simulação HIV: gp120

Status atual:
- Proteína: gp120 (HIV-1 surface protein)
- Frame: 75/100 (75% completo)
- Tempo simulado: 7.5 ns
- RMSD atual: 4.1 Å
- Energia atual: -128.3 kcal/mol

Flags:
- ✅ Estabilidade mantida
- ⚠️ Flutuação energética detectada (frame 65-70)

Atualização em tempo real a cada 5 segundos...
Pressione Ctrl+C para parar.
```

---

## 🛠️ Troubleshooting

### Problema 1: "Failed to connect to PostgreSQL"

**Sintoma**:
```
Error: Failed to connect to database
```

**Solução**:
```bash
# Verificar se PostgreSQL está rodando
pg_isready

# Se não estiver, iniciar
brew services start postgresql@15  # macOS
sudo systemctl start postgresql    # Linux

# Verificar DATABASE_URL no .env
echo $DATABASE_URL

# Criar database se não existir
createdb logline_discovery
```

---

### Problema 2: "Ollama model not found"

**Sintoma**:
```
Error: Model 'mistral:instruct' not found
```

**Solução**:
```bash
# Verificar Ollama está rodando
ollama list

# Baixar modelo se necessário
ollama pull mistral:instruct

# Verificar OLLAMA_URL no .env
echo $OLLAMA_URL  # deve ser http://localhost:11434
```

---

### Problema 3: "Redis connection refused"

**Sintoma**:
```
Warning: Failed to connect to Redis (non-critical)
```

**Solução**:
```bash
# Verificar se Redis está rodando
redis-cli ping  # deve retornar "PONG"

# Se não estiver, iniciar
brew services start redis  # macOS
sudo systemctl start redis # Linux

# Redis é OPCIONAL - sistema funciona sem ele (apenas mais lento)
```

---

### Problema 4: "Build failed - linking error"

**Sintoma**:
```
error: linking with `cc` failed
```

**Solução**:
```bash
# Instalar dependências de build
# macOS
xcode-select --install
brew install openssl@3

# Ubuntu/Debian
sudo apt install build-essential pkg-config libssl-dev

# Limpar e rebuildar
cargo clean
cargo build --release
```

---

### Problema 5: "Out of memory during build"

**Sintoma**:
```
error: could not compile ... (signal: 9, SIGKILL)
```

**Solução**:
```bash
# Build com menos paralelismo
cargo build --release -j 2  # usa apenas 2 cores

# Ou build incremental (um binário por vez)
cargo build --release -p director
cargo build --release -p discovery_dashboard
# etc...
```

---

## 📚 Próximos Passos

Após instalação bem-sucedida:

### 1. Explorar Exemplos
```bash
# Navegar para exemplos
cd examples/

# Rodar demo básico
./01_gp41_basic.sh

# Dashboard demo
./02_dashboard_demo.sh

# Slack integration demo
./03_slack_integration.sh
```

### 2. Ler Documentação Avançada
- [API Documentation](docs/api_docs.md) - Referência completa da API
- [Scientific Background](docs/hiv_background.md) - Contexto científico HIV
- [Architecture](docs/architecture.md) - Arquitetura do sistema
- [Contributing](CONTRIBUTING.md) - Como contribuir

### 3. Explorar Function Calling
```bash
# Listar todas as funções disponíveis
curl http://localhost:8080/functions | jq

# Testar cada função individualmente
curl -X POST http://localhost:8080/function_call \
  -H "Content-Type: application/json" \
  -d '{
    "function_name": "analyze_protein_folding",
    "parameters": {
      "protein_name": "gp41"
    }
  }'
```

### 4. Configurar Ambiente de Desenvolvimento
```bash
# Instalar ferramentas de desenvolvimento
cargo install cargo-watch  # auto-rebuild
cargo install cargo-edit   # gerenciar dependências
cargo install cargo-nextest # testes mais rápidos

# Setup pre-commit hooks
cp scripts/pre-commit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit

# Configurar editor (VS Code recomendado)
# - Extensão: rust-analyzer
# - Extensão: Even Better TOML
# - Extensão: CodeLLDB (debugging)
```

### 5. Rodar Testes
```bash
# Rodar todos os testes
cargo test --all

# Rodar testes de um crate específico
cargo test -p director

# Rodar com output detalhado
cargo test -- --nocapture

# Rodar testes de integração
cargo test --test integration_tests
```

---

## 🎓 Recursos de Aprendizado

### Documentação Interna
- [EXECUTIVE_SUMMARY.md](EXECUTIVE_SUMMARY.md) - Visão geral do projeto
- [STRATEGIC_ROADMAP.md](STRATEGIC_ROADMAP.md) - Estratégia de produto
- [ACTION_PLAN.md](ACTION_PLAN.md) - Plano de ação 30 dias
- [DECISION.md](DECISION.md) - Decisão estratégica oficial

### Tutoriais em Vídeo
- 📹 [Getting Started - 10min](https://youtube.com/...) *(a criar)*
- 📹 [Deep Dive gp41 Analysis - 15min](https://youtube.com/...) *(a criar)*
- 📹 [Slack Integration Setup - 8min](https://youtube.com/...) *(a criar)*

### Comunidade
- 💬 Discord: [LogLine Community](https://discord.gg/...) *(a criar)*
- 🐛 Issues: [GitHub Issues](https://github.com/danvoulez/lablab/issues)
- 💡 Discussions: [GitHub Discussions](https://github.com/danvoulez/lablab/discussions)

---

## ❓ FAQ

### P: Preciso de GPU para rodar?
**R**: Não. As simulações básicas rodam em CPU. GPU acelera mas não é necessária.

### P: Funciona em Windows?
**R**: Sim, via WSL2 (Windows Subsystem for Linux). Windows nativo não é oficialmente suportado.

### P: Posso usar outros LLMs além do Mistral?
**R**: Sim! Ollama suporta vários modelos. Edite `OLLAMA_MODEL` no `.env`.

### P: Quanto de RAM preciso?
**R**: Mínimo 8GB. Recomendado 16GB para simulações maiores.

### P: Os resultados científicos são validados?
**R**: Sim. Comparamos com literatura peer-reviewed. Ver [Scientific Validation](docs/validation.md).

### P: Posso contribuir?
**R**: Absolutamente! Ver [CONTRIBUTING.md](CONTRIBUTING.md) para guidelines.

### P: É gratuito?
**R**: Sim, 100% open source (MIT/Apache-2.0). Planos pagos futuros para managed hosting.

---

## 🆘 Precisa de Ajuda?

### Não conseguiu instalar?
1. Verifique [Troubleshooting](#troubleshooting) acima
2. Busque em [GitHub Issues](https://github.com/danvoulez/lablab/issues)
3. Crie nova issue com:
   - Sistema operacional
   - Versões das ferramentas (`rustc --version`, etc)
   - Mensagem de erro completa
   - Passos que você já tentou

### Dúvidas sobre uso?
1. Consulte [documentação](docs/)
2. Pergunte no Discord (quando disponível)
3. Abra GitHub Discussion

### Encontrou um bug?
1. Verifique se já foi reportado
2. Crie issue com reprodução mínima
3. Inclua logs relevantes

---

## ✅ Checklist de Verificação

Confirme que tudo está funcionando:

- [ ] ✅ Rust instalado (`rustc --version` retorna 1.70+)
- [ ] ✅ PostgreSQL rodando (`pg_isready` retorna success)
- [ ] ✅ Redis rodando (`redis-cli ping` retorna PONG) *opcional*
- [ ] ✅ Ollama com modelo (`ollama list` mostra mistral:instruct)
- [ ] ✅ Database criado (`psql logline_discovery` conecta)
- [ ] ✅ Projeto compila (`cargo build --release` sem erros)
- [ ] ✅ Director CLI funciona (responde perguntas)
- [ ] ✅ API responde (`curl http://localhost:8080/health`)
- [ ] ✅ Dashboard abre no navegador (http://localhost:3000)

**Se todos ✅, você está pronto! 🎉**

---

## 🚀 Comece Agora!

```bash
# 1. Clone
git clone https://github.com/danvoulez/lablab.git && cd lablab

# 2. Setup
./scripts/quick_setup.sh

# 3. Run
./target/release/director --mode cli

# 4. Divirta-se explorando! 🧬
```

**Bem-vindo ao LogLine Discovery Lab!** 🤖❤️

---

*Documento atualizado: Novembro 2025*  
*Versão: 1.0*  
*Feedback: abra issue ou discussion*
