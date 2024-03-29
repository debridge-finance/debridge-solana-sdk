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

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
    program_error::ProgramError,
};

use crate::{
    debridge_accounts::{
        AssetFeeInfo, ChainSupportInfo, ExternalCallMeta, State, TryFromAccount,
        INIT_EXTERNAL_CALL_DISCRIMINATOR, SEND_DISCRIMINATOR,
    },
    errors::InvokeError,
    flags::SetReservedFlag,
    hash::HashAdapter,
    keys::{AssetFeeInfoPubkey, BridgePubkey, ChainSupportInfoPubkey},
    Error, Pubkey, SolanaKeccak256, BPS_DENOMINATOR, DEBRIDGE_ID, SOLANA_CHAIN_ID,
};

/// Struct for forming send instruction in debridge program
#[derive(BorshSerialize, BorshDeserialize)]
pub struct SendIx {
    /// Chain id to which the tokens are sent
    pub target_chain_id: [u8; 32],
    /// Address in `target_chain_id` that will receive the transferred tokens
    pub receiver: Vec<u8>,
    /// Id of the network to which the tokens are sent
    pub is_use_asset_fee: bool,
    /// Amount of sending tokens. From this amount fee will be taken
    pub amount: u64,
    /// Additional data for tokens sending with auto external execution
    pub submission_params: Option<SendSubmissionParamsInput>,
    /// Not used
    pub referral_code: Option<u32>,
}

/// Option params for send instruction
#[derive(BorshSerialize, BorshDeserialize)]
pub struct SendSubmissionParamsInput {
    /// Reward for execution claim transaction in target chain
    pub execution_fee: u64,
    /// Flags for additional protocol features
    pub flags: [u8; 32],
    /// Reserve address for sending tokens if external call fails
    pub fallback_address: Vec<u8>,
    /// Keccak256 hash of external call buffer
    pub external_call_shortcut: [u8; 32],
}

impl SendSubmissionParamsInput {
    /// Create submission params for sending with execution fee and without external call
    ///
    /// # Arguments
    /// * `execution_fee` - amount of execution fee
    pub fn execution_fee_only(execution_fee: u64) -> Self {
        SendSubmissionParamsInput {
            execution_fee,
            flags: [0; 32],
            fallback_address: vec![0; 20],
            external_call_shortcut: SolanaKeccak256::hash(&[]),
        }
    }

    /// Create submission params for external call
    ///
    /// # Arguments
    /// * `external_call` - instructions sending in target chain
    /// * `execution_fee` - amount of execution fee
    /// * `fallback_address` -  reserve address for sending tokens if external call fails
    /// * `flags` - flags for additional debridge protocol features
    pub fn with_external_call(
        external_call: Vec<u8>,
        execution_fee: u64,
        fallback_address: Vec<u8>,
        flags: [u8; 32],
    ) -> Self {
        SendSubmissionParamsInput {
            execution_fee,
            flags,
            fallback_address,
            external_call_shortcut: SolanaKeccak256::hash(external_call.as_slice()),
        }
    }

    /// Create submission params for sending message to other chain
    ///
    /// This path is the default path for sending an authorised message
    /// on the EVM network. If your case is simple, this is probably the
    /// method you need.
    ///
    /// For everything else, please use [`SendSubmissionParamsInput::with_external_call`]
    ///
    /// # Arguments
    /// * `external_call` - instructions sending in target chain
    /// * `execution_fee` - amount of execution fee
    /// * `fallback_address` -  reserve address for sending tokens if external call fails
    pub fn with_message(
        external_call: Vec<u8>,
        execution_fee: u64,
        fallback_address: Vec<u8>,
    ) -> Self {
        let mut flags = [0; 32];
        flags.set_revert_if_external_call();
        flags.set_proxy_with_sender();

        SendSubmissionParamsInput {
            execution_fee,
            flags,
            fallback_address,
            external_call_shortcut: SolanaKeccak256::hash(external_call.as_slice()),
        }
    }
}

