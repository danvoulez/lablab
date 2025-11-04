'use client'

import { useState, useEffect } from 'react'
import { FloatingChat } from '../../../components/FloatingChat'
import { CinematicBackground } from '../../../components/CinematicBackground'
import { ConsultationMode } from '../../../components/ConsultationMode'
import { ErrorBoundary } from '../../../components/ErrorBoundary'
import { CHAT_SHOW_DELAY_MS } from '../../../lib/constants'

export default function LandingPage() {
  const [sessionStarted, setSessionStarted] = useState(false)
  const [showChat, setShowChat] = useState(false)

  // Efeito cinematográfico de entrada
  useEffect(() => {
    const timer = setTimeout(() => setShowChat(true), CHAT_SHOW_DELAY_MS)
    return () => clearTimeout(timer)
  }, [])

  const handleSessionStart = () => {
    setSessionStarted(true)
    // Transição suave para modo consulta
    document.documentElement.classList.add('session-active')
  }

  return (
    <ErrorBoundary>
      <div className={`min-h-screen transition-all duration-1000 ${
        sessionStarted ? 'bg-black' : 'bg-gradient-to-br from-blue-900 via-purple-900 to-black'
      }`}>
        
        {/* Background Cinematográfico */}
        <CinematicBackground active={!sessionStarted} />
        
        {/* Conteúdo Principal */}
        {!sessionStarted && (
          <div className="relative z-10 text-white text-center px-6 pt-20">
            {/* Logo Épica */}
            <div className="mb-8 animate-float">
              <div className="text-6xl md:text-8xl font-bold mb-4 bg-gradient-to-r from-cyan-400 to-blue-500 bg-clip-text text-transparent">
                ProteinTrace
              </div>
              <div className="text-xl md:text-2xl opacity-80">
                O Teatro da Descoberta Científica
              </div>
            </div>

            {/* Tagline Impactante */}
            <div className="max-w-2xl mx-auto mb-12">
              <p className="text-lg md:text-xl leading-relaxed opacity-90">
                Entre no laboratório virtual onde cada simulação é uma <span className="text-cyan-300">performance científica</span>, 
                e cada resultado um <span className="text-green-300">manifesto assinado</span> de evidência.
              </p>
            </div>
          </div>
        )}

        {/* Chat Flutuante */}
        {showChat && (
          <FloatingChat 
            onSessionStart={handleSessionStart}
            isVisible={!sessionStarted}
          />
        )}

        {/* Modo Consulta (ativa quando sessionStarted = true) */}
        {sessionStarted && (
          <ConsultationMode />
        )}
      </div>
    </ErrorBoundary>
  )
}
