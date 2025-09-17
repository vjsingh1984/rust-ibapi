//! Client implementation with sync/async support

pub(crate) mod builders;
pub(crate) mod common;
pub(crate) mod error_handler;
pub(crate) mod id_generator;

#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "async")]
pub mod r#async;

// Re-export the appropriate Client based on feature configuration
// When only async is enabled (the default case)
#[cfg(all(feature = "async", not(feature = "sync")))]
pub use r#async::Client;

// When only sync is enabled
#[cfg(all(feature = "sync", not(feature = "async")))]
pub use sync::Client;

// When both features are enabled, async is the default
#[cfg(all(feature = "sync", feature = "async"))]
pub use r#async::Client;

// When both features are enabled, provide sync under blocking namespace
#[cfg(all(feature = "sync", feature = "async"))]
pub mod blocking {
    pub use super::sync::Client;
    pub use crate::subscriptions::blocking::SharesChannel;
}

// Re-export subscription types from subscriptions module
#[cfg(all(feature = "sync", not(feature = "async")))]
pub use crate::subscriptions::{SharesChannel, Subscription};

// For async-only or both features
#[cfg(feature = "async")]
pub use crate::subscriptions::Subscription;

// Internal re-exports for sync - available whenever sync feature is enabled
#[cfg(feature = "sync")]
pub(crate) use crate::subscriptions::{ResponseContext, StreamDecoder};

// When both features are enabled, provide SharesChannel under the module level for internal use
#[cfg(all(feature = "sync", feature = "async"))]
pub(crate) use crate::subscriptions::blocking::SharesChannel;

// Re-export builder traits (internal use only)
pub(crate) use builders::{ClientRequestBuilders, SubscriptionBuilderExt};
