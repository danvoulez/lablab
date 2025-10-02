# 📋 TASKLIST.md - LogLine Discovery Lab Complete Task Management

**Lista completa e granular de todas as tarefas do projeto**

---

## 🎯 STATUS GERAL DO PROJETO

### **✅ COMPLETADO (V1.0-Partial)**
- 🤖 **Director**: Agente conversacional completo
- 📊 **Dashboard**: Interface web científica
- 🧠 **RAG System**: Knowledge base contextual
- 📱 **Slack Bot**: Integração profissional
- 🌐 **API REST**: 7 endpoints funcionais
- 🧬 **HIV Discovery Runner**: Correções de sintaxe e pipeline de processamento saudável

### **🔄 EM PROGRESSO**
- ⚡ **Job System**: Correções de compilação
- 📊 **Observabilidade**: Implementação OpenTelemetry

### **⏳ PLANEJADO**
- ⚡ **Cache System**: Redis + Moka otimizado
- 🌐 **Production Deploy**: Railway + Docker
- 🧬 **Scientific Expansion**: Malária, COVID-19

---

## 🔴 TAREFAS CRÍTICAS (BLOQUEADORES)

### **1. 🧬 HIV Discovery Runner - Correções Sintaxe**
**Prioridade**: 🔥 CRÍTICA | **Estimativa**: 2-3 dias | **Responsável**: Dev Backend

#### **1.1 Corrigir Função handle_process_recent**
- [x] **Arquivo**: `binaries/hiv_discovery_runner/src/main.rs:555-558`
- [x] **Problema**: Sintaxe inválida `{{ ... }}`
- [x] **Solução**: Corrigir assinatura da função
```rust
// ❌ Atual (quebrado):
async fn handle_process_recent(
    since: Option<String>,
    limit: usize,
{{ ... }}

// ✅ Corrigir para:
async fn handle_process_recent(
    since: Option<String>,
    limit: usize,
) -> Result<()> {
    // implementação
}
```

#### **1.2 Corrigir Função is_supported_file**
- [x] **Arquivo**: `binaries/hiv_discovery_runner/src/main.rs:564-569`
- [x] **Problema**: Código solto fora da função
- [x] **Solução**: Mover código para local apropriado
```rust
// ❌ Atual (quebrado):
fn is_supported_file(path: &Path) -> bool {
    // ... código válido ...
    let mut engine = build_demo_engine(); // ❌ Fora da função

// ✅ Corrigir estrutura completa
```

#### **1.3 Verificar Delimitadores**
- [x] **Tarefa**: Verificar todos os `{` `}` estão balanceados
- [x] **Ferramenta**: `cargo check -p hiv_discovery_runner`
- [x] **Critério**: Compilação sem erros

### **2. ⚡ Job Scheduler - Correções Enum**
**Prioridade**: 🔥 CRÍTICA | **Estimativa**: 1 dia | **Responsável**: Dev Backend

#### **2.1 Corrigir JobStatus Enum**
- [x] **Arquivo**: `crates/common/src/job.rs` (presumido)
- [x] **Problema**: `JobStatus::Pending` não existe
- [x] **Solução**: Adicionar variant faltando
```rust
// ✅ Adicionar ao enum:
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "job_status", rename_all = "lowercase")]
pub enum JobStatus {
    Pending,   // ✅ ADICIONAR
    Queued,
    Running,
    Completed,
    Failed,
}
```

#### **2.2 Atualizar Database Schema**
- [x] **Arquivo**: Criar migration SQL
- [x] **Comando**: `ALTER TYPE job_status ADD VALUE 'pending';`
- [x] **Teste**: Verificar compatibilidade

#### **2.3 Atualizar Referências**
- [x] **Buscar**: `JobStatus::Pending` em todo o código
- [x] **Verificar**: Todas as referências estão corretas
- [x] **Testar**: `cargo test -p job_scheduler`

### **3. 👷 Job Worker - Correções Struct**
**Prioridade**: 🔥 CRÍTICA | **Estimativa**: 1 dia | **Responsável**: Dev Backend

