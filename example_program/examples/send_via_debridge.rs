use std::str::FromStr;

use anchor_lang::InstructionData;
use debridge_solana_sdk::{HashAdapter, POLYGON_CHAIN_ID};
use debridge_solana_sdk_example::{instruction::SendViaDebridge, ID as EXAMPLE_ID};
use solana_client::{rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use solana_program::instruction::Instruction;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction, pubkey::Pubkey, signature::Signer,
    transaction::Transaction,
};

use crate::mocks::get_send_account;

mod mocks;

fn get_wrapped_sol_wallet(rpc_client: &RpcClient, payer: Pubkey) -> Pubkey {
    rpc_client
        .get_token_accounts_by_owner(
            &payer,
            TokenAccountsFilter::Mint(
                Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
            ),
        )
        .expect("Failed to get wSol wallets")
        .iter()
        .max_by_key(|wallet| wallet.account.lamports)
        .map(|wallet| Pubkey::from_str(wallet.pubkey.as_str()).expect("Failed to parse wallet"))
        .expect("There are no payer wallets")
}

fn main() {
    let rpc_client: RpcClient = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    let payer = mocks::get_config_keypair();

    let cross_chain_message = vec![];
    let ix_with_debridge_send_inside = Instruction {
        program_id: EXAMPLE_ID,
        accounts: get_send_account(
            payer.pubkey(),
            get_wrapped_sol_wallet(&rpc_client, payer.pubkey()),
            sha3::Keccak256::hash(cross_chain_message.as_slice()),
        )
        .to_vec(),
        data: SendViaDebridge {
            amount: 0,
            receiver: hex::decode("cfcc66ee5397b7cdf7228f7502d1e168518c6bb3")
                .expect("Failed to decode receiver"),
            target_chain_id: POLYGON_CHAIN_ID,
            is_use_asset_fee: false,
        }
        .data(),
    };

    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");

    let tx = Transaction::new_signed_with_payer(
        &[
            ComputeBudgetInstruction::set_compute_unit_limit(230000),
            ix_with_debridge_send_inside,
        ],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    let signature = rpc_client
        .send_transaction(&tx)
        .expect("Failed to send transaction");

    println!("Success! Transaction signature: {:?}", signature);
}
