# Copilot Instructions

## Project Purpose
This repository (codegame) teaches kids programming by letting them write game AI logic in Lua and test different versions against each other. The platform uses a Rust backend (Axum), a React + Tailwind frontend, and game runtimes in Rust (Bevy) compiled to WebAssembly.

## Preferred Behaviors
- Keep solutions beginner-friendly and explain concepts simply.
- Favor small, readable functions with clear names.
- Avoid unnecessary abstractions.
- Prefer deterministic, testable logic.
- Make small, isolated edits and follow a TDD workflow when possible.

## Language and Style
- Backend: Rust with Axum.
- Database: Postgres.
- Frontend: React with Tailwind CSS (TypeScript).
- Games: Rust with Bevy, compiled to WASM.
- Primary scripting language: Lua (AI logic).
- Keep Lua scripts minimal and self-contained.
- Use consistent naming and avoid clever one-liners.
- Prefer simple, readable Rust and React components.

## Safety and Simplicity
- Do not introduce external dependencies unless required.
- Keep runtime and tooling easy to set up for kids.
- Prefer Docker Compose for local backend + Postgres.

## Rust Workflow
- Use `cargo new` when adding new crates to keep them up to date.
- Run `cargo fmt` and `cargo clippy` to keep code quality high.

## Documentation
- Update README when adding new commands or setup steps.
- Add short comments when logic may be non-obvious to beginners.

## Git Workflow
- Commit changes to a feature branch and push to the remote.
- Use Conventional Commits for commit message format.
- Use the gh CLI to create a pull request targeting the main branch.
