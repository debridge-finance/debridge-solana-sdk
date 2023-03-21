extern crate core;

pub mod reserved_flags;

use solana_program::pubkey::Pubkey;

/// This module is responsible for the ability
/// to make on-chain send using the debridge infrastructure
pub mod sending;

/// This module is responsible for the ability
/// to check on-chain that a claim has been made
/// using the debridge infrastructure
pub mod check_claiming;

/// This module is responsible for working with debridge accounts of the program,
/// which can allow you to request meta-information necessary for sending. For
/// example, is it possible to send to some network or what commission is set at the moment
pub mod debridge_accounts;

/// This module is responsible for working with debridge accounts pubkeys of the program,
/// which can allow you to request meta-information necessary for sending. For
/// example, is it possible to send to some network or what commission is set at the moment
pub mod keys;

/// Each chain has a special id, this module provides
/// a chain id for all currently supported chains
pub mod chain_ids;

/// This crate gives general errors that may occur in the sdk using
mod errors;

pub mod estimator;
/// This module is auxiliary in working with hash
mod hash;

pub use chain_ids::*;
pub use errors::*;
pub use hash::{HashAdapter, SolanaKeccak256};

// This code exports the public keys of the program, depending on the crate features.
// The `prod` feature is enabled by default. If you want to interact with the prod environment, please do nothing.
//
// If you need access to the debridge test\devnet environment, contact the team to provide keys and use the `env` cargo-feature
cfg_match::cfg_match! {
    feature = "prod" => {
        // This feature is used by default.
        // If you just want to connect to the production environment, then it will use hardcoded keys

        /// Program of debridge program
        /// This program is responsible for sending and claiming submission
        pub static DEBRIDGE_ID: Pubkey =
            Pubkey::new_from_array(env_to_array::bs58_to_array!("DEbrdGj3HsRsAzx6uH4MKyREKxVAfBydijLUF3ygsFfh"));

        /// Program of debridge-settings program
        /// This program is responsible for settings of debridge protocol & storing confirmations
        pub static SETTINGS_ID: Pubkey =
            Pubkey::new_from_array(env_to_array::bs58_to_array!("DeSetTwWhjZq6Pz9Kfdo1KoS5NqtsM6G8ERbX4SSCSft"));
    }
    feature = "env" => {
        // If you use some custom keys to test in devnet\mainnet environment, you can pass the keys via env variables.
        // To do this, build a project with the feature `env` and without `prod`

        /// Program of debridge program
        /// This program is responsible for sending and claiming submission
        pub const DEBRIDGE_ID: Pubkey =
            Pubkey::new_from_array(env_to_array::bs58_env_to_array!("DEBRIDGE_PROGRAM_PUBKEY"));

        /// Program of debridge-settings program
        /// This program is responsible for settings of debridge protocol & storing confirmations
        pub const SETTINGS_ID: Pubkey =
            Pubkey::new_from_array(env_to_array::bs58_env_to_array!("DEBRIDGE_SETTINGS_PROGRAM_PUBKEY"));
    }
    _ => {}
}

pub mod prelude {
    pub use super::{
        chain_ids, check_claiming as debridge_check_claiming, sending as debridge_sending,
        DEBRIDGE_ID, SETTINGS_ID, SOLANA_CHAIN_ID,
    };
}

pub const BPS_DENOMINATOR: u64 = 10000_u64;

// Checking that multiple environments cannot be enabled at the same time
macro_rules! assert_unique_feature {
    () => {};
    ($first:tt $(,$rest:tt)*) => {
        $(
            #[cfg(all(feature = $first, feature = $rest))]
            compile_error!(concat!("features \"", $first, "\" and \"", $rest, "\" cannot be used together"));
        )*
        assert_unique_feature!($($rest),*);
    }
}
assert_unique_feature!("prod", "env");
