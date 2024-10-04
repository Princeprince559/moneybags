#![warn(missing_docs)]
//! API for [aleph-node](https://github.com/Cardinal-Cryptography/aleph-node) chain.
//!
//! This crate provides a Rust application interface for submitting transactions to `aleph-node` chain.
//! Most of the [pallets](https://docs.substrate.io/reference/frame-pallets/) are common to any
//! [Substrate](https://github.com/paritytech/substrate) chain, but there are some unique to `aleph-node`,
//! e.g. [`pallets::elections::ElectionsApi`].
//!

#![feature(auto_traits)]
#![feature(negative_impls)]

extern crate core;

pub use contract_transcode;
pub use subxt::ext::sp_core::Pair;
use subxt::{
    ext::sp_core::{ed25519, sr25519, H256},
    tx::PairSigner,
    OnlineClient, PolkadotConfig,
};

use crate::api::runtime_types::aleph_runtime::RuntimeCall as Call;
// generated by running `subxt codegen --derive Clone Debug Eq PartialEq | rustfmt --edition=2021 > src/aleph_zero.rs`
#[allow(clippy::all)]
mod aleph_zero;
mod connections;
pub mod contract;
/// API for pallets.
pub mod pallets;
mod runtime_types;
/// Block / session / era API.
pub mod utility;
/// Waiting for some events API.
pub mod waiting;

pub use aleph_zero::api;
pub use runtime_types::*;

/// An alias for a configuration of live chain, e.g. block index type, hash type.
pub type AlephConfig = PolkadotConfig;
/// An alias for a pallet aleph keys.
pub type AlephKeyPair = ed25519::Pair;
/// An alias for a type of a key pair that signs chain transactions.
pub type RawKeyPair = sr25519::Pair;
/// An alias for a signer object that constructs [`sr25519::Pair`] from given account id type.
pub type KeyPair = PairSigner<AlephConfig, sr25519::Pair>;
/// An alias for an account id type.
pub type AccountId = subxt::ext::sp_core::crypto::AccountId32;
/// An alias for a client type.
pub type Client = OnlineClient<AlephConfig>;
/// An alias for a hash type.
pub type BlockHash = H256;

/// An alias for an RPC client type.
pub type SubxtClient = OnlineClient<AlephConfig>;

pub use connections::{
    Connection, ConnectionApi, RootConnection, SignedConnection, SignedConnectionApi, SudoCall,
};

/// When submitting a transaction, wait for given status before proceeding.
#[derive(Copy, Clone)]
pub enum TxStatus {
    /// A tx must be included in some block.
    InBlock,
    /// A tx must be included in some finalized block.
    Finalized,
    /// A tx must be successfully submitted.
    Submitted,
}

/// Converts given seed phrase to a sr25519 [`KeyPair`] object.
/// * `seed` - a 12 or 24 word seed phrase
pub fn keypair_from_string(seed: &str) -> KeyPair {
    let pair = sr25519::Pair::from_string(seed, None).expect("Can't create pair from seed value");
    KeyPair::new(pair)
}

/// Converts given seed phrase to a sr25519 [`RawKeyPair`] object.
/// * `seed` - a 12 or 24 word seed phrase
pub fn raw_keypair_from_string(seed: &str) -> RawKeyPair {
    sr25519::Pair::from_string(seed, None).expect("Can't create pair from seed value")
}

/// Converts given seed phrase to a ed25519 [`AlephKeyPair`] object.
/// * `seed` - a 12 or 24 word seed phrase
pub fn aleph_keypair_from_string(seed: &str) -> AlephKeyPair {
    ed25519::Pair::from_string(seed, None).expect("Can't create pair from seed value")
}

/// Converts a key pair object to `AccountId`.
/// * `keypair` - a key-pair object, e.g. [`ed25519::Pair`] or [`sr25519::Pair`]
pub fn account_from_keypair<P>(keypair: &P) -> AccountId
where
    P: Pair,
    AccountId: From<<P as Pair>::Public>,
{
    AccountId::from(keypair.public())
}
