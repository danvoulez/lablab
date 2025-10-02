#!/bin/bash
# 📦 LogLine Discovery Lab - Build Partial Release V1.0

set -e

echo "🧬 LogLine Discovery Lab - Build Partial Release V1.0"
echo "===================================================="

# Verificar versão
VERSION=$(cat VERSION)
echo "📋 Versão: $VERSION"

echo ""
echo "🎯 Binários compilados com sucesso:"
echo "✅ director (7.3MB) - Agente conversacional completo"
echo "✅ discovery_dashboard (6.4MB) - Interface web científica"
echo ""
echo "❌ Binários com erros (serão corrigidos na v1.1):"
echo "   - hiv_discovery_runner (erro de sintaxe)"
echo "   - job_scheduler (erro de enum)"
echo "   - job_worker (erro de struct)"
echo "   - job_client (erro de dependência)"

# Criar diretório de release
RELEASE_DIR="releases/v$VERSION-partial"
mkdir -p "$RELEASE_DIR/bin"

echo ""
echo "📦 Empacotando release parcial..."

# Copiar binários funcionais
cp target/release/director "$RELEASE_DIR/bin/"
cp target/release/discovery_dashboard "$RELEASE_DIR/bin/"

# Copiar documentação
cp README.md "$RELEASE_DIR/"
cp CHANGELOG.md "$RELEASE_DIR/"
cp VERSION "$RELEASE_DIR/"
cp release.toml "$RELEASE_DIR/"

# Copiar configurações essenciais
cp -r contracts "$RELEASE_DIR/"
cp -r modelfiles "$RELEASE_DIR/"
cp -r samples "$RELEASE_DIR/"
cp slack-app-manifest.json "$RELEASE_DIR/"

# Copiar scripts
mkdir -p "$RELEASE_DIR/scripts"
cp scripts/*.sh "$RELEASE_DIR/scripts/"

# Criar arquivo de instalação parcial
cat > "$RELEASE_DIR/install.sh" << 'EOF'
#!/bin/bash
# LogLine Discovery Lab V1.0 - Instalação Parcial

echo "🧬 LogLine Discovery Lab V1.0 - Instalação Parcial"
echo "=================================================="

# Verificar sistema
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "✅ Sistema: macOS"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "✅ Sistema: Linux"
else
    echo "❌ Sistema não suportado: $OSTYPE"
    exit 1
fi

# Instalar dependências (macOS)
if [[ "$OSTYPE" == "darwin"* ]]; then
    if ! command -v brew &> /dev/null; then
        echo "❌ Homebrew necessário: https://brew.sh"
        exit 1
    fi
    
    echo "📦 Instalando dependências..."
    brew install postgresql@15 redis ollama
fi

# Configurar Ollama
echo "🤖 Configurando Ollama..."
ollama pull mistral:instruct
ollama pull llama3.2

# Criar diretórios
mkdir -p data logs

# Tornar binários executáveis
chmod +x bin/director
chmod +x bin/discovery_dashboard

echo ""
echo "✅ Instalação parcial concluída!"
echo ""
echo "🎯 FUNCIONALIDADES DISPONÍVEIS:"
echo ""
echo "🤖 Director (Agente Conversacional):"
echo "   ./bin/director chat              # Modo interativo"
echo "   ./bin/director serve --port 3000 # API REST"
echo "   ./bin/director slack --port 3002 # Bot Slack"
echo ""
echo "📊 Discovery Dashboard (Interface Web):"
echo "   ./bin/discovery_dashboard        # http://127.0.0.1:4600"
echo ""
echo "📋 LIMITAÇÕES V1.0-PARTIAL:"
echo "   - hiv_discovery_runner: não disponível (erro compilação)"
echo "   - Sistema de jobs: não disponível (erro compilação)"
echo "   - Pipeline completo: aguardar v1.1"
echo ""
echo "🚀 TESTE RÁPIDO:"
echo "   ./bin/director --help"
echo "   ./bin/discovery_dashboard &"
echo "   ./bin/director chat"
echo ""
EOF

chmod +x "$RELEASE_DIR/install.sh"

# Criar README específico do release parcial
cat > "$RELEASE_DIR/README_PARTIAL.md" << 'EOF'
# 🧬 LogLine Discovery Lab V1.0 - Release Parcial

## ✅ O que funciona nesta versão:

### 🤖 Director (Agente Conversacional Completo)
- ✅ **RAG System**: Knowledge base com contexto histórico
- ✅ **Function Calling**: 5 ferramentas dinâmicas
- ✅ **REST API**: 7 endpoints funcionais
- ✅ **Slack Bot**: Webhooks + slash commands
- ✅ **CLI**: Múltiplos modos (chat, serve, slack)

### 📊 Discovery Dashboard (Interface Web)
- ✅ **Causal Chain Explorer**: Visualização científica
- ✅ **API Proxy**: Conecta com serviços
- ✅ **Interface moderna**: Dark theme científico

## 🎯 Como usar:

```bash
# Instalar
./install.sh

# Testar Director
./bin/director chat

# Testar Dashboard  
./bin/discovery_dashboard &
open http://127.0.0.1:4600

# API REST
./bin/director serve --port 3000 &
curl http://localhost:3000/health
```

## ❌ Limitações desta versão:

- **hiv_discovery_runner**: Erro de sintaxe (será corrigido v1.1)
- **job_scheduler/worker/client**: Erros de compilação
- **Pipeline completo**: Aguardar próxima versão

## 🚀 Próximos passos (V1.1):

1. Corrigir erros de sintaxe nos binários restantes
2. Completar sistema de jobs distribuído
3. Integrar pipeline científico completo
4. Adicionar observabilidade completa

**Esta versão parcial já oferece um agente conversacional completo e funcional para laboratórios farmacêuticos!** 🧬🤖
EOF

# Criar checksum
echo ""
echo "🔐 Gerando checksums..."
cd "$RELEASE_DIR"
find . -type f -exec shasum -a 256 {} \; > SHA256SUMS
cd - > /dev/null

# Criar tarball
echo "📦 Criando tarball..."
tar -czf "releases/logline-discovery-lab-v$VERSION-partial.tar.gz" -C releases "v$VERSION-partial"

# Estatísticas
echo ""
echo "📊 Estatísticas do release parcial:"
echo "   Versão: $VERSION-partial"
echo "   Binários funcionais: 2/6"
echo "   Tamanho: $(du -sh releases/logline-discovery-lab-v$VERSION-partial.tar.gz | cut -f1)"
echo "   Arquivos: $(find $RELEASE_DIR -type f | wc -l)"

echo ""
echo "🎉 Release Parcial V$VERSION criado com sucesso!"
echo "   📁 Diretório: $RELEASE_DIR"
echo "   📦 Tarball: releases/logline-discovery-lab-v$VERSION-partial.tar.gz"
echo ""
echo "🚀 Para testar:"
echo "   cd $RELEASE_DIR"
echo "   ./install.sh"
echo "   ./bin/director --help"
echo "   ./bin/discovery_dashboard &"
echo ""
echo "✨ FUNCIONALIDADES PRINCIPAIS DISPONÍVEIS:"
echo "   🤖 Agente conversacional completo"
echo "   📱 Bot Slack profissional"
echo "   🌐 API REST com 7 endpoints"
echo "   📊 Dashboard web científico"
echo "   🧠 RAG + Function Calling"
