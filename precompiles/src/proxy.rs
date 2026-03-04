use core::marker::PhantomData;

use crate::{PrecompileExt, PrecompileHandleExt, PrecompileHandleExtStorage};

use alloc::format;
use fp_evm::{ExitError, PrecompileFailure};
use frame_support::dispatch::{DispatchInfo, GetDispatchInfo, PostDispatchInfo};
use frame_support::traits::IsSubType;
use frame_system::RawOrigin;
use pallet_evm::{AddressMapping, PrecompileHandle};
use pallet_subtensor_proxy as pallet_proxy;
use precompile_utils::EvmResult;
use sp_core::{H256, U256};
use sp_runtime::{
    codec::DecodeLimit,
    traits::{AsSystemOriginSigner, Dispatchable, StaticLookup},
};
use sp_std::boxed::Box;
use sp_std::convert::{TryFrom, TryInto};
use sp_std::vec;
use sp_std::vec::Vec;
use subtensor_runtime_common::ProxyType;
pub struct ProxyPrecompile<R>(PhantomData<R>);
const MAX_DECODE_DEPTH: u32 = 8;

impl<R> PrecompileExt<R::AccountId> for ProxyPrecompile<R>
where
    R: frame_system::Config
        + pallet_balances::Config
        + pallet_evm::Config
        + pallet_subtensor::Config
        + pallet_proxy::Config<ProxyType = ProxyType>
        + Send
        + Sync
        + scale_info::TypeInfo,
    R::AccountId: From<[u8; 32]> + Into<[u8; 32]>,
    <R as frame_system::Config>::RuntimeOrigin: AsSystemOriginSigner<R::AccountId> + Clone,
    <R as pallet_evm::Config>::AddressMapping: AddressMapping<R::AccountId>,
    <R as frame_system::Config>::RuntimeCall: From<pallet_subtensor::Call<R>>
        + From<pallet_proxy::Call<R>>
        + GetDispatchInfo
        + Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>
        + IsSubType<pallet_balances::Call<R>>
        + IsSubType<pallet_subtensor::Call<R>>,
    <R as pallet_evm::Config>::AddressMapping: AddressMapping<R::AccountId>,
    <<R as frame_system::Config>::Lookup as StaticLookup>::Source: From<R::AccountId>,
{
    const INDEX: u64 = 2059;
}

