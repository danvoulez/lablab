#!/usr/bin/env bash
# 🚀 LogLine Discovery Lab - Main Demo Script
# Quick demonstration of all major features

set -euo pipefail

# Cores para output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BOLD='\033[1m'
NC='\033[0m' # No Color

clear

echo -e "${BOLD}╔════════════════════════════════════════════════════╗${NC}"
echo -e "${BOLD}║     🧬 LogLine Discovery Lab - Quick Demo         ║${NC}"
echo -e "${BOLD}║  AI-Driven HIV Drug Discovery Platform            ║${NC}"
echo -e "${BOLD}╚════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${BLUE}🎯 O que é o LogLine Discovery Lab?${NC}"
echo ""
echo "Laboratório farmacêutico computacional focado em descoberta"
echo "de medicamentos para HIV usando IA conversacional."
echo ""
echo -e "${GREEN}✨ Features principais:${NC}"
echo "   🤖 Agente IA em português brasileiro"
echo "   🧬 Análise de proteínas (gp41, gp120, Rev, Tat)"
echo "   📊 Dashboard web interativo"
echo "   📱 Integração Slack profissional"
echo "   📝 Geração automática de manuscritos"
echo ""

echo -e "${BLUE}📋 Demos disponíveis:${NC}"
echo ""
echo "   1. ${YELLOW}Análise Básica gp41${NC} - Análise de folding molecular"
echo "   2. ${YELLOW}Dashboard Web${NC} - Interface científica visual"
echo "   3. ${YELLOW}Slack Integration${NC} - Bot conversacional"
echo "   4. ${YELLOW}Manuscript Generation${NC} - Papers automáticos"
echo "   5. ${YELLOW}Rodar todos${NC} - Demonstração completa"
echo "   q. Sair"
echo ""

read -p "Escolha uma opção (1-5, q): " choice
echo ""

case $choice in
    1)
        echo -e "${GREEN}🧬 Iniciando demo: Análise gp41${NC}"
        echo ""
        ./examples/01_gp41_basic.sh
        ;;
    2)
        echo -e "${GREEN}📊 Iniciando demo: Dashboard Web${NC}"
        echo ""
        ./examples/02_dashboard_demo.sh
        ;;
    3)
        echo -e "${GREEN}📱 Iniciando demo: Slack Integration${NC}"
        echo ""
        ./examples/03_slack_integration.sh
        ;;
    4)
        echo -e "${GREEN}📝 Iniciando demo: Manuscript Generation${NC}"
        echo ""
        ./examples/04_manuscript_generation.sh
        ;;
    5)
        echo -e "${GREEN}🚀 Rodando todos os demos!${NC}"
        echo ""
        
        echo -e "${BLUE}Demo 1/4: Análise gp41${NC}"
        ./examples/01_gp41_basic.sh
        echo ""
        read -p "Pressione Enter para próximo demo..." dummy
        
        echo -e "${BLUE}Demo 2/4: Dashboard Web${NC}"
        echo "Pulando (requer interação)..."
        echo ""
        read -p "Pressione Enter para próximo demo..." dummy
        
        echo -e "${BLUE}Demo 3/4: Slack Integration${NC}"
        ./examples/03_slack_integration.sh
        echo ""
        read -p "Pressione Enter para próximo demo..." dummy
        
        echo -e "${BLUE}Demo 4/4: Manuscript Generation${NC}"
        ./examples/04_manuscript_generation.sh
        echo ""
        
        echo -e "${GREEN}✅ Todos os demos completados!${NC}"
        ;;
    q|Q)
        echo -e "${BLUE}👋 Até logo!${NC}"
        exit 0
        ;;
    *)
        echo -e "${RED}❌ Opção inválida${NC}"
        exit 1
        ;;
esac

echo ""
echo -e "${BLUE}📚 Próximos passos:${NC}"
echo ""
echo "   📖 Leia o Getting Started: GETTING_STARTED.md"
echo "   🎯 Consulte o Roadmap: STRATEGIC_ROADMAP.md"
echo "   📋 Veja o Action Plan: ACTION_PLAN.md"
echo "   ⭐ Dê uma star no GitHub!"
echo ""
echo -e "${GREEN}🎉 Obrigado por experimentar o LogLine Discovery Lab!${NC}"
