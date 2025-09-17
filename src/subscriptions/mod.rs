//! Subscription types for sync/async streaming data

mod common;
pub(crate) use common::{ResponseContext, StreamDecoder};

#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "async")]
pub mod r#async;

// Re-export the appropriate subscription types based on feature configuration
#[cfg(all(feature = "sync", not(feature = "async")))]
pub use sync::{SharesChannel, Subscription, SubscriptionIter, SubscriptionOwnedIter, SubscriptionTimeoutIter, SubscriptionTryIter};

#[cfg(all(feature = "async", not(feature = "sync")))]
pub use r#async::Subscription;

// When both features are enabled, async is default
#[cfg(all(feature = "sync", feature = "async"))]
pub use r#async::Subscription;

// When both features are enabled, provide sync versions under blocking namespace
#[cfg(all(feature = "sync", feature = "async"))]
pub mod blocking {
    pub use super::sync::{SharesChannel, Subscription, SubscriptionIter, SubscriptionOwnedIter, SubscriptionTimeoutIter, SubscriptionTryIter};
}
