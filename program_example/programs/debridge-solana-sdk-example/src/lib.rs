#![allow(clippy::result_large_err)]

use anchor_lang::{prelude::*, solana_program::sysvar};
use debridge_sdk::{
    check_claiming::check_execution_context,
    common::{get_chain_native_fix_fee, is_chain_supported},
    sending::{invoke_debridge_send, SendIx},
};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod debridge_invoke_example {

    #[error_code]
    pub enum ErrorCode {
        ChainNotSupported,
    }

    use super::*;

    /// Debridge protocol allow transfer liqudity from Solana to other supported chains
    /// To send some token to other supported chain use [`debridge_sdk::sending::invoke_debridge_send`]
    ///
    /// To check if the network is supported use [`debridge_sdk::common::is_chain_supported`]
    pub fn send_via_debridge(
        ctx: Context<SendViaDebridge>,
        amount: u64,
        target_chain_id: [u8; 32],
        receiver: Vec<u8>,
        is_use_asset_fee: bool,
    ) -> Result<()> {
        if !is_chain_supported(target_chain_id, ctx.remaining_accounts) {
            return Err(ErrorCode::ChainNotSupported.into());
        }

        let send_ix = SendIx {
            target_chain_id,
            receiver,
            is_use_asset_fee,
            amount,
            submission_params: None,
            referral_code: None,
        };
        invoke_debridge_send(send_ix, ctx.remaining_accounts).map_err(|err| err.into())
    }

    /// Debridge protocol take fix fee and transfer fee while sending liqudity.
    /// The fix fee by default is taken in native solana tokens.
    /// The default native fix fee amount is setted in state account but it can setted custom native
    /// fix amount for a specific chain in chain support info account.
    ///
    /// To get native fix fee amount for specific chain use [`debridge_sdk::common::get_chain_native_fix_fee`]
    pub fn send_via_debridge_with_native_fixed_fee(
        ctx: Context<SendViaDebridge>,
        amount: u64,
        target_chain_id: [u8; 32],
        receiver: Vec<u8>,
    ) -> Result<()> {
        let send_ix = SendIx {
            target_chain_id,
            receiver,
            /// Set this parameter to [`false`] value to pay fix fees in native solana tokens
            is_use_asset_fee: false,
            amount,
            submission_params: None,
            referral_code: None,
        };

        invoke_debridge_send(send_ix, ctx.remaining_accounts).map_err(|err| err.into())
    }

    pub fn send_via_debridge_with_asset_fixed_fee(
        ctx: Context<SendViaDebridge>,
        amount: u64,
        target_chain_id: [u8; 32],
        receiver: Vec<u8>,
    ) -> Result<()> {
        let send_ix = SendIx {
            target_chain_id,
            receiver,
            /// Set this parameter to [`true`] value to pay fix fees in bridged tokens
            is_use_asset_fee: true,
            amount,
            submission_params: None,
            referral_code: None,
        };

        invoke_debridge_send(send_ix, ctx.remaining_accounts).map_err(|err| err.into())
    }

    pub fn check_claiming(
        ctx: Context<CheckClaiming>,
        source_chain_id: [u8; 32],
        native_sender: Option<Vec<u8>>,
    ) -> Result<()> {
        check_execution_context(
            &ctx.accounts.instructions,
            &ctx.accounts.submission,
            &ctx.accounts.submission_authority,
            source_chain_id,
            native_sender,
        )
        .map_err(|err| err.into())
    }
}

#[derive(Accounts)]
pub struct SendViaDebridge<'info> {
    submission: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CheckClaiming<'info> {
    submission: AccountInfo<'info>,
    submission_authority: AccountInfo<'info>,
    #[account(address = sysvar::instructions::ID)]
    instructions: AccountInfo<'info>,
}
