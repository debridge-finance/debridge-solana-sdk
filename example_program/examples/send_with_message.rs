use std::{env, str::FromStr};

use anchor_lang::InstructionData;
use debridge_solana_sdk::HashAdapter;
use debridge_solana_sdk_example::{instruction::SendMessageViaDebridge, ID as EXAMPLE_ID};
use solana_client::{rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use solana_program::instruction::Instruction;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction, pubkey::Pubkey, signature::Signer,
    signer::keypair::read_keypair_file, transaction::Transaction,
};

use crate::mocks::get_send_acount;

mod mocks;

fn main() {
    let rpc_client: RpcClient = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    let payer = read_keypair_file(env!("KEYPAIR_PATH")).expect("Failed to parse payer keypair");

    let message = hex::decode("a69b6ed0000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000050011223344556600000000000000000000000000000000000000000000000000").expect("Failed to decode external code");

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
        .expect("Failed to create send accounts list")
        .to_vec(),
        data: SendMessageViaDebridge {
            execution_fee: 0,
            message,
            fallback_address: hex::decode("bd1e72155Ce24E57D0A026e0F7420D6559A7e651")
                .expect("Failed to decode fallback address"),
            receiver: hex::decode("cfcc66ee5397b7cdf7228f7502d1e168518c6bb3")
                .expect("Failed to decode receiver"),
            target_chain_id: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 137,
            ],
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
