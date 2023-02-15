use std::{env, str::FromStr};

use anchor_lang::InstructionData;
use debridge_solana_sdk::{HashAdapter, SOLANA_CHAIN_ID};
use debridge_solana_sdk_example::{instruction::SendViaDebridge, ID as EXAMPLE_ID};
use solana_client::{rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use solana_program::instruction::Instruction;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction, pubkey::Pubkey, signature::Signer,
    signer::keypair::read_keypair_file, transaction::Transaction,
};

use crate::mocks::get_send_acount;

mod mocks;

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

    let message: Vec<u8> = vec![];

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

    let budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(230000);

    let ix = Instruction {
        program_id: EXAMPLE_ID,
        accounts: get_send_acount(
            payer.pubkey(),
            wallet,
            sha3::Keccak256::hash(message.as_slice()),
        )
        .to_vec(),
        data: SendViaDebridge {
            amount: 0,
            receiver: hex::decode("cfcc66ee5397b7cdf7228f7502d1e168518c6bb3")
                .expect("Failed to decode receiver"),
            target_chain_id: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 137,
            ],
            is_use_asset_fee: false,
        }
        .data(),
    };

    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");
    let tx = Transaction::new_signed_with_payer(
        &[budget_ix, ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    let signature = rpc_client
        .send_transaction(&tx)
        .expect("Failed to send transaction");

    println!("Success! Transaction signature: {:?}", signature);
}
