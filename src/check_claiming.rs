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

use solana_program::{
    account_info::AccountInfo, instruction::Instruction, msg, program_error::ProgramError,
    pubkey::Pubkey, sysvar, sysvar::instructions,
};

use some_to_err::*;

use crate::{
    debridge_accounts::{SubmissionAccount, TryFromAccount, EXECUTE_EXTERNAL_CALL_DISCRIMINATOR},
    Error, DEBRIDGE_ID,
};

impl From<Error> for ProgramError {
    fn from(e: Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}

pub fn get_current_instruction_program_id(
    instructions: &AccountInfo,
) -> Result<Pubkey, ProgramError> {
    instructions::load_instruction_at_checked(
        instructions::load_current_index_checked(instructions)? as usize,
        instructions,
    )
    .map(|ix| ix.program_id)
}

pub fn get_pubkey_by_index(current_ix: &Instruction, index: usize) -> Result<Pubkey, Error> {
    current_ix
        .accounts
        .get(index)
        .map(|account_meta| account_meta.pubkey)
        .ok_or(Error::WrongClaimParentInstructionAccounts)
}

pub struct ValidatedExecuteExtCallIx(Instruction);

impl ValidatedExecuteExtCallIx {
    /// This function attempts to load the current instruction from the instructions sysvar and convert it into a Instruction struct. If successful, it returns the Instruction.
    ///
    /// # Parameters
    ///
    /// - `instructions_sysvar: &AccountInfo` - The account information for the system instructions account.
    ///
    /// # Returns
    ///
    /// A `Result<Self, ProgramError>` containing either the `Instruction` struct if successful, or a `ProgramError` if there was an issue loading or converting the instruction.
    ///
    /// # Errors
    ///
    /// This function may return a `ProgramError` under the following conditions:
    ///
    /// - The key of the `instructions_sysvar` account does not match the system instructions account ID.
    /// - There was an issue loading the current instruction from the `instructions_sysvar`.
    /// - There was an issue converting the loaded instruction into a `Instruction` struct.
    pub fn try_from_current_ix(instructions_sysvar: &AccountInfo) -> Result<Self, ProgramError> {
        if instructions_sysvar.key != &sysvar::instructions::ID {
            return Err(ProgramError::IncorrectProgramId);
        }

        Ok(Self::try_from(instructions::load_instruction_at_checked(
            instructions::load_current_index_checked(instructions_sysvar)? as usize,
            instructions_sysvar,
        )?)?)
    }
    pub fn get_submission_key(&self) -> Result<Pubkey, Error> {
        get_pubkey_by_index(&self.0, 5)
    }

    pub fn get_submission_auth(&self) -> Result<Pubkey, Error> {
        get_pubkey_by_index(&self.0, 6)
    }

    pub fn validate_submission_auth(&self, candidate: &Pubkey) -> Result<(), Error> {
        self.get_submission_auth()?
            .ne(candidate)
            .then_some(Error::SubmissionAuthValidationFailed)
            .err_or(())
    }

