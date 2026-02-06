export interface User {
    id: number
    username: string
    admin: boolean
}

export interface AuthError {
    message: string
}

export async function login(username: string, password: string): Promise<User> {
    const res = await fetch('/api/users/auth', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({ username, password }),
    })

    if (!res.ok) {
        if (res.status === 404) {
            throw new Error('Invalid username or password')
        }
        throw new Error('Login failed')
    }

    return res.json()
}

export async function register(username: string, password: string): Promise<User> {
    const res = await fetch('/api/users/register', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({ username, password }),
    })

    if (!res.ok) {
        const text = await res.text()
        if (text.includes('WeakPassword')) {
            throw new Error('Password must be at least 10 characters with letters, numbers, and symbols')
        }
        if (text.includes('UsernameTooShort')) {
            throw new Error('Username must be at least 3 characters')
        }
        if (text.includes('UsernameExists')) {
            throw new Error('Username already exists')
        }
        throw new Error('Registration failed')
    }

    return res.json()
}

export async function logout(): Promise<void> {
    await fetch('/api/users/logout', { method: 'POST', credentials: 'include' })
}

export async function getMe(): Promise<User | null> {
    const res = await fetch('/api/users/me', { credentials: 'include' })

    if (!res.ok) {
        return null
    }

    return res.json()
}
