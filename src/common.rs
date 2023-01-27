use crate::Error;
use solana_program::account_info::AccountInfo;

pub fn is_chain_supported(target_chain_id: [u8; 32], remaining_accounts: &[AccountInfo]) -> bool {
    todo!()
}

pub fn get_chain_native_fix_fee(
    target_chain_id: [u8; 32],
    reamining_accounts: &[AccountInfo],
) -> u64 {
    todo!()
}

pub fn get_default_native_fix_fee(reamining_accounts: &[AccountInfo]) -> u64 {
    todo!()
}

pub fn is_asset_fee_avaliable(
    target_chain_id: [u8; 32],
    reamining_accounts: &[AccountInfo],
) -> u64 {
    todo!()
}

pub fn try_get_chain_asset_fix_fee(
    target_chain_id: [u8; 32],
    reamining_accounts: &[AccountInfo],
) -> Result<u64, Error> {
    todo!()
}
