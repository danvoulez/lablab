// components/ManifestoView.tsx
'use client'

import type { SessionData } from '../lib/types'

export function ManifestoView({ data }: { data: SessionData | null }) {
  if (!data?.manifesto) {
    return (
      <div className="h-full flex items-center justify-center text-gray-300">
        <div className="text-center space-y-2">
          <div className="text-4xl">📝</div>
          <div>Execute uma simulação para gerar o manifesto científico.</div>
        </div>
      </div>
    )
  }

  const manifesto = data.manifesto

  return (
    <div className="h-full overflow-y-auto p-6 bg-gradient-to-br from-gray-900 to-black">
      <div className="max-w-4xl mx-auto">
        
        {/* Cabeçalho Épico */}
        <div className="text-center mb-8 border-b border-gray-700 pb-8">
          <div className="text-4xl font-serif mb-2 text-amber-200">MANIFESTO CIENTÍFICO</div>
          <div className="text-gray-400 font-mono text-sm">Sessão #{manifesto.sessionId}</div>
        </div>
        
        {/* Metadados da Sessão */}
        <div className="grid grid-cols-2 gap-6 mb-8 text-sm">
          <div>
            <div className="text-gray-500">Data/Hora</div>
            <div className="text-white">{new Date(manifesto.timestamp).toLocaleString()}</div>
          </div>
          <div>
            <div className="text-gray-500">Participantes</div>
            <div className="text-white">{manifesto.participants.join(' • ')}</div>
          </div>
        </div>
        
        {/* Questão Científica */}
        <div className="mb-8 p-6 bg-gray-800/50 rounded-lg border-l-4 border-cyan-500">
          <div className="text-gray-400 text-sm mb-2">QUESTÃO INVESTIGADA</div>
          <div className="text-white text-lg italic">&ldquo;{manifesto.scientificQuestion}&rdquo;</div>
        </div>
        
        {/* Metodologia */}
        <div className="mb-8">
          <div className="text-gray-400 text-sm mb-3">METODOLOGIA APLICADA</div>
          <div className="space-y-2">
            {manifesto.methodology.map((method, index) => (
              <div key={index} className="flex items-start space-x-3 text-white">
                <div className="text-cyan-400 mt-1">▸</div>
                <div>{method}</div>
              </div>
            ))}
          </div>
        </div>
        
        {/* Descobertas */}
        <div className="mb-8">
          <div className="text-gray-400 text-sm mb-3">DESCOBERTAS E OBSERVAÇÕES</div>
          <div className="space-y-4">
            {manifesto.findings.map((finding, index) => (
              <div key={index} className="p-4 bg-gray-800/30 rounded border border-gray-700">
                <div className="text-amber-200 font-semibold mb-2">{finding.title}</div>
                <div className="text-white text-sm">{finding.description}</div>
                {finding.evidence && (
                  <div className="mt-2 text-xs text-gray-400 font-mono">
                    Evidência: {finding.evidence}
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>
        
        {/* Conclusões */}
        <div className="mb-8 p-6 bg-green-900/20 rounded-lg border border-green-700/30">
          <div className="text-gray-400 text-sm mb-3">CONCLUSÕES E RECOMENDAÇÕES</div>
          <div className="space-y-2 text-white">
            {manifesto.conclusions.map((conclusion, index) => (
              <div key={index} className="flex items-start space-x-3">
                <div className="text-green-400 mt-1">●</div>
                <div>{conclusion}</div>
              </div>
            ))}
          </div>
        </div>
        
        {/* Assinatura Digital */}
        <div className="text-center border-t border-gray-700 pt-8">
          <div className="text-gray-400 text-sm mb-2">ASSINATURA DIGITAL DO PROCESSO</div>
          <div className="font-mono text-xs text-gray-500 break-all mb-4">
            {manifesto.digitalSignature}
          </div>
          <div className="text-gray-400 text-xs">
            Este manifesto representa um registro auditável e reproduzível da investigação científica realizada.
            Todos os passos foram documentados no ledger universal LogLine.
          </div>
        </div>
      </div>
    </div>
  )
}
