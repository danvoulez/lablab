#!/bin/bash
# LogLine Director - Environment Setup
# 
# Configure your API keys here and run: source setup_env.sh

echo "🔑 Setting up LogLine Director environment..."

# OpenAI API Key (configured)
export OPENAI_API_KEY="your-openai-api-key-here"

# Google Gemini API Key (configure with your key)
export GEMINI_API_KEY="your-gemini-api-key-here"

# Verify Ollama is running (local models)
if curl -s http://localhost:11434/api/tags > /dev/null; then
    echo "✅ Ollama is running (local models available)"
else
    echo "❌ Ollama not running - start with: ollama serve"
fi

# Check API keys
if [[ -n "$OPENAI_API_KEY" && "$OPENAI_API_KEY" != "your-openai-api-key-here" ]]; then
    echo "✅ OpenAI API key configured"
else
    echo "⚠️  OpenAI API key not configured"
fi

if [[ -n "$GEMINI_API_KEY" && "$GEMINI_API_KEY" != "your-gemini-key-here" ]]; then
    echo "✅ Gemini API key configured"
else
    echo "⚠️  Gemini API key not configured"
fi

echo ""
echo "🚀 Director will use this fallback order:"
echo "1. Rules (instant) → Known commands"
echo "2. Ollama Mistral 7B (local) → Complex language"  
echo "3. Ollama Llama3.2:1b (local) → Backup"
echo "4. OpenAI GPT (cloud) → Available ✅"
echo "5. Gemini (cloud) → Available ✅"
echo ""
echo "Run Director: ./target/debug/director --chat"
