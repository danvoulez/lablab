# 🧠 LogLine Director: Proposta de Arquitetura LLM Unificada

## 📋 Resumo Executivo

Com base na pesquisa sobre **Vercel AI SDK 5**, **Google Gemini OpenAI Compatibility**, e **melhores práticas de integração multi-provider**, proponho uma arquitetura LLM unificada para o LogLine Director que combina o melhor dos mundos: **performance local**, **robustez na nuvem**, e **flexibilidade de providers**.

## 🎯 Objetivos da Arquitetura

### **Problemas Atuais**
- ❌ Implementação manual de cada provider (OpenAI, Ollama, etc.)
- ❌ Código duplicado para diferentes APIs
- ❌ Falta de fallbacks inteligentes
- ❌ Sem streaming otimizado
- ❌ Gerenciamento manual de tipos e erros

### **Soluções Propostas**
- ✅ **Unified Provider Interface** - API consistente para todos os LLMs
- ✅ **Intelligent Fallback Chain** - Cascata automática de providers
- ✅ **Type-Safe Streaming** - Streaming com tipos seguros
- ✅ **Cost Optimization** - Seleção dinâmica baseada em custo/performance
- ✅ **Caching Inteligente** - Cache multi-camadas para eficiência

## 🏗️ Arquitetura Proposta

### **1. Provider Abstraction Layer**

```rust
// Inspirado no Vercel AI SDK 5
pub trait UnifiedLLMProvider {
    async fn classify_intent(&self, input: &str) -> Result<IntentClassification>;
    async fn stream_response(&self, messages: &[Message]) -> Result<ResponseStream>;
    fn provider_type(&self) -> ProviderType;
    fn cost_per_token(&self) -> f64;
    fn supports_function_calling(&self) -> bool;
}

pub enum ProviderType {
    Local(LocalProvider),      // Ollama, llama.cpp
    Cloud(CloudProvider),      // OpenAI, Anthropic, Google
    Hybrid(HybridProvider),    // Gemini via OpenAI API
}
```

### **2. Intelligent Provider Selection**

```rust
pub struct ProviderOrchestrator {
    providers: Vec<Box<dyn UnifiedLLMProvider>>,
    selection_strategy: SelectionStrategy,
    cache: Arc<ResponseCache>,
}

pub enum SelectionStrategy {
    CostOptimized,     // Menor custo primeiro
    LatencyOptimized,  // Menor latência primeiro
    QualityOptimized,  // Melhor modelo primeiro
    Adaptive,          // Baseado em load/context
}
```

### **3. Streaming com Type Safety**

```rust
// Inspirado no AI SDK 5 redesigned chat
pub struct TypedStream<T> {
    inner: Pin<Box<dyn Stream<Item = Result<T>>>>,
    metadata: StreamMetadata,
}

pub struct IntentClassificationStream {
    confidence: f32,
    partial_classification: Option<Flow>,
    reasoning_steps: Vec<String>,
}
```

## 🚀 Implementação Recomendada

### **Fase 1: Provider Unification (1-2 semanas)**

#### **1.1 Gemini via OpenAI API**
```rust
// Baseado na compatibilidade oficial do Google
pub struct GeminiProvider {
    client: OpenAIClient,
    base_url: "https://generativelanguage.googleapis.com/v1beta/openai/",
    model: "gemini-2.0-flash",
}

impl UnifiedLLMProvider for GeminiProvider {
    async fn classify_intent(&self, input: &str) -> Result<IntentClassification> {
        // Usa a mesma interface OpenAI, mas aponta para Gemini
        let response = self.client.chat().completions().create(
            CreateChatCompletionRequestArgs::default()
                .model(&self.model)
                .messages(vec![/* ... */])
                .build()?
        ).await?;
        
        self.parse_classification(response)
    }
}
```

