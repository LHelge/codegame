import { useEffect, useState, useRef } from 'react'
import { useParams, Link } from 'react-router-dom'
import { fetchGame, type Game } from '../api/games'
import { AgentEditor } from '../components/AgentEditor'
import { useAuth } from '../context/useAuth'

// Track globally loaded WASM modules to prevent duplicate loads
const loadedWasmModules = new Set<string>()

export function GamePage() {
    const { name } = useParams<{ name: string }>()
    const { user } = useAuth()
    const [game, setGame] = useState<Game | null>(null)
    const [loading, setLoading] = useState(true)
    const [error, setError] = useState<string | null>(null)
    const [gameLoaded, setGameLoaded] = useState(false)
    const [wasmLoading, setWasmLoading] = useState(false)
    const [needsRefresh, setNeedsRefresh] = useState(false)
    const scriptRef = useRef<HTMLScriptElement | null>(null)

    useEffect(() => {
        if (!name) return
        fetchGame(name)
            .then((fetchedGame) => {
                setGame(fetchedGame)
                // Check if WASM was already loaded in a previous visit
                // Bevy games can't reinitialize on a new canvas, so a refresh is needed
                if (loadedWasmModules.has(fetchedGame.name)) {
                    setNeedsRefresh(true)
                }
            })
            .catch((err) => setError(err.message))
            .finally(() => setLoading(false))
    }, [name])

    // Cleanup script tag on unmount
    useEffect(() => {
        return () => {
            if (scriptRef.current) {
                scriptRef.current.remove()
                scriptRef.current = null
            }
        }
    }, [])

    const loadWasm = async () => {
        if (!game) return

        // Check if already loaded globally
        if (loadedWasmModules.has(game.name)) {
            setGameLoaded(true)
            return
        }

        setWasmLoading(true)
        setError(null)

        // Remove any existing script from previous attempts
        if (scriptRef.current) {
            scriptRef.current.remove()
        }

        // Load the WASM module by adding a script tag (Vite doesn't allow dynamic imports from /public)
        const script = document.createElement('script')
        script.type = 'module'
        script.id = `wasm-loader-${game.name}`
        script.textContent = `
            (async () => {
                try {
                    const init = (await import('/wasm/${game.name}/${game.name}.js')).default;
                    await init();
                    window.dispatchEvent(new CustomEvent('wasm-loaded', { detail: { game: '${game.name}' } }));
                } catch (err) {
                    // Ignore wasm-bindgen control flow exception (not a real error)
                    const message = err.message || '';
                    if (message.includes("Using exceptions for control flow")) {
                        window.dispatchEvent(new CustomEvent('wasm-loaded', { detail: { game: '${game.name}' } }));
                    } else {
                        window.dispatchEvent(new CustomEvent('wasm-error', { detail: { error: message || 'Failed to load game' } }));
                    }
                }
            })();
        `
        scriptRef.current = script

        const handleLoaded = (e: CustomEvent<{ game: string }>) => {
            if (e.detail.game === game.name) {
                loadedWasmModules.add(game.name)
                setGameLoaded(true)
                setWasmLoading(false)
                cleanup()
            }
        }

        const handleError = (e: CustomEvent<{ error: string }>) => {
            setError(e.detail.error)
            setWasmLoading(false)
            cleanup()
        }

        const cleanup = () => {
            window.removeEventListener('wasm-loaded', handleLoaded as EventListener)
            window.removeEventListener('wasm-error', handleError as EventListener)
        }

        window.addEventListener('wasm-loaded', handleLoaded as EventListener)
        window.addEventListener('wasm-error', handleError as EventListener)

        script.onerror = () => {
            setError('Failed to load game script')
            setWasmLoading(false)
            cleanup()
        }

        document.body.appendChild(script)
    }

    if (loading) {
        return (
            <div className="flex items-center justify-center p-8">
                <p className="text-slate-400">Loading game...</p>
            </div>
        )
    }

    if (!game) {
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
        <div className="mx-auto max-w-6xl px-6 py-8">
            <div className="mb-6 flex items-center gap-4">
                <Link to="/games" className="text-slate-400 hover:text-white">
                    ← Games
                </Link>
                <h1 className="text-3xl font-bold text-white">{game.display_name}</h1>
            </div>

            {error && (
                <div className="mb-4 rounded-lg bg-red-900/50 px-4 py-3 text-red-300">
                    {error}
                </div>
            )}

            {needsRefresh && (
                <div className="mb-4 flex items-center gap-4 rounded-lg bg-amber-900/50 px-4 py-3 text-amber-300">
                    <span>Game was previously loaded. Refresh the page to play again.</span>
                    <button
                        onClick={() => window.location.reload()}
                        className="rounded bg-amber-600 px-3 py-1 text-sm font-semibold text-white hover:bg-amber-500"
                    >
                        Refresh
                    </button>
                </div>
            )}

            <div className="space-y-6">
                {/* Game Canvas Section */}
                <div>
                    {!gameLoaded && !needsRefresh && (
                        <button
                            onClick={loadWasm}
                            disabled={wasmLoading}
                            className="mb-4 rounded-lg bg-green-600 px-4 py-2 text-sm font-semibold text-white hover:bg-green-500 disabled:cursor-not-allowed disabled:opacity-50"
                        >
                            {wasmLoading ? 'Loading...' : 'Load Game'}
                        </button>
                    )}

                    <canvas
                        id={`${game.name}-canvas`}
                        className="aspect-video w-full rounded-lg bg-black"
                    />
                </div>

                {/* Agent Editor Section */}
                <div className="h-[400px]">
                    <AgentEditor gameId={game.id} isLoggedIn={!!user} />
                </div>
            </div>
        </div>
    )
}
