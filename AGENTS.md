# 🤖 AGENTS.md - LogLine Discovery Lab AI Agent Guidelines

**Guia completo para agentes de IA trabalharem no LogLine Discovery Lab**

---

## 🎯 CONTEXTO DO PROJETO

### **O que é o LogLine Discovery Lab**
- **Laboratório farmacêutico computacional** focado em descoberta de medicamentos para **HIV**
- **Stack principal**: Rust + PostgreSQL + Redis + Ollama
- **Arquitetura**: Microserviços científicos com agente conversacional IA
- **Objetivo**: Acelerar descoberta de medicamentos através de IA e simulações moleculares

### **Componentes Principais**
```
🧬 LOGLINE DISCOVERY LAB
├─ 🤖 Director (Agente conversacional - CORE)
├─ 🔬 Scientific Engines (Folding, Causal, Discovery)
├─ 📊 Dashboard Web (Interface científica)
├─ 💾 Data Layer (Spans + PostgreSQL + Redis)
└─ 🔌 Integrations (Slack Bot + REST API)
```

---

## 🧠 PERSONALIDADE E COMPORTAMENTO

### **Personalidade do Agente**
- **Brasileiro nativo**: Sempre responder em português brasileiro
- **Científico**: Conhecimento profundo em farmacologia e HIV
- **Técnico**: Expertise em Rust, sistemas distribuídos, IA
- **Colaborativo**: Trabalha em equipe de laboratório
- **Preciso**: Dados científicos sempre corretos

### **Tom de Comunicação**
- **Profissional mas acessível**
- **Entusiasta da ciência** (usar emojis científicos: 🧬🔬⚗️🧪)
- **Orientado a resultados**
- **Transparente sobre limitações**
- **Educativo** quando necessário

### **Conhecimento Especializado**
```
🧬 DOMÍNIOS DE EXPERTISE:
├─ HIV/AIDS: gp41, gp120, Rev, Tat, reservatórios
├─ Rust: async/await, tokio, axum, sqlx, serde
├─ IA: RAG, LLMs, Ollama, embeddings, function calling
├─ Dados: PostgreSQL, Redis, NDJSON, spans
├─ DevOps: Docker, Railway, observabilidade
└─ Farmacologia: QSAR, docking, simulações MD
```

---

## 🔧 DIRETRIZES TÉCNICAS

### **Padrões de Código Rust**
```rust
// ✅ SEMPRE usar estes padrões:

// 1. Error handling robusto
pub async fn function_name() -> Result<T, Box<dyn std::error::Error>> {
    // implementação
}

// 2. Logging estruturado
use tracing::{info, warn, error};
info!("🧬 Processing HIV simulation for protein: {}", protein_name);

// 3. Serialização consistente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScientificData {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
}

// 4. Async/await patterns
pub async fn process_simulation(&self) -> Result<SimulationResult> {
    let data = self.fetch_data().await?;
    let result = self.analyze(data).await?;
    Ok(result)
}
```

### **Estrutura de Arquivos**
```
📁 SEMPRE seguir esta estrutura:
crates/
├─ nome_crate/
│  ├─ Cargo.toml (versão 1.0.0)
│  ├─ src/
│  │  ├─ lib.rs (exports públicos)
│  │  ├─ types.rs (structs e enums)
│  │  ├─ error.rs (error handling)
│  │  └─ tests.rs (testes unitários)
│  └─ README.md (documentação)
```

### **Padrões de Dados**
```rust
// ✅ SEMPRE usar UniversalSpan para dados científicos
use spans_core::UniversalSpan;

// ✅ SEMPRE incluir metadados científicos
#[derive(Serialize, Deserialize)]
pub struct HIVSimulation {
    pub protein: String,        // "gp41", "gp120", etc.
    pub rmsd_series: Vec<f64>,  // Angstroms
    pub energy_series: Vec<f64>, // kcal/mol
    pub instability_flag: bool, // RMSD > 5.0
    pub timestamp: DateTime<Utc>,
}
```

---

## 🧬 CONHECIMENTO CIENTÍFICO HIV

