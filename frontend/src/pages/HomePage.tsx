import { Link } from 'react-router-dom'

export function HomePage() {
    return (
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
                    <Link
                        to="/games"
                        className="rounded-lg border border-slate-700 px-4 py-2 text-sm font-semibold text-slate-200 hover:border-slate-500"
                    >
                        Browse Games
                    </Link>
                </div>
            </section>

            <aside className="rounded-2xl border border-slate-800 bg-slate-900/60 p-6 shadow-lg">
                <h2 className="text-sm font-semibold uppercase tracking-wide text-slate-400">
                    Next steps
                </h2>
                <ul className="mt-4 space-y-3 text-sm text-slate-300">
                    <li>1. Write a Lua AI in the editor.</li>
                    <li>2. Choose a game arena.</li>
                    <li>3. Run a match and review results.</li>
                </ul>
            </aside>
        </main>
    )
}
