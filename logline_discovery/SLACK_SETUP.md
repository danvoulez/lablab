# 📱 LogLine Director - Guia de Configuração Slack

## 🚀 Passo a Passo Completo

### 1. **Obter sua URL do ngrok**

```bash
# Inicie o ngrok (em um terminal separado)
ngrok http 3002

# Copie a URL que aparece (algo como: https://abc123.ngrok-free.app)
```

### 2. **Configurar o Manifest**

No arquivo `slack-app-manifest.json`, substitua **TODAS** as ocorrências de:
```
SUBSTITUA_PELA_SUA_URL_NGROK
```

Por sua URL real do ngrok, exemplo:
```
https://abc123.ngrok-free.app
```

**Locais para substituir (3 lugares):**
- Linha 21: `/slack/slash` (para slash commands)
- Linha 80: `/slack/events` (para eventos)
- Linha 93: `/slack/interactive` (para componentes interativos)

### 3. **Criar o Slack App**

1. Vá para: https://api.slack.com/apps
2. Clique **"Create New App"**
3. Escolha **"From an app manifest"**
4. Selecione seu workspace
5. Cole o conteúdo do `slack-app-manifest.json` (já editado)
6. Clique **"Create"**

### 4. **Instalar o App**

1. Vá em **"OAuth & Permissions"**
2. Clique **"Install to Workspace"**
3. Autorize as permissões
4. **Copie o "Bot User OAuth Token"** (começa com `xoxb-`)

### 5. **Iniciar o Director**

```bash
# Configure o token
export SLACK_BOT_TOKEN=xoxb-seu-token-aqui

# Inicie o bot
cd /Users/voulezvous/logline_discovery_lab/logline_discovery
./start_slack_bot.sh
```

### 6. **Testar no Slack**

```
/director como está a fila de processamento?
/lab-status
@Director preciso saber sobre as simulações de HIV
```

## ⚠️ Importante

- **ngrok deve estar rodando** na porta 3002
- **URLs no manifest** devem estar corretas
- **Token** deve ser o Bot User OAuth Token (não o User OAuth Token)
- **Director deve estar rodando** para responder aos webhooks

## 🔧 Troubleshooting

**"URL verification failed"**
→ Verifique se o Director está rodando na porta 3002

**"Invalid token"**
→ Use o Bot User OAuth Token (xoxb-...), não o User Token

**"Timeout"**
→ Verifique se a URL do ngrok está correta no manifest
