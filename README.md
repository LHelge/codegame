# codegame

A teaching-friendly platform where kids can program game AI logic in Lua and pit different versions against each other. The platform uses a Rust backend (Axum) with Postgres, a React + Tailwind frontend, and game runtimes written in Rust (Bevy) compiled to WebAssembly.

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
- Docker Compose stack for backend + Postgres

## Getting Started (Planned)
Once the project scaffold is in place, this section will include:
- Install prerequisites (Rust toolchain, Node.js)
- Run backend API (Axum) with Postgres via Docker Compose
- Run frontend (React + Tailwind)
- Build/run Bevy game (WASM)
- Write and test Lua AI scripts

## Project Structure (Planned)
```
/backend       # Rust Axum API and match services
/frontend      # React app styled with Tailwind
/games         # Rust/Bevy games (WASM builds)
	/robotsumo   # Initial game implementation
/ai            # Lua scripts and templates
/docs          # Learning materials and guides
docker-compose.yml # Local Postgres stack
```

## Contributing
Contributions and feedback are welcome. Please open an issue or PR.

## License
See [LICENSE](LICENSE).
