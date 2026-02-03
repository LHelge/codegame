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
- Database: SQLite.
- Frontend: React with Tailwind CSS (TypeScript).
- Games: Rust with Bevy, compiled to WASM.
- Primary scripting language: Lua (AI logic).
- Keep Lua scripts minimal and self-contained.
- Use consistent naming and avoid clever one-liners.
- Prefer simple, readable Rust and React components.

## Safety and Simplicity
- Do not introduce external dependencies unless required.
- Keep runtime and tooling easy to set up for kids.

## Rust Workflow
- Use `cargo new` when adding new crates to keep them up to date.
- Run `cargo fmt` and `cargo clippy` to keep code quality high.
- Use `cargo test` to verify functionality.
- Use `cargo add` to add dependencies to make sure they are up to date.
- **Always run `cargo fmt && cargo clippy` after making changes.**

## Testing
- Run all tests with `cargo test` from the workspace root or backend directory.
- Unit tests live alongside source code in `#[cfg(test)]` modules.
- Integration tests for the backend API live in `backend/tests/`.
- Use `axum-test` crate for HTTP integration tests (in-process, no server needed).
- Test utilities are in `backend/tests/common/mod.rs`.
- Each test creates an isolated in-memory SQLite database with migrations applied.
- Use `common::create_test_token()` to generate JWT tokens for authenticated requests.

## Database Workflow
- Use `sqlx-cli` for managing database migrations.
- Create migrations with `sqlx migrate add -r <name>` for reversible migrations.
- Migrations run automatically on app startup.
- The database is created automatically if it does not exist.

## Documentation
- Update README when adding new commands or setup steps.
- Add short comments when logic may be non-obvious to beginners.

## Git Workflow
- Commit changes to a feature branch and push to the remote.
- Use Conventional Commits for commit message format.
- Use the gh CLI to create a pull request targeting the main branch.
- Ensure all tests pass before creating a pull request.
- Rebase feature branches onto main before merging.

