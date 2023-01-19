use std::str::FromStr;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, instruction::Instruction, program::invoke,
};
use solana_program::instruction::AccountMeta;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

struct MetaTemplate {
    is_signer: bool,
    is_writable: bool,
}

const SEND_META_TEMPLATE: [MetaTemplate; 18] = [
    /// Bridge
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    /// Token Mint
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    /// Staking wallet
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    /// Mint authority
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    /// Chain support info
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    /// Settings program
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    /// Spl token program
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    /// State
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    /// Fee beneficiary
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    /// Nonce storage
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    /// Send from wallet
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    /// System program
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    /// External call storage
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    /// External call meta
    MetaTemplate {
        is_signer: false,
        is_writable: true,
    },
    /// Send from
    MetaTemplate {
        is_signer: true,
        is_writable: true,
    },
    /// Discount
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    /// Bridge fee
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
    /// Debridge program
    MetaTemplate {
        is_signer: false,
        is_writable: false,
    },
];

pub fn invoke_send(account_infos: &[AccountInfo]) -> ProgramResult {

    if account_infos.len() < SEND_META_TEMPLATE.len() {
        return Err(ProgramError::NotEnoughAccountKeys)
    }

    let ix = Instruction {
        //TODO: Need to check debridge id?
        program_id: account_infos[SEND_META_TEMPLATE.len() - 1].key.clone(),
        accounts: account_infos.iter().take(SEND_META_TEMPLATE.len()).zip(SEND_META_TEMPLATE)
            .map(|(acc, meta)| AccountMeta {
                pubkey: acc.key.clone(),
                is_signer: meta.is_signer,
                is_writable: meta.is_writable,
            }).collect(),
        data: vec![],
    };

    invoke(&ix, account_infos)
}
