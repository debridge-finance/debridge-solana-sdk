#![allow(clippy::result_large_err)]

use anchor_lang::{error::Error as AnchorError, prelude::*, solana_program::sysvar};
use debridge_sdk::{
    check_claiming::check_execution_context,
    sending::{
        add_all_fees, invoke_debridge_send, is_chain_supported, SendIx, SendSubmissionParamsInput,
    },
};
declare_id!("5UaXbex7paiRDykrN2GaRPW7j7goEQ1ZWqQvUwnAfFTF");

#[program]
pub mod debridge_invoke_example {

    #[error_code]
    pub enum ErrorCode {
        ChainNotSupported,
        ChainSupportInfoDeserializingFailed,
        MatchOverflowWhileCalculateInputAmount,
        FailedToCalculateAmountWithFee,
    }

    use debridge_sdk::sending::invoke_init_external_call;

    use super::*;

    /// Debridge protocol allows transfer liqudity from Solana to other supported chains
    /// To send some token to other supported chain use [`debridge_sdk::sending::invoke_debridge_send`]
    ///
    /// To check if the network is supported use [`debridge_sdk::sending::is_chain_supported`]
    pub fn send_via_debridge(
        ctx: Context<SendViaDebridge>,
        amount: u64,
        target_chain_id: [u8; 32],
        receiver: Vec<u8>,
        is_use_asset_fee: bool,
    ) -> Result<()> {
        if !is_chain_supported(ctx.remaining_accounts, target_chain_id).map_err(|err| {
            msg!(
                "Failed to deserialize chain support info account. Inner error: {}",
                err
            );
            ErrorCode::ChainSupportInfoDeserializingFailed
        })? {
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

    /// Debridge protocol takes fix fee and transfer fee while sending liqudity.
    /// The fix fee by default is taken in native solana tokens.
    /// The default native fix fee amount is setted in state account but it can setted custom native
    /// fix amount for a specific chain in chain support info account.
    ///
    /// To get default native fix fee amount use [`debridge_sdk::sending::get_default_native_fix_fee`]
    ///
    /// To get native fix fee amount for specific chain use [`debridge_sdk::sending::get_chain_native_fix_fee`]
    ///
    /// To use native fix fee set [`debridge_sdk::sending::SendIx`] `is_use_asset_fee` field to `false`
    pub fn send_via_debridge_with_native_fixed_fee(
        ctx: Context<SendViaDebridge>,
        amount: u64,
        target_chain_id: [u8; 32],
        receiver: Vec<u8>,
    ) -> Result<()> {
        let send_ix = SendIx {
            target_chain_id,
            receiver,
            is_use_asset_fee: false,
            amount,
            submission_params: None,
            referral_code: None,
        };

        invoke_debridge_send(send_ix, ctx.remaining_accounts).map_err(|err| err.into())
    }

    /// Debridge protocol takes fix fee and transfer fee while sending liqudity.
    /// The fix fee by default is taken in native solana tokens.
    /// But when transferring some tokens to certain networks, it is possible to pay in transferred tokens.
    /// It's called `asset_fix_fee`.
    ///
    /// To known `asset_fee` is avaliable use [`debridge_sdk::sending::is_asset_fee_avaliable`]
    ///
    /// To get asset fix fee amount for specific chain use [`debridge_sdk::sending::try_get_chain_asset_fix_fee`]
    ///
    /// To use asset fix fee set [`debridge_sdk::sending::SendIx`] `is_use_asset_fee` field to `true`
    pub fn send_via_debridge_with_asset_fixed_fee(
        ctx: Context<SendViaDebridge>,
        amount: u64,
        target_chain_id: [u8; 32],
        receiver: Vec<u8>,
    ) -> Result<()> {
        let send_ix = SendIx {
            target_chain_id,
            receiver,
            is_use_asset_fee: true,
            amount,
            submission_params: None,
            referral_code: None,
        };

        invoke_debridge_send(send_ix, ctx.remaining_accounts).map_err(|err| err.into())
    }

    /// Debridge protocol takes fix fee and transfer fee while sending liqudity.
    /// If needed to get exact amount tokens in target chain, all fees will need to be added to sending amount.
    ///
    /// There are three types of fees in Debridge protocol: fixed fee, transfer fee, execution fee.
    ///
    /// Fixed fee is fixed amount for any send instruction. It's named asset fixed fee. The amount depends on target chain.
    /// To get asset fix fee amount for specific chain use [`debridge_sdk::sending::try_get_chain_asset_fix_fee`]
    /// For some token fixed fee can be paid with sent tokens. In this case, you need to include this asset fixed fee in the final amount.
    ///
    /// Transfer fee is taken as part of sent tokens. To get the bps of transfer fee use [`debridge_sdk::sending::get_transfer_fee`]
    /// To add transfer fee to current amount use [`debridge_sdk::sending::add_transfer_fee`]
    ///
    /// Execution fee is reward for execute claim insctuction in target chain. It can be zero if you want to run the instruction yourself.
    /// The recommended execution fee can be obtained using debridge sdk.
    ///
    /// To add to exact amount all fees charged during the send use [`debridge_sdk::sending::add_all_fees`]
    pub fn send_via_debridge_with_exact_amount(
        ctx: Context<SendViaDebridge>,
        exect_amount: u64,
        target_chain_id: [u8; 32],
        receiver: Vec<u8>,
        execution_fee: u64,
        is_use_asset_fee: bool,
    ) -> Result<()> {
        let final_amount = add_all_fees(
            ctx.remaining_accounts,
            target_chain_id,
            exect_amount,
            execution_fee,
            is_use_asset_fee,
        )
        .map_err(|err| {
            msg!("Failed to add fees to amount. Inner error: {}", err);
            ErrorCode::FailedToCalculateAmountWithFee
        })?;

        let send_ix = SendIx {
            target_chain_id,
            receiver,
            is_use_asset_fee,
            amount: final_amount,
            submission_params: Some(SendSubmissionParamsInput::execution_fee_only(execution_fee)),
            referral_code: None,
        };

        invoke_debridge_send(send_ix, ctx.remaining_accounts).map_err(|err| err.into())
    }

    /// Debridge protocol allows to anyone execute claim transaction in target chain. It allow to create
    /// fluent user experience when user only send tokens to other chain and automatically receives in another.
    ///
    /// User adds execution fee as reward for execution his claim transaction in target chain.
    /// The recommended execution fee can be obtained using debridge sdk.
    pub fn send_via_debridge_with_execution_fee(
        ctx: Context<SendViaDebridge>,
        amount: u64,
        target_chain_id: [u8; 32],
        receiver: Vec<u8>,
        execution_fee: u64,
    ) -> Result<()> {
        let send_ix = SendIx {
            target_chain_id,
            receiver,
            is_use_asset_fee: false,
            amount,
            submission_params: Some(SendSubmissionParamsInput::execution_fee_only(execution_fee)),
            referral_code: None,
        };

        invoke_debridge_send(send_ix, ctx.remaining_accounts).map_err(|err| err.into())
    }

    /// Debridge protocol allows not only to send tokens to another network,
    /// but also to use them to call any smart contract.
    ///
    /// Used `external_call` for this. For evm-like network it will be address of smart contract function and function's arguments
    /// packed in byte vector.
    ///
    /// To use external call function needed to initialize external call storage with
    /// [`debridge_sdk::sending::invoke_init_external_call`] function and create `submission_params`
    /// with [`debridge_sdk::sending::SendSubmissionParamsInput::with_external_call`] function.
    /// Besides external call needed to provide `fallback_address`. The `fallback_address' will be used
    /// if external call fails. On this address token received in target chain will transfer.
    ///
    /// A `execution_fee` is reward reward that will received for execution claim transaction in
    /// target chain. It can be set zero if external call will be claimed by youself.
    pub fn send_via_debridge_with_external_call(
        ctx: Context<SendViaDebridge>,
        amount: u64,
        target_chain_id: [u8; 32],
        receiver: Vec<u8>,
        execution_fee: u64,
        fallback_address: Vec<u8>,
    ) -> Result<()> {
        //TODO: Add real external call
        let external_call = vec![];

        invoke_init_external_call(external_call.as_slice(), ctx.remaining_accounts)
            .map_err(AnchorError::from)?;

        let send_ix = SendIx {
            target_chain_id,
            receiver,
            is_use_asset_fee: false,
            amount,
            submission_params: Some(SendSubmissionParamsInput::with_external_call(
                external_call,
                execution_fee,
                fallback_address,
            )),
            referral_code: None,
        };

        invoke_debridge_send(send_ix, ctx.remaining_accounts).map_err(|err| err.into())
    }

    /// Debridge protocol allows call any smart contract in target chain without sending any tokens.
    /// You have to payed transfer fee for transfering execution fee in target chain.
    /// If you will claim by yourself then set execution fee to zero and don't pay transfer fee.
    /// Only fixed fee will be payed.
    ///
    /// To take into account the transfer fee use the [`debridge_sdk::sending::add_all_fees`] functions
    /// and provide zero amount and preferred execution fee and provide function's result
    /// as `amount` parameter to [`debridge_sdk::sending::SendIx`].
    ///
    /// Used `external_call` for this. For evm-like network it will be address of smart contract function and function's arguments
    /// packed in byte vector.
    ///
    /// To use external call function needed to initialize external call storage with
    /// [`debridge_sdk::sending::invoke_init_external_call`] function and create `submission_params`
    /// with [`debridge_sdk::sending::SendSubmissionParamsInput::with_external_call`] function.
    /// Besides external call needed to provide `fallback_address`. The `fallback_address' will be used
    /// if external call fails. On this address token received in target chain will transfer.
    ///
    /// A `execution_fee` is reward reward that will received for execution claim transaction in
    /// target chain. It can be set zero if external call will be claimed by youself.
    pub fn send_via_debridge_with_message_only(
        ctx: Context<SendViaDebridge>,
        target_chain_id: [u8; 32],
        receiver: Vec<u8>,
        execution_fee: u64,
        is_use_asset_fee: bool,
        fallback_address: Vec<u8>,
    ) -> Result<()> {
        //TODO: Add real external call
        let external_call = vec![];

        invoke_init_external_call(external_call.as_slice(), ctx.remaining_accounts)
            .map_err(AnchorError::from)?;

        let send_ix = SendIx {
            target_chain_id,
            receiver,
            is_use_asset_fee,
            amount: add_all_fees(
                ctx.remaining_accounts,
                target_chain_id,
                0,
                execution_fee,
                is_use_asset_fee,
            )
            .map_err(|err| {
                msg!("Failed to add fees to amount. Inner error: {}", err);
                ErrorCode::FailedToCalculateAmountWithFee
            })?,
            submission_params: Some(SendSubmissionParamsInput::with_external_call(
                external_call,
                execution_fee,
                fallback_address,
            )),
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
pub struct SendViaDebridge {}

#[derive(Accounts)]
pub struct CheckClaiming<'info> {
    submission: AccountInfo<'info>,
    submission_authority: AccountInfo<'info>,
    #[account(address = sysvar::instructions::ID)]
    instructions: AccountInfo<'info>,
}
