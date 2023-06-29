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

/// Keccak256 Hash Function
/// Isolates work with Solana's hash function
pub trait HashAdapter {
    fn hash(input: &[u8]) -> [u8; 32];
}

/// Allows you to call [`solana_program::hash::hash`] via [`HashAdapter`] trait
pub struct SolanaKeccak256;

impl HashAdapter for SolanaKeccak256 {
    fn hash(input: &[u8]) -> [u8; 32] {
        solana_program::keccak::hash(input).to_bytes()
    }
}

#[cfg(not(target_arch = "bpf"))]
impl HashAdapter for sha3::Keccak256 {
    fn hash(input: &[u8]) -> [u8; 32] {
        use sha3::Digest;
        sha3::Keccak256::digest(input).try_into().unwrap()
    }
}