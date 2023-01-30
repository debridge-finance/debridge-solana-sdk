extern crate core;

use std::{env, str::FromStr};

use lazy_static::lazy_static;
use solana_program::pubkey::Pubkey;

pub mod check_claiming;
pub mod debridge_accounts;
pub mod sending;

lazy_static! {
    pub static ref DEBRIDGE_ID: Pubkey =
        Pubkey::from_str(
            env!("DEBRIDGE_PROGRAM_PUBKEY")
        )
        .expect("Failed to parse debridge program id. Please set in DEBRIDGE_PROGRAM_PUBKEY debridge prorgam id in base58 encoding");
}

const EXECUTE_EXTERNAL_CALL_DISCRIMINATOR: [u8; 8] = [160, 89, 229, 51, 157, 62, 217, 174];
const SEND_DISCRIMINATOR: [u8; 8] = [102, 251, 20, 187, 65, 75, 12, 69];

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error(
        "Wrong parent ix. This method must be called by debridge program in execute_external call"
    )]
    WrongClaimParentInstruction,
    #[error("Wrong parent ix accounts. This method must be called by debridge program in execute_external call")]
    WrongClaimParentInstructionAccounts,
    #[error("Wrong parent ix submission. This method must be called by debridge program in execute_external call")]
    WrongClaimParentSubmission,
    #[error("Wrong parent debridge-submission authority. This method must be called by debridge program in execute_external call")]
    WrongClaimParentSubmissionAuth,
    #[error("Wrong parent debridge-submission native sender. This method must be called by debridge program in execute_external call")]
    WrongClaimParentNativeSender,
    #[error("Wrong parent debridge-submission source chain id. This method must be called by debridge program in execute_external call")]
    WrongClaimParentSourceChainId,
    #[error("Wrong parent ix program id. This method must be called by debridge program in execute_external call")]
    WrongClaimParentProgramId,
    #[error("Failed while account deserializing")]
    AccountDeserializeError,
    #[error("Provided account with wrong discriminator")]
    WrongAccountDiscriminator,
    #[error("Provided wrong debridge program id")]
    WrongDebridgeProgram,
    #[error("Account with such index not exist. Please create account list with debridge sdk")]
    WrongAccountIndex,
    #[error("Failed to find state account in provided accounts. Please create account list with debridge sdk")]
    WrongState,
    #[error("Failed to borrow account data")]
    AccountBorrowFailing,
    #[error("Asset fee not supported")]
    AssetFeeNotSupported,
}
