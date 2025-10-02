#!/bin/bash
# Script para configurar o Slack App com o manifest

echo "📱 LogLine Director - Slack App Setup"
echo "===================================="
echo ""

# Obter URL atual do ngrok
NGROK_URL=$(curl -s http://localhost:4040/api/tunnels | jq -r '.tunnels[0].public_url' 2>/dev/null)

if [ -z "$NGROK_URL" ] || [ "$NGROK_URL" = "null" ]; then
    echo "❌ ngrok não está rodando ou não foi encontrado"
    echo ""
    echo "Inicie o ngrok primeiro:"
    echo "ngrok http 3002"
    exit 1
fi

echo "🌐 URL do ngrok detectada: $NGROK_URL"
echo ""

# Atualizar o manifest com a URL correta
sed "s|https://your-ngrok-url.ngrok-free.app|$NGROK_URL|g" slack-app-manifest.json > slack-app-manifest-updated.json

echo "✅ Manifest atualizado com sua URL do ngrok"
echo ""
echo "📋 PRÓXIMOS PASSOS:"
echo ""
echo "1. Vá para https://api.slack.com/apps"
echo "2. Clique em 'Create New App'"
echo "3. Escolha 'From an app manifest'"
echo "4. Selecione seu workspace"
echo "5. Cole o conteúdo do arquivo: slack-app-manifest-updated.json"
echo ""
echo "📄 Conteúdo do manifest atualizado:"
echo "=================================="
cat slack-app-manifest-updated.json
echo ""
echo "=================================="
echo ""
echo "6. Após criar o app, vá em 'OAuth & Permissions'"
echo "7. Clique em 'Install to Workspace'"
echo "8. Copie o 'Bot User OAuth Token' (xoxb-...)"
echo "9. Configure: export SLACK_BOT_TOKEN=seu-token"
echo "10. Execute: ./start_slack_bot.sh"
echo ""
echo "🎯 URLs configuradas no manifest:"
echo "   Events: $NGROK_URL/slack/events"
echo "   Slash Commands: $NGROK_URL/slack/slash"
echo "   Interactive: $NGROK_URL/slack/interactive"