#### **3.1 Completar Struct Job**
- [x] **Arquivo**: `crates/common/src/job.rs`
- [x] **Problema**: Campos faltando na struct
- [x] **Solução**: Adicionar todos os campos necessários
```rust
// ✅ Struct completa:
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub payload: serde_json::Value,    // ✅ ADICIONAR
    pub status: JobStatus,             // ✅ ADICIONAR
    pub created_at: DateTime<Utc>,     // ✅ ADICIONAR
    pub started_at: Option<DateTime<Utc>>, // ✅ ADICIONAR
    pub completed_at: Option<DateTime<Utc>>, // ✅ ADICIONAR
    pub worker_id: Option<String>,     // ✅ ADICIONAR
    pub priority: JobPriority,         // ✅ ADICIONAR
    pub retry_count: i32,              // ✅ ADICIONAR
    pub max_retries: i32,              // ✅ ADICIONAR
    pub error_message: Option<String>, // ✅ ADICIONAR
}
```

#### **3.2 Implementar Traits Necessários**
- [x] **Traits**: `Serialize`, `Deserialize`, `sqlx::FromRow`
- [x] **Teste**: Verificar serialização JSON
- [x] **Validação**: Compatibilidade com PostgreSQL

### **4. 📞 Job Client - Correções Dependências**
**Prioridade**: 🔥 CRÍTICA | **Estimativa**: 0.5 dia | **Responsável**: Dev Backend

#### **4.1 Identificar Dependências Faltando**
- [x] **Comando**: `cargo check -p job_client`
- [x] **Analisar**: Erros de importação
- [x] **Listar**: Crates necessários (ajustes aplicados diretamente no cliente para consultas e logs)

#### **4.2 Atualizar Cargo.toml**
- [x] **Arquivo**: `binaries/job_client/Cargo.toml`
- [x] **Adicionar**: Dependências faltando *(nenhuma alteração necessária após verificação)*
- [x] **Verificar**: Versões compatíveis (compilação `cargo check -p job_client` concluída)

---

## 🟡 TAREFAS IMPORTANTES (WARNINGS)

### **5. 🤖 Director - Limpeza de Warnings**
**Prioridade**: 🟡 MÉDIA | **Estimativa**: 1 dia | **Responsável**: Dev Frontend

#### **5.1 Corrigir Variáveis Não Utilizadas**
- [x] **Arquivo**: `binaries/director/src/api.rs:145`
- [x] **Warning**: `unused variable: rag`
- [x] **Solução**: Utilizar o seletor RAG para consultar contagem real do acervo

#### **5.2 Corrigir Imports Desnecessários**
- [x] **Buscar**: `#[warn(unused_imports)]` em todo director
- [x] **Remover**: Imports não utilizados
- [x] **Verificar**: `cargo clippy -p director`

#### **5.3 Corrigir Campos Mortos**
- [x] **Arquivo**: `binaries/director/src/api.rs:277`
- [x] **Warning**: `field 'limit' is never read`
- [x] **Solução**: Utilizar o parâmetro para limitar buscas reais na base RAG

### **6. 🔬 Discovery Agent - Warning Ownership**
**Prioridade**: 🟡 BAIXA | **Estimativa**: 0.5 dia | **Responsável**: Dev Backend

#### **6.1 Verificar Clone Desnecessário**
- [ ] **Arquivo**: `crates/discovery_agent/src/triage.rs:129`
- [ ] **Atual**: `recommended.clone()` (já corrigido)
- [ ] **Validar**: Se o clone é realmente necessário
- [ ] **Otimizar**: Se possível, evitar clone

---

## 🟢 FUNCIONALIDADES NOVAS

### **7. 📊 Observabilidade Completa**
**Prioridade**: 🟢 ALTA | **Estimativa**: 1 semana | **Responsável**: Dev DevOps

#### **7.1 OpenTelemetry Setup**
- [ ] **Dependência**: Adicionar `opentelemetry` ao Cargo.toml
- [ ] **Configuração**: Setup tracing spans
- [ ] **Exporters**: Jaeger + Prometheus
- [ ] **Instrumentação**: Adicionar spans em funções críticas

```rust
// ✅ Template de implementação:
use opentelemetry::trace::Tracer;
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[tracing::instrument]
pub async fn process_hiv_simulation(protein: &str) -> Result<SimulationResult> {
    let span = tracing::Span::current();
    span.set_attribute("protein.name", protein);
    
    // lógica da simulação
    Ok(result)
}
```

#### **7.2 Métricas de Negócio**
- [ ] **Contadores**: Simulações processadas, falhas, sucessos
- [ ] **Histogramas**: Latência de classificação, tempo de simulação
- [ ] **Gauges**: Uso de memória, conexões ativas
- [ ] **Dashboard**: Grafana com métricas científicas