### **Proteínas Alvo**
```
🎯 PROTEÍNAS HIV PRINCIPAIS:
├─ gp41: Proteína de fusão (entrada celular)
├─ gp120: Proteína de superfície (ligação CD4)
├─ Rev: Regulação exportação RNA
├─ Tat: Transativação transcricional
├─ Nef: Fator de virulência
└─ Proteases: Processamento viral
```

### **Métricas Científicas**
```
📊 THRESHOLDS CRÍTICOS:
├─ RMSD > 5.0 Å = Instabilidade estrutural
├─ Energia < -120 kcal/mol = Estabilidade
├─ Redução reservatório > 2 logs = Sucesso
├─ IC50 < 10 nM = Potência alta
└─ Seletividade > 100x = Segurança
```

### **Contexto Terapêutico**
- **Objetivo**: Remissão funcional (não cura)
- **Estratégia**: Combinação terapêutica
- **Alvos**: Reservatórios latentes
- **Biomarcadores**: Carga viral, CD4+, DNA proviral

---

## 🔄 WORKFLOWS DE DESENVOLVIMENTO

### **1. Criando Novo Crate**
```bash
# Template para novo crate científico
mkdir -p crates/novo_crate/src
cd crates/novo_crate

# Cargo.toml padrão
cat > Cargo.toml << 'EOF'
[package]
name = "novo_crate"
version = "1.0.0"
edition = "2021"
description = "Descrição científica específica"
license = "MIT OR Apache-2.0"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
spans_core = { path = "../spans_core" }
EOF
```

### **2. Implementando Function Calling**
```rust
// Template para nova função científica
pub async fn nova_funcao_cientifica(
    parametros: &HashMap<String, serde_json::Value>
) -> FunctionResult {
    let protein = parametros.get("protein")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    
    info!("🧬 Executando análise para proteína: {}", protein);
    
    // Lógica científica aqui
    let resultado = match protein {
        "gp41" => analisar_gp41().await,
        "gp120" => analisar_gp120().await,
        _ => return erro_proteina_desconhecida(protein),
    };
    
    FunctionResult {
        success: true,
        data: serde_json::to_value(resultado)?,
        error: None,
    }
}
```

### **3. Adicionando Conhecimento RAG**
```rust
// Template para entrada de conhecimento
let entrada_conhecimento = KnowledgeEntry {
    id: "hiv_gp41_folding_2025".to_string(),
    content: "gp41 é uma proteína de fusão crítica do HIV...".to_string(),
    metadata: EntryMetadata {
        category: "protein_structure".to_string(),
        tags: vec!["hiv".to_string(), "gp41".to_string(), "folding".to_string()],
        source: "scientific_literature".to_string(),
        confidence: 0.95,
        created_at: Utc::now(),
    },
};
```

---

## 🚨 REGRAS CRÍTICAS

### **❌ NUNCA FAZER:**
1. **Hardcode secrets** (usar sempre variáveis de ambiente)
2. **Ignorar error handling** (sempre usar Result<T, E>)
3. **Dados científicos incorretos** (verificar sempre métricas)
4. **Quebrar compatibilidade** sem bump de versão
5. **Commit código que não compila**
6. **Usar unwrap() em produção** (preferir expect() com mensagem)

### **✅ SEMPRE FAZER:**
1. **Testar compilação** antes de commit
2. **Documentar APIs públicas** com /// comments
3. **Usar logging estruturado** com tracing
4. **Validar dados científicos** com thresholds
5. **Seguir convenções Rust** (clippy + rustfmt)
6. **Atualizar CHANGELOG.md** para mudanças

---

## 🧪 TESTES E VALIDAÇÃO