    /// Validates that the provided submission account matches the expected values according
    /// to the provided `SubmissionAccountValidation` object. If any of the expected values are
    /// incorrect, returns an error indicating which field did not match.
    ///
    /// # Arguments
    ///
    /// * `submission` - A reference to the account info of the submission account to validate.
    /// * `validators` - The `SubmissionAccountValidation` object containing the expected values
    ///                  for each field in the submission account.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the expected fields in the submission account do not match their
    /// expected values according to the provided `SubmissionAccountValidation` object.
    pub fn validate_submission_account(
        self,
        submission: &AccountInfo,
        validators: SubmissionAccountValidation,
    ) -> Result<(), Error> {
        if submission.key != &self.get_submission_key()? {
            msg!("Invalid submission account key");
            return Err(Error::WrongClaimParentSubmissionAccountKey);
        }

        let SubmissionAccount {
            claimer,
            receiver,
            fallback_address,
            token_mint,
            native_sender,
            source_chain_id,
            bump: _,
        } = SubmissionAccount::try_from_account(submission)?;

        let SubmissionAccountValidation {
            claimer_validation,
            receiver_validation,
            fallback_address_validation,
            token_mint_validation,
            native_sender_validation,
            source_chain_id_validation,
        } = validators;

        if let Some(expected_claimer) = claimer_validation {
            if claimer != expected_claimer {
                msg!(
                    "Expected claimer: {}, Actual claimer: {}",
                    expected_claimer,
                    claimer
                );
                return Err(Error::WrongClaimParentClaimer);
            }
        }

        if let Some(expected_receiver) = receiver_validation {
            if receiver != expected_receiver {
                msg!(
                    "Expected receiver: {}, Actual receiver: {}",
                    expected_receiver,
                    receiver
                );
                return Err(Error::WrongClaimParentReceiver);
            }
        }

        if let Some(expected_fallback_address) = fallback_address_validation {
            if fallback_address != expected_fallback_address {
                msg!(
                    "Expected fallback_address: {}, Actual fallback_address: {}",
                    expected_fallback_address,
                    fallback_address
                );
                return Err(Error::WrongClaimParentFallbackAddress);
            }
        }

        if let Some(expected_token_mint) = token_mint_validation {
            if token_mint != expected_token_mint {
                msg!(
                    "Expected token_mint: {}, Actual token_mint: {}",
                    expected_token_mint,
                    token_mint
                );
                return Err(Error::WrongClaimParentTokenMint);
            }
        }

        if let Some(expected_native_sender) = native_sender_validation {
            if native_sender.as_ref() != Some(&expected_native_sender) {
                msg!(
                    "Expected native_sender: {}, Actual native_sender: {:?}",
                    hex::encode(&expected_native_sender),
                    native_sender.map(hex::encode)
                );
                return Err(Error::WrongClaimParentNativeSender);
            }
        }

        if let Some(expected_source_chain_id) = source_chain_id_validation {
            if source_chain_id != expected_source_chain_id {
                msg!(
                    "Expected source_chain_id: {:?}, Actual source_chain_id: {:?}",
                    expected_source_chain_id,
                    source_chain_id
                );
                return Err(Error::WrongClaimParentSourceChainId);
            }
        }

        Ok(())
    }
}

impl TryFrom<Instruction> for ValidatedExecuteExtCallIx {
    type Error = Error;

    /// Tries to convert the given instruction to a ValidatedExecuteExtCallIx instance.
    ///
    /// If successful, returns a ValidatedExecuteExtCallIx instance wrapped in Result::Ok.
    /// Otherwise, returns a WrongClaimParentProgramId error if the program ID of the instruction
    /// is not DEBRIDGE_ID, or a WrongClaimParentInstruction error if the instruction data does
    /// not start with the EXECUTE_EXTERNAL_CALL_DISCRIMINATOR.
    ///
    /// # Arguments
    ///
    /// * ix - The instruction to validate.
    ///
    /// # Returns
    ///
    /// A Result containing either a ValidatedExecuteExtCallIx instance if successful, or
    /// a WrongClaimParentProgramId or WrongClaimParentInstruction error if validation fails.
    fn try_from(ix: Instruction) -> Result<Self, Self::Error> {
        if ix.program_id != DEBRIDGE_ID {
            msg!("Expected: {}, Actual: {}", DEBRIDGE_ID, ix.program_id);
            return Err(Error::WrongClaimParentProgramId);
        }

        if !ix.data.starts_with(&EXECUTE_EXTERNAL_CALL_DISCRIMINATOR) {
            msg!(
                "Expected: {}, Actual: {}",
                hex::encode(EXECUTE_EXTERNAL_CALL_DISCRIMINATOR),
                hex::encode(&ix.data[..8])
            );
            Err(Error::WrongClaimParentInstruction)
        } else {
            Ok(ValidatedExecuteExtCallIx(ix))
        }
    }
}

#[derive(Debug, derive_builder::Builder)]
pub struct SubmissionAccountValidation {
    /// Pubkey claimed this transaction on the Solana network
    pub claimer_validation: Option<Pubkey>,
    /// The receiver of this debridge-transaction
    pub receiver_validation: Option<Pubkey>,
    /// The key that gives the right to cancel the transfer in the receiving network
    pub fallback_address_validation: Option<Pubkey>,
    /// The address of the token that was transferred to the given submission
    pub token_mint_validation: Option<Pubkey>,
    /// Sending chain address of the sender of the message
    pub native_sender_validation: Option<Vec<u8>>,
    /// Sending chain id
    pub source_chain_id_validation: Option<[u8; 32]>,
}
