//! Server interaction tracing for debugging and monitoring
//!
//! This module provides functionality to capture and retrieve server interactions
//! globally across the application. It supports both sync and async modes.

// Common types and storage
mod common;

// Feature-specific implementations
#[cfg(feature = "sync")]
pub(crate) mod sync;

#[cfg(feature = "async")]
pub(crate) mod r#async;

// Public types - always available regardless of feature flags
pub use common::Interaction;

// Re-export API functions based on active feature
// Re-export functions based on feature configuration
#[cfg(all(feature = "sync", not(feature = "async")))]
pub use sync::{last_interaction, record_request, record_response};

#[cfg(all(feature = "async", not(feature = "sync")))]
pub use r#async::{last_interaction, record_request, record_response};

// When both features are enabled, async is default
#[cfg(all(feature = "sync", feature = "async"))]
pub use r#async::{last_interaction, record_request, record_response};

// When both features are enabled, provide sync versions under blocking namespace
#[cfg(all(feature = "sync", feature = "async"))]
pub mod blocking {
    pub use super::sync::{last_interaction, record_request, record_response};
}
