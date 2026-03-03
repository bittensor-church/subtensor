use core::marker::PhantomData;


use pallet_evm::PrecompileHandle;
use precompile_utils::EvmResult;
use sp_core::H256;

use frame_support::pallet_prelude::Encode;

use crate::{PrecompileExt, PrecompileHandleExtStorage};

pub(crate) struct SudoPrecompile<R>(PhantomData<R>);

impl<R> PrecompileExt<R::AccountId> for SudoPrecompile<R>
where
    R: frame_system::Config + pallet_sudo::Config + pallet_evm::Config,
    R::AccountId: From<[u8; 32]>,
{
    const INDEX: u64 = 2068;
}

#[precompile_utils::precompile]
impl<R> SudoPrecompile<R>
where
    R: frame_system::Config + pallet_sudo::Config + pallet_evm::Config,
{
    /// Returns the active sudo key
    #[precompile::public("getKey()")]
    #[precompile::view]
    fn get_key(handle: &mut impl PrecompileHandle) -> EvmResult<H256> {
        let __matched_val = pallet_sudo::Key::<R>::get();
        handle.record_db_read_encoded::<R>(&__matched_val)?;
        match __matched_val {
            Some(key) => {
                // Key is an AccountId, transform to H256
                let encoded = key.encode();
                let mut id_bytes = [0u8; 32];
                let len = core::cmp::min(encoded.len(), 32);
                id_bytes[0..len].copy_from_slice(&encoded[0..len]);
                Ok(H256::from(id_bytes))
            }
            None => Ok(H256::zero()),
        }
    }

    /// Returns the current pallet version from storage.
    #[precompile::public("getPalletVersion()")]
    #[precompile::view]
    fn get_pallet_version(_handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        Ok(
            <pallet_sudo::Pallet<R> as frame_support::traits::PalletInfoAccess>::crate_version()
                .major as u16
        )
    }
}