// ---
// #![no_std] if the standard library is not present.

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate serde_big_array;

big_array! { BigArray; 96 }

// ---
// Export the #[node] attribute macro.

pub use ockam_node_attribute::*;

// ---
// Export node implementation

#[cfg(all(feature = "std", feature = "ockam_node"))]
pub use ockam_node::*;

#[cfg(all(not(feature = "std"), feature = "ockam_node_no_std"))]
pub use ockam_node_no_std::*;

// ---

mod credential;
mod entity;
mod error;
mod lease;
mod old_profile;

pub use credential::*;
pub use error::*;
pub use lease::*;
pub use old_profile::*;

mod remote_mailbox;
pub use remote_mailbox::*;

pub use ockam_core::async_trait::async_trait as async_worker;
pub use ockam_core::{
    Address, Any, Encoded, Error, Message, Result, Route, Routed, RouterMessage, TransportMessage,
    Worker,
};

pub use ockam_channel::{SecureChannel, SecureChannelListenerMessage};

use rand::{CryptoRng, RngCore};

/// A type that combines the required RNG traits.
pub trait Rng: RngCore + CryptoRng {}

impl<T: RngCore + CryptoRng> Rng for T {}
