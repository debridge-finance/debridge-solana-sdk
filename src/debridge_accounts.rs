use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;

use crate::{Error, Pubkey};

trait Discriminator {
    fn discriminator() -> [u8; 8];
}

pub trait TryFromAccount: Sized + BorshSerialize + BorshDeserialize {
    type Error;

    fn try_from_account(account_info: &AccountInfo) -> Result<Self, Self::Error>;
}

impl<ACCOUNT: Discriminator + Sized + BorshSerialize + BorshDeserialize> TryFromAccount
    for ACCOUNT
{
    type Error = Error;

    fn try_from_account(account_info: &AccountInfo) -> Result<Self, Self::Error> {
        let borrow_data = account_info
            .try_borrow_data()
            .map_err(|_| Error::AccountBorrowFailing)?;
        let (discriminator, mut data) = borrow_data.split_at(8);

        if discriminator.ne(&Self::discriminator()) {
            return Err(Error::WrongAccountDiscriminator);
        }

        Self::deserialize(&mut data).map_err(|_| Error::AccountDeserializeError)
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SubmissionAccount {
    pub claimer: Pubkey,
    pub receiver: Pubkey,
    pub fallback_address: Pubkey,
    pub token_mint: Pubkey,
    pub native_sender: Option<Vec<u8>>,
    pub source_chain_id: [u8; 32],
    pub bump: u8,
}

const SUBMISSION_ACCOUNT_DISCRIMINATOR: [u8; 8] = [254, 14, 34, 50, 170, 36, 60, 191];
impl Discriminator for SubmissionAccount {
    fn discriminator() -> [u8; 8] {
        SUBMISSION_ACCOUNT_DISCRIMINATOR
    }
}

/// Internal information about chain support and commissions within it
/// # Variants
/// * [`ChainSupportInfoVariant::NotSupported`] - this chain not supported
/// * [`ChainSupportInfoVariant::Supported`] - this chain supported and we have `fixed_fee` & `transfer_fee` values for it
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub enum ChainSupportInfo {
    NotSupported,
    Supported {
        /// Fixed fee in SOL
        fixed_fee: Option<u64>,
        /// Transfer fee in bridge tokens
        transfer_fee_bps: Option<u64>,
        /// Length of address in this chain
        chain_address_len: u16,
    },
}

impl ChainSupportInfo {
    pub const SEED: &'static [u8] = b"CHAIN_SUPPORT_INFO";
}

const CHAIN_SUPPORT_INFO_ACCOUNT_DISCRIMINATOR: [u8; 8] = [175, 59, 40, 127, 55, 33, 200, 203];
impl Discriminator for ChainSupportInfo {
    fn discriminator() -> [u8; 8] {
        CHAIN_SUPPORT_INFO_ACCOUNT_DISCRIMINATOR
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub enum Status {
    Working,
    Paused,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct ConfirmationParamsGuard {
    current_timeslot: Option<u64>,
    submission_in_timeslot_count: u32,
    confirmation_threshold: u32,
    excess_confirmations: u32,
    min_confirmations: u32,
    excess_confirmation_timeslot: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct State {
    /// Current full protocol status
    pub status: Status,
    /// - 👤 Protocol Authority - multi-signature account with extra privilege for setup protocol settings
    pub protocol_authority: Pubkey,
    /// - 👤 Stop Tap - this account that has the authority to stop the protocol, but does not have the authority to start it
    pub stop_tap: Pubkey,
    /// Beneficiary of the commission within the system
    /// This is intended to be a separate profit sharing contract.
    pub fee_beneficiary: Pubkey,
    /// deBridge oracles that provide signatures for verifying external actions
    pub oracles: Vec<[u8; 20]>,
    /// Mandatory deBridge oracles that provide signatures for verifying external actions
    /// Signatures of these oracles are always required
    pub required_oracles: Vec<[u8; 20]>,
    /// Stores the logic of the required number of submissions for the actions
    confirmation_guard: ConfirmationParamsGuard,
    /// Fixed fee in SOL
    pub global_fixed_fee: u64,
    /// Transfer fee in bridge tokens
    pub global_transfer_fee_bps: u64,
}

const STATE_ACCOUNT_DISCRIMINATOR: [u8; 8] = [216, 146, 107, 94, 104, 75, 182, 177];
impl Discriminator for State {
    fn discriminator() -> [u8; 8] {
        STATE_ACCOUNT_DISCRIMINATOR
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct AssetFeeInfo {
    pub bridge_fee_bump: u8,
    /// Fee for this chain in bridge asset
    pub asset_chain_fee: Option<u64>,
}

impl AssetFeeInfo {
    pub const SEED: &'static [u8] = b"BRIDGE_FEE_INFO";
    pub const DEFAULT_ASSET_FEE_SEED: &'static [u8] = b"DEFAULT_BRIDGE_FEE_INFO";
}

const ASSET_FEE_DISCRIMINATOR: [u8; 8] = [37, 184, 34, 110, 54, 84, 57, 85];
impl Discriminator for AssetFeeInfo {
    fn discriminator() -> [u8; 8] {
        ASSET_FEE_DISCRIMINATOR
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct Bridge {
    /// Maximum amount to transfer
    pub max_amount: u64,
    /// Total locked assets
    pub balance: u64,
    /// Total locked assets in strategy (AAVE, Compound, etc)
    locked_in_strategies: u64,
    /// Minimal hot reserves in basis points (1/10000)
    pub min_reserves_bps: u64,
    pub state: BridgeState,
    /// Total collected fees
    pub collected_fee: u64,
    /// Fees that already withdrawn
    pub withdrawn_fee: u64,
    /// Total fees collected in lamports
    pub collected_native_fee: u64,
}

const BRIDGE_DISCRIMINATOR: [u8; 8] = [231, 232, 31, 98, 110, 3, 23, 59];
impl Discriminator for Bridge {
    fn discriminator() -> [u8; 8] {
        BRIDGE_DISCRIMINATOR
    }
}

impl Bridge {
    pub const SEED: &'static [u8] = b"BRIDGE";
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub enum BridgeState {
    Work,
    Paused,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub enum ExternalCallMeta {
    Accumulation {
        external_call_len: u64,
    },
    Execution {
        /// Offset to start external call
        offset: u64,
        external_call_len: u64,
        submission_auth_bump: u8,
    },
    Transferred {
        last_transfer_time: i64,
    },
    Executed,
    Failed,
}

const EXTERNAL_CALL_META_DISCRIMINATOR: [u8; 8] = [52, 154, 212, 31, 208, 203, 151, 253];
impl Discriminator for ExternalCallMeta {
    fn discriminator() -> [u8; 8] {
        EXTERNAL_CALL_META_DISCRIMINATOR
    }
}
