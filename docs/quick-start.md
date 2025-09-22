# Quick Start Guide

Get up and running with rust-ibapi in minutes.

## Prerequisites

Before you begin, ensure you have:
1. **Rust installed** - [Install Rust](https://www.rust-lang.org/tools/install)
2. **IB Gateway or TWS** - Running and configured for API connections
3. **Git** - For cloning the repository

## Critical: Choose Your Feature

⚠️ **rust-ibapi requires exactly ONE feature flag:**

```mermaid
graph LR
    Choice{Your Application Type?}
    Sync[--features sync<br/>Traditional threads]
    AsyncDefault[Async (default)]
    AsyncSync[Async + Sync\\n(--features sync)]
    SyncOnly[Sync only\\n(--no-default-features --features sync)]

    Choice -->|Do nothing| AsyncDefault
    Choice -->|Need blocking API too| AsyncSync
    Choice -->|Minimal blocking build| SyncOnly

    style Choice fill:#fff3e0
    style AsyncDefault fill:#e3f2fd
    style AsyncSync fill:#c8e6c9
    style SyncOnly fill:#ffe0b2
```

- **Default (`cargo build`)** - Modern async execution using tokio
- **`--features sync`** - Adds the blocking API (available via `client::blocking`)
- **`--no-default-features --features sync`** - Blocking build without async dependencies

## Installation

### As a Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
ibapi = "2.0"                                # Async-only (default)
ibapi = { version = "2.0", features = ["sync"] }   # Async + sync
ibapi = { version = "2.0", default-features = false, features = ["sync"] }  # Sync-only
```

### For Development

```bash
# Clone the repository
git clone https://github.com/wboayue/rust-ibapi.git
cd rust-ibapi

# Verify installation
cargo build                                   # Async (default)
cargo build --features sync                    # Async + sync
cargo build --no-default-features --features sync  # Sync-only
```

## Your First Example

### Step 1: Start IB Gateway/TWS

Ensure your IB Gateway or TWS is running with API connections enabled:

| Platform | Paper Trading | Live Trading |
|----------|--------------|--------------|
| IB Gateway | 127.0.0.1:4002 | 127.0.0.1:4001 |
| TWS | 127.0.0.1:7497 | 127.0.0.1:7496 |

### Step 2: Run a Simple Example

#### Sync Version

Create `src/main.rs`:

```rust
use ibapi::client::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to IB Gateway Paper Trading
    let client = Client::connect("127.0.0.1:4002", 100)?;
    
    // Request current time
    let server_time = client.server_time()?;
    println!("Server time: {}", server_time);
    
    // Get account summary
    let account_summary = client.account_summary()?;
    for item in account_summary {
        println!("{}: {} {}", item.tag, item.value, item.currency);
    }
    
    Ok(())
}
```

Run with:
```bash
cargo run --features sync
# or for a sync-only build
cargo run --no-default-features --features sync
```

#### Async Version

Create `src/main.rs`:

```rust
use ibapi::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to IB Gateway Paper Trading
    let client = Client::connect("127.0.0.1:4002", 100).await?;
    
    // Request current time
    let server_time = client.server_time().await?;
    println!("Server time: {}", server_time);
    
    // Get account summary
    let account_summary = client.account_summary().await?;
    for item in account_summary {
        println!("{}: {} {}", item.tag, item.value, item.currency);
    }
    
    Ok(())
}
```

Run with:
```bash
cargo run
```

## Common Operations

### Creating Contracts

The library provides a type-safe contract builder API:

```rust
// Simple stock contract
let stock = Contract::stock("AAPL").build();

// Option with required fields enforced at compile time
let option = Contract::call("AAPL")
    .strike(150.0)
    .expires_on(2024, 12, 20)
    .build();
```

For detailed documentation on creating all contract types, see the [Contract Builder Guide](contract-builder.md).

### Requesting Market Data

```rust
// Define a stock contract
let contract = Contract::stock("AAPL").build();

// Request real-time bars (sync)
let subscription = client.realtime_bars(&contract, BarSize::Sec5, WhatToShow::Trades, false)?;
for bar in subscription {
    println!("Price: {}, Volume: {}", bar.close, bar.volume);
}

// Request real-time bars (async)
let mut subscription = client.realtime_bars(&contract, BarSize::Sec5, WhatToShow::Trades, false).await?;
while let Some(bar) = subscription.next().await {
    println!("Price: {}, Volume: {}", bar.close, bar.volume);
}
```

### Placing Orders

```rust
// Create a market order
let contract = Contract::stock("AAPL");
let order = Order::market_order(Action::Buy, 100.0);