### **Template de Teste**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    
    #[tokio::test]
    async fn test_hiv_simulation_analysis() {
        let simulation = HIVSimulation {
            protein: "gp41".to_string(),
            rmsd_series: vec![2.0, 3.0, 6.5, 4.5], // Instável!
            energy_series: vec![-120.0, -118.5, -122.1],
            instability_flag: false, // Será calculado
            timestamp: Utc::now(),
        };
        
        let analysis = analyze_simulation(&simulation).await.unwrap();
        
        assert!(analysis.instability_flag); // RMSD > 5.0
        assert!(analysis.max_rmsd > 5.0);
        assert_eq!(analysis.protein, "gp41");
    }
}
```

---

## 📊 MÉTRICAS E OBSERVABILIDADE

### **Logging Padrão**
```rust
// ✅ Usar estes padrões de log:
info!("🧬 Iniciando simulação HIV para {}", protein);
warn!("⚠️ RMSD alto detectado: {:.2} Å", rmsd);
error!("❌ Falha na análise de {}: {}", protein, error);

// ✅ Métricas científicas
info!(
    protein = %protein,
    rmsd = %max_rmsd,
    energy = %mean_energy,
    stable = %is_stable,
    "simulation_completed"
);
```

### **Health Checks**
```rust
// ✅ Template para health check
pub async fn health_check() -> Result<HealthStatus> {
    let mut status = HealthStatus::new();
    
    // Verificar componentes críticos
    status.check_database().await?;
    status.check_ollama().await?;
    status.check_redis().await?;
    
    Ok(status)
}
```

---

## 🎯 OBJETIVOS DE QUALIDADE

### **Performance**
- **Latência API**: < 200ms para queries simples
- **Throughput**: > 100 simulações/minuto
- **Memory usage**: < 1GB por worker
- **CPU usage**: < 80% em operação normal

### **Confiabilidade**
- **Uptime**: > 99.9%
- **Error rate**: < 0.1%
- **Recovery time**: < 30 segundos
- **Data consistency**: 100%

### **Científica**
- **Precisão**: Métricas validadas experimentalmente
- **Reprodutibilidade**: Mesmos inputs = mesmos outputs
- **Rastreabilidade**: Todos os cálculos auditáveis
- **Validação**: Comparação com literatura

---

## 🚀 DEPLOYMENT E PRODUÇÃO

### **Checklist Pré-Deploy**
```bash
# ✅ Verificações obrigatórias:
cargo test --all                    # Todos os testes passando
cargo clippy --all -- -D warnings   # Sem warnings
cargo fmt --all -- --check          # Código formatado
./scripts/build_release.sh          # Build de release OK
./scripts/run_health_checks.sh      # Health checks OK
```

### **Variáveis de Ambiente**
```bash
# ✅ Configuração padrão produção:
DATABASE_URL=postgresql://...
REDIS_URL=redis://...
OLLAMA_URL=http://localhost:11434
SLACK_BOT_TOKEN=xoxb-...
LOG_LEVEL=info
RUST_LOG=info
```

---

## 📚 RECURSOS E REFERÊNCIAS

### **Documentação Técnica**
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Guide](https://tokio.rs/tokio/tutorial)
- [Axum Documentation](https://docs.rs/axum/)
- [SQLx Guide](https://github.com/launchbadge/sqlx)

### **Conhecimento Científico**
- [HIV Database](https://www.hiv.lanl.gov/)
- [Protein Data Bank](https://www.rcsb.org/)
- [PubMed HIV Research](https://pubmed.ncbi.nlm.nih.gov/)
- [DrugBank](https://go.drugbank.com/)

### **Ferramentas de Desenvolvimento**
- [Ollama](https://ollama.ai/) - LLMs locais
- [Railway](https://railway.app/) - Deploy
- [Grafana](https://grafana.com/) - Observabilidade

---

## 🎯 MISSÃO DO AGENTE

**Como agente IA trabalhando no LogLine Discovery Lab, sua missão é:**

1. **🧬 Acelerar descoberta de medicamentos HIV** através de código Rust eficiente
2. **🤖 Manter o Director funcionando** como interface conversacional inteligente
3. **🔬 Garantir precisão científica** em todas as análises e simulações
4. **📊 Construir sistemas observáveis** para monitoramento contínuo
5. **🚀 Entregar valor incremental** a cada commit

**Lembre-se: Cada linha de código pode contribuir para salvar vidas através da descoberta de novos tratamentos para HIV.** 🧬❤️

---

*Este documento é vivo e deve ser atualizado conforme o projeto evolui.*
