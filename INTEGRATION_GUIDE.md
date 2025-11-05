# LogLine Discovery Lab - Integration Guide

## 🧬 Visão Geral

Este repositório contém dois componentes principais que agora estão integrados:

1. **Backend (Director)**: API Rust para simulação de proteínas e agente IA
2. **Frontend (Protein Cinema)**: Interface Next.js cinematográfica para visualização

## 🚀 Como Executar

### Pré-requisitos

- **Rust** (1.70+) e Cargo
- **Node.js** (18+) e npm
- **PostgreSQL** (opcional, para features avançadas)
- **Ollama** (opcional, para LLM local)

### 1. Iniciar o Backend (Director API)

```bash
cd logline_discovery
cargo run --bin director
```

O backend iniciará na porta **3001** por padrão.

**Endpoints disponíveis:**
- `GET /health` - Health check
- `POST /api/simulate_protein` - Simular estrutura de proteína
- `POST /api/chat` - Conversar com o agente
- `POST /api/classify` - Classificar intenções
- `GET /api/functions` - Listar funções disponíveis

### 2. Iniciar o Frontend (Protein Cinema)

```bash
cd protein-cinema-chatgpt

# Criar arquivo de configuração (primeira vez)
cp .env.local.example .env.local

# Instalar dependências (primeira vez)
npm install

# Iniciar em modo desenvolvimento
npm run dev
```

O frontend iniciará em **http://localhost:3000**

### 3. Usar o Sistema

1. Abra o navegador em `http://localhost:3000`
2. Digite uma sequência FASTA ou descrição de hipótese no chat
3. O backend processará e retornará a simulação
4. Explore as abas:
   - **Simulation**: Visualização 3D interativa da proteína
   - **Analysis**: Métricas e gráficos de confiança (pLDDT)
   - **Replay**: Timeline auditável dos passos executados
   - **Manifesto**: Documento científico assinado digitalmente

## 🔧 Configuração

### Backend

Variáveis de ambiente (opcional):
```bash
export DATABASE_URL="postgresql://user:pass@localhost/logline"
export OLLAMA_URL="http://localhost:11434"
export PORT="3001"
```

### Frontend

Edite `.env.local`:
```bash
NEXT_PUBLIC_API_BASE_URL=http://localhost:3001
```

## 📊 Arquitetura

```
┌─────────────────────────────────────┐
│   Frontend (Next.js)                │
│   - Protein Cinema Interface        │
│   - 3D Visualization (3Dmol.js)     │
│   - Real-time Chat                  │
└──────────────┬──────────────────────┘
               │
               │ HTTP/JSON
               │
┌──────────────▼──────────────────────┐
│   Backend (Rust/Axum)               │
│   - Director API                    │
│   - Protein Simulation              │
│   - RAG + LLM Agent                 │
│   - Cryptographic Evidence          │
└─────────────────────────────────────┘
```

## 🧬 Exemplo de Uso

### Simulação Simples

Chat input:
```
>sp|P69905|HBA_HUMAN Hemoglobin subunit alpha
MVLSPADKTNVKAAWGKVGAHAGEYGAEALERMFLSFPTTKTYFPHF
```

O sistema retornará:
- Estrutura PDB 3D
- Perfil de confiança pLDDT
- Manifesto científico assinado
- Audit trail completo

## 🔐 Segurança

- Todas as simulações geram hash criptográfico SHA-256
- Manifesto assinado digitalmente
- Audit trail imutável
- CORS configurado para desenvolvimento (ajustar em produção)

## 🧪 Testing

### Backend
```bash
cd logline_discovery
cargo test
cargo check --bin director
```

### Frontend
```bash
cd protein-cinema-chatgpt
npm run build
npm run lint
```

## 📝 Próximos Passos

- [ ] Implementar autenticação JWT
- [ ] Adicionar cache Redis para resultados
- [ ] Integrar engine de folding real (OpenMM)
- [ ] Deploy em produção (Railway/Vercel)
- [ ] Adicionar testes E2E

## 🐛 Troubleshooting

### "Backend não está disponível"
- Verifique se o Director está rodando: `curl http://localhost:3001/health`
- Confirme a porta no `.env.local` do frontend

### "3Dmol falha ao carregar"
- Verifique conexão com internet (CDN)
- Tente limpar cache do navegador

### "Erro de compilação no backend"
- Execute `cargo clean && cargo build`
- Verifique versão do Rust: `rustc --version`

## 📚 Documentação Adicional

- [Backend API Reference](./logline_discovery/README.md)
- [Frontend Components](./protein-cinema-chatgpt/README.md)
- [Merge Dialogue](./Merge-Dialogue.md)
- [Hints and Tasklist](./Hints%20and%20Tasklist.md)

## 🤝 Contribuindo

Este é um projeto de descoberta de medicamentos para HIV. Toda contribuição é bem-vinda!

---

**LogLine Discovery Lab** - Transformando simulações computacionais em evidências científicas auditáveis. 🧬✨
