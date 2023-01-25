use std::{env, str::FromStr};

use lazy_static::lazy_static;
use solana_program::pubkey::Pubkey;

pub mod check_claiming;
pub mod sending;

lazy_static! {
    pub static ref DEBRIDGE_ID: Pubkey =
        Pubkey::from_str(
            env!("DEBRIDGE_PROGRAM_PUBKEY")
        )
        .expect("Failed to parse debridge program id. Please set in DEBRIDGE_PROGRAM_PUBKEY debridge prorgam id in base58 encoding");
}

const EXECUTE_EXTERNAL_CALL_DISCRIMINATOR: [u8; 8] = [160, 89, 229, 51, 157, 62, 217, 174];
const SUBMISSION_ACCOUNT_DISCRIMINATOR: [u8; 8] = [254, 14, 34, 50, 170, 36, 60, 191];
const SEND_DISCRIMINATOR: [u8; 8] = [102, 251, 20, 187, 65, 75, 12, 69];

#[derive(Debug, thiserror::Error)]
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
    #[error("Failed while submission account deserializing")]
    SubmissionDeserializeError,
    #[error("Provided submssion account with wrong discriminator")]
    WrongSubmissionAccountDiscriminator,
    #[error("Provided wrong debridge program id")]
    WrongDebridgeProgram,
}
