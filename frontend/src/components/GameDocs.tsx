import { useState } from 'react'

interface GameDocsProps {
    gameName: string
}

// Game-specific API documentation
const gameApiDocs: Record<string, { title: string; description: string; functions: { name: string; description: string; example?: string }[] }> = {
    robotsumo: {
        title: 'Robot Sumo API',
        description: 'Control your robot in a sumo-style battle. Push your opponent out of the ring to win!',
        functions: [
            { name: 'move_forward()', description: 'Move the robot forward', example: 'move_forward()' },
            { name: 'move_backward()', description: 'Move the robot backward', example: 'move_backward()' },
            { name: 'turn_left()', description: 'Rotate the robot left', example: 'turn_left()' },
            { name: 'turn_right()', description: 'Rotate the robot right', example: 'turn_right()' },
            { name: 'get_position()', description: 'Get your robot\'s current x, y position', example: 'local x, y = get_position()' },
            { name: 'get_opponent_position()', description: 'Get the opponent\'s x, y position', example: 'local ox, oy = get_opponent_position()' },
            { name: 'get_distance_to_opponent()', description: 'Get distance to opponent', example: 'local dist = get_distance_to_opponent()' },
            { name: 'get_distance_to_edge()', description: 'Get distance to nearest ring edge', example: 'local edge = get_distance_to_edge()' },
        ],
    },
    snake: {
        title: 'Snake API',
        description: 'Control your snake to eat food and grow longer. Avoid walls and your own tail!',
        functions: [
            { name: 'turn_left()', description: 'Turn the snake left', example: 'turn_left()' },
            { name: 'turn_right()', description: 'Turn the snake right', example: 'turn_right()' },
            { name: 'get_head_position()', description: 'Get the snake head position', example: 'local x, y = get_head_position()' },
            { name: 'get_food_position()', description: 'Get the food position', example: 'local fx, fy = get_food_position()' },
            { name: 'get_direction()', description: 'Get current direction (up, down, left, right)', example: 'local dir = get_direction()' },
            { name: 'get_length()', description: 'Get current snake length', example: 'local len = get_length()' },
        ],
    },
}

// General Lua documentation
const luaDocs = {
    title: 'Lua Quick Reference',
    sections: [
        {
            name: 'Variables',
            content: `-- Local variables (recommended)
local x = 10
local name = "player"
local active = true

-- Multiple assignment
local a, b = 1, 2`,
        },
        {
            name: 'Conditionals',
            content: `if score > 100 then
    print("High score!")
elseif score > 50 then
    print("Good job!")
else
    print("Keep trying!")
end`,
        },
        {
            name: 'Loops',
            content: `-- For loop
for i = 1, 10 do
    print(i)
end

-- While loop
while health > 0 do
    fight()
end`,
        },
        {
            name: 'Functions',
            content: `-- Define a function
function attack(target)
    local damage = calculate_damage()
    return damage
end

-- Call it
local result = attack(enemy)`,
        },
        {
            name: 'Math',
            content: `-- Basic math
local sum = 5 + 3
local diff = 10 - 4
local product = 6 * 7
local quotient = 20 / 4

-- Math library
local dist = math.sqrt(x*x + y*y)
local angle = math.atan2(y, x)
local rounded = math.floor(3.7)  -- 3
local absolute = math.abs(-5)    -- 5`,
        },
        {
            name: 'Comparison',
            content: `-- Operators
a == b  -- equal
a ~= b  -- not equal
a < b   -- less than
a > b   -- greater than
a <= b  -- less or equal
a >= b  -- greater or equal

-- Logical
a and b  -- both true
a or b   -- either true
not a    -- negation`,
        },
    ],
}

export function GameDocs({ gameName }: GameDocsProps) {
    const [showGameDocs, setShowGameDocs] = useState(false)
    const [showLuaDocs, setShowLuaDocs] = useState(false)

    return (
        <div className="space-y-6">
            {/* Game API Docs */}
            <div className="rounded-lg border border-slate-700 bg-slate-800/50">
                <button
                    onClick={() => setShowGameDocs(!showGameDocs)}
                    className="flex w-full items-center justify-between px-4 py-3 text-left text-sm font-medium text-white hover:bg-slate-700/50"
                >
                    <span>📖 {gameApiDocs[gameName]?.title || 'Game API'}</span>
                    <span className="text-slate-500">{showGameDocs ? '▼' : '▶'}</span>
                </button>
                {showGameDocs && gameApiDocs[gameName] && (
                    <div className="border-t border-slate-700 px-4 py-3">
                        <p className="mb-3 text-sm text-slate-400">{gameApiDocs[gameName].description}</p>
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
                            {gameApiDocs[gameName].functions.map((fn, idx) => (
                                <div key={idx} className="rounded bg-slate-900 px-3 py-2">
                                    <code className="text-sm font-mono text-indigo-400">{fn.name}</code>
                                    <p className="mt-0.5 text-xs text-slate-400">{fn.description}</p>
                                    {fn.example && (
                                        <pre className="mt-1 text-xs text-slate-500 font-mono">{fn.example}</pre>
                                    )}
                                </div>
                            ))}
                        </div>
                    </div>
                )}
                {showGameDocs && !gameApiDocs[gameName] && (
                    <div className="border-t border-slate-700 px-4 py-3">
                        <p className="text-sm text-slate-400">No API documentation available for this game yet.</p>
                    </div>
                )}
            </div>

            {/* Lua Docs */}
            <div className="rounded-lg border border-slate-700 bg-slate-800/50">
                <button
                    onClick={() => setShowLuaDocs(!showLuaDocs)}
                    className="flex w-full items-center justify-between px-4 py-3 text-left text-sm font-medium text-white hover:bg-slate-700/50"
                >
                    <span>🌙 {luaDocs.title}</span>
                    <span className="text-slate-500">{showLuaDocs ? '▼' : '▶'}</span>
                </button>
                {showLuaDocs && (
                    <div className="border-t border-slate-700 px-4 py-3">
                        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
                            {luaDocs.sections.map((section, idx) => (
                                <div key={idx} className="rounded bg-slate-900 p-3">
                                    <h4 className="mb-2 text-xs font-semibold text-indigo-400">{section.name}</h4>
                                    <pre className="text-xs text-slate-300 font-mono whitespace-pre-wrap overflow-x-auto">{section.content}</pre>
                                </div>
                            ))}
                        </div>
                    </div>
                )}
            </div>
        </div>
    )
}