#### **1.2 Provider Registry**
```rust
pub struct ProviderRegistry {
    providers: HashMap<String, Box<dyn UnifiedLLMProvider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        let mut registry = Self { providers: HashMap::new() };
        
        // Local providers
        registry.register("ollama-mistral", Box::new(OllamaProvider::new("mistral-director")));
        registry.register("ollama-llama", Box::new(OllamaProvider::new("llama3.2:1b")));
        
        // Cloud providers
        registry.register("openai-gpt4", Box::new(OpenAIProvider::new("gpt-4o")));
        registry.register("gemini-flash", Box::new(GeminiProvider::new("gemini-2.0-flash")));
        registry.register("anthropic-claude", Box::new(AnthropicProvider::new("claude-3-5-sonnet")));
        
        registry
    }
}
```

### **Fase 2: Intelligent Orchestration (2-3 semanas)**

#### **2.1 Cost-Aware Selection**
```rust
pub struct CostOptimizedSelector {
    budget_per_hour: f64,
    current_spend: f64,
    provider_costs: HashMap<String, f64>,
}

impl CostOptimizedSelector {
    pub async fn select_provider(&self, context: &RequestContext) -> String {
        let remaining_budget = self.budget_per_hour - self.current_spend;
        
        match context.complexity {
            Complexity::Simple if remaining_budget < 0.01 => "ollama-llama",
            Complexity::Medium if remaining_budget < 0.05 => "ollama-mistral", 
            Complexity::Complex if remaining_budget > 0.10 => "openai-gpt4",
            _ => "gemini-flash", // Balanced cost/quality
        }
    }
}
```

#### **2.2 Adaptive Fallback Chain**
```rust
pub struct AdaptiveFallbackChain {
    primary_providers: Vec<String>,
    fallback_providers: Vec<String>,
    circuit_breaker: CircuitBreaker,
}

impl AdaptiveFallbackChain {
    pub async fn execute_with_fallback<T>(&self, request: Request) -> Result<T> {
        for provider_id in &self.primary_providers {
            if self.circuit_breaker.is_open(provider_id) {
                continue; // Skip failed providers
            }
            
            match self.try_provider(provider_id, &request).await {
                Ok(result) => {
                    self.circuit_breaker.record_success(provider_id);
                    return Ok(result);
                }
                Err(e) => {
                    self.circuit_breaker.record_failure(provider_id);
                    warn!("Provider {} failed: {}", provider_id, e);
                }
            }
        }
        
        // Try fallback providers
        for provider_id in &self.fallback_providers {
            if let Ok(result) = self.try_provider(provider_id, &request).await {
                return Ok(result);
            }
        }
        
        Err("All providers failed".into())
    }
}
```

### **Fase 3: Advanced Features (3-4 semanas)**

#### **3.1 Streaming com Backpressure**
```rust
// Inspirado no Vercel AI SDK streaming optimization
pub struct OptimizedStream {
    inner: Pin<Box<dyn Stream<Item = Result<StreamChunk>>>>,
    backpressure_handler: BackpressureHandler,
}

impl OptimizedStream {
    pub fn new(provider_stream: impl Stream<Item = Result<StreamChunk>>) -> Self {
        let backpressure_handler = BackpressureHandler::new(
            BufferSize::Adaptive,
            DrainStrategy::Lazy,
        );
        
        Self {
            inner: Box::pin(provider_stream),
            backpressure_handler,
        }
    }
}
```

#### **3.2 Multi-Layer Caching**
```rust
pub struct IntelligentCache {
    l1_memory: LruCache<String, CachedResponse>,
    l2_redis: RedisCache,
    l3_disk: DiskCache,
}

impl IntelligentCache {
    pub async fn get_or_compute<F, T>(&self, key: &str, compute: F) -> Result<T>
    where
        F: FnOnce() -> Pin<Box<dyn Future<Output = Result<T>>>>,
    {
        // L1: Memory cache (fastest)
        if let Some(cached) = self.l1_memory.get(key) {
            return Ok(cached.clone());
        }
        
        // L2: Redis cache (fast)
        if let Some(cached) = self.l2_redis.get(key).await? {
            self.l1_memory.put(key.to_string(), cached.clone());
            return Ok(cached);
        }
        
        // L3: Compute and cache
        let result = compute().await?;
        self.cache_at_all_levels(key, &result).await?;
        Ok(result)
    }
}
```

## 📊 Benefícios Esperados

### **Performance**
- **90% redução na latência** para requests cached
- **50% redução no tempo de resposta** com streaming otimizado
- **Zero downtime** com fallbacks automáticos