#### **7.3 Health Checks Avançados**
- [ ] **PostgreSQL**: Verificar conectividade e latência
- [ ] **Redis**: Verificar cache e performance
- [ ] **Ollama**: Verificar modelos disponíveis
- [ ] **Disk Space**: Verificar espaço para simulações

#### **7.4 Alertas Inteligentes**
- [ ] **Simulações falhando**: > 5% taxa de erro
- [ ] **Latência alta**: > 500ms para queries
- [ ] **Memória alta**: > 80% uso
- [ ] **Disk space**: < 10% disponível

### **8. ⚡ Cache Inteligente**
**Prioridade**: 🟢 MÉDIA | **Estimativa**: 3 dias | **Responsável**: Dev Backend

#### **8.1 Redis Cache Implementation**
- [ ] **Setup**: Configurar conexão Redis
- [ ] **RAG Cache**: Cache resultados de busca
- [ ] **LLM Cache**: Cache classificações
- [ ] **Function Cache**: Cache resultados de function calling

```rust
// ✅ Template de implementação:
use redis::AsyncCommands;

pub struct IntelligentCache {
    redis: redis::aio::Connection,
    local: moka::future::Cache<String, serde_json::Value>,
}

impl IntelligentCache {
    pub async fn get_or_compute<F, T>(&self, key: &str, compute: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
        T: Serialize + DeserializeOwned,
    {
        // Tentar cache local primeiro
        if let Some(cached) = self.local.get(key).await {
            return Ok(serde_json::from_value(cached)?);
        }
        
        // Tentar Redis
        if let Ok(cached) = self.redis.get::<_, String>(key).await {
            let value: T = serde_json::from_str(&cached)?;
            return Ok(value);
        }
        
        // Computar e cachear
        let result = compute.await?;
        self.cache_result(key, &result).await?;
        Ok(result)
    }
}
```

#### **8.2 Moka Local Cache**
- [ ] **Setup**: Configurar cache em memória
- [ ] **Eviction**: Políticas LRU inteligentes
- [ ] **TTL**: Time-to-live configurável
- [ ] **Métricas**: Hit/miss rates

#### **8.3 Cache Warming**
- [ ] **Startup**: Pre-carregar dados frequentes
- [ ] **Background**: Refresh automático
- [ ] **Invalidation**: Invalidar quando dados mudam
- [ ] **Monitoring**: Métricas de performance

### **9. 🧬 Scientific Engine Expansion**
**Prioridade**: 🟢 BAIXA | **Estimativa**: 2 semanas | **Responsável**: Dev Científico

#### **9.1 Malária Support**
- [ ] **Proteínas**: Plasmodium falciparum targets
- [ ] **Métricas**: IC50, selectivity, ADMET
- [ ] **Knowledge Base**: Literatura malária
- [ ] **Function Calling**: Malária-specific tools

#### **9.2 COVID-19 Support**
- [ ] **Proteínas**: SARS-CoV-2 variants
- [ ] **Métricas**: Binding affinity, escape mutations
- [ ] **Knowledge Base**: COVID research
- [ ] **Monitoring**: Variant tracking

#### **9.3 ST-GNN Implementation**
- [ ] **Python Integration**: PyTorch + PyG
- [ ] **Subprocess**: Async Python execution
- [ ] **Data Pipeline**: Rust ↔ Python bridge
- [ ] **Results**: Embeddings + attention weights

---

## 🌐 DEPLOYMENT E PRODUÇÃO

### **10. 🚀 Railway Production Deploy**
**Prioridade**: 🟢 MÉDIA | **Estimativa**: 1 semana | **Responsável**: Dev DevOps

#### **10.1 Railway Configuration**
- [ ] **railway.toml**: Configuração completa
- [ ] **Environment Variables**: Secrets management
- [ ] **Database**: PostgreSQL Railway addon
- [ ] **Redis**: Redis Railway addon

#### **10.2 CI/CD Pipeline**
- [ ] **GitHub Actions**: Build + test + deploy
- [ ] **Docker**: Multi-stage build otimizado
- [ ] **Health Checks**: Verificação pós-deploy
- [ ] **Rollback**: Estratégia de rollback automático

#### **10.3 Monitoring Production**
- [ ] **Uptime**: 99.9% SLA
- [ ] **Performance**: < 200ms latência
- [ ] **Errors**: < 0.1% error rate
- [ ] **Alerts**: Slack notifications

