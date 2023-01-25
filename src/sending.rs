use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    program_error::ProgramError,
};

use crate::{Error, DEBRIDGE_ID, SEND_DISCRIMINATOR};

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
    // Bridge fee
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

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SendIx {
    target_chain_id: [u8; 32],
    receiver: Vec<u8>,
    is_use_asset_fee: bool,
    amount: u64,
    submission_params: Option<SendSubmissionParamsInput>,
    referral_code: Option<u32>,
}

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