#[precompile_utils::precompile]
impl<R> ProxyPrecompile<R>
where
    R: frame_system::Config
        + pallet_balances::Config
        + pallet_evm::Config
        + pallet_subtensor::Config
        + pallet_proxy::Config<ProxyType = ProxyType>
        + Send
        + Sync
        + scale_info::TypeInfo,
    R::AccountId: From<[u8; 32]> + Into<[u8; 32]>,
    <R as frame_system::Config>::RuntimeOrigin: AsSystemOriginSigner<R::AccountId> + Clone,
    <R as pallet_evm::Config>::AddressMapping: AddressMapping<R::AccountId>,
    <R as frame_system::Config>::RuntimeCall: From<pallet_subtensor::Call<R>>
        + From<pallet_proxy::Call<R>>
        + GetDispatchInfo
        + Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>
        + IsSubType<pallet_balances::Call<R>>
        + IsSubType<pallet_subtensor::Call<R>>,
    <<R as frame_system::Config>::Lookup as StaticLookup>::Source: From<R::AccountId>,
{
    #[precompile::public("createPureProxy(uint8,uint32,uint16)")]
    pub fn create_pure_proxy(
        handle: &mut impl PrecompileHandle,
        proxy_type_: u8,
        delay: u32,
        index: u16,
    ) -> EvmResult<H256> {
        let account_id = handle.caller_account_id::<R>();
        let proxy_type =
            ProxyType::try_from(proxy_type_).map_err(|_| PrecompileFailure::Error {
                exit_status: ExitError::Other("Invalid proxy type".into()),
            })?;

        let call = pallet_proxy::Call::<R>::create_pure {
            proxy_type,
            delay: delay.into(),
            index,
        };

        handle.try_dispatch_runtime_call::<R, _>(call, RawOrigin::Signed(account_id.clone()))?;

        // Success!
        // Try to get proxy address
        let proxy_address: [u8; 32] =
            pallet_proxy::pallet::Pallet::<R>::pure_account(&account_id, &proxy_type, index, None)
                .map_err(|_| PrecompileFailure::Error {
                    exit_status: ExitError::Other("Proxy not found".into()),
                })?
                .into();

        // Check if in the proxies map
        let proxy_entry = pallet_proxy::pallet::Pallet::<R>::proxies(proxy_address.into());
        if proxy_entry
            .0
            .iter()
            .any(|p| account_id == p.delegate && proxy_type == p.proxy_type)
        {
            return Ok(proxy_address.into());
        }

        Err(PrecompileFailure::Error {
            exit_status: ExitError::Other("Proxy not found".into()),
        })
    }

    #[precompile::public("killPureProxy(bytes32,uint8,uint16,uint32,uint32)")]
    pub fn kill_pure_proxy(
        handle: &mut impl PrecompileHandle,
        spawner: H256,
        proxy_type: u8,
        index: u16,
        height: u32,
        ext_index: u32,
    ) -> EvmResult<()> {
        let account_id = handle.caller_account_id::<R>();
        let proxy_type = ProxyType::try_from(proxy_type).map_err(|_| PrecompileFailure::Error {
            exit_status: ExitError::Other("Invalid proxy type".into()),
        })?;

        let call = pallet_proxy::Call::<R>::kill_pure {
            spawner: <<R as frame_system::Config>::Lookup as StaticLookup>::Source::from(
                spawner.0.into(),
            ),
            proxy_type,
            index,
            height: height.into(),
            ext_index: ext_index.into(),
        };

        handle.try_dispatch_runtime_call::<R, _>(call, RawOrigin::Signed(account_id))
    }

    #[precompile::public("proxyCall(bytes32,uint8[],uint8[])")]
    pub fn proxy_call(
        handle: &mut impl PrecompileHandle,
        real: H256,
        force_proxy_type: Vec<u8>,
        call: Vec<u8>,
    ) -> EvmResult<()> {
        let account_id = handle.caller_account_id::<R>();

        let call = <R as pallet_proxy::Config>::RuntimeCall::decode_with_depth_limit(
            MAX_DECODE_DEPTH,
            &mut &call[..],
        )
        .map_err(|_| PrecompileFailure::Error {
            exit_status: ExitError::Other("The raw call data not correctly encoded".into()),
        })?;

        let mut proxy_type: Option<ProxyType> = None;
        if let Some(p) = force_proxy_type.first() {
            let proxy_type_ = ProxyType::try_from(*p).map_err(|_| PrecompileFailure::Error {
                exit_status: ExitError::Other("Invalid proxy type".into()),
            })?;
            proxy_type = Some(proxy_type_);
        };

        let call = pallet_proxy::Call::<R>::proxy {
            real: <<R as frame_system::Config>::Lookup as StaticLookup>::Source::from(
                real.0.into(),
            ),
            force_proxy_type: proxy_type,
            call: Box::new(call),
        };

        handle.try_dispatch_runtime_call::<R, _>(call, RawOrigin::Signed(account_id))?;

        let real_account_id = R::AccountId::from(real.0.into());

        let last_call_result = pallet_proxy::LastCallResult::<R>::get(real_account_id);
        handle.record_db_read_encoded::<R>(&last_call_result)?;
        match last_call_result {
            Some(last_call_result) => match last_call_result {
                Ok(()) => Ok(()),
                Err(e) => Err(PrecompileFailure::Error {
                    exit_status: ExitError::Other(format!("{e:?}").into()),
                }),
            },
            None => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Proxy execution failed".into()),
            }),
        }
    }

    #[precompile::public("addProxy(bytes32,uint8,uint32)")]
    pub fn add_proxy(
        handle: &mut impl PrecompileHandle,
        delegate: H256,
        proxy_type: u8,
        delay: u32,
    ) -> EvmResult<()> {
        let account_id = handle.caller_account_id::<R>();
        let proxy_type = ProxyType::try_from(proxy_type).map_err(|_| PrecompileFailure::Error {
            exit_status: ExitError::Other("Invalid proxy type".into()),
        })?;

        let call = pallet_proxy::Call::<R>::add_proxy {
            delegate: <<R as frame_system::Config>::Lookup as StaticLookup>::Source::from(
                delegate.0.into(),
            ),
            proxy_type,
            delay: delay.into(),
        };

        handle.try_dispatch_runtime_call::<R, _>(call, RawOrigin::Signed(account_id))
    }

    #[precompile::public("removeProxy(bytes32,uint8,uint32)")]
    pub fn remove_proxy(
        handle: &mut impl PrecompileHandle,
        delegate: H256,
        proxy_type: u8,
        delay: u32,
    ) -> EvmResult<()> {
        let account_id = handle.caller_account_id::<R>();
        let proxy_type = ProxyType::try_from(proxy_type).map_err(|_| PrecompileFailure::Error {
            exit_status: ExitError::Other("Invalid proxy type".into()),
        })?;

        let call = pallet_proxy::Call::<R>::remove_proxy {
            delegate: <<R as frame_system::Config>::Lookup as StaticLookup>::Source::from(
                delegate.0.into(),
            ),
            proxy_type,
            delay: delay.into(),
        };

        handle.try_dispatch_runtime_call::<R, _>(call, RawOrigin::Signed(account_id))
    }

    #[precompile::public("removeProxies()")]
    pub fn remove_proxies(handle: &mut impl PrecompileHandle) -> EvmResult<()> {
        let account_id = handle.caller_account_id::<R>();

        let call = pallet_proxy::Call::<R>::remove_proxies {};

        handle.try_dispatch_runtime_call::<R, _>(call, RawOrigin::Signed(account_id))
    }

    #[precompile::public("pokeDeposit()")]
    pub fn poke_deposit(handle: &mut impl PrecompileHandle) -> EvmResult<()> {
        let account_id = handle.caller_account_id::<R>();

        let call = pallet_proxy::Call::<R>::poke_deposit {};

        handle.try_dispatch_runtime_call::<R, _>(call, RawOrigin::Signed(account_id))
    }

    #[precompile::public("getProxies(bytes32)")]
    #[precompile::view]
    pub fn get_proxies(
        handle: &mut impl PrecompileHandle,
        account_id: H256,
    ) -> EvmResult<Vec<(H256, U256, U256)>> {
        let account_id = R::AccountId::from(account_id.0.into());

        let proxies_tuple = pallet_proxy::pallet::Pallet::<R>::proxies(account_id);
        handle.record_db_read_encoded::<R>(&proxies_tuple)?;
        let mut result: Vec<(H256, U256, U256)> = vec![];
        for proxy in proxies_tuple.0 {
            let delegate: [u8; 32] = proxy.delegate.into();
            let proxy_type: u8 = proxy.proxy_type.into();
            let delay: u32 = proxy
                .delay
                .try_into()
                .map_err(|_| PrecompileFailure::Error {
                    exit_status: ExitError::Other("Invalid delay".into()),
                })?;

            result.push((delegate.into(), proxy_type.into(), delay.into()));
        }

        Ok(result)
    }

    /// Returns the result of the last proxy call for an account.
    /// Returns (exists, success) — exists is true if a result is stored, success is true if it succeeded.
    #[precompile::public("getLastCallResult(bytes32)")]
    #[precompile::view]
    pub fn get_last_call_result(
        handle: &mut impl PrecompileHandle,
        account_id: H256,
    ) -> EvmResult<(bool, bool)> {
        let account_id = R::AccountId::from(account_id.0.into());
        let __matched_val = pallet_proxy::LastCallResult::<R>::get(account_id);
        handle.record_db_read_encoded::<R>(&__matched_val)?;
        match __matched_val {
            Some(result) => Ok((true, result.is_ok())),
            None => Ok((false, false)),
        }
    }

    /// Returns the current pallet version from storage.
    #[precompile::public("getPalletVersion()")]
    #[precompile::view]
    pub fn get_pallet_version(_handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        Ok(
            <pallet_proxy::pallet::Pallet<R> as frame_support::traits::PalletInfoAccess>::crate_version()
                .major as u16
        )
    }

    /// Returns the announcements made by the proxy.
    #[precompile::public("getAnnouncements(bytes32)")]
    #[precompile::view]
    pub fn get_announcements(
        handle: &mut impl PrecompileHandle,
        account_id: H256,
    ) -> EvmResult<Vec<(H256, H256, u32)>> {
        let account_id = R::AccountId::from(account_id.0.into());
        let announcements_tuple = pallet_proxy::Announcements::<R>::get(&account_id);
        handle.record_db_read_encoded::<R>(&announcements_tuple)?;
        
        let mut result = vec![];
        for announcement in announcements_tuple.0 {
            use frame_support::pallet_prelude::Encode;
            use sp_runtime::codec::Decode;
            let encoded = announcement.encode();
            
            let maybe_decoded = <(
                R::AccountId,
                <<R as pallet_proxy::Config>::CallHasher as sp_runtime::traits::Hash>::Output,
                <<<R as frame_system::Config>::Block as sp_runtime::traits::Block>::Header as sp_runtime::traits::Header>::Number,
            )>::decode(&mut &encoded[..]);
            
            if let Ok((real, call_hash, height)) = maybe_decoded {
                let mut real_bytes = [0u8; 32];
                let real_encoded = real.encode();
                let len = core::cmp::min(real_encoded.len(), 32);
                real_bytes[0..len].copy_from_slice(&real_encoded[0..len]);
                
                let mut hash_bytes = [0u8; 32];
                let hash_encoded = call_hash.encode();
                let len = core::cmp::min(hash_encoded.len(), 32);
                hash_bytes[0..len].copy_from_slice(&hash_encoded[0..len]);
                
                use sp_runtime::traits::UniqueSaturatedInto;
                let height_u32: u32 = height.unique_saturated_into();
                
                result.push((
                    H256::from(real_bytes),
                    H256::from(hash_bytes),
                    height_u32,
                ));
            }
        }

        Ok(result)
    }
}