### **11. 🐳 Docker Optimization**
**Prioridade**: 🟢 BAIXA | **Estimativa**: 2 dias | **Responsável**: Dev DevOps

#### **11.1 Multi-stage Dockerfile**
- [ ] **Builder**: Rust compilation stage
- [ ] **Runtime**: Minimal runtime image
- [ ] **Size**: < 100MB final image
- [ ] **Security**: Non-root user

#### **11.2 Docker Compose**
- [ ] **Services**: Director + Dashboard + DB + Redis
- [ ] **Networks**: Isolated networks
- [ ] **Volumes**: Persistent data
- [ ] **Secrets**: Secure secret management

---

## 🧪 TESTES E QUALIDADE

### **12. 🧪 Test Coverage Expansion**
**Prioridade**: 🟢 MÉDIA | **Estimativa**: 1 semana | **Responsável**: Dev QA

#### **12.1 Unit Tests**
- [ ] **Target**: > 80% code coverage
- [ ] **Scientific**: Validar cálculos HIV
- [ ] **API**: Testar todos endpoints
- [ ] **RAG**: Testar busca e relevância

#### **12.2 Integration Tests**
- [ ] **Database**: PostgreSQL integration
- [ ] **Redis**: Cache integration
- [ ] **Ollama**: LLM integration
- [ ] **Slack**: Webhook integration

#### **12.3 Performance Tests**
- [ ] **Load**: 1000 requests/second
- [ ] **Stress**: Degradação graceful
- [ ] **Memory**: Leak detection
- [ ] **Latency**: P95 < 500ms

### **13. 📚 Documentation**
**Prioridade**: 🟢 BAIXA | **Estimativa**: 3 dias | **Responsável**: Dev Docs

#### **13.1 API Documentation**
- [ ] **OpenAPI**: Swagger/OpenAPI spec
- [ ] **Examples**: Request/response examples
- [ ] **Authentication**: Token usage
- [ ] **Rate Limits**: Usage guidelines

#### **13.2 Scientific Documentation**
- [ ] **Algorithms**: Documentar cálculos
- [ ] **Thresholds**: Justificar métricas
- [ ] **Validation**: Comparação literatura
- [ ] **References**: Bibliografia científica

#### **13.3 User Guides**
- [ ] **Installation**: Guia passo-a-passo
- [ ] **Configuration**: Todas as opções
- [ ] **Troubleshooting**: Problemas comuns
- [ ] **FAQ**: Perguntas frequentes

---

## 🔧 MELHORIAS TÉCNICAS

### **14. 🦀 Rust Code Quality**
**Prioridade**: 🟢 BAIXA | **Estimativa**: 2 dias | **Responsável**: Dev Backend

#### **14.1 Clippy Compliance**
- [ ] **Command**: `cargo clippy --all -- -D warnings`
- [ ] **Fix**: Todos os warnings
- [ ] **CI**: Adicionar ao pipeline
- [ ] **Rules**: Configurar clippy.toml

#### **14.2 Rustfmt Consistency**
- [ ] **Command**: `cargo fmt --all`
- [ ] **Config**: rustfmt.toml customizado
- [ ] **CI**: Verificar formatação
- [ ] **Pre-commit**: Hook automático

#### **14.3 Dependency Audit**
- [ ] **Security**: `cargo audit`
- [ ] **Updates**: `cargo outdated`
- [ ] **Licenses**: Verificar compatibilidade
- [ ] **Size**: Otimizar dependências

### **15. 🔒 Security Hardening**
**Prioridade**: 🟢 ALTA | **Estimativa**: 3 dias | **Responsável**: Dev Security

#### **15.1 Secrets Management**
- [ ] **Environment**: Nunca hardcode secrets
- [ ] **Rotation**: Rotação automática tokens
- [ ] **Encryption**: Encrypt at rest
- [ ] **Audit**: Log access to secrets

#### **15.2 Input Validation**
- [ ] **Sanitization**: Sanitizar inputs
- [ ] **SQL Injection**: Usar prepared statements
- [ ] **XSS**: Escape HTML outputs
- [ ] **Rate Limiting**: Prevenir abuse

#### **15.3 Network Security**
- [ ] **HTTPS**: TLS everywhere
- [ ] **CORS**: Configurar corretamente
- [ ] **Headers**: Security headers
- [ ] **Firewall**: Restrict access

---

## 📊 MÉTRICAS E KPIs

### **16. 📈 Business Metrics**
**Prioridade**: 🟢 MÉDIA | **Estimativa**: 2 dias | **Responsável**: Dev Analytics

