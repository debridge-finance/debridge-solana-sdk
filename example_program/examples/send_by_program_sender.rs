use std::str::FromStr;

use anchor_lang::{InstructionData, ToAccountMetas};
use debridge_solana_sdk::{HashAdapter, POLYGON_CHAIN_ID};
use debridge_solana_sdk_example::{
    accounts::SendViaDebridgeWithSender, instruction::SendMessageViaDebridgeWithProgramSender,
    ID as EXAMPLE_ID, PROGRAM_SENDER_SEED,
};
use rand::Rng;
use solana_client::{rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use solana_program::instruction::Instruction;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction, pubkey::Pubkey, signature::Signer,
    transaction::Transaction,
};

use crate::mocks::get_send_account_with_creator;

mod mocks;

const ECHO_CONTRACT_ADDRESS: &str = "cfcc66ee5397b7cdf7228f7502d1e168518c6bb3";
const FALLBACK_ADDRESS: &str = "bd1e72155Ce24E57D0A026e0F7420D6559A7e651";

fn find_lagest_wallet(rpc_client: &RpcClient, owner: Pubkey, token_mint: Pubkey) -> Pubkey {
    rpc_client
        .get_token_accounts_by_owner(&owner, TokenAccountsFilter::Mint(token_mint))
        .expect("Failed to get wSol wallets")
        .iter()
        .max_by_key(|wallet| wallet.account.lamports)
        .map(|wallet| Pubkey::from_str(wallet.pubkey.as_str()).expect("Failed to parse wallet"))
        .expect("There are no payer wallets")
}

fn main() {
    let rpc_client: RpcClient = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    let payer = mocks::get_config_keypair();
    let message = mocks::get_echo_external_call(rand::thread_rng().gen::<[u8; 32]>().into())
        .expect("Failed to create message");

    let token_mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();

    let wallet = find_lagest_wallet(&rpc_client, payer.pubkey(), token_mint);

    let budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(230000);

    let program_sender = Pubkey::find_program_address(&[PROGRAM_SENDER_SEED], &EXAMPLE_ID).0;

    let program_sender_wallet = Pubkey::find_program_address(
        &[
            program_sender.as_ref(),
            spl_token::ID.as_ref(),
            token_mint.as_ref(),
        ],
        &spl_associated_token_account::ID,
    )
    .0;

    let create_wallet =
        spl_associated_token_account::instruction::create_associated_token_account_idempotent(
            &payer.pubkey(),
            &program_sender,
            &token_mint,
            &spl_token::ID,
        );

    let ix = Instruction {
        program_id: EXAMPLE_ID,
        accounts: SendViaDebridgeWithSender {
            program_sender,
            program_sender_wallet,
        }
        .to_account_metas(None)
        .into_iter()
        .chain(
            get_send_account_with_creator(
                payer.pubkey(),
                wallet,
                sha3::Keccak256::hash(message.as_slice()),
                program_sender,
            )
            .into_iter(),
        )
        .collect(),
        data: SendMessageViaDebridgeWithProgramSender {
            execution_fee: 0,
            message,
            fallback_address: hex::decode(FALLBACK_ADDRESS)
                .expect("Failed to decode fallback address"),
            receiver: hex::decode(ECHO_CONTRACT_ADDRESS).expect("Failed to decode receiver"),
            target_chain_id: POLYGON_CHAIN_ID,
        }
        .data(),
    };

    let signature = rpc_client
        .send_transaction(&Transaction::new_signed_with_payer(
            &[budget_ix, create_wallet, ix],
            Some(&payer.pubkey()),
            &[&payer],
            rpc_client
                .get_latest_blockhash()
                .expect("Failed to get blockhash"),
        ))
        .expect("Failed to send transaction");

    println!("Success! Transaction signature: {:?}", signature);
}
