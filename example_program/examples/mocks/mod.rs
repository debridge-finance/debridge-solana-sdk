use std::path::PathBuf;

use debridge_solana_sdk::keys::{ExternalCallMetaPubkey, ExternalCallStoragePubkey};
use env_to_array::bs58_to_array;
use solana_sdk::{
    instruction::AccountMeta,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
};

/// Takes keypair from from file by path in `KEYPAIR_PATH` env variable,
/// if not presented then from '~/.config/solana.id.json' path
pub fn get_config_keypair() -> Keypair {
    let keypair = option_env!("KEYPAIR_PATH")
        .map(PathBuf::from)
        .or_else(|| dirs::config_dir().map(|p| p.join("solana").join("id.json")))
        .unwrap();
    read_keypair_file(keypair).expect("Failed to parse payer keypair")
}

macro_rules! pubkey {
    ($b58:literal) => {
        Pubkey::new_from_array(bs58_to_array!($b58))
    };
}

#[allow(dead_code)]
pub fn get_send_account(payer: Pubkey, wallet: Pubkey, shortcut: [u8; 32]) -> [AccountMeta; 18] {
    get_send_account_with_creator(payer, wallet, shortcut, payer)
}
pub fn get_send_account_with_creator(
    payer: Pubkey,
    wallet: Pubkey,
    shortcut: [u8; 32],
    external_call_creator: Pubkey,
) -> [AccountMeta; 18] {
    let external_call_storage =
        Pubkey::find_external_call_storage_address(&shortcut, &external_call_creator).0;
    println!("Ex storage: {:?}", external_call_storage);
    [
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: pubkey!("6SW1N9Rq2TqT3uQCD4F5zwtTTSFSarZmfyrk829SzsBX"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: pubkey!("So11111111111111111111111111111111111111112"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: pubkey!("8gjgVkHXTttCoSGGtzucFkJUWujQ8pgWuvnHCLSN7i3o"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: pubkey!("7FmGdfJfDrrM6P68y7jijjj4xU9rH3hsUK2Kyp54iJUx"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: pubkey!("8L81QZBfwA6Xi9zd49fyUfMRWJBCAxiUxd6jGHPnu1BQ"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: pubkey!("DeSetTwWhjZq6Pz9Kfdo1KoS5NqtsM6G8ERbX4SSCSft"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: pubkey!("CcjkxrCJvfXrmds78hwCnovkdmTgE12wqojiVLrtW1qn"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: pubkey!("5MgAaNomDg4Y88v7gJ7LSWAyoLpDfcfbXZGQQnFddjJT"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: pubkey!("2LKQceMRwfJNZovtSbsHmfszDYM5kTZHajFry2nqD2pi"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: wallet,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: pubkey!("11111111111111111111111111111111"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: external_call_storage,
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: Pubkey::find_external_call_meta_address(&external_call_storage).0,
        },
        AccountMeta {
            is_signer: true,
            is_writable: true,
            pubkey: payer,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: pubkey!("4kQYWVy6Vu8YUXVp5BgQC12ZX1HLRUfkK3bLzBFFjnNW"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: pubkey!("APMGxdbtubfWLQUACsN2yv2pxkvAgWwuxBe8ohFYoB37"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: pubkey!("DEbrdGj3HsRsAzx6uH4MKyREKxVAfBydijLUF3ygsFfh"),
        },
    ]
}
