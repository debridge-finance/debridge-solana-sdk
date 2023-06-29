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

/// This constant represents the solana chain id
/// Check <https://chainlist.org/> for other chain ids
///
/// Within our network, we use the chain id as the
/// identifier for each network.
pub const SOLANA_CHAIN_ID: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 115,
    111, 108,
];

/// This constant represents the eth chain id
/// Check <https://chainlist.org/> for other chain ids
///
/// Within our network, we use the chain id as the
/// identifier for each network.
pub const ETHEREUM_CHAIN_ID: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
];

/// This constant represents the bnb chain id
/// Check <https://chainlist.org/> for other chain ids
///
/// Within our network, we use the chain id as the
/// identifier for each network.
pub const BNB_CHAIN_CHAIN_ID: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56,
];

/// This constant represents the heco chain id
/// Check <https://chainlist.org/> for other chain ids
///
/// Within our network, we use the chain id as the
/// identifier for each network.
pub const HECO_CHAIN_ID: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    128,
];

/// This constant represents the polygon chain id
/// Check <https://chainlist.org/> for other chain ids
///
/// Within our network, we use the chain id as the
/// identifier for each network.
pub const POLYGON_CHAIN_ID: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    137,
];

/// This constant represents the arbitrum chain id
/// Check <https://chainlist.org/> for other chain ids
///
/// Within our network, we use the chain id as the
/// identifier for each network.
pub const ARBITRUM_CHAIN_ID: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 164,
    177,
];

/// This constant represents the avalanche chain id
/// Check <https://chainlist.org/> for other chain ids
///
/// Within our network, we use the chain id as the
/// identifier for each network.
pub const AVALANCHE_CHAIN_ID: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 168,
    106,
];

/// This constant represents the fantom chain id
/// Check <https://chainlist.org/> for other chain ids
///
/// Within our network, we use the chain id as the
/// identifier for each network.
pub const FANTOM_CHAIN_ID: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    250,
];