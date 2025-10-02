#!/bin/bash
# Script para iniciar o Director Slack Bot

echo "🚀 Iniciando LogLine Director Slack Bot..."
echo ""
echo "📱 URLs para configurar no Slack:"
echo "   Event Subscriptions: https://2294e4e8cc25.ngrok-free.app/slack/events"
echo "   Slash Commands: https://2294e4e8cc25.ngrok-free.app/slack/slash"
echo "   Interactive Components: https://2294e4e8cc25.ngrok-free.app/slack/interactive"
echo ""
echo "🔑 Configure sua SLACK_BOT_TOKEN como variável de ambiente:"
echo "   export SLACK_BOT_TOKEN=your-slack-bot-token-here"
echo ""

# Verificar se o token está configurado
if [ -z "$SLACK_BOT_TOKEN" ]; then
    echo "❌ SLACK_BOT_TOKEN não configurado!"
    echo ""
    echo "Configure assim:"
    echo "export SLACK_BOT_TOKEN=your-slack-bot-token-here"
    echo "./start_slack_bot.sh"
    exit 1
fi

echo "✅ Token configurado, iniciando bot..."
RUST_LOG=info ./target/debug/director slack --port 3002 --token "$SLACK_BOT_TOKEN"
