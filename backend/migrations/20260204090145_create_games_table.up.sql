-- Create games table
CREATE TABLE games (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    wasm_filename TEXT NOT NULL
);

-- Insert initial games
INSERT INTO games (name, wasm_filename) VALUES ('robotsumo', 'robotsumo');
INSERT INTO games (name, wasm_filename) VALUES ('snake', 'snake');