/// Invoke send instruction in debridge program
///
/// # Arguments
/// * `send_ix` - [`SendIx`] structure to send debridge instruction creation
/// * `account_infos` - account forming by client from debridge-typescript-sdk
pub fn invoke_debridge_send(send_ix: SendIx, account_infos: &[AccountInfo]) -> ProgramResult {
    invoke_debridge_send_signed(send_ix, account_infos, &[])
}

/// Invoke send instruction in debridge program with using PDA account as send_from.
/// You can using this function for sending message on behalf of the program.
///
/// # Arguments
/// * `send_ix` - [`SendIx`] structure to send debridge instruction creation
/// * `account_infos` - account forming by client from debridge-typescript-sdk
/// * `signer_seeds` - parameter is a slice of `u8` slices where the inner slices represent the seeds used to derive PDA account
pub fn invoke_debridge_send_signed(
    send_ix: SendIx,
    account_infos: &[AccountInfo],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    if account_infos.len() < SEND_META_TEMPLATE.len() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    if account_infos[SEND_META_TEMPLATE.len() - 1]
        .key
        .ne(&DEBRIDGE_ID)
    {
        return Err(Error::WrongDebridgeProgramId.into());
    }

    let ix = Instruction {
        program_id: DEBRIDGE_ID,
        accounts: account_infos
            .iter()
            .take(SEND_META_TEMPLATE.len())
            .zip(SEND_META_TEMPLATE)
            .map(|(acc, meta)| AccountMeta {
                pubkey: *acc.key,
                is_signer: meta.is_signer,
                is_writable: meta.is_writable,
            })
            .collect(),
        data: [
            SEND_DISCRIMINATOR.as_slice(),
            send_ix.try_to_vec()?.as_slice(),
        ]
        .concat(),
    };

    invoke_signed(&ix, account_infos, signers_seeds)
}

/// Struct for forming send instruction in debridge program
#[derive(BorshSerialize, BorshDeserialize)]
pub struct InitExternalCallIx {
    /// Len of external call array
    pub external_call_len: u32,
    /// Target chain id
    pub chain_id: [u8; 32],
    /// Keccak hash of external call
    pub external_call_shortcut: [u8; 32],
    /// Message that send and try to execute in target chain
    pub external_call: Vec<u8>,
}

/// Create account for storing external call buffer
///
/// In the case of a large external call it can be called
/// by a separate transaction in conjunction with
/// `update_external_call` method from TS client
///
/// # Arguments
/// * `external_call` - instructions sending in target chain
/// * `account_infos` - account forming by client from debridge-typescript-sdk
pub fn invoke_init_external_call(
    external_call: &[u8],
    account_infos: &[AccountInfo],
) -> Result<(), ProgramError> {
    invoke_init_external_call_signed(external_call, account_infos, &[])
}

/// Create account for storing external call buffer with using PDA account as send_from.
/// You can using this function for sending message on behalf of the program.
///
/// # Arguments
/// * `external_call` - instructions sending in target chain
/// * `account_infos` - account forming by client from debridge-typescript-sdk
/// * `signer_seeds` - parameter is a slice of `u8` slices where the inner slices represent the seeds used to derive PDA account
pub fn invoke_init_external_call_signed(
    external_call: &[u8],
    account_infos: &[AccountInfo],
    signers_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    let external_call_storage = account_infos[EXTERNAL_CALL_STORAGE_INDEX].clone();
    let external_call_meta = account_infos[EXTERNAL_CALL_META_INDEX].clone();
    let send_from = account_infos[SEND_FROM_INDEX].clone();
    let system_program = account_infos[SYSTEM_PROGRAM_INDEX].clone();
    let debridge_program = account_infos[DEBRIDGE_PROGRAM_INDEX].clone();

    if external_call_meta.owner.eq(&DEBRIDGE_ID) {
        return match ExternalCallMeta::try_from_account(&external_call_meta)? {
            ExternalCallMeta::Transferred { .. } => Ok(()),
            _ => Err(InvokeError::SdkError(Error::ExternalStorageWrongState).into()),
        };
    }

    let accounts = vec![
        AccountMeta {
            pubkey: *external_call_storage.key,
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: *external_call_meta.key,
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: *send_from.key,
            is_signer: true,
            is_writable: true,
        },
        AccountMeta {
            pubkey: *system_program.key,
            is_signer: false,
            is_writable: false,
        },
    ];

    invoke_signed(
        &Instruction::new_with_bytes(
            DEBRIDGE_ID,
            &[
                INIT_EXTERNAL_CALL_DISCRIMINATOR.as_slice(),
                InitExternalCallIx {
                    external_call_len: external_call.len() as u32,
                    chain_id: SOLANA_CHAIN_ID,
                    external_call_shortcut: SolanaKeccak256::hash(external_call),
                    external_call: external_call.to_vec(),
                }
                .try_to_vec()
                .map_err(ProgramError::from)?
                .as_slice(),
            ]
            .concat(),
            accounts,
        ),
        &[
            external_call_storage,
            external_call_meta,
            send_from,
            system_program,
            debridge_program,
        ],
        signers_seeds,
    )?;

    Ok(())
}

