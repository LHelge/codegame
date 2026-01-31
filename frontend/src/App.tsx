import { useEffect, useState } from 'react'

function App() {
  const [hello, setHello] = useState<string | null>(null)
  const [gameLoaded, setGameLoaded] = useState(false)

  useEffect(() => {
    fetch('/api/hello')
      .then((res) => res.text())
      .then(setHello)
      .catch(() => setHello('failed to load'))
  }, [])

  const loadGame = async () => {
    const wasmUrl = '/wasm/robotsumo.js'
    const module = await import(/* @vite-ignore */ wasmUrl)
    await module.default()
    setGameLoaded(true)
  }

  return (
    <div className="min-h-screen">
      <header className="mx-auto flex w-full max-w-5xl items-center justify-between px-6 py-8">
        <div className="text-lg font-semibold">{hello ?? 'loading...'}</div>
        <div className="rounded-full bg-slate-800 px-3 py-1 text-sm text-slate-300">
          robotsumo
        </div>
      </header>

      <main className="mx-auto grid w-full max-w-5xl gap-10 px-6 pb-16 md:grid-cols-[2fr_1fr]">
        <section className="space-y-6">
          <h1 className="text-4xl font-bold leading-tight text-white md:text-5xl">
            Teach game AI by coding Lua scripts.
          </h1>
          <p className="text-lg text-slate-300">
            Start small, experiment fast, and compare strategies in head-to-head matches.
          </p>
          <div className="flex flex-wrap gap-3">
            <button className="rounded-lg bg-indigo-500 px-4 py-2 text-sm font-semibold text-white hover:bg-indigo-400">
              Create AI
            </button>
            <button className="rounded-lg border border-slate-700 px-4 py-2 text-sm font-semibold text-slate-200 hover:border-slate-500">
              Run Match
            </button>
          </div>
          {!gameLoaded && (
            <button
              onClick={loadGame}
              className="rounded-lg bg-green-600 px-4 py-2 text-sm font-semibold text-white hover:bg-green-500"
            >
              Load Robot Sumo
            </button>
          )}
          <canvas id="robotsumo-canvas" className="mt-4 aspect-video w-full rounded-lg bg-black" />
        </section>

        <aside className="rounded-2xl border border-slate-800 bg-slate-900/60 p-6 shadow-lg">
          <h2 className="text-sm font-semibold uppercase tracking-wide text-slate-400">
            Next steps
          </h2>
          <ul className="mt-4 space-y-3 text-sm text-slate-300">
            <li>1. Write a Lua AI in the editor.</li>
            <li>2. Choose the robotsumo arena.</li>
            <li>3. Run a match and review results.</li>
          </ul>
        </aside>
      </main>
    </div>
  )
}

export default App
