#![allow(clippy::result_large_err)]

use anchor_lang::{prelude::*, solana_program::sysvar};
use debridge_sdk::{
    check_claiming::check_execution_context,
    sending::{invoke_send, SendIx},
};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod debridge_invoke_example {

    use super::*;

    pub fn send_via_debridge(ctx: Context<SendViaDebridge>, send_ix: SendIx) -> Result<()> {
        invoke_send(send_ix, ctx.remaining_accounts).map_err(|err| err.into())
    }

    pub fn check_claiming(
        ctx: Context<CheckClaiming>,
        native_sender: Option<Vec<u8>>,
    ) -> Result<()> {
        check_execution_context(
            &ctx.accounts.instructions,
            &ctx.accounts.submission,
            &ctx.accounts.submission_authority,
            native_sender,
        )
        .map_err(|err| err.into())
    }
}

#[derive(Accounts)]
pub struct SendViaDebridge {}

#[derive(Accounts)]
pub struct CheckClaiming<'info> {
    submission: AccountInfo<'info>,
    submission_authority: AccountInfo<'info>,
    #[account(address = sysvar::instructions::ID)]
    instructions: AccountInfo<'info>,
}
