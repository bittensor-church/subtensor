use core::marker::PhantomData;

use frame_support::dispatch::{DispatchInfo, GetDispatchInfo, PostDispatchInfo};
use frame_support::traits::{IsSubType, fungible::Inspect};
use frame_system::RawOrigin;
use pallet_evm::PrecompileHandle;
use precompile_utils::EvmResult;
use sp_core::{H256, U256};
use sp_runtime::traits::{AsSystemOriginSigner, Dispatchable, StaticLookup, UniqueSaturatedInto};

use crate::{PrecompileExt, PrecompileHandleExt};

pub(crate) struct BalanceTransferPrecompile<R>(PhantomData<R>);

impl<R> PrecompileExt<R::AccountId> for BalanceTransferPrecompile<R>
where
    R: frame_system::Config
        + pallet_balances::Config
        + pallet_evm::Config
        + pallet_subtensor::Config
        + Send
        + Sync
        + scale_info::TypeInfo,
    R::AccountId: From<[u8; 32]>,
    <R as frame_system::Config>::RuntimeOrigin: AsSystemOriginSigner<R::AccountId> + Clone,
    <R as frame_system::Config>::RuntimeCall: GetDispatchInfo
        + Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>
        + IsSubType<pallet_balances::Call<R>>
        + IsSubType<pallet_subtensor::Call<R>>,
    <R as frame_system::Config>::RuntimeCall: From<pallet_balances::Call<R>>
        + GetDispatchInfo
        + Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
    <<R as frame_system::Config>::Lookup as StaticLookup>::Source: From<R::AccountId>,
    <R as pallet_balances::Config>::Balance: TryFrom<U256>,
{
    const INDEX: u64 = 2048;
}

