use alloc::vec;
use core::marker::PhantomData;
use frame_support::traits::GetStorageVersion;

use pallet_evm::PrecompileHandle;
use precompile_utils::EvmResult;
use sp_core::H256;

use crate::PrecompileExt;

pub(crate) struct SchedulerPrecompile<R>(PhantomData<R>);

impl<R> PrecompileExt<R::AccountId> for SchedulerPrecompile<R>
where
    R: frame_system::Config + pallet_scheduler::Config + pallet_evm::Config,
    R::AccountId: From<[u8; 32]>,
{
    const INDEX: u64 = 2067;
}

#[precompile_utils::precompile]
impl<R> SchedulerPrecompile<R>
where
    R: frame_system::Config + pallet_scheduler::Config + pallet_evm::Config,
{
    /// Returns the active scheduler lookup key
    #[precompile::public("getLookup(bytes32)")]
    #[precompile::view]
    fn get_lookup(_handle: &mut impl PrecompileHandle, name: H256) -> EvmResult<(u64, u32)> {
        let name_bytes: [u8; 32] = name.0;
        
        match pallet_scheduler::Lookup::<R>::get(&name_bytes) {
            Some((block_number, task_index)) => {
                use sp_runtime::traits::UniqueSaturatedInto;
                Ok((
                    block_number.unique_saturated_into(),
                    task_index
                ))
            },
            None => Ok((0, 0)),
        }
    }

    /// Returns the block number at which the agenda began incomplete execution.
    #[precompile::public("getIncompleteSince()")]
    #[precompile::view]
    fn get_incomplete_since(_handle: &mut impl PrecompileHandle) -> EvmResult<u64> {
        use sp_runtime::traits::UniqueSaturatedInto;
        match pallet_scheduler::IncompleteSince::<R>::get() {
            Some(block_number) => Ok(block_number.unique_saturated_into()),
            None => Ok(0),
        }
    }

    /// Returns the current pallet version from storage.
    #[precompile::public("getPalletVersion()")]
    #[precompile::view]
    fn get_pallet_version(_handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        Ok(
            <pallet_scheduler::Pallet<R> as frame_support::traits::PalletInfoAccess>::crate_version()
                .major as u16
        )
    }
}
