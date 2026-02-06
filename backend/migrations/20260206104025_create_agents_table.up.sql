-- Agents table: stores user AI scripts for games with versioning
CREATE TABLE agents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    game_id INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    code TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    -- Each user can have multiple agents per game, but names must be unique per user+game
    UNIQUE(user_id, game_id, name)
);

-- Index for fast lookups by user and game
CREATE INDEX idx_agents_user_game ON agents(user_id, game_id);