#[precompile_utils::precompile]
impl<R> BalanceTransferPrecompile<R>
where
    R: frame_system::Config
        + pallet_balances::Config
        + pallet_evm::Config
        + pallet_subtensor::Config
        + Send
        + Sync
        + scale_info::TypeInfo,
    R::AccountId: From<[u8; 32]>,
    <R as frame_system::Config>::RuntimeOrigin: AsSystemOriginSigner<R::AccountId> + Clone,
    <R as frame_system::Config>::RuntimeCall: GetDispatchInfo
        + Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>
        + IsSubType<pallet_balances::Call<R>>
        + IsSubType<pallet_subtensor::Call<R>>,
    <R as frame_system::Config>::RuntimeCall: From<pallet_balances::Call<R>>
        + GetDispatchInfo
        + Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
    <<R as frame_system::Config>::Lookup as StaticLookup>::Source: From<R::AccountId>,
    <R as pallet_balances::Config>::Balance: TryFrom<U256>,
{
    #[precompile::public("getPalletVersion()")]
    #[precompile::view]
    fn get_pallet_version(_handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        Ok(
            <pallet_balances::Pallet<R> as frame_support::traits::PalletInfoAccess>::crate_version()
                .major as u16
        )
    }

    #[precompile::public("getInactiveIssuance()")]
    #[precompile::view]
    fn get_inactive_issuance(_handle: &mut impl PrecompileHandle) -> EvmResult<u64> {
        Ok(pallet_balances::InactiveIssuance::<R>::get().unique_saturated_into())
    }

    #[precompile::public("getAccount(bytes32)")]
    #[precompile::view]
    fn get_account(
        _handle: &mut impl PrecompileHandle,
        account: H256,
    ) -> EvmResult<(u64, u64, u64, u64)> {
        let account_id = R::AccountId::from(account.0);
        let account_data = pallet_balances::Account::<R>::get(&account_id);
        Ok((
            account_data.free.unique_saturated_into(),
            account_data.reserved.unique_saturated_into(),
            account_data.frozen.unique_saturated_into(),
            0,
        ))
    }

    #[precompile::public("getLocks(bytes32)")]
    #[precompile::view]
    fn get_locks(
        _handle: &mut impl PrecompileHandle,
        account: H256,
    ) -> EvmResult<sp_std::vec::Vec<(sp_core::H256, u64, u8)>> {
        let account_id = R::AccountId::from(account.0);
        let locks = pallet_balances::Locks::<R>::get(&account_id);
        
        // Convert to a format returnable by the EVM. Id is 8 bytes, so we pad it.
        let result = locks.into_iter().map(|lock| {
            use frame_support::pallet_prelude::Encode;
            let encoded = lock.id.encode();
            let mut id_bytes = [0u8; 32];
            let len = core::cmp::min(encoded.len(), 32);
            id_bytes[0..len].copy_from_slice(&encoded[0..len]);
            (H256::from(id_bytes), lock.amount.unique_saturated_into(), lock.reasons as u8)
        }).collect();
        
        Ok(result)
    }

    #[precompile::public("getReserves(bytes32)")]
    #[precompile::view]
    fn get_reserves(
        _handle: &mut impl PrecompileHandle,
        account: H256,
    ) -> EvmResult<sp_std::vec::Vec<(sp_core::H256, u64)>> {
        let account_id = R::AccountId::from(account.0);
        let reserves = pallet_balances::Reserves::<R>::get(&account_id);
        
        let result = reserves.into_iter().map(|reserve| {
            use frame_support::pallet_prelude::Encode;
            let encoded = reserve.id.encode();
            let mut id_bytes = [0u8; 32];
            let len = core::cmp::min(encoded.len(), 32);
            id_bytes[0..len].copy_from_slice(&encoded[0..len]);
            (H256::from(id_bytes), reserve.amount.unique_saturated_into())
        }).collect();
        
        Ok(result)
    }

    #[precompile::public("getHolds(bytes32)")]
    #[precompile::view]
    fn get_holds(
        _handle: &mut impl PrecompileHandle,
        account: H256,
    ) -> EvmResult<sp_std::vec::Vec<(sp_core::H256, u64)>> {
        let account_id = R::AccountId::from(account.0);
        let holds = pallet_balances::Holds::<R>::get(&account_id);
        
        let result = holds.into_iter().map(|hold| {
            use frame_support::pallet_prelude::Encode;
            let encoded = hold.id.encode();
            let mut id_bytes = [0u8; 32];
            let len = core::cmp::min(encoded.len(), 32);
            id_bytes[0..len].copy_from_slice(&encoded[0..len]);
            (H256::from(id_bytes), hold.amount.unique_saturated_into())
        }).collect();
        
        Ok(result)
    }

    #[precompile::public("getFreezes(bytes32)")]
    #[precompile::view]
    fn get_freezes(
        _handle: &mut impl PrecompileHandle,
        account: H256,
    ) -> EvmResult<sp_std::vec::Vec<(sp_core::H256, u64)>> {
        let account_id = R::AccountId::from(account.0);
        let freezes = pallet_balances::Freezes::<R>::get(&account_id);
        
        let result = freezes.into_iter().map(|freeze| {
            use frame_support::pallet_prelude::Encode;
            let encoded = freeze.id.encode();
            let mut id_bytes = [0u8; 32];
            let len = core::cmp::min(encoded.len(), 32);
            id_bytes[0..len].copy_from_slice(&encoded[0..len]);
            (H256::from(id_bytes), freeze.amount.unique_saturated_into())
        }).collect();
        
        Ok(result)
    }

    #[precompile::public("transfer(bytes32)")]
    #[precompile::payable]
    fn transfer(handle: &mut impl PrecompileHandle, address: H256) -> EvmResult<()> {
        let amount_sub = handle.try_convert_apparent_value::<R>()?;

        if amount_sub.is_zero() {
            return Ok(());
        }

        let dest = R::AccountId::from(address.0).into();

        let call = pallet_balances::Call::<R>::transfer_allow_death {
            dest,
            value: amount_sub.unique_saturated_into(),
        };

        handle.try_dispatch_runtime_call::<R, _>(call, RawOrigin::Signed(Self::account_id()))
    }

    /// Returns the total issuance of the native token.
    #[precompile::public("getTotalIssuance()")]
    #[precompile::view]
    fn get_total_issuance(_handle: &mut impl PrecompileHandle) -> EvmResult<u64> {
        let total = <pallet_balances::Pallet<R> as Inspect<R::AccountId>>::total_issuance();
        Ok(total.unique_saturated_into())
    }

    /// Returns the free balance of an account.
    #[precompile::public("getFreeBalance(bytes32)")]
    #[precompile::view]
    fn get_free_balance(_handle: &mut impl PrecompileHandle, account: H256) -> EvmResult<u64> {
        let account_id = R::AccountId::from(account.0);
        let balance = <pallet_balances::Pallet<R> as Inspect<R::AccountId>>::balance(&account_id);
        Ok(balance.unique_saturated_into())
    }
}
