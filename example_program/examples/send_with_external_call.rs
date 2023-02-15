use std::{env, str::FromStr};

use anchor_lang::InstructionData;
use debridge_solana_sdk::{reserved_flags::SetReservedFlag, HashAdapter, SOLANA_CHAIN_ID};
use debridge_solana_sdk_example::{instruction::SendViaDebridgeWithExternalCall, ID as EXAMPLE_ID};
use solana_client::{rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use solana_program::instruction::{AccountMeta, Instruction};
use solana_sdk::{
    pubkey::Pubkey, signature::Signer, signer::keypair::read_keypair_file, transaction::Transaction,
};

fn get_send_acount(payer: Pubkey, wallet: Pubkey, shortcut: [u8; 32]) -> [AccountMeta; 18] {
    let external_call_storage = find_external_call_storage_address(&shortcut, &payer).0;

    [
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
            pubkey: find_external_call_meta_address(&external_call_storage).0,
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
    ]
}

fn find_external_call_storage_address(shortcut: &[u8; 32], owner: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"EXTERNAL_CALL_STORAGE",
            shortcut,
            owner.as_ref(),
            &SOLANA_CHAIN_ID,
        ],
        &debridge_solana_sdk::get_debridge_id(),
    )
}

fn find_external_call_meta_address(external_call_storage: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"EXTERNAL_CALL_META", external_call_storage.as_ref()],
        &debridge_solana_sdk::get_debridge_id(),
    )
}

fn main() {
    let rpc_client: RpcClient = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    let payer = read_keypair_file(env!("KEYPAIR_PATH")).expect("Failed to parse payer keypair");

    let external_call = hex::decode("a69b6ed0000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000060011223344550000000000000000000000000000000000000000000000000000").expect("Failed to decode external code");

    let wallet = rpc_client
        .get_token_accounts_by_owner(
            &payer.pubkey(),
            TokenAccountsFilter::Mint(
                Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
            ),
        )
        .expect("Failed to get wSol wallets")
        .iter()
        .max_by_key(|wallet| wallet.account.lamports)
        .map(|wallet| Pubkey::from_str(wallet.pubkey.as_str()).expect("Failed to parse wallet"))
        .expect("There are no payer wallets");

    let mut flags = [0; 32];
    flags.set_proxy_with_sender();
    flags.set_revert_if_external_call();

    let ix = Instruction {
        program_id: EXAMPLE_ID,
        accounts: get_send_acount(
            payer.pubkey(),
            wallet,
            sha3::Keccak256::hash(external_call.as_slice()),
        )
        .to_vec(),
        data: SendViaDebridgeWithExternalCall {
            amount: 0,
            execution_fee: 0,
            external_call,
            fallback_address: hex::decode("bd1e72155Ce24E57D0A026e0F7420D6559A7e651")
                .expect("Failed to decode fallback address"),
            receiver: hex::decode("cfcc66ee5397b7cdf7228f7502d1e168518c6bb3")
                .expect("Failed to decode receiver"),
            target_chain_id: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 137,
            ],
            reserved_flag: flags,
        }
        .data(),
    };

    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);

    rpc_client
        .send_transaction(&tx)
        .expect("Failed to send transaction");
}
