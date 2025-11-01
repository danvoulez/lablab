#!/usr/bin/env bash
# 📊 Exemplo 2: Dashboard Demo
# Demonstra interface web científica do LogLine Discovery Lab

set -euo pipefail

echo "📊 LogLine Discovery Lab - Dashboard Demo"
echo "=========================================="
echo ""

# Cores para output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Verificar se dashboard está buildado
if [ ! -f "./target/release/discovery_dashboard" ]; then
    echo -e "${YELLOW}⚠️  Dashboard não encontrado. Buildando...${NC}"
    cargo build --release -p discovery_dashboard
fi

echo -e "${BLUE}📋 Passo 1: Verificando pré-requisitos${NC}"
echo ""

# Verificar PostgreSQL
if ! pg_isready -q; then
    echo -e "${YELLOW}⚠️  PostgreSQL não está rodando${NC}"
    echo "Execute: brew services start postgresql@15"
    exit 1
fi
echo -e "${GREEN}✅ PostgreSQL: OK${NC}"

echo ""
echo -e "${BLUE}📋 Passo 2: Iniciando Dashboard${NC}"
echo ""

PORT=3000
echo "Dashboard será iniciado em: http://localhost:$PORT"
echo ""

# Verificar se porta está livre
if lsof -Pi :$PORT -sTCP:LISTEN -t >/dev/null 2>&1 ; then
    echo -e "${YELLOW}⚠️  Porta $PORT já está em uso${NC}"
    echo "Matando processo existente..."
    lsof -ti:$PORT | xargs kill -9 2>/dev/null || true
    sleep 1
fi

echo -e "${GREEN}🚀 Iniciando servidor...${NC}"
echo ""

# Iniciar dashboard em background
./target/release/discovery_dashboard --port $PORT &
DASHBOARD_PID=$!

# Aguardar servidor iniciar
echo "Aguardando servidor iniciar..."
sleep 3

# Verificar se servidor está rodando
if ! curl -s -f http://localhost:$PORT/health > /dev/null; then
    echo -e "${YELLOW}⚠️  Servidor não respondeu no health check${NC}"
    echo "Verifique logs acima para detalhes do erro"
    kill $DASHBOARD_PID 2>/dev/null || true
    exit 1
fi

echo -e "${GREEN}✅ Servidor rodando!${NC}"
echo ""

echo -e "${BLUE}📋 Passo 3: Funcionalidades disponíveis${NC}"
echo ""

echo "🌐 Acesse no navegador: ${YELLOW}http://localhost:$PORT${NC}"
echo ""
echo "📊 Funcionalidades:"
echo "   • Dashboard principal: /"
echo "   • Status simulações HIV: /simulations"
echo "   • Análise de proteínas: /proteins"
echo "   • Base de conhecimento: /knowledge"
echo "   • Métricas científicas: /metrics"
echo "   • Health check (API): /health"
echo ""

echo -e "${BLUE}📋 Passo 4: Testando endpoints API${NC}"
echo ""

# Health check
echo -e "${GREEN}✓${NC} Health check:"
curl -s http://localhost:$PORT/health | jq '.' || echo "{\"status\": \"ok\"}"
echo ""

# Verificar se há simulações
echo -e "${GREEN}✓${NC} Simulações recentes:"
curl -s http://localhost:$PORT/api/simulations/recent 2>/dev/null | jq '.' || echo "[]"
echo ""

# Estatísticas
echo -e "${GREEN}✓${NC} Estatísticas:"
curl -s http://localhost:$PORT/api/stats 2>/dev/null | jq '.' || echo "{}"
echo ""

echo -e "${BLUE}📋 Passo 5: Explorando visualmente${NC}"
echo ""

echo "👉 Abra o navegador em: http://localhost:$PORT"
echo ""
echo "O que você verá:"
echo "   📈 Gráficos de RMSD e energia molecular"
echo "   🧬 Lista de proteínas analisadas (gp41, gp120, Rev, Tat)"
echo "   📊 Métricas de estabilidade estrutural"
echo "   🔍 Interface de busca na base de conhecimento"
echo "   ⚡ Status de simulações em tempo real"
echo ""

# Abrir navegador automaticamente (macOS)
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Abrindo navegador automaticamente..."
    open "http://localhost:$PORT"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if command -v xdg-open &> /dev/null; then
        xdg-open "http://localhost:$PORT"
    fi
fi

echo ""
echo -e "${YELLOW}⏳ Dashboard está rodando...${NC}"
echo ""
echo "Pressione Ctrl+C para parar o servidor"
echo ""

# Função de cleanup
cleanup() {
    echo ""
    echo -e "${BLUE}🛑 Parando servidor...${NC}"
    kill $DASHBOARD_PID 2>/dev/null || true
    echo -e "${GREEN}✅ Servidor parado${NC}"
    exit 0
}

# Registrar cleanup
trap cleanup SIGINT SIGTERM

# Aguardar indefinidamente (até Ctrl+C)
wait $DASHBOARD_PID

cleanup
