use anchor_lang::prelude::*;
use debridge_sdk::sending::invoke_send;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod debridge_invoke_test {
    use anchor_lang::solana_program::program::invoke;
    use super::*;

    pub fn send_via_debridge(ctx: Context<SendViaDebridge>) -> Result<()> {

        invoke_send(ctx.remaining_accounts).map_err(|err| err.into())
    }

}

#[derive(Accounts)]
pub struct SendViaDebridge {}
