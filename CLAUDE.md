# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Quick Start

The rust-ibapi crate is a Rust implementation of the Interactive Brokers TWS API with both synchronous and asynchronous support.

**Default:** The crate uses async by default. No feature flags needed for async usage.

```bash
# Build with async support (default)
cargo build

# Build with sync support only
cargo build --no-default-features --features sync

# Build with both sync and async support
cargo build --features sync

# Run tests
cargo test                    # async (default)
cargo test --features sync   # both features
```

**Feature combinations:**
- **Default (async only)**: `cargo build` - Async API available
- **Sync only**: `cargo build --no-default-features --features sync` - Sync API only
- **Both features**: `cargo build --features sync` - Async API by default, sync under `blocking::` namespace

## Documentation Index

### Getting Started
- [**Quick Start Guide**](docs/quick-start.md) - Get up and running in minutes
- [**Examples Guide**](docs/examples.md) - Running and writing examples
- [**Troubleshooting**](docs/troubleshooting.md) - Common issues and solutions

### Core Concepts
- [**Architecture Overview**](docs/architecture.md) - System design, components, and module organization
- [**Feature Flags**](docs/feature-flags.md) - Sync vs async modes and feature guards
- [**API Patterns**](docs/api-patterns.md) - Builder patterns, protocol versions, and common patterns

### Development
- [**Code Style Guidelines**](docs/code-style.md) - Coding standards and conventions
- [**Build and Test**](docs/build-and-test.md) - Build commands, testing patterns, and CI
- [**Testing Patterns**](docs/testing-patterns.md) - Table-driven tests and MockGateway
- [**Extending the API**](docs/extending-api.md) - Adding new TWS API functionality

## Key Points to Remember

1. **Async by default**: The crate uses async by default. Sync is opt-in via `--no-default-features --features sync`
2. **Test different configurations**: Test async (default), sync-only, and when relevant, both features enabled
3. **Follow module structure**: Use the common pattern for shared logic between sync/async
4. **Minimal comments**: Keep comments concise, avoid stating the obvious
5. **Run quality checks**: Before committing, run `cargo fmt`, `cargo clippy --features sync`, and `cargo clippy --features async`

## Connection Settings

When running examples or tests:
- **IB Gateway Paper Trading**: 127.0.0.1:4002 (recommended)
- **IB Gateway Live Trading**: 127.0.0.1:4001
- **TWS Paper Trading**: 127.0.0.1:7497
- **TWS Live Trading**: 127.0.0.1:7496

## Environment Variables

```bash
# Set log level
RUST_LOG=debug cargo run --example <example_name>

# Record TWS messages for debugging
IBAPI_RECORDING_DIR=/tmp/tws-messages cargo run --example <example_name>
```

## Quick Commands

```bash
# Format code
cargo fmt

# Run clippy (both modes separately)
cargo clippy --features sync
cargo clippy --features async

# Run all tests
just test

# Generate coverage report (opens HTML report in browser)
just cover
```

For detailed information on any topic, refer to the linked documentation files above.

## Git Commit Guidelines

- DO NOT include "Generated with Claude Code" or similar attribution in commit messages
- Keep commit messages focused on the technical changes and their purpose
