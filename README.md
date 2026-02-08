# codegame

A teaching-friendly platform where kids can program game AI logic in Lua and pit different versions against each other. The platform uses a Rust backend (Axum) with SQLite, a React + Tailwind frontend, and game runtimes written in Rust (Bevy) compiled to WebAssembly.

## Goals
- Make learning game AI approachable and fun for kids.
- Allow quick iteration on Lua-based AI strategies.
- Provide simple tooling to run AI-vs-AI matches.

## Planned Features
- Lua sandbox for AI scripts
- Match runner to evaluate different AI versions
- Simple scoring and replay summary
- Web UI to manage AIs and run matches
- Game runtime in Rust/Bevy compiled to WASM

## Quick Start with Docker

Create a `docker-compose.yml` file:

```yaml
services:
  backend:
    image: ghcr.io/lhelge/codegame-backend:latest
    environment:
      - RUST_LOG=info
      - DATABASE_URL=sqlite:/app/data/codegame.db
      - JWT_SECRET=change-this-in-production
    volumes:
      - backend-data:/app/data
    expose:
      - "3000"
    restart: unless-stopped

  frontend:
    image: ghcr.io/lhelge/codegame-frontend:latest
    ports:
      - "80:80"
    depends_on:
      - backend
    restart: unless-stopped

volumes:
  backend-data:
```

Then run:

```bash
docker compose up -d
```

Open http://localhost in your browser.

## Local Development

### Prerequisites
- Rust toolchain (1.93+)
- Node.js (22+)
- wasm-pack (for building games)

### Backend

```bash
cd backend
cp .env.example .env  # First time only
sqlx database create  # First time only
sqlx migrate run      # First time or after new migrations
cargo run
```

The backend must be run from inside the `backend/` directory. The API runs at http://localhost:3000

### Frontend

```bash
cd frontend
npm install
npm run dev
```

The dev server runs at http://localhost:5173

### Building Games (WASM)

```bash
# Install prerequisites (first time only)
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

# Build all games
./scripts/build-games.sh
```

The script builds all games and copies them to `frontend/public/wasm/`.

### Running Games Natively

You can run any game directly on your desktop for quick testing. The `native`
feature enables windowing support (X11):

```bash
cd games
cargo run -p snake --features native
cargo run -p robotsumo --features native
```

The `native` feature is not included by default so that CI and WASM builds
don't require system windowing libraries.

## Project Structure
```
/backend       # Rust Axum API
/frontend      # React app styled with Tailwind
/games         # Rust/Bevy games (WASM builds)
  /robotsumo   # Robot sumo game
  /snake       # Snake game
/scripts       # Build and utility scripts
/ai            # Lua scripts and templates
/docs          # Learning materials and guides
```

## Contributing
Contributions and feedback are welcome. Please open an issue or PR.

## License
See [LICENSE](LICENSE).
