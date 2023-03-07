use std::str::FromStr;

use debridge_solana_sdk::{
    keys::{ExternalCallMetaPubkey, ExternalCallStoragePubkey},
    Error,
};
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};
pub fn get_send_account(
    payer: Pubkey,
    wallet: Pubkey,
    shortcut: [u8; 32],
) -> Result<[AccountMeta; 18], Error> {
    let external_call_storage = Pubkey::find_external_call_storage_address(&shortcut, &payer)?.0;
    Ok([
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: Pubkey::from_str("6SW1N9Rq2TqT3uQCD4F5zwtTTSFSarZmfyrk829SzsBX")
                .expect("Failed to parse pubkey"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: Pubkey::from_str("So11111111111111111111111111111111111111112")
                .expect("Failed to parse pubkey"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: Pubkey::from_str("8gjgVkHXTttCoSGGtzucFkJUWujQ8pgWuvnHCLSN7i3o")
                .expect("Failed to parse pubkey"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: Pubkey::from_str("7FmGdfJfDrrM6P68y7jijjj4xU9rH3hsUK2Kyp54iJUx")
                .expect("Failed to parse pubkey"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: Pubkey::from_str("8L81QZBfwA6Xi9zd49fyUfMRWJBCAxiUxd6jGHPnu1BQ")
                .expect("Failed to parse pubkey"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: Pubkey::from_str("DeSetTwWhjZq6Pz9Kfdo1KoS5NqtsM6G8ERbX4SSCSft")
                .expect("Failed to parse pubkey"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
                .expect("Failed to parse pubkey"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: Pubkey::from_str("CcjkxrCJvfXrmds78hwCnovkdmTgE12wqojiVLrtW1qn")
                .expect("Failed to parse pubkey"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: Pubkey::from_str("5MgAaNomDg4Y88v7gJ7LSWAyoLpDfcfbXZGQQnFddjJT")
                .expect("Failed to parse pubkey"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: Pubkey::from_str("2LKQceMRwfJNZovtSbsHmfszDYM5kTZHajFry2nqD2pi")
                .expect("Failed to parse pubkey"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: wallet,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: Pubkey::from_str("11111111111111111111111111111111")
                .expect("Failed to parse pubkey"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: external_call_storage,
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: Pubkey::find_external_call_meta_address(&external_call_storage)?.0,
        },
        AccountMeta {
            is_signer: true,
            is_writable: true,
            pubkey: payer,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: Pubkey::from_str("4kQYWVy6Vu8YUXVp5BgQC12ZX1HLRUfkK3bLzBFFjnNW")
                .expect("Failed to parse pubkey"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: Pubkey::from_str("APMGxdbtubfWLQUACsN2yv2pxkvAgWwuxBe8ohFYoB37")
                .expect("Failed to parse pubkey"),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: Pubkey::from_str("DEbrdGj3HsRsAzx6uH4MKyREKxVAfBydijLUF3ygsFfh")
                .expect("Failed to parse pubkey"),
        },
    ])
}