/// Send message to other chain without liquidity.
/// Perform debridge send flow with zero amount
///
/// # Arguments
/// * `external_call` - instructions sending in target chain
/// * `target_chain_id` - chain id to which the tokens are sent
/// * `receiver` - send message to other chain without liquidity.
/// * `execution_fee` - chain id to which the tokens are sent
/// * `fallback_address` - reserve address for sending tokens if external call fails
/// * `account_infos` - account forming by client from debridge-typescript-sdk
pub fn invoke_send_message(
    external_call: Vec<u8>,
    target_chain_id: [u8; 32],
    receiver: Vec<u8>,
    execution_fee: u64,
    fallback_address: Vec<u8>,
    account_infos: &[AccountInfo],
) -> Result<(), InvokeError> {
    invoke_send_message_signed(
        external_call,
        target_chain_id,
        receiver,
        execution_fee,
        fallback_address,
        account_infos,
        &[],
    )
}

/// Send message to other chain without liquidity with using PDA account as send_from.
/// You can using this function for sending message on behalf of the program.
/// Perform debridge send flow with zero amount.
///
/// # Arguments
/// * `external_call` - instructions sending in target chain
/// * `target_chain_id` - chain id to which the tokens are sent
/// * `receiver` - send message to other chain without liquidity.
/// * `execution_fee` - chain id to which the tokens are sent
/// * `fallback_address` - reserve address for sending tokens if external call fails
/// * `account_infos` - account forming by client from debridge-typescript-sdk
/// * `signer_seeds` - parameter is a slice of `u8` slices where the inner slices represent the seeds used to derive PDA account
pub fn invoke_send_message_signed(
    external_call: Vec<u8>,
    target_chain_id: [u8; 32],
    receiver: Vec<u8>,
    execution_fee: u64,
    fallback_address: Vec<u8>,
    account_infos: &[AccountInfo],
    signers_seeds: &[&[&[u8]]],
) -> Result<(), InvokeError> {
    invoke_init_external_call_signed(external_call.as_slice(), account_infos, signers_seeds)?;

    let send_ix = SendIx {
        target_chain_id,
        receiver,
        is_use_asset_fee: false,
        amount: add_all_fees(account_infos, target_chain_id, 0, execution_fee, false)
            .map_err(|_| Error::AmountOverflowedWhileAddingFee)?,
        submission_params: Some(SendSubmissionParamsInput::with_message(
            external_call,
            execution_fee,
            fallback_address,
        )),
        referral_code: None,
    };

    invoke_debridge_send_signed(send_ix, account_infos, signers_seeds)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use borsh::BorshSerialize;

    use crate::sending::{SendIx, SendSubmissionParamsInput, SEND_DISCRIMINATOR};

    #[test]
    fn test_send_ix_consistency() {
        let send_ix = SendIx {
            target_chain_id: [13; 32],
            receiver: vec![14; 32],
            is_use_asset_fee: false,
            amount: 1000,
            submission_params: Some(SendSubmissionParamsInput {
                execution_fee: 100,
                flags: [1; 32],
                fallback_address: vec![15; 32],
                external_call_shortcut: [16; 32],
            }),
            referral_code: Some(2000),
        };

        assert_eq!(
            SEND_DISCRIMINATOR
                .into_iter()
                .chain(send_ix.try_to_vec().expect("Unreachable"))
                .collect::<Vec<u8>>(),
            vec![
                102, 251, 20, 187, 65, 75, 12, 69, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 32,
                0, 0, 0, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
                14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 0, 232, 3, 0, 0, 0, 0, 0,
                0, 1, 100, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 32, 0, 0, 0, 15, 15, 15, 15, 15, 15,
                15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
                15, 15, 15, 15, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
                16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 1, 208, 7, 0, 0
            ]
        )
    }
}

