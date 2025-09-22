# Repository Guidelines

## Project Structure & Module Organization
Core crate code sits in `src/`, with request/response handling spread across modules such as `client`, `connection`, `market_data`, `orders`, and `transport`; shared primitives live in `common` and `messages`. Store fixtures in `src/testdata` and export public surfaces through `lib.rs` and `prelude.rs`. Keep integration scenarios under `tests/`, grouping related data beneath `tests/data`. Place runnable walkthroughs in `examples/`, and record long-form notes under `docs/` or `MIGRATION.md`. Large generated artifacts belong in `results/`.

## Build, Test, and Development Commands
- `cargo build` produces the default async client; add `--features sync` for blocking support or disable defaults for sync-only builds.
- `cargo test`, `cargo test --features sync`, and `cargo test --no-default-features --features sync` cover the full feature matrix; `just test` runs the last two configurations automatically.
- `cargo fmt --all` enforces the project formatter settings; run this before committing.
- `cargo clippy -- -D warnings`, `cargo clippy --features sync -- -D warnings`, and `cargo clippy --no-default-features --features sync -- -D warnings` keep all modes lint-clean.
- `just cover` (tarpaulin) produces `tarpaulin-report.html` for coverage review.

## Coding Style & Naming Conventions
Follow Rust defaults with 4-space indentation, `snake_case` for functions and modules, `UpperCamelCase` for types, and `SCREAMING_SNAKE_CASE` for constants. Prefer module-level `pub use` re-exports via `prelude.rs` for consumer ergonomics, and expose blocking variants under `client::blocking`/`subscriptions::blocking` whenever both features are enabled. Run `cargo fmt` (uses `rustfmt.toml` `max_width = 150`) and the clippy matrix before opening a PR. Document public APIs with `///` doc comments and favor explicit enums over string literals for IB identifiers.

## Testing Guidelines
Unit tests live beside their modules behind `#[cfg(test)]`; integration tests belong in `tests/` and should exercise both sync and async feature gates. Name tests for the scenario under validation (e.g., `test_order_submission_async`). For external API interactions, stub network calls with fixtures from `tests/data` to keep runs deterministic. Generate coverage with `just cover` and review newly uncovered gaps before merging.

## Commit & Pull Request Guidelines
Adopt the existing Conventional Commit format (`feat:`, `fix:`, `docs:`, etc.) and mention related issues or tickets in parentheses when relevant. Each PR should summarize behavior changes, call out new feature flags, and link to migration notes if client interfaces shift. Include test evidence (command output or coverage summary) and note any follow-up work in the description. Request reviewers familiar with the touched modules and ensure changelog or docs updates accompany user-facing changes.
