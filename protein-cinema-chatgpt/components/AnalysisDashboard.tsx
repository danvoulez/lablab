// components/AnalysisDashboard.tsx
'use client'

import { useMemo } from 'react'
import type { SessionData } from '../lib/types'
import { PLDDT_BINS, PLDDT_LABELS } from '../lib/constants'

function Stat({ label, value, suffix = '' }: { label: string; value: string | number; suffix?: string }) {
  return (
    <div className="p-3 bg-gray-900/40 rounded-lg border border-gray-800">
      <div className="text-xs text-gray-400">{label}</div>
      <div className="text-lg text-white">{value}{suffix}</div>
    </div>
  )
}

export function AnalysisDashboard({ data }: { data?: SessionData | null }) {
  const stats = useMemo(() => {
    const p = data?.plddt ?? []
    if (!p.length) return null
    const min = Math.min(...p)
    const max = Math.max(...p)
    const mean = Math.round(p.reduce((a, b) => a + b, 0) / p.length)
    const bins = PLDDT_BINS
    const hist = Array(bins.length - 1).fill(0)
    p.forEach((v) => {
      for (let i = 0; i < bins.length - 1; i++) {
        if (v >= bins[i] && v < bins[i + 1]) hist[i]++
      }
    })
    return { min, max, mean, hist, bins }
  }, [data?.plddt])

  if (!stats) {
    return (
      <div className="h-full flex items-center justify-center text-gray-300">
        Rode uma simulação para ver as métricas.
      </div>
    )
  }

  const total = (data?.plddt?.length ?? 1)
  const labels = PLDDT_LABELS

  return (
    <div className="h-full overflow-y-auto p-4 space-y-4">
      <div className="grid grid-cols-3 gap-3">
        <Stat label="pLDDT mín." value={stats.min} />
        <Stat label="pLDDT máx." value={stats.max} />
        <Stat label="pLDDT médio" value={stats.mean} />
      </div>

      <div className="p-4 bg-gray-900/40 rounded-lg border border-gray-800">
        <div className="text-sm text-gray-300 mb-3">Distribuição pLDDT</div>
        <div className="space-y-2">
          {stats.hist.map((count, i) => {
            const pct = Math.round((count / total) * 100)
            return (
              <div key={i} className="flex items-center gap-2">
                <div className="w-16 text-xs text-gray-400">{labels[i]}</div>
                <div className="flex-1 h-3 bg-gray-800 rounded">
                  <div
                    className="h-3 rounded bg-cyan-500 transition-all"
                    style={{ width: `${pct}%` }}
                    aria-label={`Faixa ${labels[i]}: ${pct}%`}
                  />
                </div>
                <div className="w-10 text-xs text-gray-300 text-right">{pct}%</div>
              </div>
            )
          })}
        </div>
      </div>

      <div className="text-xs text-gray-400">
        Artefato: <span className="font-mono">{data?.structureHash}</span>
      </div>
    </div>
  )
}
