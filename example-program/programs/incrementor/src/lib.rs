// Because anchor
#![allow(clippy::result_large_err)]

use anchor_lang::{prelude::*, solana_program::sysvar};
use debridge_solana_sdk::check_claiming;

declare_id!("RsJUECkXgzsAkFxXgjuaHwxwzZZ1rbckyWnu8xYwZN4");

#[program]
pub mod incrementor {
    use super::*;

    #[derive(Accounts)]
    pub struct InrecementGlobalState<'info> {
        #[account(
            init_if_needed,
            seeds = [
                b"STATE"
            ],
            bump,
            space = 8 + 8,
            payer = payer,
        )]
        global_state: Account<'info, GlobalState>,
        #[account(mut)]
        payer: Signer<'info>,
        #[account(address = sysvar::instructions::ID)]
        instructions: AccountInfo<'info>,
        system_program: Program<'info, System>,
    }
    pub fn increment_global_state(ctx: Context<InrecementGlobalState>) -> Result<()> {
        let validated_execute_ext_call =
            check_claiming::ValidatedExecuteExtCallIx::try_from_current_ix(
                &ctx.accounts.instructions,
            )?;

        validated_execute_ext_call
            .validate_submission_auth(ctx.accounts.payer.key)
            .map_err(|err| {
                msg!("Error while check signer == submission auth: {}", err);
                ProgramError::InvalidAccountData
            })?;

        ctx.accounts.global_state.value += 1;

        Ok(())
    }

    #[derive(Accounts)]
    #[instruction(
        native_sender: [u8; 20],
    )]
    pub struct IncrementUserState<'info> {
        #[account(
            init_if_needed,
            seeds = [
                b"USER",
                native_sender.as_slice(),
            ],
            bump,
            space = 8 + 8,
            payer = payer,
        )]
        user_state: Account<'info, UserState>,
        submission: AccountInfo<'info>,
        #[account(mut)]
        payer: Signer<'info>,
        #[account(address = sysvar::instructions::ID)]
        instructions: AccountInfo<'info>,
        system_program: Program<'info, System>,
    }
    pub fn increment_user_state(
        ctx: Context<IncrementUserState>,
        native_sender: [u8; 20],
    ) -> Result<()> {
        let validated_execute_ext_call =
            check_claiming::ValidatedExecuteExtCallIx::try_from_current_ix(
                &ctx.accounts.instructions,
            )?;

        validated_execute_ext_call
            .validate_submission_auth(ctx.accounts.payer.key)
            .map_err(|err| {
                msg!("Error while check signer == submission auth: {}", err);
                ProgramError::InvalidAccountData
            })?;

        // Validate submission account & native sender
        validated_execute_ext_call
            .validate_submission_account(
                &ctx.accounts.submission,
                check_claiming::SubmissionAccountValidation {
                    native_sender_validation: Some(native_sender.to_vec()),

                    // You can validate either part of the context or the whole context
                    source_chain_id_validation: None,
                    claimer_validation: None,
                    receiver_validation: None,
                    fallback_address_validation: None,
                    token_mint_validation: None,
                },
            )
            .map_err(|err| {
                msg!("Error while check debridge execution context: {}", err);
                ProgramError::InvalidArgument
            })?;

        ctx.accounts.user_state.value += 1;

        Ok(())
    }

    #[derive(Accounts)]
    #[instruction(
        native_sender: [u8; 20],
    )]
    pub struct MultiplyStates<'info> {
        #[account(
            seeds = [
                b"USER",
                native_sender.as_slice(),
            ],
            bump,
        )]
        user_state: Account<'info, UserState>,
        #[account(
            seeds = [
                b"STATE"
            ],
            bump,
        )]
        global_state: Account<'info, GlobalState>,
    }
    pub fn multiply_states(ctx: Context<MultiplyStates>) -> Result<()> {
        let global = ctx.accounts.global_state.value;
        let user = ctx.accounts.user_state.value;

        emit!(Multiplied {
            multiplication: global * user,
            global,
            user,
        });

        Ok(())
    }
}

#[event]
struct Multiplied {
    global: u64,
    user: u64,
    multiplication: u64,
}

#[account]
pub struct GlobalState {
    value: u64,
}

#[account]
pub struct UserState {
    value: u64,
}
