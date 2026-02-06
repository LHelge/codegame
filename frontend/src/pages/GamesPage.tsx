import { useEffect, useState } from 'react'
import { Link } from 'react-router-dom'
import { fetchGames, type Game } from '../api/games'

export function GamesPage() {
    const [games, setGames] = useState<Game[]>([])
    const [loading, setLoading] = useState(true)
    const [error, setError] = useState<string | null>(null)

    useEffect(() => {
        fetchGames()
            .then(setGames)
            .catch((err) => setError(err.message))
            .finally(() => setLoading(false))
    }, [])

    if (loading) {
        return (
            <div className="flex items-center justify-center p-8">
                <p className="text-slate-400">Loading games...</p>
            </div>
        )
    }

    if (error) {
        return (
            <div className="flex items-center justify-center p-8">
                <p className="text-red-400">Error: {error}</p>
            </div>
        )
    }

    return (
        <div className="mx-auto max-w-4xl px-6 py-8">
            <h1 className="mb-6 text-3xl font-bold text-white">Games</h1>
            <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {games.map((game) => (
                    <Link
                        key={game.id}
                        to={`/games/${game.name}`}
                        className="rounded-xl border border-slate-700 bg-slate-800/50 p-6 transition hover:border-indigo-500 hover:bg-slate-800"
                    >
                        <h2 className="text-xl font-semibold text-white">{game.display_name}</h2>
                        <p className="mt-2 text-sm text-slate-400">Click to play</p>
                    </Link>
                ))}
            </div>
        </div>
    )
}
