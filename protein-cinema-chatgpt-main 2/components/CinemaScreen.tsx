// components/CinemaScreen.tsx
'use client'

import { useEffect, useState } from 'react'
import { ProteinTheater } from './ProteinTheater'
import { ManifestoView } from './ManifestoView'
import { SessionReplay } from './SessionReplay'
import { AnalysisDashboard } from './AnalysisDashboard'
import type { SessionData } from '../lib/types'
import { THINKING_ANIMATION_INTERVAL_MS } from '../lib/constants'

type TabId = 'simulation' | 'analysis' | 'replay' | 'manifesto'

interface CinemaScreenProps {
  activeTab: TabId
  sessionData: SessionData | null
  isThinking: boolean
}

export function CinemaScreen({ activeTab, sessionData, isThinking }: CinemaScreenProps) {
  const [thinkingEmoji, setThinkingEmoji] = useState<string>('🧬')

  // Efeitos visuais durante "pensamento"
  useEffect(() => {
    if (isThinking) {
      const effects = ['🧬', '⚗️', '🔍', '📊', '🎯']
      let i = 0
      const interval = setInterval(() => {
        setThinkingEmoji(effects[i % effects.length])
        i++
      }, THINKING_ANIMATION_INTERVAL_MS)
      return () => clearInterval(interval)
    }
  }, [isThinking])

  const renderContent = () => {
    if (isThinking) {
      return (
        <div className="h-full flex items-center justify-center flex-col">
          <div className="text-6xl mb-4 animate-bounce">
            {thinkingEmoji}
          </div>
          <div className="text-xl text-gray-200 animate-pulse">
            LogLine Bio está realizando a simulação...
          </div>
          <div className="text-sm text-gray-400 mt-2">
            Cada passo está sendo auditado e registrado
          </div>
        </div>
      )
    }

    switch (activeTab) {
      case 'simulation':
        return <ProteinTheater data={sessionData} />
      case 'manifesto':
        return <ManifestoView data={sessionData} />
      case 'replay':
        return <SessionReplay data={sessionData} />
      case 'analysis':
        return <AnalysisDashboard data={sessionData} />
      default:
        return (
          <div className="h-full flex items-center justify-center text-gray-300">
            <div className="text-center">
              <div className="text-6xl mb-4">🎭</div>
              <div>O Teatro da Descoberta aguarda sua consulta</div>
            </div>
          </div>
        )
    }
  }

  return (
    <div className="h-full bg-black relative overflow-hidden">
      
      {/* Header Cinematográfico */}
      <div className="absolute top-0 left-0 right-0 z-10 p-4 bg-gradient-to-b from-black/80 to-transparent">
        <div className="flex justify-between items-center">
          <div className="flex items-center space-x-3">
            <div className="w-2 h-2 bg-red-500 rounded-full animate-pulse"></div>
            <div className="text-sm font-mono">SESSÃO ATIVA</div>
          </div>
          <div className="text-xs text-gray-400 font-mono">
            {sessionData?.sessionId ? `#${sessionData.sessionId}` : 'INICIANDO...'}
          </div>
        </div>
      </div>
      
      {/* Conteúdo Principal */}
      <div className="h-full pt-16 pb-4">
        {renderContent()}
      </div>
      
      {/* Efeitos Visuais */}
      <div className="absolute inset-0 pointer-events-none">
        <div className="absolute top-0 left-0 w-1 h-full bg-gradient-to-b from-cyan-500/20 to-transparent"></div>
        <div className="absolute top-0 right-0 w-1 h-full bg-gradient-to-b from-blue-500/20 to-transparent"></div>
      </div>
    </div>
  )
}
