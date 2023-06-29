/*
 * Copyright (C) 2023 debridge
 *
 * This file is part of debridge-solana-sdk.
 *
 * debridge-solana-sdk is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * debridge-solana-sdk is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with debridge-solana-sdk. If not, see <https://www.gnu.org/licenses/>.
 */

use std::str::FromStr;

use anchor_lang::InstructionData;
use debridge_solana_sdk::{HashAdapter, POLYGON_CHAIN_ID};
use debridge_solana_sdk_example_program::{
    instruction::SendViaDebridgeWithExecutionFee, ID as EXAMPLE_ID,
};
use solana_client::{rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use solana_program::instruction::Instruction;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction, pubkey::Pubkey, signature::Signer,
    transaction::Transaction,
};

mod mocks;

fn main() {
    let rpc_client: RpcClient = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    let payer = mocks::get_config_keypair();

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
        accounts: mocks::get_send_account(
            payer.pubkey(),
            wallet,
            sha3::Keccak256::hash(message.as_slice()),
        )
        .to_vec(),
        data: SendViaDebridgeWithExecutionFee {
            amount: 100,
            receiver: hex::decode("cfcc66ee5397b7cdf7228f7502d1e168518c6bb3")
                .expect("Failed to decode receiver"),
            target_chain_id: POLYGON_CHAIN_ID,
            execution_fee: 10,
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