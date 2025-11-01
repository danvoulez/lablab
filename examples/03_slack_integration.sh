#!/usr/bin/env bash
# 📱 Exemplo 3: Slack Integration Demo
# Demonstra integração do LogLine Discovery Lab com Slack

set -euo pipefail

echo "📱 LogLine Discovery Lab - Slack Integration Demo"
echo "=================================================="
echo ""

# Cores para output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}📋 Passo 1: Pré-requisitos${NC}"
echo ""

echo "Para configurar a integração Slack, você precisará:"
echo ""
echo "1. ✅ Criar um Slack App no workspace"
echo "2. ✅ Obter Bot Token (xoxb-...)"
echo "3. ✅ Obter Signing Secret"
echo "4. ✅ Configurar variáveis de ambiente"
echo ""

# Verificar se variáveis estão configuradas
if [ -z "${SLACK_BOT_TOKEN:-}" ] || [ -z "${SLACK_SIGNING_SECRET:-}" ]; then
    echo -e "${YELLOW}⚠️  Variáveis Slack não configuradas${NC}"
    echo ""
    echo "Configure no arquivo .env:"
    echo ""
    echo "SLACK_BOT_TOKEN=xoxb-your-token-here"
    echo "SLACK_SIGNING_SECRET=your-signing-secret"
    echo ""
    echo -e "${BLUE}📚 Guia completo de setup: docs/slack_setup.md${NC}"
    echo ""
    echo "Continuando com demonstração teórica..."
    DEMO_MODE="theory"
else
    echo -e "${GREEN}✅ Variáveis Slack configuradas${NC}"
    DEMO_MODE="live"
fi

echo ""
echo -e "${BLUE}📋 Passo 2: Criar Slack App (se ainda não criou)${NC}"
echo ""

if [ "$DEMO_MODE" == "theory" ]; then
    echo "1. Acesse: https://api.slack.com/apps"
    echo "2. Clique 'Create New App'"
    echo "3. Escolha 'From scratch'"
    echo "4. Nome: 'LogLine Director'"
    echo "5. Workspace: Escolha seu workspace"
    echo ""
    
    echo -e "${BLUE}📋 Passo 3: Configurar Bot Scopes${NC}"
    echo ""
    echo "Em 'OAuth & Permissions', adicione scopes:"
    echo "   • chat:write - Enviar mensagens"
    echo "   • chat:write.public - Mensagens em canais públicos"
    echo "   • commands - Slash commands"
    echo "   • app_mentions:read - Ler mentions"
    echo "   • channels:history - Ler histórico canais"
    echo ""
    
    echo -e "${BLUE}📋 Passo 4: Criar Slash Commands${NC}"
    echo ""
    echo "Em 'Slash Commands', crie:"
    echo ""
    echo "   /director"
    echo "   → Request URL: https://your-domain.com/slack/commands"
    echo "   → Description: Consultar agente IA sobre HIV"
    echo "   → Usage Hint: [sua pergunta sobre HIV]"
    echo ""
    echo "   /hiv-status"
    echo "   → Request URL: https://your-domain.com/slack/commands"
    echo "   → Description: Status das simulações HIV"
    echo ""
    echo "   /knowledge-search"
    echo "   → Request URL: https://your-domain.com/slack/commands"
    echo "   → Description: Buscar na base de conhecimento"
    echo "   → Usage Hint: [termo de busca]"
    echo ""
    echo "   /analyze-protein"
    echo "   → Request URL: https://your-domain.com/slack/commands"
    echo "   → Description: Analisar proteína específica"
    echo "   → Usage Hint: [gp41|gp120|Rev|Tat]"
    echo ""
    
    echo -e "${BLUE}📋 Passo 5: Instalar no Workspace${NC}"
    echo ""
    echo "1. Em 'Install App', clique 'Install to Workspace'"
    echo "2. Autorize as permissões"
    echo "3. Copie o 'Bot User OAuth Token' (começa com xoxb-)"
    echo "4. Em 'Basic Information', copie 'Signing Secret'"
    echo ""
    
    echo -e "${BLUE}📋 Passo 6: Configurar .env${NC}"
    echo ""
    echo "Adicione ao arquivo .env:"
    echo ""
    echo "SLACK_BOT_TOKEN=xoxb-..."
    echo "SLACK_SIGNING_SECRET=..."
    echo ""
fi

