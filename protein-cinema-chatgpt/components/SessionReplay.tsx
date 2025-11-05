// components/SessionReplay.tsx
'use client'

import { useMemo, useState } from 'react'
import type { SessionData } from '../lib/types'

export function SessionReplay({ data }: { data?: SessionData | null }) {
  const [idx, setIdx] = useState(0)

  const steps = useMemo(() => data?.auditTrail ?? [], [data?.auditTrail])
  const current = useMemo(() => steps[idx] ?? 'Aguardando sessão…', [idx, steps])

  if (!steps.length) {
    return (
      <div className="h-full flex items-center justify-center text-gray-300">
        Nenhum evento ainda. Rode uma simulação para habilitar o replay.
      </div>
    )
  }

  return (
    <div className="h-full flex flex-col p-4">
      <div className="text-sm text-gray-300 mb-3">
        Etapa <span className="font-mono">{idx + 1}</span>/<span className="font-mono">{steps.length}</span>
      </div>
      <div className="flex-1 bg-gray-900/40 rounded-lg border border-gray-800 p-4">
        <div className="text-2xl">🎞️</div>
        <div className="mt-2 text-gray-100">{current}</div>
      </div>
      <input
        type="range"
        min={0}
        max={steps.length - 1}
        value={idx}
        onChange={(e) => setIdx(Number(e.target.value))}
        className="mt-4 w-full accent-cyan-500"
        aria-label="Controle de replay"
      />
    </div>
  )
}