/// Get State account structure from sending accounts
///
/// # Arguments
/// * `account_infos` - account forming by client from debridge-typescript-sdk
pub fn get_state(account_infos: &[AccountInfo]) -> Result<State, Error> {
    get_account_by_index(account_infos, STATE_INDEX)
}

/// Get Chain Support info account  account structure from sending accounts
///
/// # Arguments
/// * `account_infos` - account forming by client from debridge-typescript-sdk
/// * `target_chain_id` - chain id to which the tokens are sent
pub fn get_chain_support_info(
    account_infos: &[AccountInfo],
    target_chain_id: [u8; 32],
) -> Result<ChainSupportInfo, Error> {
    check_chain_support_info_account(account_infos, target_chain_id)
        .and_then(|()| get_account_by_index(account_infos, CHAIN_SUPPORT_INFO_INDEX))
}

/// Check that provided chain support info account refers to `target_chain_id`
///
/// # Arguments
/// * `account_infos` - account forming by client from debridge-typescript-sdk
/// * `target_chain_id` - chain id to which the tokens are sent}
pub fn check_chain_support_info_account(
    account_infos: &[AccountInfo],
    target_chain_id: [u8; 32],
) -> Result<(), Error> {
    account_infos
        .get(CHAIN_SUPPORT_INFO_INDEX)
        .ok_or(Error::WrongAccountIndex)
        .and_then(|chain_support_info| {
            Pubkey::find_chain_support_info_address(&target_chain_id)?
                .0
                .eq(chain_support_info.key)
                .then_some(())
                .ok_or(Error::WrongChainSupportInfo)
        })
}

/// Get Bridge asset fee info account account structure from sending accounts
///
/// # Arguments
/// * `account_infos` - account forming by client from debridge-typescript-sdk
/// * `target_chain_id` - chain id to which the tokens are sent
pub fn get_asset_fee_info(
    account_infos: &[AccountInfo],
    target_chain_id: [u8; 32],
) -> Result<AssetFeeInfo, Error> {
    account_infos
        .get(TOKEN_MINT_INDEX)
        .zip(account_infos.get(ASSET_FEE_INDEX))
        .ok_or(Error::WrongAccountIndex)
        .and_then(|(token_mint, asset_fee)| {
            Pubkey::find_asset_fee_info_address(
                &Pubkey::find_bridge_address(token_mint.key).0,
                &target_chain_id,
            )?
            .0
            .eq(asset_fee.key)
            .then_some(())
            .ok_or(Error::WrongBridgeFeeInfo)
        })
        .and_then(|()| get_account_by_index(account_infos, ASSET_FEE_INDEX))
}

/// Parse account structure from sending accounts by index
///
/// # Arguments
/// * `account_infos` - account forming by client from debridge-typescript-sdk
/// * `account_index` - account index from sending accounts  
pub fn get_account_by_index<T: TryFromAccount<Error = Error>>(
    account_infos: &[AccountInfo],
    account_index: usize,
) -> Result<T, Error> {
    if account_infos.len() <= account_index {
        return Err(Error::WrongAccountIndex);
    }
    T::try_from_account(&account_infos[account_index])
}

