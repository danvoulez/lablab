// components/FloatingChat.tsx
'use client'

import { useState, useRef, useEffect } from 'react'
import { sanitizeMarkdown } from '../lib/sanitize'
import { generateUUID } from '../lib/crypto-utils'
import { ASSISTANT_RESPONSE_DELAY_MS, SESSION_START_DELAY_MS } from '../lib/constants'
import type { Message } from '../lib/types'

const INITIAL_SUGGESTIONS = [
  "🧬 Quero prever uma estrutura proteica nova",
  "🔍 Simular o impacto de uma mutação CRISPR", 
  "📚 Entender como o AlphaFold funciona",
  "⚗️ Explorar exemplos de descobertas recentes",
  "🎓 Sou estudante, me guie pela ciência"
]

export function FloatingChat({ onSessionStart, isVisible }: { 
  onSessionStart: () => void, 
  isVisible: boolean 
}) {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: '1',
      type: 'assistant',
      content: "Olá! Sou o **LogLine Bio**, seu assistente no Teatro da Descoberta. Sou um biólogo computacional especializado em tornar a complexidade proteica acessível e fascinante. Como posso guiar sua exploração científica hoje?",
      timestamp: new Date().toISOString()
    }
  ])
  const [showSuggestions, setShowSuggestions] = useState(true)
  const messagesEndRef = useRef<HTMLDivElement>(null)

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" })
  }

  useEffect(() => {
    scrollToBottom()
  }, [messages])

  const handleSuggestionClick = async (suggestion: string) => {
    setShowSuggestions(false)
    
    // Adiciona mensagem do usuário
    const userMessage: Message = {
      id: generateUUID(),
      type: 'user',
      content: suggestion,
      timestamp: new Date().toISOString()
    }
    
    setMessages(prev => [...prev, userMessage])
    
    // Simula resposta do assistente
    setTimeout(() => {
      const response = generateAssistantResponse(suggestion)
      const assistantMessage: Message = {
        id: generateUUID(),
        type: 'assistant',
        content: response,
        timestamp: new Date().toISOString()
      }
      
      setMessages(prev => [...prev, assistantMessage])
      
      // Inicia sessão para certos tipos de consulta
      if (suggestion.includes('prever') || suggestion.includes('simular') || suggestion.includes('mutação')) {
        setTimeout(onSessionStart, SESSION_START_DELAY_MS)
      }
    }, ASSISTANT_RESPONSE_DELAY_MS)
  }

  const generateAssistantResponse = (userInput: string): string => {
    if (userInput.includes('prever') || userInput.includes('estrutura')) {
      return "Excelente! Vamos embarcar numa jornada de predição estrutural. Irei guiá-lo através do AlphaFold enquanto construímos um **manifesto auditável** completo. Esta sessão ficará registrada como uma consulta científica válida. Preparado para começar?"
    } else if (userInput.includes('CRISPR') || userInput.includes('mutação')) {
      return "Fascinante! A simulação de edições genéticas é onde a magia acontece. Vamos explorar como uma pequena alteração pode transformar uma proteína, criando um **registro validado** do impacto estrutural. Pronto para essa investigação?"
    } else if (userInput.includes('estudante') || userInput.includes('guie')) {
      return "Maravilhoso! Adoro iniciar novos cientistas nesta jornada. Vou explicar cada conceito como se estivéssemos num laboratório, transformando complexidade em clareza. Quer começar com uma demonstração prática?"
    }
    
    return "Perfeito! Como biólogo computacional, posso transformar sua curiosidade em descoberta concreta. Vamos criar uma sessão interativa onde cada passo será documentado como evidência científica. Por onde gostaria de começar?"
  }

  if (!isVisible) return null

  return (
    <div className="fixed bottom-6 left-1/2 transform -translate-x-1/2 w-full max-w-md px-4 z-50">
      <div className="bg-white/95 backdrop-blur-lg rounded-2xl shadow-2xl border border-white/20">
        
        {/* Header do Chat */}
        <div className="p-4 border-b border-gray-200 flex items-center space-x-3">
          <div className="w-3 h-3 bg-green-400 rounded-full animate-pulse"></div>
          <div className="flex-1">
            <div className="font-semibold text-gray-800">LogLine Bio</div>
            <div className="text-xs text-gray-500">Biólogo Computacional • Online</div>
          </div>
        </div>
        
        {/* Área de Mensagens */}
        <div className="h-64 overflow-y-auto p-4 space-y-4">
          {messages.map((message) => (
            <div
              key={message.id}
              className={`flex ${message.type === 'user' ? 'justify-end' : 'justify-start'}`}
            >
              <div
                className={`max-w-[80%] rounded-2xl px-4 py-2 ${
                  message.type === 'user'
                    ? 'bg-blue-500 text-white rounded-br-none'
                    : 'bg-gray-100 text-gray-800 rounded-bl-none'
                }`}
              >
                <div className="text-sm whitespace-pre-wrap" 
                     dangerouslySetInnerHTML={{ __html: sanitizeMarkdown(message.content) }} />
                <div className="text-xs opacity-60 mt-1">
                  {new Date(message.timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                </div>
              </div>
            </div>
          ))}
          <div ref={messagesEndRef} />
        </div>
        
        {/* Sugestões */}
        {showSuggestions && (
          <div className="p-4 border-t border-gray-200">
            <div className="text-xs text-gray-500 mb-2">Sugestões para começar:</div>
            <div className="space-y-2">
              {INITIAL_SUGGESTIONS.map((suggestion, index) => (
                <button
                  key={index}
                  onClick={() => handleSuggestionClick(suggestion)}
                  className="w-full text-left text-sm text-gray-700 hover:bg-gray-50 rounded-lg px-3 py-2 transition-colors"
                >
                  {suggestion}
                </button>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  )
}
