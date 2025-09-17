//! Builder patterns for request and subscription creation
//!
//! This module provides unified builder patterns that work with both sync and async modes.

#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "async")]
pub mod r#async;

mod common;

// Re-export builders based on feature configuration
#[cfg(all(feature = "sync", not(feature = "async")))]
pub use sync::{ClientRequestBuilders, SubscriptionBuilderExt};

#[cfg(all(feature = "async", not(feature = "sync")))]
pub use r#async::{ClientRequestBuilders, SubscriptionBuilderExt};

// When both features are enabled, async is default
#[cfg(all(feature = "sync", feature = "async"))]
pub use r#async::{ClientRequestBuilders, SubscriptionBuilderExt};

// When both features are enabled, provide sync versions under blocking namespace
#[cfg(all(feature = "sync", feature = "async"))]
pub mod blocking {
    pub use super::sync::{ClientRequestBuilders, SubscriptionBuilderExt};
}
