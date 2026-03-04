#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(unused_parens)]

use core::marker::PhantomData;

use alloc::vec::Vec;
use frame_support::pallet_prelude::Encode;
use pallet_evm::PrecompileHandle;
use precompile_utils::{EvmResult, precompile};
use sp_core::{H160, H256, U256};
use subtensor_runtime_common::{NetUid, AlphaCurrency, TaoCurrency, NetUidStorageIndex, MechId};
use crate::{PrecompileExt, PrecompileHandleExt, PrecompileHandleExtStorage};

pub struct SubtensorPrecompile<R>(PhantomData<R>);

impl<R> PrecompileExt<R::AccountId> for SubtensorPrecompile<R>
where
    R: frame_system::Config
        + pallet_subtensor::Config
        + pallet_evm::Config,
    R::AccountId: From<[u8; 32]> + Into<[u8; 32]>,
{
    const INDEX: u64 = 2071;
}

#[precompile_utils::precompile]
impl<R> SubtensorPrecompile<R>
where
    R: frame_system::Config
        + pallet_subtensor::Config
        + pallet_evm::Config,
    R::AccountId: From<[u8; 32]> + Into<[u8; 32]>,
{
    #[precompile::public("getPalletVersion()")]
    #[precompile::view]
    fn get_pallet_version(_handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        Ok(<pallet_subtensor::Pallet<R> as frame_support::traits::PalletInfoAccess>::crate_version().major as u16)
    }

    #[precompile::public("getMinActivityCutoff()")]
    #[precompile::view]
    fn get_min_activity_cutoff(handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        let val = pallet_subtensor::MinActivityCutoff::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getAdminFreezeWindow()")]
    #[precompile::view]
    fn get_admin_freeze_window(handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        let val = pallet_subtensor::AdminFreezeWindow::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getOwnerHyperparamRateLimit()")]
    #[precompile::view]
    fn get_owner_hyperparam_rate_limit(handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        let val = pallet_subtensor::OwnerHyperparamRateLimit::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getColdkeySwapScheduleDuration()")]
    #[precompile::view]
    fn get_coldkey_swap_schedule_duration(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::ColdkeySwapScheduleDuration::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ use sp_runtime::traits::UniqueSaturatedInto; let v: u64 = val.unique_saturated_into(); U256::from(v) })
    }

    #[precompile::public("getColdkeySwapRescheduleDuration()")]
    #[precompile::view]
    fn get_coldkey_swap_reschedule_duration(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::ColdkeySwapRescheduleDuration::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ use sp_runtime::traits::UniqueSaturatedInto; let v: u64 = val.unique_saturated_into(); U256::from(v) })
    }

    #[precompile::public("getDissolveNetworkScheduleDuration()")]
    #[precompile::view]
    fn get_dissolve_network_schedule_duration(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::DissolveNetworkScheduleDuration::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ use sp_runtime::traits::UniqueSaturatedInto; let v: u64 = val.unique_saturated_into(); U256::from(v) })
    }

    #[precompile::public("getLastHotkeySwapOnNetuid(uint16,bytes32)")]
    #[precompile::view]
    fn get_last_hotkey_swap_on_netuid(handle: &mut impl PrecompileHandle, _key1: u16, _key2: H256) -> EvmResult<U256> {
        let val = pallet_subtensor::LastHotkeySwapOnNetuid::<R>::get(subtensor_runtime_common::NetUid::from(_key1), R::AccountId::from(_key2.0));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getNextStakeJobId()")]
    #[precompile::view]
    fn get_next_stake_job_id(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::NextStakeJobId::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getMaxDelegateTake()")]
    #[precompile::view]
    fn get_max_delegate_take(handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        let val = pallet_subtensor::MaxDelegateTake::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getMinDelegateTake()")]
    #[precompile::view]
    fn get_min_delegate_take(handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        let val = pallet_subtensor::MinDelegateTake::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getMaxChildkeyTake()")]
    #[precompile::view]
    fn get_max_childkey_take(handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        let val = pallet_subtensor::MaxChildkeyTake::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getMinChildkeyTake()")]
    #[precompile::view]
    fn get_min_childkey_take(handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        let val = pallet_subtensor::MinChildkeyTake::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getDelegates(bytes32)")]
    #[precompile::view]
    fn get_delegates(handle: &mut impl PrecompileHandle, _key1: H256) -> EvmResult<u16> {
        let val = pallet_subtensor::Delegates::<R>::get(R::AccountId::from(_key1.0));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getChildkeyTake(bytes32,uint16)")]
    #[precompile::view]
    fn get_childkey_take(handle: &mut impl PrecompileHandle, _key1: H256, _key2: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::ChildkeyTake::<R>::get(R::AccountId::from(_key1.0), subtensor_runtime_common::NetUid::from(_key2));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getPendingChildKeys(uint16,bytes32)")]
    #[precompile::view]
    fn get_pending_child_keys(handle: &mut impl PrecompileHandle, _key1: u16, _key2: H256) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::PendingChildKeys::<R>::get(subtensor_runtime_common::NetUid::from(_key1), R::AccountId::from(_key2.0));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getChildKeys(bytes32,uint16)")]
    #[precompile::view]
    fn get_child_keys(handle: &mut impl PrecompileHandle, _key1: H256, _key2: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::ChildKeys::<R>::get(R::AccountId::from(_key1.0), subtensor_runtime_common::NetUid::from(_key2));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getParentKeys(bytes32,uint16)")]
    #[precompile::view]
    fn get_parent_keys(handle: &mut impl PrecompileHandle, _key1: H256, _key2: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::ParentKeys::<R>::get(R::AccountId::from(_key1.0), subtensor_runtime_common::NetUid::from(_key2));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getAlphaDividendsPerSubnet(uint16,bytes32)")]
    #[precompile::view]
    fn get_alpha_dividends_per_subnet(handle: &mut impl PrecompileHandle, _key1: u16, _key2: H256) -> EvmResult<U256> {
        let val = pallet_subtensor::AlphaDividendsPerSubnet::<R>::get(subtensor_runtime_common::NetUid::from(_key1), R::AccountId::from(_key2.0));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getRootAlphaDividendsPerSubnet(uint16,bytes32)")]
    #[precompile::view]
    fn get_root_alpha_dividends_per_subnet(handle: &mut impl PrecompileHandle, _key1: u16, _key2: H256) -> EvmResult<U256> {
        let val = pallet_subtensor::RootAlphaDividendsPerSubnet::<R>::get(subtensor_runtime_common::NetUid::from(_key1), R::AccountId::from(_key2.0));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getBlockEmission()")]
    #[precompile::view]
    fn get_block_emission(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::BlockEmission::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getLastHotkeyEmissionOnNetuid(bytes32,uint16)")]
    #[precompile::view]
    fn get_last_hotkey_emission_on_netuid(handle: &mut impl PrecompileHandle, _key1: H256, _key2: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::LastHotkeyEmissionOnNetuid::<R>::get(R::AccountId::from(_key1.0), subtensor_runtime_common::NetUid::from(_key2));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getSubnetLimit()")]
    #[precompile::view]
    fn get_subnet_limit(handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        let val = pallet_subtensor::SubnetLimit::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getTotalIssuance()")]
    #[precompile::view]
    fn get_total_issuance(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::TotalIssuance::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getTotalStake()")]
    #[precompile::view]
    fn get_total_stake(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::TotalStake::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getSubnetMovingAlpha()")]
    #[precompile::view]
    fn get_subnet_moving_alpha(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::SubnetMovingAlpha::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val.to_bits()))
    }

    #[precompile::public("getSubnetMovingPrice(uint16)")]
    #[precompile::view]
    fn get_subnet_moving_price(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::SubnetMovingPrice::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val.to_bits()))
    }

    #[precompile::public("getRootProp(uint16)")]
    #[precompile::view]
    fn get_root_prop(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::RootProp::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val.to_bits()))
    }

    #[precompile::public("getSubnetTaoProvided(uint16)")]
    #[precompile::view]
    fn get_subnet_tao_provided(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::SubnetTaoProvided::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getSubnetAlphaInProvided(uint16)")]
    #[precompile::view]
    fn get_subnet_alpha_in_provided(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::SubnetAlphaInProvided::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getStakingHotkeys(bytes32)")]
    #[precompile::view]
    fn get_staking_hotkeys(handle: &mut impl PrecompileHandle, _key1: H256) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::StakingHotkeys::<R>::get(R::AccountId::from(_key1.0));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getOwnedHotkeys(bytes32)")]
    #[precompile::view]
    fn get_owned_hotkeys(handle: &mut impl PrecompileHandle, _key1: H256) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::OwnedHotkeys::<R>::get(R::AccountId::from(_key1.0));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getAutoStakeDestination(bytes32,uint16)")]
    #[precompile::view]
    fn get_auto_stake_destination(handle: &mut impl PrecompileHandle, _key1: H256, _key2: u16) -> EvmResult<H256> {
        let val = pallet_subtensor::AutoStakeDestination::<R>::get(R::AccountId::from(_key1.0), subtensor_runtime_common::NetUid::from(_key2));
        handle.record_db_read_encoded::<R>(&val)?;
        if let Some(val) = val { Ok({ let arr: [u8; 32] = val.into(); H256::from(arr) }) } else { Ok(Default::default()) }
    }

    #[precompile::public("getAutoStakeDestinationColdkeys(bytes32,uint16)")]
    #[precompile::view]
    fn get_auto_stake_destination_coldkeys(handle: &mut impl PrecompileHandle, _key1: H256, _key2: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::AutoStakeDestinationColdkeys::<R>::get(R::AccountId::from(_key1.0), subtensor_runtime_common::NetUid::from(_key2));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getColdkeySwapScheduled(bytes32)")]
    #[precompile::view]
    fn get_coldkey_swap_scheduled(handle: &mut impl PrecompileHandle, _key1: H256) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::ColdkeySwapScheduled::<R>::get(R::AccountId::from(_key1.0));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getTotalHotkeyAlpha(bytes32,uint16)")]
    #[precompile::view]
    fn get_total_hotkey_alpha(handle: &mut impl PrecompileHandle, _key1: H256, _key2: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::TotalHotkeyAlpha::<R>::get(R::AccountId::from(_key1.0), subtensor_runtime_common::NetUid::from(_key2));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getTotalHotkeyAlphaLastEpoch(bytes32,uint16)")]
    #[precompile::view]
    fn get_total_hotkey_alpha_last_epoch(handle: &mut impl PrecompileHandle, _key1: H256, _key2: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::TotalHotkeyAlphaLastEpoch::<R>::get(R::AccountId::from(_key1.0), subtensor_runtime_common::NetUid::from(_key2));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getTotalHotkeyShares(bytes32,uint16)")]
    #[precompile::view]
    fn get_total_hotkey_shares(handle: &mut impl PrecompileHandle, _key1: H256, _key2: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::TotalHotkeyShares::<R>::get(R::AccountId::from(_key1.0), subtensor_runtime_common::NetUid::from(_key2));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getAlphaMapLastKey()")]
    #[precompile::view]
    fn get_alpha_map_last_key(handle: &mut impl PrecompileHandle) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::AlphaMapLastKey::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getTokenSymbol(uint16)")]
    #[precompile::view]
    fn get_token_symbol(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::TokenSymbol::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getSubnetTaoFlow(uint16)")]
    #[precompile::view]
    fn get_subnet_tao_flow(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::SubnetTaoFlow::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getSubnetEmaTaoFlow(uint16)")]
    #[precompile::view]
    fn get_subnet_ema_tao_flow(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::SubnetEmaTaoFlow::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        if let Some(val) = val { Ok(Encode::encode(&val)) } else { Ok(Default::default()) }
    }

    #[precompile::public("getTaoFlowCutoff()")]
    #[precompile::view]
    fn get_tao_flow_cutoff(handle: &mut impl PrecompileHandle) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::TaoFlowCutoff::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getFlowNormExponent()")]
    #[precompile::view]
    fn get_flow_norm_exponent(handle: &mut impl PrecompileHandle) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::FlowNormExponent::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getFlowEmaSmoothingFactor()")]
    #[precompile::view]
    fn get_flow_ema_smoothing_factor(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::FlowEmaSmoothingFactor::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getUsedWork(bytes)")]
    #[precompile::view]
    fn get_used_work(handle: &mut impl PrecompileHandle, _key1: alloc::vec::Vec<u8>) -> EvmResult<U256> {
        let val = pallet_subtensor::UsedWork::<R>::get(_key1);
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getMaxRegistrationsPerBlock(uint16)")]
    #[precompile::view]
    fn get_max_registrations_per_block(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::MaxRegistrationsPerBlock::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getTotalNetworks()")]
    #[precompile::view]
    fn get_total_networks(handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        let val = pallet_subtensor::TotalNetworks::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getNetworkImmunityPeriod()")]
    #[precompile::view]
    fn get_network_immunity_period(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::NetworkImmunityPeriod::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getStartCallDelay()")]
    #[precompile::view]
    fn get_start_call_delay(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::StartCallDelay::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getNetworkMinLockCost()")]
    #[precompile::view]
    fn get_network_min_lock_cost(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::NetworkMinLockCost::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getNetworkLastLockCost()")]
    #[precompile::view]
    fn get_network_last_lock_cost(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::NetworkLastLockCost::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getNetworkLockReductionInterval()")]
    #[precompile::view]
    fn get_network_lock_reduction_interval(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::NetworkLockReductionInterval::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getSubnetOwnerCut()")]
    #[precompile::view]
    fn get_subnet_owner_cut(handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        let val = pallet_subtensor::SubnetOwnerCut::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getNetworkRateLimit()")]
    #[precompile::view]
    fn get_network_rate_limit(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::NetworkRateLimit::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getNominatorMinRequiredStake()")]
    #[precompile::view]
    fn get_nominator_min_required_stake(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::NominatorMinRequiredStake::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getWeightsVersionKeyRateLimit()")]
    #[precompile::view]
    fn get_weights_version_key_rate_limit(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::WeightsVersionKeyRateLimit::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getLastRateLimitedBlock(bytes32)")]
    #[precompile::view]
    fn get_last_rate_limited_block(handle: &mut impl PrecompileHandle, _key1: H256) -> EvmResult<U256> {
        let val = pallet_subtensor::LastRateLimitedBlock::<R>::get(pallet_subtensor::RateLimitKey::LastTxBlock(R::AccountId::from(_key1.0)));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getTransferToggle(uint16)")]
    #[precompile::view]
    fn get_transfer_toggle(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<bool> {
        let val = pallet_subtensor::TransferToggle::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getSubnetLocked(uint16)")]
    #[precompile::view]
    fn get_subnet_locked(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::SubnetLocked::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getLargestLocked(uint16)")]
    #[precompile::view]
    fn get_largest_locked(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::LargestLocked::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getTempo(uint16)")]
    #[precompile::view]
    fn get_tempo(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::Tempo::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getFirstEmissionBlockNumber(uint16)")]
    #[precompile::view]
    fn get_first_emission_block_number(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::FirstEmissionBlockNumber::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        if let Some(val) = val { Ok(U256::from(val)) } else { Ok(Default::default()) }
    }

    #[precompile::public("getIsNetworkMember(bytes32,uint16)")]
    #[precompile::view]
    fn get_is_network_member(handle: &mut impl PrecompileHandle, _key1: H256, _key2: u16) -> EvmResult<bool> {
        let val = pallet_subtensor::IsNetworkMember::<R>::get(R::AccountId::from(_key1.0), subtensor_runtime_common::NetUid::from(_key2));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getNetworkRegisteredAt(uint16)")]
    #[precompile::view]
    fn get_network_registered_at(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::NetworkRegisteredAt::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getPendingServerEmission(uint16)")]
    #[precompile::view]
    fn get_pending_server_emission(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::PendingServerEmission::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getPendingValidatorEmission(uint16)")]
    #[precompile::view]
    fn get_pending_validator_emission(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::PendingValidatorEmission::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getPendingRootAlphaDivs(uint16)")]
    #[precompile::view]
    fn get_pending_root_alpha_divs(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::PendingRootAlphaDivs::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getPendingOwnerCut(uint16)")]
    #[precompile::view]
    fn get_pending_owner_cut(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::PendingOwnerCut::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getBlocksSinceLastStep(uint16)")]
    #[precompile::view]
    fn get_blocks_since_last_step(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::BlocksSinceLastStep::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getLastMechansimStepBlock(uint16)")]
    #[precompile::view]
    fn get_last_mechansim_step_block(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::LastMechansimStepBlock::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getSubnetOwner(uint16)")]
    #[precompile::view]
    fn get_subnet_owner(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<H256> {
        let val = pallet_subtensor::SubnetOwner::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let arr: [u8; 32] = val.into(); H256::from(arr) })
    }

    #[precompile::public("getSubnetOwnerHotkey(uint16)")]
    #[precompile::view]
    fn get_subnet_owner_hotkey(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<H256> {
        let val = pallet_subtensor::SubnetOwnerHotkey::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let arr: [u8; 32] = val.into(); H256::from(arr) })
    }

    #[precompile::public("getRecycleOrBurn(uint16)")]
    #[precompile::view]
    fn get_recycle_or_burn(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u8> {
        let val = pallet_subtensor::RecycleOrBurn::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok((val as u8))
    }

    #[precompile::public("getRegistrationsThisInterval(uint16)")]
    #[precompile::view]
    fn get_registrations_this_interval(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::RegistrationsThisInterval::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getPOWRegistrationsThisInterval(uint16)")]
    #[precompile::view]
    fn get_pow_registrations_this_interval(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::POWRegistrationsThisInterval::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getBurnRegistrationsThisInterval(uint16)")]
    #[precompile::view]
    fn get_burn_registrations_this_interval(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::BurnRegistrationsThisInterval::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getMinAllowedUids(uint16)")]
    #[precompile::view]
    fn get_min_allowed_uids(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::MinAllowedUids::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getMaxAllowedUids(uint16)")]
    #[precompile::view]
    fn get_max_allowed_uids(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::MaxAllowedUids::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getMaxWeightsLimit(uint16)")]
    #[precompile::view]
    fn get_max_weights_limit(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::MaxWeightsLimit::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getMaxAllowedValidators(uint16)")]
    #[precompile::view]
    fn get_max_allowed_validators(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::MaxAllowedValidators::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getAdjustmentInterval(uint16)")]
    #[precompile::view]
    fn get_adjustment_interval(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::AdjustmentInterval::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getBondsPenalty(uint16)")]
    #[precompile::view]
    fn get_bonds_penalty(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::BondsPenalty::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getValidatorPruneLen(uint16)")]
    #[precompile::view]
    fn get_validator_prune_len(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::ValidatorPruneLen::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getScalingLawPower(uint16)")]
    #[precompile::view]
    fn get_scaling_law_power(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::ScalingLawPower::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getTargetRegistrationsPerInterval(uint16)")]
    #[precompile::view]
    fn get_target_registrations_per_interval(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::TargetRegistrationsPerInterval::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getLastAdjustmentBlock(uint16)")]
    #[precompile::view]
    fn get_last_adjustment_block(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::LastAdjustmentBlock::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getRegistrationsThisBlock(uint16)")]
    #[precompile::view]
    fn get_registrations_this_block(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::RegistrationsThisBlock::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getRAORecycledForRegistration(uint16)")]
    #[precompile::view]
    fn get_rao_recycled_for_registration(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::RAORecycledForRegistration::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getTxRateLimit()")]
    #[precompile::view]
    fn get_tx_rate_limit(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::TxRateLimit::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getTxDelegateTakeRateLimit()")]
    #[precompile::view]
    fn get_tx_delegate_take_rate_limit(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::TxDelegateTakeRateLimit::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getTxChildkeyTakeRateLimit()")]
    #[precompile::view]
    fn get_tx_childkey_take_rate_limit(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::TxChildkeyTakeRateLimit::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getSubtokenEnabled(uint16)")]
    #[precompile::view]
    fn get_subtoken_enabled(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<bool> {
        let val = pallet_subtensor::SubtokenEnabled::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getImmuneOwnerUidsLimit(uint16)")]
    #[precompile::view]
    fn get_immune_owner_uids_limit(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::ImmuneOwnerUidsLimit::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getStakeWeight(uint16)")]
    #[precompile::view]
    fn get_stake_weight(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::StakeWeight::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getKeys(uint16,uint16)")]
    #[precompile::view]
    fn get_keys(handle: &mut impl PrecompileHandle, _key1: u16, _key2: u16) -> EvmResult<H256> {
        let val = pallet_subtensor::Keys::<R>::get(subtensor_runtime_common::NetUid::from(_key1), _key2);
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let arr: [u8; 32] = val.into(); H256::from(arr) })
    }

    #[precompile::public("getLoadedEmission(uint16)")]
    #[precompile::view]
    fn get_loaded_emission(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::LoadedEmission::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        if let Some(val) = val { Ok(Encode::encode(&val)) } else { Ok(Default::default()) }
    }

    #[precompile::public("getActive(uint16)")]
    #[precompile::view]
    fn get_active(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::Active::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getRank(uint16)")]
    #[precompile::view]
    fn get_rank(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::Rank::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getTrust(uint16)")]
    #[precompile::view]
    fn get_trust(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::Trust::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getConsensus(uint16)")]
    #[precompile::view]
    fn get_consensus(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::Consensus::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getIncentive(uint16)")]
    #[precompile::view]
    fn get_incentive(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::Incentive::<R>::get(subtensor_runtime_common::NetUidStorageIndex::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getDividends(uint16)")]
    #[precompile::view]
    fn get_dividends(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::Dividends::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getEmission(uint16)")]
    #[precompile::view]
    fn get_emission(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::Emission::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getLastUpdate(uint16)")]
    #[precompile::view]
    fn get_last_update(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::LastUpdate::<R>::get(subtensor_runtime_common::NetUidStorageIndex::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getValidatorTrust(uint16)")]
    #[precompile::view]
    fn get_validator_trust(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::ValidatorTrust::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getPruningScores(uint16)")]
    #[precompile::view]
    fn get_pruning_scores(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::PruningScores::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getValidatorPermit(uint16)")]
    #[precompile::view]
    fn get_validator_permit(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::ValidatorPermit::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getWeights(uint16,uint16)")]
    #[precompile::view]
    fn get_weights(handle: &mut impl PrecompileHandle, _key1: u16, _key2: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::Weights::<R>::get(subtensor_runtime_common::NetUidStorageIndex::from(_key1), _key2);
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getBonds(uint16,uint16)")]
    #[precompile::view]
    fn get_bonds(handle: &mut impl PrecompileHandle, _key1: u16, _key2: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::Bonds::<R>::get(subtensor_runtime_common::NetUidStorageIndex::from(_key1), _key2);
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getBlockAtRegistration(uint16,uint16)")]
    #[precompile::view]
    fn get_block_at_registration(handle: &mut impl PrecompileHandle, _key1: u16, _key2: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::BlockAtRegistration::<R>::get(subtensor_runtime_common::NetUid::from(_key1), _key2);
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getAxons(uint16,bytes32)")]
    #[precompile::view]
    fn get_axons(handle: &mut impl PrecompileHandle, _key1: u16, _key2: H256) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::Axons::<R>::get(subtensor_runtime_common::NetUid::from(_key1), R::AccountId::from(_key2.0));
        handle.record_db_read_encoded::<R>(&val)?;
        if let Some(val) = val { Ok(Encode::encode(&val)) } else { Ok(Default::default()) }
    }

    #[precompile::public("getNeuronCertificates(uint16,bytes32)")]
    #[precompile::view]
    fn get_neuron_certificates(handle: &mut impl PrecompileHandle, _key1: u16, _key2: H256) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::NeuronCertificates::<R>::get(subtensor_runtime_common::NetUid::from(_key1), R::AccountId::from(_key2.0));
        handle.record_db_read_encoded::<R>(&val)?;
        if let Some(val) = val { Ok(Encode::encode(&val)) } else { Ok(Default::default()) }
    }

    #[precompile::public("getPrometheus(uint16,bytes32)")]
    #[precompile::view]
    fn get_prometheus(handle: &mut impl PrecompileHandle, _key1: u16, _key2: H256) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::Prometheus::<R>::get(subtensor_runtime_common::NetUid::from(_key1), R::AccountId::from(_key2.0));
        handle.record_db_read_encoded::<R>(&val)?;
        if let Some(val) = val { Ok(Encode::encode(&val)) } else { Ok(Default::default()) }
    }

    #[precompile::public("getIdentitiesV2(bytes32)")]
    #[precompile::view]
    fn get_identities_v2(handle: &mut impl PrecompileHandle, _key1: H256) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::IdentitiesV2::<R>::get(R::AccountId::from(_key1.0));
        handle.record_db_read_encoded::<R>(&val)?;
        if let Some(val) = val { Ok(Encode::encode(&val)) } else { Ok(Default::default()) }
    }

    #[precompile::public("getSubnetIdentitiesV3(uint16)")]
    #[precompile::view]
    fn get_subnet_identities_v3(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::SubnetIdentitiesV3::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        if let Some(val) = val { Ok(Encode::encode(&val)) } else { Ok(Default::default()) }
    }

    #[precompile::public("getTransactionKeyLastBlock(bytes32,uint16,uint16)")]
    #[precompile::view]
    fn get_transaction_key_last_block(handle: &mut impl PrecompileHandle, _key1: H256, _key2: u16, _key3: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::TransactionKeyLastBlock::<R>::get((R::AccountId::from(_key1.0), subtensor_runtime_common::NetUid::from(_key2), _key3));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getLastTxBlock(bytes32)")]
    #[precompile::view]
    fn get_last_tx_block(handle: &mut impl PrecompileHandle, _key1: H256) -> EvmResult<U256> {
        let val = pallet_subtensor::LastTxBlock::<R>::get(R::AccountId::from(_key1.0));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getLastTxBlockChildKeyTake(bytes32)")]
    #[precompile::view]
    fn get_last_tx_block_child_key_take(handle: &mut impl PrecompileHandle, _key1: H256) -> EvmResult<U256> {
        let val = pallet_subtensor::LastTxBlockChildKeyTake::<R>::get(R::AccountId::from(_key1.0));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getLastTxBlockDelegateTake(bytes32)")]
    #[precompile::view]
    fn get_last_tx_block_delegate_take(handle: &mut impl PrecompileHandle, _key1: H256) -> EvmResult<U256> {
        let val = pallet_subtensor::LastTxBlockDelegateTake::<R>::get(R::AccountId::from(_key1.0));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getStakeThreshold()")]
    #[precompile::view]
    fn get_stake_threshold(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::StakeThreshold::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getWeightCommits(uint16,bytes32)")]
    #[precompile::view]
    fn get_weight_commits(handle: &mut impl PrecompileHandle, _key1: u16, _key2: H256) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::WeightCommits::<R>::get(subtensor_runtime_common::NetUidStorageIndex::from(_key1), R::AccountId::from(_key2.0));
        handle.record_db_read_encoded::<R>(&val)?;
        if let Some(val) = val { Ok(Encode::encode(&val)) } else { Ok(Default::default()) }
    }

    #[precompile::public("getTimelockedWeightCommits(uint16,uint256)")]
    #[precompile::view]
    fn get_timelocked_weight_commits(handle: &mut impl PrecompileHandle, _key1: u16, _key2: U256) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::TimelockedWeightCommits::<R>::get(subtensor_runtime_common::NetUidStorageIndex::from(_key1), _key2.try_into().unwrap_or(0u64));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getCRV3WeightCommits(uint16,uint256)")]
    #[precompile::view]
    fn get_crv3_weight_commits(handle: &mut impl PrecompileHandle, _key1: u16, _key2: U256) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::CRV3WeightCommits::<R>::get(subtensor_runtime_common::NetUidStorageIndex::from(_key1), _key2.try_into().unwrap_or(0u64));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getCRV3WeightCommitsV2(uint16,uint256)")]
    #[precompile::view]
    fn get_crv3_weight_commits_v2(handle: &mut impl PrecompileHandle, _key1: u16, _key2: U256) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::CRV3WeightCommitsV2::<R>::get(subtensor_runtime_common::NetUidStorageIndex::from(_key1), _key2.try_into().unwrap_or(0u64));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getLastColdkeyHotkeyStakeBlock(bytes32,bytes32)")]
    #[precompile::view]
    fn get_last_coldkey_hotkey_stake_block(handle: &mut impl PrecompileHandle, _key1: H256, _key2: H256) -> EvmResult<U256> {
        let val = pallet_subtensor::LastColdkeyHotkeyStakeBlock::<R>::get(R::AccountId::from(_key1.0), R::AccountId::from(_key2.0));
        handle.record_db_read_encoded::<R>(&val)?;
        if let Some(val) = val { Ok(U256::from(val)) } else { Ok(Default::default()) }
    }

    #[precompile::public("getStakingOperationRateLimiter(bytes32,bytes32,uint16)")]
    #[precompile::view]
    fn get_staking_operation_rate_limiter(handle: &mut impl PrecompileHandle, _key1: H256, _key2: H256, _key3: u16) -> EvmResult<bool> {
        let val = pallet_subtensor::StakingOperationRateLimiter::<R>::get((R::AccountId::from(_key1.0), R::AccountId::from(_key2.0), subtensor_runtime_common::NetUid::from(_key3)));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getRootClaimableThreshold(uint16)")]
    #[precompile::view]
    fn get_root_claimable_threshold(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<U256> {
        let val = pallet_subtensor::RootClaimableThreshold::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val.to_bits()))
    }

    #[precompile::public("getRootClaimable(bytes32)")]
    #[precompile::view]
    fn get_root_claimable(handle: &mut impl PrecompileHandle, _key1: H256) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::RootClaimable::<R>::get(R::AccountId::from(_key1.0));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getRootClaimed(uint16,bytes32,bytes32)")]
    #[precompile::view]
    fn get_root_claimed(handle: &mut impl PrecompileHandle, _key1: u16, _key2: H256, _key3: H256) -> EvmResult<U256> {
        let val = pallet_subtensor::RootClaimed::<R>::get((subtensor_runtime_common::NetUid::from(_key1), R::AccountId::from(_key2.0), R::AccountId::from(_key3.0)));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getRootClaimType(bytes32)")]
    #[precompile::view]
    fn get_root_claim_type(handle: &mut impl PrecompileHandle, _key1: H256) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::RootClaimType::<R>::get(R::AccountId::from(_key1.0));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(Encode::encode(&val))
    }

    #[precompile::public("getStakingColdkeysByIndex(uint256)")]
    #[precompile::view]
    fn get_staking_coldkeys_by_index(handle: &mut impl PrecompileHandle, _key1: U256) -> EvmResult<H256> {
        let val = pallet_subtensor::StakingColdkeysByIndex::<R>::get(_key1.try_into().unwrap_or(0u64));
        handle.record_db_read_encoded::<R>(&val)?;
        if let Some(val) = val { Ok({ let arr: [u8; 32] = val.into(); H256::from(arr) }) } else { Ok(Default::default()) }
    }

    #[precompile::public("getStakingColdkeys(bytes32)")]
    #[precompile::view]
    fn get_staking_coldkeys(handle: &mut impl PrecompileHandle, _key1: H256) -> EvmResult<U256> {
        let val = pallet_subtensor::StakingColdkeys::<R>::get(R::AccountId::from(_key1.0));
        handle.record_db_read_encoded::<R>(&val)?;
        if let Some(val) = val { Ok(U256::from(val)) } else { Ok(Default::default()) }
    }

    #[precompile::public("getNumStakingColdkeys()")]
    #[precompile::view]
    fn get_num_staking_coldkeys(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::NumStakingColdkeys::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getNumRootClaim()")]
    #[precompile::view]
    fn get_num_root_claim(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::NumRootClaim::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getAssociatedEvmAddress(uint16,uint16)")]
    #[precompile::view]
    fn get_associated_evm_address(handle: &mut impl PrecompileHandle, _key1: u16, _key2: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::AssociatedEvmAddress::<R>::get(subtensor_runtime_common::NetUid::from(_key1), _key2);
        handle.record_db_read_encoded::<R>(&val)?;
        if let Some(val) = val { Ok(Encode::encode(&val)) } else { Ok(Default::default()) }
    }

    #[precompile::public("getNextSubnetLeaseId()")]
    #[precompile::view]
    fn get_next_subnet_lease_id(handle: &mut impl PrecompileHandle) -> EvmResult<u32> {
        let val = pallet_subtensor::NextSubnetLeaseId::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getAccumulatedLeaseDividends(uint32)")]
    #[precompile::view]
    fn get_accumulated_lease_dividends(handle: &mut impl PrecompileHandle, _key1: u32) -> EvmResult<U256> {
        let val = pallet_subtensor::AccumulatedLeaseDividends::<R>::get(_key1);
        handle.record_db_read_encoded::<R>(&val)?;
        Ok({ let v: u64 = val.into(); U256::from(v) })
    }

    #[precompile::public("getCommitRevealWeightsVersion()")]
    #[precompile::view]
    fn get_commit_reveal_weights_version(handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        let val = pallet_subtensor::CommitRevealWeightsVersion::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getNetworkRegistrationStartBlock()")]
    #[precompile::view]
    fn get_network_registration_start_block(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::NetworkRegistrationStartBlock::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

    #[precompile::public("getMinNonImmuneUids(uint16)")]
    #[precompile::view]
    fn get_min_non_immune_uids(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::MinNonImmuneUids::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getMechanismCountCurrent(uint16)")]
    #[precompile::view]
    fn get_mechanism_count_current(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<u16> {
        let val = pallet_subtensor::MechanismCountCurrent::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(u16::from(val))
    }

    #[precompile::public("getMechanismEmissionSplit(uint16)")]
    #[precompile::view]
    fn get_mechanism_emission_split(handle: &mut impl PrecompileHandle, _key1: u16) -> EvmResult<alloc::vec::Vec<u8>> {
        let val = pallet_subtensor::MechanismEmissionSplit::<R>::get(subtensor_runtime_common::NetUid::from(_key1));
        handle.record_db_read_encoded::<R>(&val)?;
        if let Some(val) = val { Ok(Encode::encode(&val)) } else { Ok(Default::default()) }
    }

    #[precompile::public("getHasMigrationRun(bytes)")]
    #[precompile::view]
    fn get_has_migration_run(handle: &mut impl PrecompileHandle, _key1: alloc::vec::Vec<u8>) -> EvmResult<bool> {
        let val = pallet_subtensor::HasMigrationRun::<R>::get(_key1);
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(val)
    }

    #[precompile::public("getPendingChildKeyCooldown()")]
    #[precompile::view]
    fn get_pending_child_key_cooldown(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor::PendingChildKeyCooldown::<R>::get();
        handle.record_db_read_encoded::<R>(&val)?;
        Ok(U256::from(val))
    }

}