use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::instructions;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
};

use crate::{
    debridge_accounts::{SubmissionAccount, TryFromAccount, EXECUTE_EXTERNAL_CALL_DISCRIMINATOR},
    Error, DEBRIDGE_ID,
};

impl From<Error> for ProgramError {
    fn from(e: Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}

pub fn get_program_id(instructions: &AccountInfo) -> Result<Pubkey, ProgramError> {
    instructions::load_instruction_at_checked(
        instructions::load_current_index_checked(instructions)? as usize,
        instructions,
    )
    .map(|ix| ix.program_id)
}

pub fn get_key_by_index(current_ix: &Instruction, index: usize) -> Result<Pubkey, ProgramError> {
    Ok(current_ix
        .accounts
        .get(index)
        .map(|account_meta| account_meta.pubkey)
        .ok_or(Error::WrongClaimParentInstructionAccounts)?)
}

pub fn get_submission_key(current_ix: &Instruction) -> Result<Pubkey, ProgramError> {
    get_key_by_index(current_ix, 5)
}

pub fn get_submission_auth(current_ix: &Instruction) -> Result<Pubkey, ProgramError> {
    get_key_by_index(current_ix, 6)
}

pub fn check_execute_ix(current_ix: &Instruction) -> ProgramResult {
    if !current_ix
        .data
        .starts_with(&EXECUTE_EXTERNAL_CALL_DISCRIMINATOR)
    {
        msg!(
            "Expected: {}, Actual: {}",
            hex::encode(EXECUTE_EXTERNAL_CALL_DISCRIMINATOR),
            hex::encode(current_ix.data.iter().take(8).copied().collect::<Vec<_>>()),
        );
        Err(Error::WrongClaimParentInstruction.into())
    } else {
        Ok(())
    }
}

/// Check that current instruction is called with [`solana_program::program::invoke_signed`]
/// function during Debridge's `execute_external_call` instruction call.
/// Also this function can check `native_sender` (user who call send function in source chain).
///
/// # Arguments
/// * `instructions` - [`solana_program::sysvar::instructions::ID`] account for previous instruction checking
/// * `submission` - Debridge account contains submission claiming information
/// * `submission_authority` - Debridge authority with sign proof invoking from `execute_external_call` instruction  
/// * `source_chain_id` - the source chain from which the send was made
/// * `native_sender` - initiator address of Debridge Send function call in source chain
pub fn check_execution_context(
    instructions: &AccountInfo,
    submission: &AccountInfo,
    submission_authority: &AccountInfo,
    source_chain_id: [u8; 32],
    native_sender: Option<Vec<u8>>,
) -> ProgramResult {
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

    check_execute_ix(&current_ix)?;

    let submission_key_from_ix = get_submission_key(&current_ix)?;

    if submission.key.ne(&submission_key_from_ix) {
        msg!(
            "Expected: {}, Actual: {}",
            submission.key,
            submission_key_from_ix,
        );
        return Err(Error::WrongClaimParentSubmission.into());
    }

    let submission_auth_key = get_submission_auth(&current_ix)?;

    if submission_authority.key.ne(&submission_auth_key) {
        msg!(
            "Expected: {}, Actual: {}",
            submission_auth_key,
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

    let submission_account = SubmissionAccount::try_from_account(submission)?;

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
