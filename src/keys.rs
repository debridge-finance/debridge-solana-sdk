use std::str::FromStr;

use solana_program::pubkey::ParsePubkeyError;

use crate::{
    debridge_accounts::{AssetFeeInfo, Bridge, ChainSupportInfo},
    Error, Pubkey, DEBRIDGE_ID_RAW, SETTINGS_ID_RAW, SOLANA_CHAIN_ID,
};

pub trait ChainSupportInfoPubkey {
    fn find_chain_support_info_address(chain_id: &[u8; 32]) -> Result<(Pubkey, u8), Error> {
        Ok(Pubkey::find_program_address(
            &[ChainSupportInfo::SEED, chain_id],
            &Pubkey::from_str(SETTINGS_ID_RAW).map_err(|_| Error::WrongSettingProgramId)?,
        ))
    }
    fn create_chain_support_info_address(
        chain_id: &[u8; 32],
        bump: u8,
    ) -> Result<Option<Pubkey>, ParsePubkeyError> {
        Ok(Pubkey::create_program_address(
            &[ChainSupportInfo::SEED, chain_id, &[bump][..]],
            &Pubkey::from_str(SETTINGS_ID_RAW)?,
        )
        .ok())
    }
}
impl ChainSupportInfoPubkey for Pubkey {}

pub trait AssetFeeInfoPubkey {
    fn find_asset_fee_info_address(
        bridge: &Pubkey,
        chain_id: &[u8; 32],
    ) -> Result<(Pubkey, u8), Error> {
        Ok(Pubkey::find_program_address(
            &[AssetFeeInfo::SEED, bridge.as_ref(), chain_id],
            &Pubkey::from_str(SETTINGS_ID_RAW).map_err(|_| Error::WrongSettingProgramId)?,
        ))
    }

    fn create_asset_fee_info_address(
        bridge: &Pubkey,
        chain_id: &[u8; 32],
        bump: u8,
    ) -> Result<Option<Pubkey>, Error> {
        Ok(Pubkey::create_program_address(
            &[AssetFeeInfo::SEED, bridge.as_ref(), chain_id, &[bump]],
            &Pubkey::from_str(SETTINGS_ID_RAW).map_err(|_| Error::WrongSettingProgramId)?,
        )
        .ok())
    }

    fn default_bridge_fee_address() -> Result<(Pubkey, u8), Error> {
        Ok(Pubkey::find_program_address(
            &[AssetFeeInfo::DEFAULT_ASSET_FEE_SEED],
            &Pubkey::from_str(SETTINGS_ID_RAW).map_err(|_| Error::WrongSettingProgramId)?,
        ))
    }
}
impl AssetFeeInfoPubkey for Pubkey {}

pub trait BridgePubkey {
    fn find_bridge_address(token_mint: &Pubkey) -> Result<(Pubkey, u8), Error> {
        Ok(Pubkey::find_program_address(
            &[Bridge::SEED, token_mint.as_ref()],
            &Pubkey::from_str(SETTINGS_ID_RAW).map_err(|_| Error::WrongSettingProgramId)?,
        ))
    }
    fn create_bridge_address(token_mint: &Pubkey, bump: u8) -> Result<Option<Pubkey>, Error> {
        Ok(Pubkey::create_program_address(
            &[Bridge::SEED, token_mint.as_ref(), &[bump]],
            &Pubkey::from_str(SETTINGS_ID_RAW).map_err(|_| Error::WrongSettingProgramId)?,
        )
        .ok())
    }
}
impl BridgePubkey for Pubkey {}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use solana_program::pubkey::Pubkey;

    use crate::keys::ChainSupportInfoPubkey;

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
    fn find_external_call_storage_address(
        shortcut: &[u8; 32],
        owner: &Pubkey,
    ) -> Result<(Pubkey, u8), Error> {
        Ok(Pubkey::find_program_address(
            &[
                b"EXTERNAL_CALL_STORAGE",
                shortcut,
                owner.as_ref(),
                &SOLANA_CHAIN_ID,
            ],
            &Pubkey::from_str(DEBRIDGE_ID_RAW).map_err(|_| Error::WrongDebridgeProgramId)?,
        ))
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
            &Pubkey::from_str(DEBRIDGE_ID_RAW).map_err(|_| Error::WrongDebridgeProgramId)?,
        )
        .ok())
    }
}
impl ExternalCallStoragePubkey for Pubkey {}

pub trait ExternalCallMetaPubkey {
    fn find_external_call_meta_address(
        external_call_storage: &Pubkey,
    ) -> Result<(Pubkey, u8), Error> {
        Ok(Pubkey::find_program_address(
            &[b"EXTERNAL_CALL_META", external_call_storage.as_ref()],
            &Pubkey::from_str(DEBRIDGE_ID_RAW).map_err(|_| Error::WrongDebridgeProgramId)?,
        ))
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
            &Pubkey::from_str(DEBRIDGE_ID_RAW).map_err(|_| Error::WrongDebridgeProgramId)?,
        )
        .ok())
    }
}
impl ExternalCallMetaPubkey for Pubkey {}