// Place the order
let order_id = client.next_order_id();
client.place_order(order_id, &contract, &order)?;
```

### Getting Account Information

```rust
// Get positions
let positions = client.positions()?;
for position in positions {
    println!("{}: {} shares", position.contract.symbol, position.size);
}

// Get account values
let account_values = client.account_values()?;
for value in account_values {
    println!("{}: {}", value.key, value.value);
}
```

## Running Examples

The repository includes many examples in the `examples/` directory:

```bash
# List all examples
ls examples/

# Run a sync example
cargo run --features sync --example account_summary

# Run an async example  
cargo run --example async_account_summary

# Run with debug logging
RUST_LOG=debug cargo run --features sync --example market_data
```

### Popular Examples

| Example | Description | Command |
|---------|-------------|---------|
| `account_summary` | Display account information | `cargo run --features sync --example account_summary` |
| `market_data` | Stream real-time quotes | `cargo run --features sync --example market_data` |
| `place_order` | Place a simple order | `cargo run --features sync --example place_order` |
| `historical_data` | Fetch historical bars | `cargo run --features sync --example historical_data` |
| `contract_details` | Get contract information | `cargo run --features sync --example contract_details` |

Async examples run without additional flags. Use commands like `cargo run --example async_market_data` to explore the default async API.

## Troubleshooting

### Common Issues and Solutions

#### "cannot find module `client::blocking`" Error
```text
error[E0432]: unresolved import `ibapi::client::blocking`
```
**Solution**: Add the `sync` feature (`cargo build --features sync`) or disable defaults and enable `sync` for a sync-only build.

#### Connection Refused
```bash
Error: Connection refused (os error 111)
```
**Solution**: 
1. Ensure IB Gateway/TWS is running
2. Check the port number (4002 for paper, 4001 for live)
3. Enable API connections in IB Gateway/TWS settings

#### API Not Configured
```bash
Error: API connection not configured
```
**Solution**: In IB Gateway/TWS:
1. Go to Configuration → API → Settings
2. Enable "Enable ActiveX and Socket Clients"
3. Add 127.0.0.1 to trusted IPs
4. Disable "Read-Only API"

#### No Market Data Permissions
```bash
Error: No market data permissions
```
**Solution**: Ensure your IB account has market data subscriptions for the requested symbols.

### Debug Logging

Enable detailed logging to troubleshoot issues:

```bash
# Basic debug logging
RUST_LOG=debug cargo run --features sync --example your_example

# Trace-level logging (very verbose)
RUST_LOG=trace cargo run --features sync --example your_example

# Log only ibapi messages
RUST_LOG=ibapi=debug cargo run --features sync --example your_example

# Record all TWS messages for analysis
IBAPI_RECORDING_DIR=/tmp/tws-messages cargo run --features sync --example your_example
```

### Getting Help

1. **Check the examples** - Most common use cases are demonstrated
2. **Read the API docs** - `cargo doc --open --features sync`
3. **Review test cases** - Tests show expected behavior
4. **GitHub Issues** - Search existing issues or create a new one
5. **Documentation** - See [docs/](.) for detailed guides

## Next Steps

Now that you're up and running:

1. **Explore More Examples** - Check out the `examples/` directory
2. **Read the Architecture Guide** - Understand how rust-ibapi works internally
3. **Learn the API Patterns** - See [API Patterns](api-patterns.md)
4. **Contribute** - See [Contributing Guide](../CONTRIBUTING.md)

## Quick Reference

### Essential Commands

```bash
# Build
cargo build
cargo build --features sync
cargo build --no-default-features --features sync

# Test
cargo test
cargo test --features sync
cargo test --no-default-features --features sync

# Run example
cargo run --example async_connect
cargo run --features sync --example orders

# Generate docs
cargo doc --open

# Check code
cargo clippy -- -D warnings
cargo clippy --features sync -- -D warnings
cargo clippy --no-default-features --features sync -- -D warnings
cargo fmt --check
```

### Connection Endpoints

| Environment | Host | Port |
|------------|------|------|
| IB Gateway Paper | 127.0.0.1 | 4002 |
| IB Gateway Live | 127.0.0.1 | 4001 |
| TWS Paper | 127.0.0.1 | 7497 |
| TWS Live | 127.0.0.1 | 7496 |

### Feature Selection Guide

Choose **sync** if you:
- Are new to Rust async programming
- Want simpler, traditional code
- Don't need high concurrency
- Prefer familiar thread-based patterns

Choose **async** if you:
- Need high performance
- Want to handle many concurrent operations
- Are comfortable with async/await
- Use other async libraries (tokio ecosystem)

Remember: You must choose exactly one!
