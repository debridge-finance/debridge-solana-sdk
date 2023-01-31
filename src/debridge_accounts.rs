use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;

use crate::{Error, Pubkey};

pub trait Discriminator {
    fn discriminator() -> [u8; 8];
}

pub trait TryFromAccount: Sized + BorshSerialize + BorshDeserialize {
    type Error;

    fn try_from_accounts(account_info: &AccountInfo) -> Result<Self, Self::Error>;
}

impl<ACCOUNT: Discriminator + Sized + BorshSerialize + BorshDeserialize> TryFromAccount
    for ACCOUNT
{
    type Error = Error;

    fn try_from_accounts(account_info: &AccountInfo) -> Result<Self, Self::Error> {
        let borrow_data = account_info
            .try_borrow_data()
            .map_err(|_| Error::AccountBorrowFailing)?;
        let (discriminator, data) = borrow_data.split_at(8);

        if discriminator.ne(&Self::discriminator()) {
            return Err(Error::WrongAccountDiscriminator);
        }

        Self::try_from_slice(data).map_err(|_| Error::AccountDeserializeError)
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
/// * [`ChainSupportInfoVariant::Supported] - this chain supported and we have `fixed_fee` & `transfer_fee` values for it
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

const ASSET_FEE_DISCRIMINATOR: [u8; 8] = [37, 184, 34, 110, 54, 84, 57, 85];
impl Discriminator for AssetFeeInfo {
    fn discriminator() -> [u8; 8] {
        ASSET_FEE_DISCRIMINATOR
    }
}