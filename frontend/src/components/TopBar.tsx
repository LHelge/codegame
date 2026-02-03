import { useAuth } from '../context/useAuth'

interface TopBarProps {
    onLoginClick: () => void
}

export function TopBar({ onLoginClick }: TopBarProps) {
    const { user, loading, logout } = useAuth()

    return (
        <header className="mx-auto flex w-full max-w-5xl items-center justify-between px-6 py-4">
            <div className="text-lg font-semibold text-white">codegame</div>
            <div className="flex items-center gap-4">
                <div className="rounded-full bg-slate-800 px-3 py-1 text-sm text-slate-300">
                    robotsumo
                </div>
                {loading ? (
                    <div className="h-8 w-20 animate-pulse rounded bg-slate-700" />
                ) : user ? (
                    <div className="flex items-center gap-3">
                        <span className="text-sm text-slate-300">
                            {user.username}
                            {user.admin && (
                                <span className="ml-1 text-xs text-amber-400">(admin)</span>
                            )}
                        </span>
                        <button
                            onClick={logout}
                            className="rounded-lg border border-slate-700 px-3 py-1.5 text-sm text-slate-300 hover:border-slate-500 hover:text-white"
                        >
                            Logout
                        </button>
                    </div>
                ) : (
                    <button
                        onClick={onLoginClick}
                        className="rounded-lg bg-indigo-500 px-4 py-1.5 text-sm font-semibold text-white hover:bg-indigo-400"
                    >
                        Login
                    </button>
                )}
            </div>
        </header>
    )
}
