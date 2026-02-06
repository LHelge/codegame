-- Create games table
CREATE TABLE games (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL
);

-- Insert initial games
INSERT INTO games (name, display_name) VALUES ('robotsumo', 'Robot Sumo');
INSERT INTO games (name, display_name) VALUES ('snake', 'Snake');
