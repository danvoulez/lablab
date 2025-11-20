export default function Loading() {
  return (
    <div className="min-h-screen bg-black text-white flex items-center justify-center">
      <div className="text-center">
        <div className="text-6xl mb-4 animate-bounce">🧬</div>
        <div className="text-xl text-gray-400 animate-pulse">
          Carregando ProteinTrace...
        </div>
      </div>
    </div>
  )
}
