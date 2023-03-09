use solana_program::pubkey::ParsePubkeyError;

use crate::{
    debridge_accounts::{AssetFeeInfo, Bridge, ChainSupportInfo},
    Error, Pubkey, DEBRIDGE_ID, SETTINGS_ID, SOLANA_CHAIN_ID,
};

/// This trait is responsible for finding the pubkey for the [`ChainSupportInfo`] account
pub trait ChainSupportInfoPubkey {
    fn find_chain_support_info_address(chain_id: &[u8; 32]) -> Result<(Pubkey, u8), Error> {
        Ok(Pubkey::find_program_address(
            &[ChainSupportInfo::SEED, chain_id],
            &SETTINGS_ID,
        ))
    }
    fn create_chain_support_info_address(
        chain_id: &[u8; 32],
        bump: u8,
    ) -> Result<Option<Pubkey>, ParsePubkeyError> {
        Ok(Pubkey::create_program_address(
            &[ChainSupportInfo::SEED, chain_id, &[bump][..]],
            &SETTINGS_ID,
        )
        .ok())
    }
}
impl ChainSupportInfoPubkey for Pubkey {}

/// This trait is responsible for finding the pubkey for the [`AssetFeeInfo`] account
pub trait AssetFeeInfoPubkey {
    fn find_asset_fee_info_address(
        bridge: &Pubkey,
        chain_id: &[u8; 32],
    ) -> Result<(Pubkey, u8), Error> {
        Ok(Pubkey::find_program_address(
            &[AssetFeeInfo::SEED, bridge.as_ref(), chain_id],
            &SETTINGS_ID,
        ))
    }

    fn create_asset_fee_info_address(
        bridge: &Pubkey,
        chain_id: &[u8; 32],
        bump: u8,
    ) -> Result<Option<Pubkey>, Error> {
        Ok(Pubkey::create_program_address(
            &[AssetFeeInfo::SEED, bridge.as_ref(), chain_id, &[bump]],
            &SETTINGS_ID,
        )
        .ok())
    }

    fn default_bridge_fee_address() -> Result<(Pubkey, u8), Error> {
        Ok(Pubkey::find_program_address(
            &[AssetFeeInfo::DEFAULT_ASSET_FEE_SEED],
            &SETTINGS_ID,
        ))
    }
}
impl AssetFeeInfoPubkey for Pubkey {}

pub trait BridgePubkey {
    fn find_bridge_address(token_mint: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[Bridge::SEED, token_mint.as_ref()], &SETTINGS_ID)
    }

    fn create_bridge_address(token_mint: &Pubkey, bump: u8) -> Result<Option<Pubkey>, Error> {
        Ok(Pubkey::create_program_address(
            &[Bridge::SEED, token_mint.as_ref(), &[bump]],
            &SETTINGS_ID,
        )
        .ok())
    }
}
impl BridgePubkey for Pubkey {}

#[cfg(test)]
mod tests {
    use crate::keys::ChainSupportInfoPubkey;
    use std::str::FromStr;

    use solana_program::pubkey::Pubkey;

    #[test]
    fn find_chain_support_info_test() {
        let target_chain_id = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 137,
        ];
        assert_eq!(
            Pubkey::find_chain_support_info_address(&target_chain_id)
                .unwrap()
                .0,
            Pubkey::from_str("8L81QZBfwA6Xi9zd49fyUfMRWJBCAxiUxd6jGHPnu1BQ").unwrap()
        );
    }
}

pub trait ExternalCallStoragePubkey {
    fn find_external_call_storage_address(shortcut: &[u8; 32], owner: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                b"EXTERNAL_CALL_STORAGE",
                shortcut,
                owner.as_ref(),
                &SOLANA_CHAIN_ID,
            ],
            &DEBRIDGE_ID,
        )
    }

    fn create_external_call_storage_address(
        shortcut: &[u8; 32],
        owner: &Pubkey,
        bump: u8,
    ) -> Result<Option<Pubkey>, Error> {
        Ok(Pubkey::create_program_address(
            &[
                b"EXTERNAL_CALL_STORAGE",
                shortcut,
                owner.as_ref(),
                &SOLANA_CHAIN_ID,
                &[bump],
            ],
            &DEBRIDGE_ID,
        )
        .ok())
    }
}
impl ExternalCallStoragePubkey for Pubkey {}

pub trait ExternalCallMetaPubkey {
    fn find_external_call_meta_address(external_call_storage: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[b"EXTERNAL_CALL_META", external_call_storage.as_ref()],
            &DEBRIDGE_ID,
        )
    }

    fn create_external_call_meta_address(
        external_call_storage: &[u8; 32],
        bump: u8,
    ) -> Result<Option<Pubkey>, Error> {
        Ok(Pubkey::create_program_address(
            &[
                b"EXTERNAL_CALL_META",
                external_call_storage.as_ref(),
                &[bump],
            ],
            &DEBRIDGE_ID,
        )
        .ok())
    }
}
impl ExternalCallMetaPubkey for Pubkey {}
