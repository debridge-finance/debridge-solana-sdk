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

#![allow(clippy::result_large_err)]

use anchor_lang::{prelude::*, solana_program::sysvar};
use debridge_solana_sdk::{
    check_claiming, estimator,
    sending::{SendIx, SendSubmissionParamsInput},
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
        NotEnoughAccountProvided,
        FailedToEstimateExpenses,
    }

    use anchor_lang::solana_program::{program, program_error::ProgramError};
    use debridge_solana_sdk::{
        prelude::*,
        sending,
        sending::{SEND_FROM_INDEX, SEND_FROM_WALLET_INDEX},
    };
    use spl_token::solana_program::system_instruction;

    use super::*;

    /// Debridge protocol allows transfer liquidity from Solana to other supported chains
    /// To send some token to other supported chain use [`debridge_solana_sdk::sending::invoke_debridge_send`]
    ///
    /// To check if the network is supported you can use [`debridge_solana_sdk::sending::is_chain_supported`]
    pub fn send_via_debridge(
        ctx: Context<SendViaDebridge>,
        amount: u64,
        target_chain_id: [u8; 32],
        receiver: Vec<u8>,
        is_use_asset_fee: bool,
    ) -> Result<()> {
        sending::invoke_debridge_send(
            SendIx {
                target_chain_id,
                receiver,
                is_use_asset_fee,
                amount,
                submission_params: None,
                referral_code: None,
            },
            ctx.remaining_accounts,
        )
        .map_err(|err| err.into())
    }

    /// Debridge protocol takes fix fee and transfer fee while sending liquidity.
    /// The fix fee by default is taken in native solana tokens.
    /// The default native fix fee amount is set in state account but it can set custom native
    /// fix amount for a specific chain in chain support info account.
    ///
    /// To get default native fix fee amount use [`debridge_solana_sdk::sending::get_default_native_fix_fee`]
    ///
    /// To get native fix fee amount for specific chain use [`debridge_solana_sdk::sending::get_chain_native_fix_fee`]
    ///
    /// To use native fix fee set [`debridge_solana_sdk::sending::SendIx`] `is_use_asset_fee` field to `false`
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

        sending::invoke_debridge_send(send_ix, ctx.remaining_accounts).map_err(|err| err.into())
    }

    /// Debridge protocol takes fix fee and transfer fee while sending liquidity.
    /// The fix fee by default is taken in native solana tokens.
    /// But when transferring some tokens to certain networks, it is possible to pay in transferred tokens.
    /// It's called `asset_fix_fee`.
    ///
    /// To known `asset_fee` is available use [`debridge_solana_sdk::sending::is_asset_fee_available`]
    ///
    /// To get asset fix fee amount for specific chain use [`debridge_solana_sdk::sending::try_get_chain_asset_fix_fee`]
    ///
    /// To use asset fix fee set [`debridge_solana_sdk::sending::SendIx`] `is_use_asset_fee` field to `true`
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

        if sending::is_asset_fee_available(ctx.remaining_accounts, target_chain_id)
            .map_err(|_| ErrorCode::NotEnoughAccountProvided)?
        {
            sending::invoke_debridge_send(send_ix, ctx.remaining_accounts)
                .map_err(error::Error::from)
        } else {
            msg!("Asset fee not available for provided tokens and target chain id");

            Ok(())
        }
    }

    /// Debridge protocol takes fix fee and transfer fee while sending liquidity.
    /// If needed to get exact amount tokens in target chain, all fees will need to be added to sending amount.
    ///
    /// There are three types of fees in Debridge protocol: fixed fee, transfer fee, execution fee.
    ///
    /// Fixed fee is fixed amount for any send instruction. It's named asset fixed fee. The amount depends on target chain.
    /// To get asset fix fee amount for specific chain use [`debridge_solana_sdk::sending::try_get_chain_asset_fix_fee`]
    /// For some token fixed fee can be paid with sent tokens. In this case, you need to include this asset fixed fee in the final amount.
    ///
    /// Transfer fee is taken as part of sent tokens. To get the bps of transfer fee use [`debridge_solana_sdk::sending::get_transfer_fee`]
    /// To add transfer fee to current amount use [`debridge_solana_sdk::sending::add_transfer_fee`]
    ///
    /// Execution fee is reward for execute claim instruction in target chain. It can be zero if you want to run the instruction yourself.
    /// The recommended execution fee can be obtained using debridge sdk.
    ///
    /// To add to exact amount all fees charged during the send use [`debridge_solana_sdk::sending::add_all_fees`]
    pub fn send_via_debridge_with_exact_amount(
        ctx: Context<SendViaDebridge>,
        exact_amount: u64,
        target_chain_id: [u8; 32],
        receiver: Vec<u8>,
        execution_fee: u64,
        is_use_asset_fee: bool,
    ) -> Result<()> {
        let final_amount = sending::add_all_fees(
            ctx.remaining_accounts,
            target_chain_id,
            exact_amount,
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

        sending::invoke_debridge_send(send_ix, ctx.remaining_accounts).map_err(|err| err.into())
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

        sending::invoke_debridge_send(send_ix, ctx.remaining_accounts).map_err(|err| err.into())
    }

    /// Debridge protocol allows not only to send tokens to another network,
    /// but also to use them to call any smart contract.
    ///
    /// Used `external_call` for this. For evm-like network it will be address of smart contract function and function's arguments
    /// packed in byte vector.
    ///
    /// To use external call function needed to initialize external call storage with
    /// [`debridge_solana_sdk::sending::invoke_init_external_call`] function and create `submission_params`
    /// with [`debridge_solana_sdk::sending::SendSubmissionParamsInput::with_external_call`] function.
    /// Besides external call needed to provide `fallback_address`. The `fallback_address' will be used
    /// if external call fails. On this address token received in target chain will transfer.
    ///
    /// A `execution_fee` is reward reward that will received for execution claim transaction in
    /// target chain. It can be set zero if external call will be claimed by yourself.
    #[allow(clippy::too_many_arguments)]
    pub fn send_via_debridge_with_external_call(
        ctx: Context<SendViaDebridge>,
        amount: u64,
        target_chain_id: [u8; 32],
        receiver: Vec<u8>,
        execution_fee: u64,
        fallback_address: Vec<u8>,
        flags: [u8; 32],
        external_call: Vec<u8>,
    ) -> Result<()> {
        debridge_sending::invoke_init_external_call(
            external_call.as_slice(),
            ctx.remaining_accounts,
        )?;

        let send_ix = SendIx {
            target_chain_id,
            receiver,
            is_use_asset_fee: false,
            amount,
            submission_params: Some(SendSubmissionParamsInput::with_external_call(
                external_call,
                execution_fee,
                fallback_address,
                flags,
            )),
            referral_code: None,
        };

        sending::invoke_debridge_send(send_ix, ctx.remaining_accounts).map_err(|err| err.into())
    }

    /// deBridge protocol allows calling any smart contract in target chain without sending any tokens.
    /// You have to pay only a transfer fee for sending an execution fee to another chain.
    /// If you claim by yourself, set execution fee to zero, you don’t need to pay transfer fee at all.
    /// Only fixed fee will be taken.
    ///
    /// Used `external_call` for this. For evm-like network it will be address of smart contract
    /// function and function's arguments packed in byte vector.
    ///
    /// To send message with external call function use [`debridge_solana_sdk::sending::invoke_send_message`]
    /// function. This function will create external call storage, calculate transfer fee for
    /// transferring execution fee and send the message to target chain.
    /// Besides external call needed to provide `fallback_address`. The `fallback_address' will be used
    /// if external call fails. On this address token received in target chain will transfer.
    ///
    /// A `execution_fee` is reward reward that will received for execution claim transaction in
    /// target chain. It can be set zero if external call will be claimed by yourself.
    pub fn send_message_via_debridge(
        ctx: Context<SendViaDebridge>,
        target_chain_id: [u8; 32],
        receiver: Vec<u8>,
        execution_fee: u64,
        fallback_address: Vec<u8>,
        message: Vec<u8>,
    ) -> Result<()> {
        sending::invoke_send_message(
            message,
            target_chain_id,
            receiver,
            execution_fee,
            fallback_address,
            ctx.remaining_accounts,
        )
        .map_err(ProgramError::from)?;

        Ok(())
    }

    /// One of the function of Debridge protocol is transferring message between two program or
    /// smart contract. For this, solana uses PDA accounts. With the help of pda accounts,
    /// the program ensures that it initiated the call to the send function of the debridge program
    ///
    /// To use this feature, you need to use [`debridge_solana_sdk::sending::set_send_from_account`]
    /// and provide PDA account and wallet that belongs to this account. Then you have to use debrige
    /// sdk function with `_signed` postfix. In this example we use [`debridge_solana_sdk::sending::invoke_send_message_signed`].
    /// These functions additionally need to pass signers seeds and bump.
    pub fn send_message_via_debridge_with_program_sender<'info>(
        ctx: Context<'_, '_, '_, 'info, SendViaDebridgeWithSender<'info>>,
        target_chain_id: [u8; 32],
        receiver: Vec<u8>,
        execution_fee: u64,
        fallback_address: Vec<u8>,
        message: Vec<u8>,
    ) -> Result<()> {
        program::invoke(
            &system_instruction::transfer(
                ctx.remaining_accounts[SEND_FROM_INDEX].key,
                ctx.accounts.program_sender.key,
                estimator::get_native_sender_lamports_expenses(
                    sending::get_chain_native_fix_fee(ctx.remaining_accounts, target_chain_id)
                        .map_err(|_| ErrorCode::FailedToCalculateAmountWithFee)?,
                    message.len(),
                )
                .map_err(|_| ErrorCode::FailedToEstimateExpenses)?,
            ),
            &[
                ctx.remaining_accounts[SEND_FROM_INDEX].clone(),
                ctx.accounts.program_sender.clone(),
            ],
        )?;

        program::invoke(
            &spl_token::instruction::transfer(
                &spl_token::ID,
                ctx.remaining_accounts[SEND_FROM_WALLET_INDEX].key,
                ctx.accounts.program_sender_wallet.key,
                ctx.remaining_accounts[SEND_FROM_INDEX].key,
                &[],
                sending::add_all_fees(
                    ctx.remaining_accounts,
                    target_chain_id,
                    0,
                    execution_fee,
                    false,
                )
                .map_err(|_| ErrorCode::FailedToCalculateAmountWithFee)?,
            )?,
            &[
                ctx.remaining_accounts[SEND_FROM_WALLET_INDEX].clone(),
                ctx.accounts.program_sender_wallet.clone(),
                ctx.remaining_accounts[SEND_FROM_INDEX].clone(),
            ],
        )?;

        let mut accounts = ctx.remaining_accounts.to_vec();

        sending::set_send_from_account(
            accounts.as_mut_slice(),
            ctx.accounts.program_sender.clone(),
            ctx.accounts.program_sender_wallet.clone(),
        );

        let bump = *ctx.bumps.get("program_sender").expect("Failed to get bump");

        sending::invoke_send_message_signed(
            message,
            target_chain_id,
            receiver,
            execution_fee,
            fallback_address,
            accounts.as_slice(),
            &[&[PROGRAM_SENDER_SEED, &[bump]]],
        )
        .map_err(ProgramError::from)?;

        Ok(())
    }

    /// Debridge protocol allows to execute some Solana instructions from evm-like chains.
    /// Execution occurs using the debridge's `execute_external_call` instruction.
    ///
    /// The `execute_external_call` instruction invokes provided from evm instruction
    /// stored and verified in external_call_storage with Solana Cross-Program Invocations and
    /// [`anchor_lang::solana_program::program::invoke_signed`] function.
    ///
    /// Often there is a task to check that the program instruction is called from the
    /// `execute_external_call` instruction by [`anchor_lang::solana_program::program::invoke_signed`].
    ///
    /// For this task you can use [`debridge_solana_sdk::check_claiming::check_execution_context`] function.
    /// For it you need to provide `submission` and `submission_authority` accounts and `source_chain_id`.
    ///
    /// Also you can check `native_sender`. It's user who call send function in source chain. With this
    /// function you can let two contracts communicate with each other.
    pub fn check_claiming(
        ctx: Context<CheckClaiming>,
        source_chain_id: [u8; 32],
        native_sender_validation: Option<Vec<u8>>,
    ) -> Result<()> {
        check_claiming::ValidatedExecuteExtCallIx::try_from_current_ix(&ctx.accounts.instructions)?
            .validate_submission_account(
                &ctx.accounts.submission,
                check_claiming::SubmissionAccountValidation {
                    source_chain_id_validation: Some(source_chain_id),
                    native_sender_validation,

                    // You can validate either part of the context or the whole context
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

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendViaDebridge {}

pub const PROGRAM_SENDER_SEED: &[u8] = b"PROGRAM_SENDER";

#[derive(Accounts)]
pub struct SendViaDebridgeWithSender<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_SENDER_SEED],
        bump,
    )]
    program_sender: AccountInfo<'info>,
    #[account(mut)]
    program_sender_wallet: AccountInfo<'info>,
}

pub trait FindProgramSender {
    fn find_program_sender() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[PROGRAM_SENDER_SEED], &ID)
    }
}
impl FindProgramSender for Pubkey {}

pub trait FindProgramSenderWallet {
    fn find_program_sender_wallet(token_mint: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                Pubkey::find_program_sender().0.as_ref(),
                spl_token::ID.as_ref(),
                token_mint.as_ref(),
            ],
            &spl_associated_token_account::ID,
        )
    }
}
impl FindProgramSenderWallet for Pubkey {}

#[derive(Accounts)]
pub struct CheckClaiming<'info> {
    submission: AccountInfo<'info>,
    submission_authority: AccountInfo<'info>,
    #[account(address = sysvar::instructions::ID)]
    instructions: AccountInfo<'info>,
}