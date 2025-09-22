# Feature Flags

rust-ibapi enables async support by default. Add the optional `sync` feature when you need the blocking API, or disable defaults to build a sync-only crate.

## Feature Selection Flow

```mermaid
graph TD
    Start[cargo build/test/run]
    Default[Async (default)]
    Both[Async + Sync\n(--features sync)]
    SyncOnly[Sync only\n(--no-default-features --features sync)]

    Start --> Default
    Default -->|Add --features sync| Both
    Default -->|Disable defaults + sync| SyncOnly

    style Default fill:#e3f2fd
    style Both fill:#c8e6c9
    style SyncOnly fill:#ffe0b2
```

## Available Features

- **`async`** *(default)*: Tokio-based, non-blocking implementation
- **`sync`** *(optional)*: Thread-based implementation using crossbeam channels

When both features are enabled, async types remain the primary exports and the blocking equivalents are available under `client::blocking` and `subscriptions::blocking`.

## Feature Guard Pattern

Use standard feature guards, but remember that both features can be active simultaneously:

```rust
#[cfg(feature = "sync")]
use crate::client::blocking::Client as BlockingClient;

#[cfg(feature = "async")]
use crate::Client; // Async client
```

If you need code that behaves differently when *only* one mode is present, gate with `#[cfg(all(feature = "sync", not(feature = "async")))]` and similar patterns.

## Module Organization Pattern

Modules keep shared types at the top level, add mode-specific implementations, and expose blocking APIs behind a nested namespace when both features are active:

```rust
pub struct MyType { /* shared */ }

#[cfg(feature = "sync")]
mod sync;

#[cfg(feature = "async")]
mod r#async;

#[cfg(feature = "async")]
pub use r#async::my_async_fn;

#[cfg(feature = "sync")]
pub(crate) use sync::my_sync_fn;

#[cfg(feature = "sync")]
pub mod blocking {
    pub use super::sync::my_sync_fn;
}
```

## Usage in Cargo.toml

```toml
# Async-only (default)
ibapi = "2.0"

# Async + sync
ibapi = { version = "2.0", features = ["sync"] }

# Sync-only
ibapi = { version = "2.0", default-features = false, features = ["sync"] }
```

## Testing With Features

```bash
cargo test                                   # Async (default)
cargo test --features sync                   # Async + sync
cargo test --no-default-features --features sync  # Sync-only
```

Run the same matrix for `cargo clippy` to keep both implementations lint-clean.

## Key Differences

### Async Mode
- Uses the tokio runtime and async/await
- `Subscription` implements `Stream`
- Message dispatch handled via async channels

### Sync Mode
- Uses threads and crossbeam channels
- `subscriptions::blocking::Subscription` implements `Iterator`
- Blocking helpers exposed through `client::blocking`

Pick the combination that matches your deployment: default async for new integrations, add `sync` when you need existing blocking workflows, or disable defaults for minimal sync-only builds.
