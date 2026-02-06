export interface Agent {
    id: number
    user_id: number
    game_id: number
    name: string
    code: string
    created_at: string
    updated_at: string
}

export interface CreateAgentRequest {
    game_id: number
    name: string
    code?: string
}

export interface UpdateAgentRequest {
    name?: string
    code?: string
}

export async function fetchAgents(gameId: number): Promise<Agent[]> {
    const response = await fetch(`/api/agents?game_id=${gameId}`, {
        credentials: 'include',
    })
    if (!response.ok) {
        if (response.status === 401) {
            return []
        }
        throw new Error('Failed to fetch agents')
    }
    return response.json()
}

export async function fetchAgent(id: number): Promise<Agent> {
    const response = await fetch(`/api/agents/${id}`, {
        credentials: 'include',
    })
    if (!response.ok) {
        throw new Error('Failed to fetch agent')
    }
    return response.json()
}

export async function createAgent(request: CreateAgentRequest): Promise<Agent> {
    const response = await fetch('/api/agents', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify(request),
    })
    if (!response.ok) {
        const text = await response.text()
        if (text.includes('UNIQUE constraint failed')) {
            throw new Error('An agent with that name already exists')
        }
        throw new Error('Failed to create agent')
    }
    return response.json()
}

export async function updateAgent(id: number, request: UpdateAgentRequest): Promise<Agent> {
    const response = await fetch(`/api/agents/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify(request),
    })
    if (!response.ok) {
        throw new Error('Failed to update agent')
    }
    return response.json()
}

export async function deleteAgent(id: number): Promise<void> {
    const response = await fetch(`/api/agents/${id}`, {
        method: 'DELETE',
        credentials: 'include',
    })
    if (!response.ok) {
        throw new Error('Failed to delete agent')
    }
}
