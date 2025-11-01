#!/usr/bin/env bash
# 🧬 Exemplo 1: Análise Básica de gp41
# Demonstra análise de folding da proteína gp41 do HIV

set -euo pipefail

echo "🧬 LogLine Discovery Lab - Demo gp41"
echo "===================================="
echo ""

# Cores para output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Verificar se director está buildado
if [ ! -f "./target/release/director" ]; then
    echo -e "${YELLOW}⚠️  Director não encontrado. Buildando...${NC}"
    cargo build --release -p director
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

# Verificar Ollama
if ! ollama list | grep -q "mistral:instruct"; then
    echo -e "${YELLOW}⚠️  Modelo Ollama não encontrado${NC}"
    echo "Execute: ollama pull mistral:instruct"
    exit 1
fi
echo -e "${GREEN}✅ Ollama: OK${NC}"

echo ""
echo -e "${BLUE}📋 Passo 2: Preparando consulta${NC}"
echo ""

# Query para análise de gp41
QUERY="Analisar a proteína gp41 do HIV-1. Quero saber:\n- Estabilidade estrutural (RMSD)\n- Energia molecular\n- Potenciais instabilidades"

echo -e "Query: ${YELLOW}${QUERY}${NC}"
echo ""

echo -e "${BLUE}📋 Passo 3: Executando análise via Director${NC}"
echo ""

# Criar arquivo temporário com a query
TEMP_QUERY=$(mktemp)
echo -e "$QUERY" > "$TEMP_QUERY"

# Executar director via stdin
echo -e "${GREEN}🤖 Director processando...${NC}"
echo ""

# Opção 1: Via CLI interativo (simulado com echo)
cat "$TEMP_QUERY" | ./target/release/director --mode cli 2>/dev/null || {
    echo -e "${YELLOW}⚠️  Erro ao executar director. Tentando modo alternativo...${NC}"
    
    # Opção 2: Via API (se CLI falhar)
    echo "Iniciando director em modo API..."
    ./target/release/director --mode api --port 8080 &
    DIRECTOR_PID=$!
    
    # Aguardar API iniciar
    sleep 3
    
    # Fazer request via curl
    echo -e "${GREEN}📡 Fazendo request via API...${NC}"
    curl -s -X POST http://localhost:8080/rag/query \
        -H "Content-Type: application/json" \
        -d "{\"query\": \"$QUERY\", \"max_results\": 5}" | jq '.'
    
    # Cleanup
    kill $DIRECTOR_PID 2>/dev/null || true
}

# Cleanup
rm -f "$TEMP_QUERY"

echo ""
echo -e "${BLUE}📋 Passo 4: Interpretando resultados${NC}"
echo ""

echo -e "${GREEN}✅ Análise completa!${NC}"
echo ""
echo "📊 Métricas esperadas:"
echo "   - RMSD < 5.0 Å → Estrutura estável"
echo "   - Energia < -120 kcal/mol → Configuração favorável"
echo "   - Sem flags de instabilidade → Proteína bem formada"
echo ""
echo "🧬 Proteína gp41:"
echo "   - Função: Fusão de membranas (entrada celular do HIV)"
echo "   - Importância: Alvo terapêutico crítico"
echo "   - Estrutura: Hélices transmembrana + domínio citoplásmico"
echo ""

echo -e "${BLUE}📚 Próximos passos:${NC}"
echo "   1. Testar outras proteínas: gp120, Rev, Tat"
echo "   2. Ver dashboard: ./examples/02_dashboard_demo.sh"
echo "   3. Configurar Slack: ./examples/03_slack_integration.sh"
echo "   4. Gerar manuscrito: ./examples/04_manuscript_generation.sh"
echo ""

echo -e "${GREEN}🎉 Demo completo!${NC}"
