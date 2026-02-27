use alloc::vec;
use core::marker::PhantomData;
use frame_support::traits::GetStorageVersion;

use pallet_evm::PrecompileHandle;
use precompile_utils::EvmResult;
use sp_core::H256;

use frame_support::pallet_prelude::Encode;
use sp_runtime::traits::UniqueSaturatedInto;

use crate::PrecompileExt;

pub(crate) struct MultisigPrecompile<R>(PhantomData<R>);

impl<R> PrecompileExt<R::AccountId> for MultisigPrecompile<R>
where
    R: frame_system::Config
        + pallet_multisig::Config
        + pallet_evm::Config,
    R::AccountId: From<[u8; 32]> + Into<[u8; 32]>,
{
    const INDEX: u64 = 2069;
}

#[precompile_utils::precompile]
impl<R> MultisigPrecompile<R>
where
    R: frame_system::Config
        + pallet_multisig::Config
        + pallet_evm::Config,
    R::AccountId: From<[u8; 32]> + Into<[u8; 32]>,
{
    /// Returns multisig info for a given account and call hash.
    #[precompile::public("getMultisig(bytes32,bytes32)")]
    #[precompile::view]
    fn get_multisig(
        _handle: &mut impl PrecompileHandle,
        account: H256,
        call_hash: H256,
    ) -> EvmResult<(u64, sp_std::vec::Vec<H256>, u16)> {
        let account_id = R::AccountId::from(account.0);
        
        let hash_bytes: [u8; 32] = call_hash.0;
        let mut key = sp_std::vec::Vec::with_capacity(32);
        key.extend_from_slice(&hash_bytes);
        
        match pallet_multisig::Multisigs::<R>::get(&account_id, &hash_bytes) {
            Some(multisig) => {
                let approvals_len = multisig.approvals.len() as u16;
                let approvals = multisig.approvals.into_iter().map(|a| {
                    let encoded = a.encode();
                    let mut id_bytes = [0u8; 32];
                    let len = core::cmp::min(encoded.len(), 32);
                    id_bytes[0..len].copy_from_slice(&encoded[0..len]);
                    H256::from(id_bytes)
                }).collect();
                
                Ok((
                    multisig.deposit.unique_saturated_into(),
                    approvals,
                    approvals_len,
                ))
            },
            None => Ok((0, vec![], 0)),
        }
    }

    /// Returns the current pallet version from storage.
    #[precompile::public("getPalletVersion()")]
    #[precompile::view]
    fn get_pallet_version(_handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        // pallet_multisig doesn't natively expose `current_storage_version()`.
        // We'll return the crate version instead as a fallback, 
        // similar to other implementations without explicit storage versions.
        Ok(
            <pallet_multisig::Pallet<R> as frame_support::traits::PalletInfoAccess>::crate_version()
                .major as u16
        )
    }
}
