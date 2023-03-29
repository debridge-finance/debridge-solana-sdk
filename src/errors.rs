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
    #[error("Provided wrong setting program id")]
    WrongSettingProgramId,
    #[error("Provided wrong debridge program id")]
    WrongDebridgeProgramId,
    #[error("Provided external storage with wrong. External storage have to be not initialized or be in Transferred state")]
    ExternalStorageWrongState,
    #[error("Failed to get rent")]
    FailedToGetRent,
    #[error("Wrong parent debridge-submission claimer. This method must be called by debridge program in execute_external call")]
    WrongClaimParentClaimer,
    #[error("Wrong parent debridge-submission receiver. This method must be called by debridge program in execute_external call")]
    WrongClaimParentReceiver,
    #[error("Wrong parent debridge-submission fallback address. This method must be called by debridge program in execute_external call")]
    WrongClaimParentFallbackAddress,
    #[error("Wrong parent debridge-submission token mint. This method must be called by debridge program in execute_external call")]
    WrongClaimParentTokenMint,
    #[error("Wrong parent debridge-submission source chain id. This method must be called by debridge program in execute_external call")]
    WrongClaimParentSourceChainId,
    #[error("Wrong parent debridge-submission account key. This method must be called by debridge program in execute_external call")]
    WrongClaimParentSubmissionAccountKey,
    #[error("Wrong parent debridge-submission native sender. This method must be called by debridge program in execute_external call")]
    WrongClaimParentNativeSender,
}

use solana_program::program_error::ProgramError;

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
