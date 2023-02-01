use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    program_error::ProgramError,
};

use crate::{
    debridge_accounts::{AssetFeeInfo, ChainSupportInfo, State, TryFromAccount},
    Error, HashAdapter, Pubkey, BPS_DENOMINATOR, DEBRIDGE_ID_RAW, INIT_EXTERNAL_CALL_DISCRIMINATOR,
    SEND_DISCRIMINATOR, SOLANA_CHAIN_ID,
};

const CHAIN_SUPPORT_INFO_INDEX: usize = 4;
const STATE_INDEX: usize = 7;
const ASSET_FEE_INDEX: usize = 16;

const EXTERNAL_CALL_STORAGE_INDEX: usize = 12;
const EXTERNAL_CALL_META_INDEX: usize = 13;
const SEND_FROM_INDEX: usize = 14;
const SYSTEM_PROGRAM_INDEX: usize = 11;
const DEBRIDGE_PROGRAM_INDEX: usize = 17;

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

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SendSubmissionParamsInput {
    pub execution_fee: u64,
    pub reserved_flag: [u8; 32],
    pub fallback_address: Vec<u8>,
    pub external_call_shortcut: [u8; 32],
}

impl SendSubmissionParamsInput {
    pub fn execution_fee_only(execution_fee: u64) -> Self {
        SendSubmissionParamsInput {
            execution_fee,
            reserved_flag: [0; 32],
            fallback_address: vec![0; 20],
            external_call_shortcut: sha3::Keccak256::hash(&[]),
        }
    }

    pub fn with_external_call(
        external_call: Vec<u8>,
        execution_fee: u64,
        fallback_address: Vec<u8>,
    ) -> Self {
        SendSubmissionParamsInput {
            execution_fee,
            reserved_flag: [0; 32],
            fallback_address,
            external_call_shortcut: sha3::Keccak256::hash(external_call.as_slice()),
        }
    }
}

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

