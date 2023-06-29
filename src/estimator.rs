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