/// Check the possibility of sending to the chain by chain id
///
/// # Arguments
/// * `account_infos` - account forming by client from debridge-typescript-sdk
/// * `target_chain_id` - chain id to which the tokens are sent
pub fn is_chain_supported(
    account_infos: &[AccountInfo],
    target_chain_id: [u8; 32],
) -> Result<bool, Error> {
    Ok(
        match get_chain_support_info(account_infos, target_chain_id)? {
            ChainSupportInfo::Supported { .. } => true,
            ChainSupportInfo::NotSupported => false,
        },
    )
}

/// Get transfer fee bps for sending current tokens to target chain id
///
/// # Arguments
/// * `account_infos` - account forming by client from debridge-typescript-sdk
/// * `target_chain_id` - chain id to which the tokens are sent
pub fn get_transfer_fee(
    account_infos: &[AccountInfo],
    target_chain_id: [u8; 32],
) -> Result<u64, Error> {
    get_transfer_fee_for_chain(account_infos, target_chain_id).and_then(|chain_fee| {
        chain_fee
            .map(Ok)
            .unwrap_or_else(|| Ok(get_state(account_infos)?.global_transfer_fee_bps))
    })
}

/// Some networks have their own transfer fee bps
/// Get own transfer fee bps to target chain id if defined
///
/// # Arguments
/// * `account_infos` - account forming by client from debridge-typescript-sdk
/// * `target_chain_id` - chain id to which the tokens are sent
pub fn get_transfer_fee_for_chain(
    account_infos: &[AccountInfo],
    target_chain_id: [u8; 32],
) -> Result<Option<u64>, Error> {
    get_chain_support_info(account_infos, target_chain_id).and_then(|chain_support_info| {
        match chain_support_info {
            ChainSupportInfo::NotSupported => Err(Error::TargetChainNotSupported),
            ChainSupportInfo::Supported {
                transfer_fee_bps, ..
            } => Ok(transfer_fee_bps),
        }
    })
}

/// Get native fixed fee for sending to target chain id
///
/// # Arguments
/// * `account_infos` - account forming by client from debridge-typescript-sdk
/// * `target_chain_id` - chain id to which the tokens are sent
pub fn get_chain_native_fix_fee(
    remaining_accounts: &[AccountInfo],
    _target_chain_id: [u8; 32],
) -> Result<u64, Error> {
    match get_chain_support_info(remaining_accounts, _target_chain_id)? {
        ChainSupportInfo::NotSupported => get_default_native_fix_fee(remaining_accounts),
        ChainSupportInfo::Supported { fixed_fee, .. } => fixed_fee
            .map(Ok)
            .unwrap_or_else(|| get_default_native_fix_fee(remaining_accounts)),
    }
}

/// Get default native fixed fee
///
/// # Arguments
/// * `account_infos` - account forming by client from debridge-typescript-sdk
pub fn get_default_native_fix_fee(account_infos: &[AccountInfo]) -> Result<u64, Error> {
    Ok(get_state(account_infos)?.global_fixed_fee)
}

/// Checks the availability of payment fixed fee in transferring tokens
///
/// # Arguments
/// * `account_infos` - account forming by client from debridge-typescript-sdk
/// * `target_chain_id` - chain id to which the tokens are sent
pub fn is_asset_fee_available(
    account_infos: &[AccountInfo],
    target_chain_id: [u8; 32],
) -> Result<bool, Error> {
    match get_asset_fee_info(account_infos, target_chain_id) {
        Ok(asset_fee) => Ok(asset_fee.asset_chain_fee.is_some()),
        Err(err) if err == Error::WrongAccountIndex => Err(err),
        Err(_) => Ok(false),
    }
}

/// Try to get assets fixed fee for sending a current tokens to target chain id
///
/// # Arguments
/// * `account_infos` - account forming by client from debridge-typescript-sdk
/// * `target_chain_id` - chain id to which the tokens are sent
pub fn try_get_chain_asset_fix_fee(
    account_infos: &[AccountInfo],
    target_chain_id: [u8; 32],
) -> Result<u64, Error> {
    get_asset_fee_info(account_infos, target_chain_id)?
        .asset_chain_fee
        .ok_or(Error::AssetFeeNotSupported)
}

