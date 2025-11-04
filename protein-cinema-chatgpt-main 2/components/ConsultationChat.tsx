// components/ConsultationChat.tsx
'use client'

import { useEffect, useRef, useState } from 'react'
import type { SessionData, Message } from '../lib/types'
import { niceId, samplePLDDT, signManifesto, sha256Base64 } from '../lib/manifest'
import { sanitizeMarkdown, sanitizeUserInput } from '../lib/sanitize'
import { generateUUID } from '../lib/crypto-utils'
import { SIMULATION_DELAY_MS } from '../lib/constants'

const INITIAL_MESSAGE: Message = {
  id: 'm0',
  type: 'assistant',
  content:
    'Bem-vindo ao laboratório. Envie uma sequência FASTA ou descreva sua hipótese (ex: "simular mutação G12D").',
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

    // "Simulação" fake com dados plausíveis
    const sessionId = niceId()
    const startedAt = new Date().toISOString()
    const plddt = samplePLDDT(120)

    // Gerar PDB mínimo (1CRN pequeno) – suficiente para visualizar no 3Dmol
    const pdb = `ATOM      1  N   THR A   1      26.017  24.447   2.842  1.00 50.00           N
ATOM      2  CA  THR A   1      25.948  23.027   2.451  1.00 50.00           C
ATOM      3  C   THR A   1      24.505  22.574   2.106  1.00 50.00           C
ATOM      4  O   THR A   1      24.226  21.375   2.118  1.00 50.00           O
ATOM      5  N   THR A   2      23.615  23.504   1.771  1.00 50.00           N
ATOM      6  CA  THR A   2      22.211  23.185   1.444  1.00 50.00           C
ATOM      7  C   THR A   2      21.327  23.229   2.697  1.00 50.00           C
ATOM      8  O   THR A   2      21.767  23.650   3.774  1.00 50.00           O
ATOM      9  N   CYS A   3      20.079  22.806   2.546  1.00 50.00           N
ATOM     10  CA  CYS A   3      19.123  22.788   3.657  1.00 50.00           C
ATOM     11  C   CYS A   3      17.670  22.896   3.185  1.00 50.00           C
ATOM     12  O   CYS A   3      17.396  22.852   1.985  1.00 50.00           O
TER`

    // Hash dos principais outputs
    const structureHash = await sha256Base64(pdb + JSON.stringify(plddt))
    const auditTrail = [
      'Sessão iniciada',
      'Entrada validada',
      'Predição estrutural (mock)',
      'Cálculo pLDDT',
      'Geração de manifesto'
    ]

    const baseSession: SessionData = {
      sessionId,
      startedAt,
      inputSequence: trimmed.startsWith('>') ? trimmed : undefined,
      structureHash,
      confidence: { overall: Math.round(plddt.reduce((a, b) => a + b, 0) / plddt.length) },
      plddt,
      auditTrail,
      models: [{ name: 'Model-1', format: 'pdb', content: pdb }]
    }

    // Assinatura inicial do "manifesto"
    const manifestoPayload = {
      sessionId,
      timestamp: startedAt,
      scientificQuestion:
        'Investigação da relação estrutura-função por predição computacional (demo)',
      methodology: [
        'Predição mock estilo AlphaFold',
        'Perfil pLDDT por resíduo',
        'Hash de artefatos & trilha de auditoria'
      ],
      conclusions: [
        'Pipeline executado com integridade computável',
        'Artefatos hashados e prontos para reprodutibilidade'
      ],
      findings: [{ title: 'Modelo 3D', description: 'Estrutura PDB gerada (demo)', evidence: structureHash }]
    }
    const signature = await signManifesto(manifestoPayload)

    const session: SessionData = {
      ...baseSession,
      manifesto: {
        ...manifestoPayload,
        participants: ['Usuário Pesquisador', 'LogLine Bio (Assistente)'],
        digitalSignature: signature,
        auditTrail
      }
    }

    // Simula tempo de computação com "cinema thinking"
    setTimeout(() => {
      setMessages((m) => [
        ...m,
        {
          id: generateUUID(),
          type: 'assistant',
          content:
            'Simulação concluída. Ative as abas **Análise**, **Replay** ou **Manifesto** para explorar os resultados.',
          timestamp: new Date().toISOString()
        }
      ])
      onSessionUpdate(session)
      onThinkingState(false)
    }, SIMULATION_DELAY_MS)
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
