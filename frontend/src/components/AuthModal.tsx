import { useState, FormEvent } from 'react'
import { useAuth } from '../context/useAuth'

interface AuthModalProps {
    isOpen: boolean
    onClose: () => void
}

export function AuthModal({ isOpen, onClose }: AuthModalProps) {
    const { login, register } = useAuth()
    const [mode, setMode] = useState<'login' | 'register'>('login')
    const [username, setUsername] = useState('')
    const [password, setPassword] = useState('')
    const [error, setError] = useState<string | null>(null)
    const [loading, setLoading] = useState(false)

    if (!isOpen) return null

    const handleSubmit = async (e: FormEvent) => {
        e.preventDefault()
        setError(null)
        setLoading(true)

        try {
            if (mode === 'login') {
                await login(username, password)
            } else {
                await register(username, password)
            }
            onClose()
            setUsername('')
            setPassword('')
        } catch (err) {
            setError(err instanceof Error ? err.message : 'An error occurred')
        } finally {
            setLoading(false)
        }
    }

    const switchMode = () => {
        setMode(mode === 'login' ? 'register' : 'login')
        setError(null)
    }

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center">
            {/* Backdrop */}
            <div
                className="absolute inset-0 bg-black/60 backdrop-blur-sm"
                onClick={onClose}
            />

            {/* Modal */}
            <div className="relative z-10 w-full max-w-md rounded-2xl border border-slate-700 bg-slate-900 p-6 shadow-xl">
                <button
                    onClick={onClose}
                    className="absolute right-4 top-4 text-slate-400 hover:text-white"
                    aria-label="Close"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        className="h-5 w-5"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                    >
                        <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M6 18L18 6M6 6l12 12"
                        />
                    </svg>
                </button>

                <h2 className="mb-6 text-2xl font-bold text-white">
                    {mode === 'login' ? 'Welcome back' : 'Create account'}
                </h2>

                <form onSubmit={handleSubmit} className="space-y-4">
                    <div>
                        <label
                            htmlFor="username"
                            className="mb-1 block text-sm font-medium text-slate-300"
                        >
                            Username
                        </label>
                        <input
                            id="username"
                            type="text"
                            value={username}
                            onChange={(e) => setUsername(e.target.value)}
                            className="w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2 text-white placeholder-slate-500 focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
                            placeholder="Enter your username"
                            required
                            autoComplete="username"
                        />
                    </div>

                    <div>
                        <label
                            htmlFor="password"
                            className="mb-1 block text-sm font-medium text-slate-300"
                        >
                            Password
                        </label>
                        <input
                            id="password"
                            type="password"
                            value={password}
                            onChange={(e) => setPassword(e.target.value)}
                            className="w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2 text-white placeholder-slate-500 focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
                            placeholder="Enter your password"
                            required
                            autoComplete={mode === 'login' ? 'current-password' : 'new-password'}
                        />
                        {mode === 'register' && (
                            <p className="mt-1 text-xs text-slate-400">
                                At least 10 characters with letters, numbers, and symbols
                            </p>
                        )}
                    </div>

                    {error && (
                        <div className="rounded-lg bg-red-900/50 px-4 py-2 text-sm text-red-300">
                            {error}
                        </div>
                    )}

                    <button
                        type="submit"
                        disabled={loading}
                        className="w-full rounded-lg bg-indigo-500 px-4 py-2 font-semibold text-white hover:bg-indigo-400 disabled:cursor-not-allowed disabled:opacity-50"
                    >
                        {loading
                            ? 'Please wait...'
                            : mode === 'login'
                                ? 'Login'
                                : 'Create account'}
                    </button>
                </form>

                <div className="mt-6 text-center text-sm text-slate-400">
                    {mode === 'login' ? (
                        <>
                            Don't have an account?{' '}
                            <button
                                onClick={switchMode}
                                className="font-medium text-indigo-400 hover:text-indigo-300"
                            >
                                Register
                            </button>
                        </>
                    ) : (
                        <>
                            Already have an account?{' '}
                            <button
                                onClick={switchMode}
                                className="font-medium text-indigo-400 hover:text-indigo-300"
                            >
                                Login
                            </button>
                        </>
                    )}
                </div>
            </div>
        </div>
    )
}
