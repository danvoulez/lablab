#!/bin/bash
# 📦 LogLine Discovery Lab - Build Release V1.0

set -e

echo "🧬 LogLine Discovery Lab - Build Release V1.0"
echo "============================================="

# Verificar versão
VERSION=$(cat VERSION)
echo "📋 Versão: $VERSION"

# Verificar dependências
echo ""
echo "🔍 Verificando dependências..."

# Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo não encontrado"
    exit 1
fi
echo "✅ Rust: $(rustc --version)"

# PostgreSQL
if ! command -v psql &> /dev/null; then
    echo "❌ PostgreSQL não encontrado"
    exit 1
fi
echo "✅ PostgreSQL: $(psql --version | head -n1)"

# Redis
if ! command -v redis-server &> /dev/null; then
    echo "❌ Redis não encontrado"
    exit 1
fi
echo "✅ Redis: $(redis-server --version)"

# Ollama
if ! command -v ollama &> /dev/null; then
    echo "❌ Ollama não encontrado"
    exit 1
fi
echo "✅ Ollama: $(ollama --version)"

echo ""
echo "🔨 Iniciando build release..."

# Limpar builds anteriores
echo "🧹 Limpando builds anteriores..."
cargo clean

# Build otimizado
echo "⚡ Build otimizado (release)..."
cargo build --release --all

# Verificar binários
echo ""
echo "🎯 Verificando binários gerados..."

BINARIES=(
    "target/release/director"
    "target/release/hiv_discovery_runner"
    "target/release/discovery_dashboard"
    "target/release/job_scheduler"
    "target/release/job_worker"
    "target/release/job_client"
)

for binary in "${BINARIES[@]}"; do
    if [ -f "$binary" ]; then
        echo "✅ $binary"
    else
        echo "❌ $binary (não encontrado)"
    fi
done

# Executar testes
echo ""
echo "🧪 Executando testes..."
cargo test --release --all

# Criar diretório de release
RELEASE_DIR="releases/v$VERSION"
mkdir -p "$RELEASE_DIR"

echo ""
echo "📦 Empacotando release..."

# Copiar binários
cp -r target/release/* "$RELEASE_DIR/" 2>/dev/null || true

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

# Criar arquivo de instalação
cat > "$RELEASE_DIR/install.sh" << 'EOF'
#!/bin/bash
# LogLine Discovery Lab V1.0 - Instalação

echo "🧬 LogLine Discovery Lab V1.0 - Instalação"
echo "=========================================="

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

echo ""
echo "✅ Instalação concluída!"
echo ""
echo "🚀 Para iniciar:"
echo "   ./scripts/setup.sh"
echo "   ./scripts/run_lab.sh"
echo ""
echo "🤖 Para usar o Director:"
echo "   ./director chat"
echo "   ./director serve --port 3000"
echo ""
EOF

chmod +x "$RELEASE_DIR/install.sh"

# Criar checksum
echo ""
echo "🔐 Gerando checksums..."
cd "$RELEASE_DIR"
find . -type f -exec shasum -a 256 {} \; > SHA256SUMS
cd - > /dev/null

# Criar tarball
echo "📦 Criando tarball..."
tar -czf "releases/logline-discovery-lab-v$VERSION.tar.gz" -C releases "v$VERSION"

# Estatísticas
echo ""
echo "📊 Estatísticas do release:"
echo "   Versão: $VERSION"
echo "   Binários: $(ls -1 $RELEASE_DIR/target/release/ 2>/dev/null | wc -l || echo 0)"
echo "   Tamanho: $(du -sh releases/logline-discovery-lab-v$VERSION.tar.gz | cut -f1)"
echo "   Arquivos: $(find $RELEASE_DIR -type f | wc -l)"

echo ""
echo "🎉 Release V$VERSION criado com sucesso!"
echo "   📁 Diretório: $RELEASE_DIR"
echo "   📦 Tarball: releases/logline-discovery-lab-v$VERSION.tar.gz"
echo ""
echo "🚀 Para testar:"
echo "   cd $RELEASE_DIR"
echo "   ./install.sh"
echo "   ./director --version"
