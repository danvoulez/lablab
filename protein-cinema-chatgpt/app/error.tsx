'use client'

import { useEffect } from 'react'

export default function Error({
  error,
  reset,
}: {
  error: Error & { digest?: string }
  reset: () => void
}) {
  useEffect(() => {
    console.error('Application error:', error)
  }, [error])

  return (
    <div className="min-h-screen bg-black text-white flex items-center justify-center p-6">
      <div className="max-w-md text-center">
        <div className="text-6xl mb-4">⚠️</div>
        <h1 className="text-2xl font-bold mb-4">Algo deu errado</h1>
        <p className="text-gray-400 mb-6">
          Ocorreu um erro inesperado. Por favor, tente novamente.
        </p>
        <button
          onClick={reset}
          className="px-6 py-3 bg-cyan-600 text-white rounded-lg hover:bg-cyan-700 transition-colors"
        >
          Tentar novamente
        </button>
        {error.digest && (
          <p className="mt-4 text-xs text-gray-600">
            ID do erro: {error.digest}
          </p>
        )}
      </div>
    </div>
  )
}