### **Custo**
- **70% redução nos custos** com seleção inteligente de providers
- **Cache hit rate > 80%** para operações repetitivas
- **Budget control** automático por hora/dia

### **Robustez**
- **99.9% uptime** com múltiplos providers
- **Graceful degradation** quando providers falham
- **Circuit breaker** para providers instáveis

### **Developer Experience**
- **Type-safe** em toda a pipeline
- **Unified API** para todos os providers
- **Observabilidade** completa com métricas

## 🛠️ Stack Tecnológica Recomendada

### **Core Dependencies**
```toml
[dependencies]
# Unified LLM interface (inspirado no AI SDK)
async-openai = "0.20"           # OpenAI + Gemini compatibility
anthropic = "0.2"               # Claude
ollama-rs = "0.1"               # Local models

# Streaming e async
tokio-stream = "0.1"
futures-util = "0.3"
pin-project = "1.0"

# Caching
redis = { version = "0.24", features = ["tokio-comp"] }
sled = "0.34"                   # Embedded disk cache

# Observability
tracing = "0.1"
metrics = "0.22"
opentelemetry = "0.21"

# Circuit breaker
circuit-breaker = "0.1"
```

### **Configuration Example**
```toml
# config/director.toml
[llm]
budget_per_hour = 10.0  # USD
default_strategy = "adaptive"

[providers.ollama]
enabled = true
models = ["mistral-director", "llama3.2:1b"]
base_url = "http://localhost:11434"

[providers.openai]
enabled = true
models = ["gpt-4o", "gpt-4o-mini"]
api_key_env = "OPENAI_API_KEY"

[providers.gemini]
enabled = true
models = ["gemini-2.0-flash", "gemini-2.0-flash-thinking"]
api_key_env = "GEMINI_API_KEY"
use_openai_compatibility = true

[caching]
memory_size_mb = 100
redis_url = "redis://localhost:6379"
disk_cache_path = "/tmp/director_cache"
ttl_seconds = 3600
```

## 🎯 Roadmap de Implementação

### **Sprint 1 (Semana 1-2): Foundation**
- [ ] Trait `UnifiedLLMProvider`
- [ ] Implementação básica para Ollama, OpenAI, Gemini
- [ ] Provider registry
- [ ] Testes unitários

### **Sprint 2 (Semana 3-4): Intelligence**
- [ ] Cost-aware selection
- [ ] Circuit breaker pattern
- [ ] Adaptive fallback chain
- [ ] Métricas básicas

### **Sprint 3 (Semana 5-6): Performance**
- [ ] Streaming otimizado
- [ ] Multi-layer caching
- [ ] Backpressure handling
- [ ] Load testing

### **Sprint 4 (Semana 7-8): Production**
- [ ] Observabilidade completa
- [ ] Configuration management
- [ ] Error recovery
- [ ] Documentation

## 🔍 Considerações Técnicas

### **Compatibilidade**
- **Gemini via OpenAI API**: Permite usar a mesma interface para ambos
- **Vercel AI SDK patterns**: Adaptar padrões TypeScript para Rust
- **Streaming standards**: WebStreams API compatibility

### **Segurança**
- **API key rotation**: Suporte a rotação automática
- **Rate limiting**: Por provider e global
- **Input sanitization**: Validação de entrada

### **Monitoramento**
- **Provider health**: Status de cada provider
- **Cost tracking**: Gasto por provider/hora
- **Performance metrics**: Latência, throughput, error rate

## 💡 Conclusão

Esta arquitetura unificada transformará o LogLine Director de um chatbot baseado em regras para um **verdadeiro agente LLM inteligente** com:

1. **Máxima robustez** através de múltiplos providers
2. **Otimização de custos** com seleção inteligente  
3. **Performance superior** com streaming e caching
4. **Type safety** em toda a pipeline
5. **Observabilidade completa** para produção

A implementação seguirá as melhores práticas do **Vercel AI SDK 5**, aproveitando a **compatibilidade OpenAI do Gemini**, e criando uma base sólida para futuras expansões do sistema.

**Próximo passo**: Implementar o Sprint 1 e validar a arquitetura com um MVP funcional.