#### **16.1 Scientific KPIs**
- [ ] **Simulações/dia**: Target > 1000
- [ ] **Accuracy**: > 95% vs literatura
- [ ] **Discovery Rate**: Novos hits/semana
- [ ] **Time to Insight**: < 1 hora

#### **16.2 Technical KPIs**
- [ ] **Uptime**: > 99.9%
- [ ] **Response Time**: P95 < 200ms
- [ ] **Error Rate**: < 0.1%
- [ ] **Throughput**: > 100 RPS

#### **16.3 User Experience KPIs**
- [ ] **Slack Adoption**: > 80% team usage
- [ ] **API Usage**: > 1000 calls/day
- [ ] **Dashboard Views**: > 100/day
- [ ] **User Satisfaction**: > 4.5/5

---

## 🎯 ROADMAP TEMPORAL

### **📅 SPRINT 1 (Semana 1) - Correções Críticas**
- 🔥 **Dia 1-2**: Corrigir hiv_discovery_runner
- 🔥 **Dia 3**: Corrigir job_scheduler enum
- 🔥 **Dia 4**: Corrigir job_worker struct
- 🔥 **Dia 5**: Corrigir job_client dependências
- ✅ **Entrega**: Todos os binários compilando

### **📅 SPRINT 2 (Semana 2) - Observabilidade**
- 📊 **Dia 1-2**: OpenTelemetry setup
- 📊 **Dia 3**: Métricas de negócio
- 📊 **Dia 4**: Health checks avançados
- 📊 **Dia 5**: Alertas inteligentes
- ✅ **Entrega**: Observabilidade completa

### **📅 SPRINT 3 (Semana 3) - Cache e Performance**
- ⚡ **Dia 1-2**: Redis cache implementation
- ⚡ **Dia 3**: Moka local cache
- ⚡ **Dia 4**: Cache warming
- ⚡ **Dia 5**: Performance testing
- ✅ **Entrega**: Sistema de cache otimizado

### **📅 SPRINT 4 (Semana 4) - Deploy e Produção**
- 🚀 **Dia 1-2**: Railway configuration
- 🚀 **Dia 3**: CI/CD pipeline
- 🚀 **Dia 4**: Production monitoring
- 🚀 **Dia 5**: Load testing
- ✅ **Entrega**: Deploy produção funcional

---

## 🏆 CRITÉRIOS DE SUCESSO

### **✅ V1.1 Release Criteria**
- [ ] **Compilação**: Todos os binários compilam sem erros
- [ ] **Testes**: > 80% code coverage
- [ ] **Performance**: < 200ms latência P95
- [ ] **Observabilidade**: Métricas + alertas funcionando
- [ ] **Cache**: Hit rate > 70%
- [ ] **Deploy**: Produção estável > 99.9% uptime

### **✅ V1.2 Release Criteria**
- [ ] **Expansão**: Suporte malária + COVID-19
- [ ] **ST-GNN**: Análise espaciotemporal
- [ ] **Mobile**: Interface mobile básica
- [ ] **API**: Rate limiting + authentication
- [ ] **Documentation**: Documentação completa
- [ ] **Security**: Audit de segurança aprovado

---

## 📞 RESPONSABILIDADES

### **👥 EQUIPE SUGERIDA**
- **🧬 Dev Científico**: Algoritmos, validação, conhecimento domínio
- **🦀 Dev Backend**: Rust, APIs, database, performance
- **🌐 Dev Frontend**: Dashboard, UX, interfaces
- **🚀 Dev DevOps**: Deploy, monitoring, infrastructure
- **🔒 Dev Security**: Segurança, compliance, audits
- **📊 Dev Analytics**: Métricas, KPIs, business intelligence
- **📚 Dev Docs**: Documentação, guias, tutoriais
- **🧪 Dev QA**: Testes, qualidade, validação

### **📋 PROCESSO DE TASK MANAGEMENT**
1. **Priorização**: Crítico → Importante → Desejável
2. **Estimativa**: Story points ou dias
3. **Assignee**: Responsável principal
4. **Review**: Code review obrigatório
5. **Testing**: Testes antes de merge
6. **Documentation**: Atualizar docs relevantes

---

**🎯 Esta tasklist é um documento vivo e deve ser atualizada conforme o projeto evolui. Cada task completada nos aproxima do objetivo de acelerar a descoberta de medicamentos para HIV através de IA.** 🧬🤖❤️
