import { useState, useEffect, useCallback } from 'react'
import CodeMirror from '@uiw/react-codemirror'
import { StreamLanguage } from '@codemirror/language'
import { lua } from '@codemirror/legacy-modes/mode/lua'
import { type Agent, fetchAgents, createAgent, updateAgent, deleteAgent } from '../api/agents'

interface AgentEditorProps {
    gameId: number
    isLoggedIn: boolean
}

export function AgentEditor({ gameId, isLoggedIn }: AgentEditorProps) {
    const [agents, setAgents] = useState<Agent[]>([])
    const [selectedAgent, setSelectedAgent] = useState<Agent | null>(null)
    const [code, setCode] = useState('')
    const [name, setName] = useState('')
    const [isCreating, setIsCreating] = useState(false)
    const [loading, setLoading] = useState(true)
    const [saving, setSaving] = useState(false)
    const [error, setError] = useState<string | null>(null)

    // Load agents when component mounts or gameId changes
    useEffect(() => {
        if (!isLoggedIn) {
            setLoading(false)
            return
        }
        loadAgents()
    }, [gameId, isLoggedIn])

    const loadAgents = async () => {
        setLoading(true)
        setError(null)
        try {
            const data = await fetchAgents(gameId)
            setAgents(data)
            // Select first agent if available
            if (data.length > 0 && !selectedAgent) {
                selectAgent(data[0])
            }
        } catch (err) {
            setError(err instanceof Error ? err.message : 'Failed to load agents')
        } finally {
            setLoading(false)
        }
    }

    const selectAgent = (agent: Agent) => {
        setSelectedAgent(agent)
        setCode(agent.code)
        setName(agent.name)
        setIsCreating(false)
        setError(null)
    }

    const startCreating = () => {
        setSelectedAgent(null)
        setCode('')
        setName('')
        setIsCreating(true)
        setError(null)
    }

    const onCodeChange = useCallback((value: string) => {
        setCode(value)
    }, [])

    const handleSave = async () => {
        if (!name.trim()) {
            setError('Agent name is required')
            return
        }

        setSaving(true)
        setError(null)

        try {
            if (isCreating) {
                const newAgent = await createAgent({
                    game_id: gameId,
                    name: name.trim(),
                    code,
                })
                setAgents([...agents, newAgent])
                selectAgent(newAgent)
            } else if (selectedAgent) {
                const updated = await updateAgent(selectedAgent.id, {
                    name: name.trim(),
                    code,
                })
                setAgents(agents.map(a => a.id === updated.id ? updated : a))
                setSelectedAgent(updated)
            }
        } catch (err) {
            setError(err instanceof Error ? err.message : 'Failed to save agent')
        } finally {
            setSaving(false)
        }
    }

    const handleDelete = async () => {
        if (!selectedAgent) return
        if (!confirm(`Delete agent "${selectedAgent.name}"?`)) return

        setSaving(true)
        setError(null)

        try {
            await deleteAgent(selectedAgent.id)
            const remaining = agents.filter(a => a.id !== selectedAgent.id)
            setAgents(remaining)
            if (remaining.length > 0) {
                selectAgent(remaining[0])
            } else {
                setSelectedAgent(null)
                setCode('')
                setName('')
            }
        } catch (err) {
            setError(err instanceof Error ? err.message : 'Failed to delete agent')
        } finally {
            setSaving(false)
        }
    }

    if (!isLoggedIn) {
        return (
            <div className="rounded-lg border border-slate-700 bg-slate-800/50 p-4">
                <p className="text-sm text-slate-400">Log in to create and manage your AI agents.</p>
            </div>
        )
    }

    if (loading) {
        return (
            <div className="rounded-lg border border-slate-700 bg-slate-800/50 p-4">
                <p className="text-sm text-slate-400">Loading agents...</p>
            </div>
        )
    }

    return (
        <div className="flex h-full flex-col rounded-lg border border-slate-700 bg-slate-800/50">
            {/* Header */}
            <div className="flex items-center justify-between border-b border-slate-700 px-4 py-3">
                <h2 className="text-lg font-semibold text-white">Your Agents</h2>
                <button
                    onClick={startCreating}
                    className="rounded bg-indigo-600 px-3 py-1.5 text-sm font-medium text-white hover:bg-indigo-500"
                >
                    + New Agent
                </button>
            </div>

            <div className="flex flex-1 overflow-hidden">
                {/* Agent List Sidebar */}
                <div className="w-48 flex-shrink-0 overflow-y-auto border-r border-slate-700">
                    {agents.length === 0 && !isCreating ? (
                        <p className="p-4 text-sm text-slate-400">No agents yet</p>
                    ) : (
                        <ul>
                            {agents.map(agent => (
                                <li key={agent.id}>
                                    <button
                                        onClick={() => selectAgent(agent)}
                                        className={`w-full px-4 py-2 text-left text-sm transition ${selectedAgent?.id === agent.id && !isCreating
                                            ? 'bg-indigo-600 text-white'
                                            : 'text-slate-300 hover:bg-slate-700'
                                            }`}
                                    >
                                        <div className="font-medium truncate">{agent.name}</div>
                                    </button>
                                </li>
                            ))}
                        </ul>
                    )}
                </div>

                {/* Editor Panel */}
                <div className="flex min-h-0 flex-1 flex-col overflow-y-auto p-4">
                    {selectedAgent || isCreating ? (
                        <>
                            {/* Name Input */}
                            <div className="mb-3 flex-shrink-0">
                                <label className="mb-1 block text-sm font-medium text-slate-300">
                                    Agent Name
                                </label>
                                <input
                                    type="text"
                                    value={name}
                                    onChange={e => setName(e.target.value)}
                                    placeholder="My Awesome Agent"
                                    className="w-full rounded border border-slate-600 bg-slate-700 px-3 py-2 text-white placeholder-slate-400 focus:border-indigo-500 focus:outline-none"
                                />
                            </div>

                            {/* Code Editor */}
                            <div className="mb-3 h-64 flex-shrink-0 overflow-hidden rounded border border-slate-600">
                                <label className="block bg-slate-800 px-3 py-2 text-sm font-medium text-slate-300 border-b border-slate-600">
                                    Lua Code
                                </label>
                                <CodeMirror
                                    value={code}
                                    onChange={onCodeChange}
                                    extensions={[StreamLanguage.define(lua)]}
                                    theme="dark"
                                    placeholder="-- Write your AI logic here"
                                    height="100%"
                                    style={{ height: 'calc(100% - 2.5rem)' }}
                                    basicSetup={{
                                        lineNumbers: true,
                                        highlightActiveLineGutter: true,
                                        highlightActiveLine: true,
                                        foldGutter: true,
                                        bracketMatching: true,
                                        closeBrackets: true,
                                        autocompletion: false,
                                        indentOnInput: true,
                                    }}
                                />
                            </div>

                            {/* Error Message */}
                            {error && (
                                <div className="mb-3 flex-shrink-0 rounded border border-red-600 bg-red-600/10 px-3 py-2 text-sm text-red-400">
                                    {error}
                                </div>
                            )}

                            {/* Action Buttons */}
                            <div className="flex flex-shrink-0 items-center gap-2">
                                <button
                                    onClick={handleSave}
                                    disabled={saving}
                                    className="rounded bg-green-600 px-4 py-2 text-sm font-medium text-white hover:bg-green-500 disabled:opacity-50"
                                >
                                    {saving ? 'Saving...' : isCreating ? 'Create Agent' : 'Save Changes'}
                                </button>
                                {!isCreating && selectedAgent && (
                                    <button
                                        onClick={handleDelete}
                                        disabled={saving}
                                        className="rounded bg-red-600 px-4 py-2 text-sm font-medium text-white hover:bg-red-500 disabled:opacity-50"
                                    >
                                        Delete
                                    </button>
                                )}
                            </div>
                        </>
                    ) : (
                        <div className="flex flex-1 items-center justify-center text-slate-400">
                            <p>Select an agent or create a new one</p>
                        </div>
                    )}
                </div>
            </div>
        </div>
    )
}
