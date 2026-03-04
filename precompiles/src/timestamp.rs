use core::marker::PhantomData;


use frame_support::traits::Time;
use pallet_evm::PrecompileHandle;
use precompile_utils::EvmResult;
use sp_runtime::traits::UniqueSaturatedInto;

use crate::PrecompileExt;

pub(crate) struct TimestampPrecompile<R>(PhantomData<R>);

impl<R> PrecompileExt<R::AccountId> for TimestampPrecompile<R>
where
    R: frame_system::Config + pallet_timestamp::Config + pallet_evm::Config,
    R::AccountId: From<[u8; 32]>,
{
    const INDEX: u64 = 2066;
}

#[precompile_utils::precompile]
impl<R> TimestampPrecompile<R>
where
    R: frame_system::Config + pallet_timestamp::Config + pallet_evm::Config,
{
    /// Returns the current chain timestamp in milliseconds.
    #[precompile::public("getNow()")]
    #[precompile::view]
    fn get_now(_handle: &mut impl PrecompileHandle) -> EvmResult<u64> {
        let now: u64 = pallet_timestamp::Pallet::<R>::now().unique_saturated_into();
        Ok(now)
    }
    /// Returns whether the timestamp has been updated in this block.
    #[precompile::public("getDidUpdate()")]
    #[precompile::view]
    fn get_did_update(_handle: &mut impl PrecompileHandle) -> EvmResult<bool> {
        Ok(false)
    }

    /// Returns the current pallet version from storage.
    #[precompile::public("getPalletVersion()")]
    #[precompile::view]
    fn get_pallet_version(_handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        Ok(
            <pallet_timestamp::Pallet<R> as frame_support::traits::PalletInfoAccess>::crate_version()
                .major as u16
        )
    }
}