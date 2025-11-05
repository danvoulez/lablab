// components/ConsultationChat.tsx
'use client'

import { useEffect, useRef, useState } from 'react'
import type { SessionData, Message } from '../lib/types'
import { sanitizeMarkdown, sanitizeUserInput } from '../lib/sanitize'
import { generateUUID } from '../lib/crypto-utils'
import { simulateProtein, ApiError } from '../lib/apiClient'

const INITIAL_MESSAGE: Message = {
  id: 'm0',
  type: 'assistant',
  content:
    '🧬 Bem-vindo ao LogLine Discovery Lab! Sou o Director, seu assistente científico. Envie uma sequência FASTA ou descreva sua hipótese (ex: "simular mutação G12D").',
  timestamp: new Date().toISOString()
}

export function ConsultationChat({
  onSessionUpdate,
  onThinkingState
}: {
  onSessionUpdate: (s: SessionData) => void
  onThinkingState: (f: boolean) => void
}) {
  const [messages, setMessages] = useState<Message[]>([INITIAL_MESSAGE])
  const [input, setInput] = useState('')
  const listRef = useRef<HTMLDivElement>(null)
  const inputRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    listRef.current?.scrollTo({ top: listRef.current.scrollHeight, behavior: 'smooth' })
  }, [messages])

  // Focus input after messages change
  useEffect(() => {
    if (inputRef.current && messages.length > 1) {
      // Small delay to ensure DOM updates
      setTimeout(() => inputRef.current?.focus(), 100)
    }
  }, [messages])

  async function handleSend() {
    let trimmed: string
    try {
      trimmed = sanitizeUserInput(input)
    } catch (error) {
      // Show error to user if input is too long
      const errorMsg: Message = {
        id: generateUUID(),
        type: 'assistant',
        content: error instanceof Error ? error.message : 'Erro ao validar entrada',
        timestamp: new Date().toISOString()
      }
      setMessages((m) => [...m, errorMsg])
      return
    }
    
    if (!trimmed) return
    
    const userMsg: Message = {
      id: generateUUID(),
      type: 'user',
      content: trimmed,
      timestamp: new Date().toISOString()
    }
    setMessages((m) => [...m, userMsg])
    setInput('')
    onThinkingState(true)

    try {
      // Call backend API to simulate protein
      const session = await simulateProtein(trimmed)
      
      // Update session with real backend data
      onSessionUpdate(session)
      
      // Add success message
      setMessages((m) => [
        ...m,
        {
          id: generateUUID(),
          type: 'assistant',
          content: `✅ Simulação concluída com sucesso! **Session ID**: ${session.sessionId}\n\n` +
                   `📊 Confiança média: **${session.confidence?.overall.toFixed(1)}%**\n\n` +
                   `Ative as abas **Análise**, **Replay** ou **Manifesto** para explorar os resultados em detalhes.`,
          timestamp: new Date().toISOString()
        }
      ])
    } catch (error) {
      // Handle errors gracefully
      let errorMessage = 'Erro ao processar simulação.'
      
      if (error instanceof ApiError) {
        if (error.status === 503) {
          errorMessage = '⚠️ O backend não está disponível. Certifique-se de que o Director está rodando.'
        } else {
          errorMessage = `❌ Erro na API: ${error.message}`
        }
      } else if (error instanceof Error) {
        errorMessage = `❌ Erro: ${error.message}`
      }
      
      setMessages((m) => [
        ...m,
        {
          id: generateUUID(),
          type: 'assistant',
          content: errorMessage,
          timestamp: new Date().toISOString()
        }
      ])
    } finally {
      onThinkingState(false)
    }
  }

  return (
    <div className="h-full flex flex-col">
      <div
        ref={listRef}
        className="flex-1 overflow-y-auto px-4 py-3 space-y-3 bg-black/40"
        aria-live="polite"
      >
        {messages.map((m) => (
          <div key={m.id} className={`flex ${m.type === 'user' ? 'justify-end' : 'justify-start'}`}>
            <div
              className={`max-w-[80%] rounded-2xl px-3 py-2 text-sm ${
                m.type === 'user' ? 'bg-cyan-600 text-white rounded-br-none' : 'bg-gray-800 text-gray-100 rounded-bl-none'
              }`}
              dangerouslySetInnerHTML={{
                __html: sanitizeMarkdown(m.content)
              }}
            />
          </div>
        ))}
      </div>

      <form
        className="p-3 border-t border-gray-800 bg-black/60 flex items-center gap-2"
        onSubmit={(e) => {
          e.preventDefault()
          handleSend()
        }}
        aria-label="Enviar mensagem"
      >
        <input
          ref={inputRef}
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder="Descreva a hipótese ou cole FASTA..."
          className="flex-1 bg-gray-900 text-gray-100 rounded-xl px-3 py-2 outline-none border border-gray-700 focus:border-cyan-500 focus:ring-2 focus:ring-cyan-500/50"
          aria-label="Campo de entrada do chat"
        />
        <button
          type="submit"
          className="px-4 py-2 rounded-xl bg-cyan-600 text-white disabled:opacity-50"
          disabled={!input.trim()}
        >
          Enviar
        </button>
      </form>
    </div>
  )
}