const OVERFLOW_ERR: Error = Error::AmountOverflowedWhileAddingFee;

/// Add all fees that will be taken to receive exact amount
///
/// # Arguments
/// * `account_infos` - account forming by client from debridge-typescript-sdk
/// * `target_chain_id` - chain id to which the tokens are sent
/// * `exact_amount` - amount that will be received in target chain
/// * `execution_fee` - amount of execution fee
/// * `is_use_asset_fee` - determines how the fee will be paid. True: sending tokens, false: Sol
pub fn add_all_fees(
    account_infos: &[AccountInfo],
    target_chain_id: [u8; 32],
    exact_amount: u64,
    execution_fee: u64,
    is_use_asset_fee: bool,
) -> Result<u64, Error> {
    add_transfer_fee(
        account_infos,
        target_chain_id,
        exact_amount
            .checked_add(execution_fee)
            .ok_or(OVERFLOW_ERR)?
            .checked_add(
                is_use_asset_fee
                    .then(|| try_get_chain_asset_fix_fee(account_infos, target_chain_id))
                    .transpose()?
                    .unwrap_or(0),
            )
            .ok_or(OVERFLOW_ERR)?,
    )
}

/// Add transfer fee that will be taken to send exact amount to target chain
///
/// # Arguments
/// * `account_infos` - account forming by client from debridge-typescript-sdk
/// * `target_chain_id` - chain id to which the tokens are sent
/// * `exact_amount` - amount that will be send in target chain
pub fn add_transfer_fee(
    account_infos: &[AccountInfo],
    target_chain_id: [u8; 32],
    exact_amount: u64,
) -> Result<u64, Error> {
    let transfer_fee_bps = get_transfer_fee(account_infos, target_chain_id)?;

    u128::from(exact_amount)
        .checked_mul(u128::from(BPS_DENOMINATOR))
        .ok_or(OVERFLOW_ERR)?
        .checked_div(u128::from(
            BPS_DENOMINATOR
                .checked_sub(transfer_fee_bps)
                .ok_or(OVERFLOW_ERR)?,
        ))
        .ok_or(OVERFLOW_ERR)?
        .try_into()
        .map_err(|_| OVERFLOW_ERR)
}

pub fn set_send_from_account<'a>(
    account_infos: &mut [AccountInfo<'a>],
    send_from: AccountInfo<'a>,
    send_from_wallet: AccountInfo<'a>,
) {
    account_infos[SEND_FROM_INDEX] = send_from;
    account_infos[SEND_FROM_WALLET_INDEX] = send_from_wallet;
}

pub const CHAIN_SUPPORT_INFO_INDEX: usize = 4;
pub const STATE_INDEX: usize = 7;
pub const ASSET_FEE_INDEX: usize = 16;
pub const TOKEN_MINT_INDEX: usize = 1;

pub const EXTERNAL_CALL_STORAGE_INDEX: usize = 12;
pub const EXTERNAL_CALL_META_INDEX: usize = 13;
pub const SEND_FROM_INDEX: usize = 14;
pub const SEND_FROM_WALLET_INDEX: usize = 10;
pub const SYSTEM_PROGRAM_INDEX: usize = 11;
pub const DEBRIDGE_PROGRAM_INDEX: usize = 17;

struct MetaTemplate {
    is_signer: bool,
    is_writable: bool,
}

const SEND_META_TEMPLATE: [MetaTemplate; 18] = [
    // 0: Bridge
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // 1: Token Mint
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // 2: Staking wallet
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // 3: Mint authority
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    // 4: Chain support info
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    // 5: Settings program
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    // 6: Spl token program
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    // 7: State
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // 8: Fee beneficiary
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // 9: Nonce storage
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // 10: Send from wallet
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // 11: System program
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    // 12: External call storage
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // 13: External call meta
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // 14: Send from
    MetaTemplate {
        is_signer: true,
        is_writable: true,
    },
    // 15: Discount
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    // 16: Asset fee
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    // 17: Debridge program
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
];