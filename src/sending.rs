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
    Error, DEBRIDGE_ID, SEND_DISCRIMINATOR,
};

const CHAIN_SUPPORT_INFO_INDEX: usize = 4;
const STATE_INDEX: usize = 7;
const ASSET_FEE_INDEX: usize = 16;
struct MetaTemplate {
    is_signer: bool,
    is_writable: bool,
}

const SEND_META_TEMPLATE: [MetaTemplate; 18] = [
    // Bridge
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // Token Mint
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // Staking wallet
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // Mint authority
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    // Chain support info
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    // Settings program
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    // Spl token program
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    // State
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // Fee beneficiary
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // Nonce storage
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // Send from wallet
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // System program
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    // External call storage
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // External call meta
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    // Send from
    MetaTemplate {
        is_signer: true,
        is_writable: true,
    },
    // Discount
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    // Asset fee
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    // Debridge program
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
        .ne(&DEBRIDGE_ID)
    {
        return Err(Error::WrongDebridgeProgram.into());
    }

    let ix = Instruction {
        program_id: *DEBRIDGE_ID,
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
    T::try_from_accounts(&remaining_accounts[account_index]).map(Into::into)
}

pub fn is_chain_supported(
    _target_chain_id: [u8; 32],
    remaining_accounts: &[AccountInfo],
) -> Result<bool, Error> {
    Ok(
        match get_chain_support_info(remaining_accounts, _target_chain_id)? {
            ChainSupportInfo::Supported { .. } => true,
            ChainSupportInfo::NotSupported => false,
        },
    )
}

pub fn get_chain_native_fix_fee(
    _target_chain_id: [u8; 32],
    reamining_accounts: &[AccountInfo],
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
    target_chain_id: [u8; 32],
    reamining_accounts: &[AccountInfo],
) -> Result<bool, Error> {
    match get_asset_fee_info(reamining_accounts, target_chain_id) {
        Ok(asset_fee) => Ok(asset_fee.asset_chain_fee.is_some()),
        Err(err) if err == Error::WrongAccountIndex => Err(err),
        Err(_) => Ok(false),
    }
}

pub fn try_get_chain_asset_fix_fee(
    target_chain_id: [u8; 32],
    reamining_accounts: &[AccountInfo],
) -> Result<u64, Error> {
    get_asset_fee_info(reamining_accounts, target_chain_id)?
        .asset_chain_fee
        .ok_or(Error::AssetFeeNotSupported)
}
