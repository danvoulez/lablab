// components/ConsultationMode.tsx
'use client'

import { useState, useRef, useEffect, useMemo } from 'react'
import { ConsultationChat } from './ConsultationChat'
import { CinemaScreen } from './CinemaScreen'
import { BottomMenu } from './BottomMenu'
import type { SessionData } from '../lib/types'
import { MIN_SWIPE_DISTANCE } from '../lib/constants'

type TabId = 'simulation' | 'analysis' | 'replay' | 'manifesto'

export function ConsultationMode() {
  const [activeTab, setActiveTab] = useState<TabId>('simulation')
  const [sessionData, setSessionData] = useState<SessionData | null>(null)
  const [isThinking, setIsThinking] = useState(false)
  
  const touchStart = useRef<number | null>(null)
  const tabs: Array<typeof activeTab> = useMemo(() => ['simulation', 'analysis', 'replay', 'manifesto'], [])

  // Efeito de transição cinematográfica
  useEffect(() => {
    document.body.style.overflow = 'hidden'
    
    // Keyboard navigation
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'ArrowLeft') {
        e.preventDefault()
        const i = tabs.indexOf(activeTab)
        if (i > 0) setActiveTab(tabs[i - 1])
      } else if (e.key === 'ArrowRight') {
        e.preventDefault()
        const i = tabs.indexOf(activeTab)
        if (i < tabs.length - 1) setActiveTab(tabs[i + 1])
      } else if (e.key >= '1' && e.key <= '4') {
        // Number keys 1-4 to switch tabs directly
        e.preventDefault()
        const index = parseInt(e.key) - 1
        if (index < tabs.length) setActiveTab(tabs[index])
      }
    }
    
    window.addEventListener('keydown', handleKeyDown)
    
    return () => { 
      document.body.style.overflow = 'auto'
      window.removeEventListener('keydown', handleKeyDown)
    }
  }, [activeTab, tabs])

  function onTouchStart(e: React.TouchEvent) {
    touchStart.current = e.changedTouches[0].clientX
  }
  
  function onTouchEnd(e: React.TouchEvent) {
    if (touchStart.current == null) return
    const dx = e.changedTouches[0].clientX - touchStart.current
    if (Math.abs(dx) < MIN_SWIPE_DISTANCE) return
    const dir = dx < 0 ? 1 : -1
    const i = tabs.indexOf(activeTab)
    const next = Math.min(tabs.length - 1, Math.max(0, i + dir))
    setActiveTab(tabs[next])
    touchStart.current = null
  }

  return (
    <div className="h-screen flex flex-col bg-black text-white" onTouchStart={onTouchStart} onTouchEnd={onTouchEnd}>
      
      {/* Tela Cinema - grows to fill available space */}
      <div className="flex-[7] min-h-0 relative overflow-hidden">
        <CinemaScreen 
          activeTab={activeTab}
          sessionData={sessionData}
          isThinking={isThinking}
        />
      </div>
      
      {/* Barra Inferior - takes up 3/10 of space */}
      <div className="flex-[3] min-h-0 flex flex-col border-t border-gray-800">
        
        {/* Chat da Consulta */}
        <div className="flex-1 min-h-0 overflow-hidden">
          <ConsultationChat 
            onSessionUpdate={setSessionData}
            onThinkingState={setIsThinking}
          />
        </div>
        
        {/* Menu Inferior */}
        <BottomMenu 
          activeTab={activeTab}
          onTabChange={setActiveTab}
          sessionData={sessionData}
        />
      </div>
    </div>
  )
}
