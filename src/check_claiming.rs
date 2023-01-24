use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

//TODO: Fill it
const DEBRIDGE_ID: Pubkey = Pubkey::new_from_array([0; 32]);
const EXECUTE_EXTERNAL_CALL_DISCRIMINATOR: [u8; 8] = [0; 8];
const SUBMISSION_ACCOUNT_DISCRIMINATOR: [u8; 8] = [0; 8];

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SubmissionAccount {
    pub claimer: Pubkey,
    pub receiver: Pubkey,
    pub fallback_address: Pubkey,
    pub token_mint: Pubkey,
    pub native_sender: Option<Vec<u8>>,
    pub source_chain_id: [u8; 32],
    pub bump: u8,
}

impl TryFrom<&AccountInfo<'_>> for SubmissionAccount {
    type Error = ProgramError;

    fn try_from(account_info: &AccountInfo) -> Result<Self, Self::Error> {
        let borrow_data = account_info.try_borrow_data()?;
        let (discriminator, data) = borrow_data.split_at(8);

        if discriminator.ne(&SUBMISSION_ACCOUNT_DISCRIMINATOR) {
            return Err(Error::WrongSubmissionAccountDiscriminator.into());
        }

        SubmissionAccount::try_from_slice(data)
            .map_err(|_| Error::SubmissionDeserializeError.into())
    }
}

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
}

impl From<Error> for ProgramError {
    fn from(e: Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}

pub fn check_execution_context(
    instructions: &AccountInfo,
    submission: &AccountInfo,
    submission_authority: &AccountInfo,
    source_chain_id: [u8; 32],
    native_sender: Option<Vec<u8>>,
) -> ProgramResult {
    use solana_program::sysvar::instructions;

    let current_ix = instructions::load_instruction_at_checked(
        instructions::load_current_index_checked(instructions)? as usize,
        instructions,
    )?;

    if current_ix.program_id != DEBRIDGE_ID {
        msg!(
            "Expected: {}, Actual: {}",
            DEBRIDGE_ID,
            current_ix.program_id
        );
        return Err(Error::WrongClaimParentProgramId.into());
    }

    if !current_ix
        .data
        .starts_with(&EXECUTE_EXTERNAL_CALL_DISCRIMINATOR)
    {
        msg!(
            "Expected: {}, Actual: {}",
            hex::encode(EXECUTE_EXTERNAL_CALL_DISCRIMINATOR),
            hex::encode(current_ix.data.iter().take(8).copied().collect::<Vec<_>>()),
        );
        return Err(Error::WrongClaimParentInstruction.into());
    }

    let submission_key_from_ix = current_ix
        .accounts
        .get(5)
        .ok_or(Error::WrongClaimParentInstructionAccounts)?;

    if submission.key.ne(&submission_key_from_ix.pubkey) {
        msg!(
            "Expected: {}, Actual: {}",
            submission.key,
            submission_key_from_ix.pubkey,
        );
        return Err(Error::WrongClaimParentSubmission.into());
    }

    let submission_auth_key = current_ix
        .accounts
        .get(6)
        .ok_or(Error::WrongClaimParentInstructionAccounts)?;

    if submission_authority.key.ne(&submission_auth_key.pubkey) {
        msg!(
            "Expected: {}, Actual: {}",
            submission_auth_key.pubkey,
            submission_authority.key,
        );
        return Err(Error::WrongClaimParentSubmissionAuth.into());
    }

    if !submission_authority.is_signer {
        msg!(
            "Expected submission_authority is signer. Submission auth: {}",
            submission_authority.key
        )
    }

    let submission_account = SubmissionAccount::try_from(submission)?;

    if submission_account.source_chain_id.ne(&source_chain_id) {
        msg!(
            "Expected: {:?}, Actual: {:?}",
            source_chain_id,
            submission_account.source_chain_id
        );
        return Err(Error::WrongClaimParentNativeSender.into());
    }

    if native_sender
        .as_ref()
        .and_then(|sender| {
            submission_account
                .native_sender
                .as_ref()
                .map(|submission_sender| submission_sender.ne(sender))
        })
        .unwrap_or(false)
    {
        msg!(
            "Expected: {}, Actual: {}",
            &native_sender
                .as_ref()
                .map(hex::encode)
                .unwrap_or_else(|| "None".to_owned()),
            &submission_account
                .native_sender
                .as_ref()
                .map(hex::encode)
                .unwrap_or_else(|| "None".to_owned())
        );
        return Err(Error::WrongClaimParentNativeSender.into());
    }

    Ok(())
}