/// Invoke send instruction in debridge program
///
/// # Arguments
/// * `send_ix` - [`SendIx`] structure to send debridge instruction creation
/// * `account_infos` - account forming by client from debridge-typesctipr-sdk
pub fn invoke_debridge_send(send_ix: SendIx, account_infos: &[AccountInfo]) -> ProgramResult {
    if account_infos.len() < SEND_META_TEMPLATE.len() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    if account_infos[SEND_META_TEMPLATE.len() - 1]
        .key
        .ne(&Pubkey::from_str(DEBRIDGE_ID_RAW).unwrap())
    {
        return Err(Error::WrongDebridgeProgram.into());
    }

    let ix = Instruction {
        program_id: Pubkey::from_str(DEBRIDGE_ID_RAW).unwrap(),
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

    invoke(&ix, account_infos)
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

pub fn invoke_init_external_call(
    external_call: &[u8],
    account_infos: &[AccountInfo],
) -> Result<[u8; 32], ProgramError> {
    let external_call_shortcut = sha3::Keccak256::hash(external_call);

    let external_call_storage = account_infos[EXTERNAL_CALL_STORAGE_INDEX].clone();
    let external_call_meta = account_infos[EXTERNAL_CALL_META_INDEX].clone();
    let send_from = account_infos[SEND_FROM_INDEX].clone();
    let system_program = account_infos[SYSTEM_PROGRAM_INDEX].clone();
    let debridge_program = account_infos[DEBRIDGE_PROGRAM_INDEX].clone();
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

    invoke(
        &Instruction::new_with_bytes(
            Pubkey::from_str(DEBRIDGE_ID_RAW).unwrap(),
            &[
                INIT_EXTERNAL_CALL_DISCRIMINATOR.as_slice(),
                InitExternalCallIx {
                    external_call_len: external_call.len() as u32,
                    chain_id: SOLANA_CHAIN_ID,
                    external_call_shortcut: sha3::Keccak256::hash(external_call),
                    external_call: external_call.to_vec(),
                }
                .try_to_vec()?
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
    )?;
    Ok(external_call_shortcut)
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
                reserved_flag: [1; 32],
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

pub fn get_state(remaining_accounts: &[AccountInfo]) -> Result<State, Error> {
    get_account_by_index(remaining_accounts, STATE_INDEX)
}

pub fn get_chain_support_info(
    remaining_accounts: &[AccountInfo],
    _target_chain_id: [u8; 32],
) -> Result<ChainSupportInfo, Error> {
    get_account_by_index(remaining_accounts, CHAIN_SUPPORT_INFO_INDEX)
}

pub fn get_asset_fee_info(
    remaining_accounts: &[AccountInfo],
    _target_chain_id: [u8; 32],
) -> Result<AssetFeeInfo, Error> {
    get_account_by_index(remaining_accounts, ASSET_FEE_INDEX)
}

pub fn get_account_by_index<T: TryFromAccount<Error = Error>>(
    remaining_accounts: &[AccountInfo],
    account_index: usize,
) -> Result<T, Error> {
    if remaining_accounts.len() <= account_index {
        return Err(Error::WrongAccountIndex);
    }
    T::try_from_accounts(&remaining_accounts[account_index])
}

pub fn is_chain_supported(
    remaining_accounts: &[AccountInfo],
    _target_chain_id: [u8; 32],
) -> Result<bool, Error> {
    Ok(
        match get_chain_support_info(remaining_accounts, _target_chain_id)? {
            ChainSupportInfo::Supported { .. } => true,
            ChainSupportInfo::NotSupported => false,
        },
    )
}

pub fn get_transfer_fee(
    _remaining_accoutns: &[AccountInfo],
    _target_chain_id: [u8; 32],
) -> Result<u64, Error> {
    todo!()
}

const OVERFLOW_ERR: Error = Error::AmountOverflowedWhileAddingFee;
pub fn add_all_fees(
    remaining_accounts: &[AccountInfo],
    target_chain_id: [u8; 32],
    amount: u64,
    execution_fee: u64,
    is_use_asset_fee: bool,
) -> Result<u64, Error> {
    add_transfer_fee(
        remaining_accounts,
        target_chain_id,
        amount
            .checked_add(execution_fee)
            .ok_or(OVERFLOW_ERR)?
            .checked_add(
                is_use_asset_fee
                    .then(|| try_get_chain_asset_fix_fee(remaining_accounts, target_chain_id))
                    .transpose()?
                    .unwrap_or(0),
            )
            .ok_or(OVERFLOW_ERR)?,
    )
}

pub fn add_transfer_fee(
    remaining_accoutns: &[AccountInfo],
    target_chain_id: [u8; 32],
    amount: u64,
) -> Result<u64, Error> {
    let transfer_fee_bps = get_transfer_fee(remaining_accoutns, target_chain_id)?;

    u128::from(amount)
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

pub fn get_chain_native_fix_fee(
    reamining_accounts: &[AccountInfo],
    _target_chain_id: [u8; 32],
) -> Result<u64, Error> {
    match get_chain_support_info(reamining_accounts, _target_chain_id)? {
        ChainSupportInfo::NotSupported => get_default_native_fix_fee(reamining_accounts),
        ChainSupportInfo::Supported { fixed_fee, .. } => fixed_fee
            .map(Ok)
            .unwrap_or_else(|| get_default_native_fix_fee(reamining_accounts)),
    }
}

pub fn get_default_native_fix_fee(reamining_accounts: &[AccountInfo]) -> Result<u64, Error> {
    Ok(get_state(reamining_accounts)?.global_fixed_fee)
}

pub fn is_asset_fee_avaliable(
    reamining_accounts: &[AccountInfo],
    target_chain_id: [u8; 32],
) -> Result<bool, Error> {
    match get_asset_fee_info(reamining_accounts, target_chain_id) {
        Ok(asset_fee) => Ok(asset_fee.asset_chain_fee.is_some()),
        Err(err) if err == Error::WrongAccountIndex => Err(err),
        Err(_) => Ok(false),
    }
}

pub fn try_get_chain_asset_fix_fee(
    reamining_accounts: &[AccountInfo],
    target_chain_id: [u8; 32],
) -> Result<u64, Error> {
    get_asset_fee_info(reamining_accounts, target_chain_id)?
        .asset_chain_fee
        .ok_or(Error::AssetFeeNotSupported)
}
