use solana_program::{rent::Rent, sysvar::Sysvar};

use crate::{debridge_accounts::ExternalCallMeta, Error};

pub fn get_native_sender_lamports_expenses(
    fix_fee: u64,
    external_call_len: usize,
) -> Result<u64, Error> {
    let rent = Rent::get().map_err(|_| Error::FailedToGetRent)?;

    let external_call_rent = rent.minimum_balance(8 + external_call_len);
    let external_call_meta_rent = rent.minimum_balance(ExternalCallMeta::SPACE);

    Ok(external_call_rent + external_call_meta_rent + fix_fee)
}