echo -e "${BLUE}📋 Passo 7: Iniciar Bot${NC}"
echo ""

if [ "$DEMO_MODE" == "live" ]; then
    # Verificar se director está buildado
    if [ ! -f "./target/release/director" ]; then
        echo -e "${YELLOW}⚠️  Director não encontrado. Buildando...${NC}"
        cargo build --release -p director
    fi
    
    echo -e "${GREEN}🚀 Iniciando Slack bot...${NC}"
    echo ""
    
    # Iniciar bot
    ./target/release/director --mode slack &
    BOT_PID=$!
    
    # Aguardar bot iniciar
    sleep 3
    
    echo -e "${GREEN}✅ Bot rodando!${NC}"
    echo ""
    
    echo -e "${BLUE}📋 Passo 8: Testar no Slack${NC}"
    echo ""
    
    echo "No seu workspace Slack, teste:"
    echo ""
    echo "1. ${GREEN}/director${NC} Como funciona a proteína gp41?"
    echo "   → Resposta completa com contexto científico"
    echo ""
    echo "2. ${GREEN}/hiv-status${NC}"
    echo "   → Status atual das simulações HIV"
    echo ""
    echo "3. ${GREEN}/knowledge-search${NC} mecanismos de fusão"
    echo "   → Busca documentos relevantes na base"
    echo ""
    echo "4. ${GREEN}/analyze-protein${NC} gp41"
    echo "   → Análise detalhada da proteína"
    echo ""
    echo "5. ${GREEN}@director${NC} gerar manuscrito sobre gp120"
    echo "   → Mention para geração de manuscrito"
    echo ""
    
    echo -e "${YELLOW}⏳ Bot está rodando...${NC}"
    echo ""
    echo "Pressione Ctrl+C para parar o bot"
    echo ""
    
    # Função de cleanup
    cleanup() {
        echo ""
        echo -e "${BLUE}🛑 Parando bot...${NC}"
        kill $BOT_PID 2>/dev/null || true
        echo -e "${GREEN}✅ Bot parado${NC}"
        exit 0
    }
    
    # Registrar cleanup
    trap cleanup SIGINT SIGTERM
    
    # Aguardar indefinidamente (até Ctrl+C)
    wait $BOT_PID
    
    cleanup
else
    echo "Comando para iniciar bot (quando configurado):"
    echo ""
    echo "  ${GREEN}./target/release/director --mode slack${NC}"
    echo ""
fi

echo -e "${BLUE}📋 Exemplos de Uso${NC}"
echo ""

echo "💬 Conversação natural:"
echo "   User: ${YELLOW}/director Qual a diferença entre gp41 e gp120?${NC}"
echo "   Bot: 🤖 A gp41 e gp120 são proteínas complementares do HIV..."
echo "       [resposta detalhada com fontes científicas]"
echo ""

echo "📊 Monitoramento:"
echo "   User: ${YELLOW}/hiv-status${NC}"
echo "   Bot: 📊 Status atual das simulações HIV:"
echo "       ✅ gp41: Estável (RMSD: 3.2 Å)"
echo "       ⚠️ gp120: Em análise (75% completo)"
echo "       ✅ Rev: Completo"
echo "       ✅ Tat: Estável"
echo ""

echo "🔍 Busca de conhecimento:"
echo "   User: ${YELLOW}/knowledge-search inibidores de fusão${NC}"
echo "   Bot: 📚 Encontrados 5 documentos sobre 'inibidores de fusão':"
echo "       1. [0.95] HIV Fusion Inhibitors (2023)"
echo "       2. [0.89] gp41 Targeting Strategies (2022)"
echo "       ..."
echo ""

echo "🧬 Análise de proteína:"
echo "   User: ${YELLOW}/analyze-protein gp41${NC}"
echo "   Bot: 🔬 Analisando gp41..."
echo "       📊 RMSD: 3.2 Å (✅ Estável)"
echo "       📊 Energia: -135.4 kcal/mol (✅ Favorável)"
echo "       ✅ Sem instabilidades detectadas"
echo ""

echo -e "${BLUE}📚 Recursos Adicionais${NC}"
echo ""
echo "• Documentação completa: docs/slack_setup.md"
echo "• Troubleshooting: docs/slack_troubleshooting.md"
echo "• Exemplos avançados: examples/slack_advanced.md"
echo ""

echo -e "${GREEN}✅ Demo completo!${NC}"
