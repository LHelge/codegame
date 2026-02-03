import { useState, useEffect, useCallback, type ReactNode } from 'react'
import { getMe, login as apiLogin, register as apiRegister, logout as apiLogout, type User } from '../api/auth'
import { AuthContext } from './AuthContextType'

export function AuthProvider({ children }: { children: ReactNode }) {
    const [user, setUser] = useState<User | null>(null)
    const [loading, setLoading] = useState(true)

    useEffect(() => {
        getMe()
            .then(setUser)
            .finally(() => setLoading(false))
    }, [])

    const login = useCallback(async (username: string, password: string) => {
        const user = await apiLogin(username, password)
        setUser(user)
    }, [])

    const register = useCallback(async (username: string, password: string) => {
        const user = await apiRegister(username, password)
        setUser(user)
    }, [])

    const logout = useCallback(async () => {
        await apiLogout()
        setUser(null)
    }, [])

    return (
        <AuthContext.Provider value={{ user, loading, login, register, logout }}>
            {children}
        </AuthContext.Provider>
    )
}
