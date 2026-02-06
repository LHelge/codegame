import { useEffect, useState } from 'react'
import { useParams, Link } from 'react-router-dom'
import { fetchGame, type Game } from '../api/games'

export function GamePage() {
    const { id } = useParams<{ id: string }>()
    const [game, setGame] = useState<Game | null>(null)
    const [loading, setLoading] = useState(true)
    const [error, setError] = useState<string | null>(null)
    const [gameLoaded, setGameLoaded] = useState(false)

    useEffect(() => {
        if (!id) return
        fetchGame(parseInt(id, 10))
            .then(setGame)
            .catch((err) => setError(err.message))
            .finally(() => setLoading(false))
    }, [id])

    const loadWasm = async () => {
        if (!game) return

        // Load the WASM module by adding a script tag (Vite doesn't allow dynamic imports from /public)
        const script = document.createElement('script')
        script.type = 'module'
        script.textContent = `
            import init from '/wasm/${game.wasm_filename}/${game.wasm_filename}.js';
            await init();
        `
        document.body.appendChild(script)
        setGameLoaded(true)
    }

    if (loading) {
        return (
            <div className="flex items-center justify-center p-8">
                <p className="text-slate-400">Loading game...</p>
            </div>
        )
    }

    if (error || !game) {
        return (
            <div className="flex flex-col items-center justify-center gap-4 p-8">
                <p className="text-red-400">Error: {error || 'Game not found'}</p>
                <Link to="/games" className="text-indigo-400 hover:text-indigo-300">
                    ← Back to games
                </Link>
            </div>
        )
    }

    return (
        <div className="mx-auto max-w-4xl px-6 py-8">
            <div className="mb-6 flex items-center gap-4">
                <Link to="/games" className="text-slate-400 hover:text-white">
                    ← Games
                </Link>
                <h1 className="text-3xl font-bold capitalize text-white">{game.name}</h1>
            </div>

            {!gameLoaded && (
                <button
                    onClick={loadWasm}
                    className="mb-4 rounded-lg bg-green-600 px-4 py-2 text-sm font-semibold text-white hover:bg-green-500"
                >
                    Load Game
                </button>
            )}

            <canvas
                id={`${game.wasm_filename}-canvas`}
                className="aspect-video w-full rounded-lg bg-black"
            />
        </div>
    )
}
