# Copilot Instructions

## Project Purpose
This repository (codegame) teaches kids programming by letting them write game AI logic in Lua and test different versions against each other. The platform uses a Rust backend (Axum), a React + Tailwind frontend, and game runtimes in Rust (Bevy) compiled to WebAssembly.

## Preferred Behaviors
- Keep solutions beginner-friendly and explain concepts simply.
- Favor small, readable functions with clear names.
- Avoid unnecessary abstractions.
- Prefer deterministic, testable logic.
- Make small, isolated edits and follow a TDD workflow when possible.
- **Do not use `sed` or other terminal commands to modify files.** Use the editor tools instead so changes show up in VSCode diffs.

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
- **Use compile-time checked queries** with `sqlx::query!` and `sqlx::query_scalar!` macros.
- **Run `cargo sqlx prepare --workspace` after changing queries or migrations** to update offline metadata.
- Commit the `.sqlx` directory to the repository for CI offline builds.
- For SQLite booleans, use type annotations: `admin as "admin: bool"` in SELECT queries.

## Documentation
- Update README when adding new commands or setup steps.
- Add short comments when logic may be non-obvious to beginners.

## Git Workflow
- Commit changes to a feature branch and push to the remote.
- Use Conventional Commits for commit message format.
- Use the gh CLI to create a pull request targeting the main branch.
- Ensure all tests pass before creating a pull request.
- Rebase feature branches onto main before merging.

## CI/CD
- GitHub Actions runs on all PRs and pushes to main.
- CI checks: `cargo fmt --check`, `cargo clippy`, `cargo test`.
- All CI checks must pass before merging.

## Releases
- Create releases by pushing semver tags: `git tag v1.0.0 && git push --tags`
- CI automatically builds Docker images (amd64) and pushes to ghcr.io.
- Images: `ghcr.io/<owner>/codegame-backend` and `ghcr.io/<owner>/codegame-frontend`
- Tags follow semver: `v1.2.3` creates tags `1.2.3`, `1.2`, `1`, and `latest`.
- A GitHub Release is automatically created with release notes.

## Docker
- Backend and frontend have multistage Dockerfiles for minimal production images.
- Backend uses distroless base image (~20MB).
- Frontend uses nginx-alpine and includes WASM game builds.
- Use `docker compose up` to run the full stack locally in containers.
- For local development without containers, run `cargo run` in backend/ and `npm run dev` in frontend/.