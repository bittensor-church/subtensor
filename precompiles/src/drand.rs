use core::marker::PhantomData;


use pallet_evm::PrecompileHandle;
use precompile_utils::{EvmResult, prelude::UnboundedBytes};
use sp_core::H256;

use crate::{PrecompileExt, PrecompileHandleExtStorage};

pub(crate) struct DrandPrecompile<R>(PhantomData<R>);

impl<R> PrecompileExt<R::AccountId> for DrandPrecompile<R>
where
    R: frame_system::Config + pallet_drand::Config + pallet_evm::Config,
    R::AccountId: From<[u8; 32]>,
{
    const INDEX: u64 = 2065;
}

#[precompile_utils::precompile]
impl<R> DrandPrecompile<R>
where
    R: frame_system::Config + pallet_drand::Config + pallet_evm::Config,
{
    /// Returns the last stored drand round number.
    #[precompile::public("getLastStoredRound()")]
    #[precompile::view]
    fn get_last_stored_round(handle: &mut impl PrecompileHandle) -> EvmResult<u64> {
        {
            let val = pallet_drand::LastStoredRound::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
            Ok(val)
        }
    }

    /// Returns the oldest stored drand round number.
    #[precompile::public("getOldestStoredRound()")]
    #[precompile::view]
    fn get_oldest_stored_round(handle: &mut impl PrecompileHandle) -> EvmResult<u64> {
        {
            let val = pallet_drand::OldestStoredRound::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
            Ok(val)
        }
    }

    /// Returns the pulse (randomness, signature) for a specific round.
    /// If the round is not found, returns empty bytes.
    #[precompile::public("getPulse(uint64)")]
    #[precompile::view]
    fn get_pulse(
        handle: &mut impl PrecompileHandle,
        round: u64,
    ) -> EvmResult<(UnboundedBytes, UnboundedBytes)> {
        let __matched_val = pallet_drand::Pulses::<R>::get(round);
        handle.record_db_read_encoded::<R>(&__matched_val)?;
        match __matched_val {
            Some(pulse) => {
                let randomness: UnboundedBytes = pulse.randomness.into_inner().into();
                let signature: UnboundedBytes = pulse.signature.into_inner().into();
                Ok((randomness, signature))
            }
            None => Ok((
                UnboundedBytes::from(&b""[..]),
                UnboundedBytes::from(&b""[..]),
            )),
        }
    }

    /// Returns the randomness from the latest stored round as bytes32.
    /// Returns zero bytes if no pulse is stored.
    #[precompile::public("getCurrentRandomness()")]
    #[precompile::view]
    fn get_current_randomness(handle: &mut impl PrecompileHandle) -> EvmResult<H256> {
        let last_round = pallet_drand::LastStoredRound::<R>::get();
        handle.record_db_read_encoded::<R>(&last_round)?;
        let __matched_val = pallet_drand::Pulses::<R>::get(last_round);
        handle.record_db_read_encoded::<R>(&__matched_val)?;
        match __matched_val {
            Some(pulse) => {
                let rand = pulse.randomness.into_inner();
                let bounded: [u8; 32] = rand.try_into().unwrap_or([0u8; 32]);
                Ok(H256::from(bounded))
            }
            None => Ok(H256::zero()),
        }
    }

    /// Returns the drand beacon configuration.
    #[precompile::public("getBeaconConfig()")]
    #[precompile::view]
    fn get_beacon_config(handle: &mut impl PrecompileHandle) -> EvmResult<(u64, u32, UnboundedBytes, bool)> {
        let config = pallet_drand::BeaconConfig::<R>::get();
        handle.record_db_read_encoded::<R>(&config)?;
        let pubkey = config.public_key.into_inner().into();
        Ok((config.genesis_time.into(), config.period, pubkey, true))
    }

    /// Returns if a specific migration has run.
    #[precompile::public("getHasMigrationRun(bytes32)")]
    #[precompile::view]
    fn get_has_migration_run(
        handle: &mut impl PrecompileHandle,
        migration_hash: H256,
    ) -> EvmResult<bool> {
        let hash_bytes: [u8; 32] = migration_hash.0;
        let bounded_key: sp_runtime::BoundedVec<u8, _> = sp_runtime::BoundedVec::try_from(hash_bytes.to_vec()).unwrap_or_default();
        {
            let val = pallet_drand::HasMigrationRun::<R>::get(&bounded_key);
        handle.record_db_read_encoded::<R>(&val)?;
            Ok(val)
        }
    }

    /// Returns the block when the next unsigned transaction will be accepted.
    #[precompile::public("getNextUnsignedAt()")]
    #[precompile::view]
    fn get_next_unsigned_at(_handle: &mut impl PrecompileHandle) -> EvmResult<u64> {
        Ok(0)
    }

    /// Returns the current pallet version from storage.
    #[precompile::public("getPalletVersion()")]
    #[precompile::view]
    fn get_pallet_version(_handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        Ok(
            <pallet_drand::Pallet<R> as frame_support::traits::PalletInfoAccess>::crate_version()
                .major as u16
        )
    }
}