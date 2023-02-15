extern crate core;

use std::env;

use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

pub mod check_claiming;
pub mod debridge_accounts;
pub mod keys;
pub mod reserved_flags;
pub mod sending;

//TODO: Create pubkey with procedure macros
const DEBRIDGE_ID_RAW: &str = env!("DEBRIDGE_PROGRAM_PUBKEY");
const SETTINGS_ID_RAW: &str = env!("SETTINGS_PROGRAM_PUBKEY");
const EXECUTE_EXTERNAL_CALL_DISCRIMINATOR: [u8; 8] = [160, 89, 229, 51, 157, 62, 217, 174];
const SEND_DISCRIMINATOR: [u8; 8] = [102, 251, 20, 187, 65, 75, 12, 69];
const INIT_EXTERNAL_CALL_DISCRIMINATOR: [u8; 8] = [82, 77, 58, 138, 145, 157, 41, 253];

pub const SOLANA_CHAIN_ID: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 115,
    111, 108,
];
pub const BPS_DENOMINATOR: u64 = 10000_u64;

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
    #[error("Provided ChainSupportInfo for other target chain id. Please create account list with debridge sdk")]
    WrongChainSupportInfo,
    #[error("Provided target chain id not supported")]
    TargetChainNotSupported,
    #[error("Provided BridgeFee for other target chain id or other token mint. Please create account list with debridge sdk")]
    WrongBridgeFeeInfo,
    #[error("Failed to find state account in provided accounts. Please create account list with debridge sdk")]
    WrongState,
    #[error("Failed to borrow account data")]
    AccountBorrowFailing,
    #[error("Asset fee not supported")]
    AssetFeeNotSupported,
    #[error("Amount too big for sending. Adding fee overflow max sending amount")]
    AmountOverflowedWhileAddingFee,
    #[error("Wrong ")]
    WrongSettingProgramId,
    #[error("Provided external storage with wrong. External storage have to be not initialized or be in Transferred state")]
    ExternalStorageWrongState,
}

pub enum InvokeError {
    SdkError(Error),
    SolanaProgramError(ProgramError),
}

impl From<Error> for InvokeError {
    fn from(err: Error) -> Self {
        InvokeError::SdkError(err)
    }
}

impl From<ProgramError> for InvokeError {
    fn from(err: ProgramError) -> Self {
        InvokeError::SolanaProgramError(err)
    }
}

impl From<InvokeError> for ProgramError {
    fn from(err: InvokeError) -> Self {
        match err {
            InvokeError::SdkError(err) => err.into(),
            InvokeError::SolanaProgramError(err) => err,
        }
    }
}

pub trait HashAdapter {
    fn hash(input: &[u8]) -> [u8; 32];
}

impl HashAdapter for sha3::Keccak256 {
    fn hash(input: &[u8]) -> [u8; 32] {
        use sha3::Digest;
        Self::digest(input).as_slice().try_into().unwrap()
    }
}

pub fn get_debridge_id() -> Pubkey {
    Pubkey::from_str(DEBRIDGE_ID_RAW).unwrap()
}

pub fn get_settings_id() -> Pubkey {
    Pubkey::from_str(SETTINGS_ID_RAW).unwrap()
}
