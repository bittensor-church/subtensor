use core::marker::PhantomData;

use frame_support::traits::GetStorageVersion;
use pallet_evm::PrecompileHandle;
use precompile_utils::EvmResult;
use sp_core::{H256, U256};
use subtensor_runtime_common::NetUid;

use crate::{PrecompileExt, PrecompileHandleExt};

pub struct SwapPrecompile<R>(PhantomData<R>);

impl<R> PrecompileExt<R::AccountId> for SwapPrecompile<R>
where
    R: frame_system::Config
        + pallet_subtensor_swap::Config
        + pallet_evm::Config,
    R::AccountId: From<[u8; 32]>,
{
    const INDEX: u64 = 2070; // NOTE: CHECK IF THIS INDEX IS AVAILABLE
}

#[precompile_utils::precompile]
impl<R> SwapPrecompile<R>
where
    R: frame_system::Config
        + pallet_subtensor_swap::Config
        + pallet_evm::Config,
    R::AccountId: From<[u8; 32]>,
{
    #[precompile::public("getFeeRate(uint16)")]
    #[precompile::view]
    fn get_fee_rate(_handle: &mut impl PrecompileHandle, netuid: u16) -> EvmResult<u16> {
        let val = pallet_subtensor_swap::FeeRate::<R>::get(NetUid::from(netuid));
        Ok(val)
    }

    #[precompile::public("getFeeGlobalTao(uint16)")]
    #[precompile::view]
    fn get_fee_global_tao(_handle: &mut impl PrecompileHandle, netuid: u16) -> EvmResult<U256> {
        let val = pallet_subtensor_swap::FeeGlobalTao::<R>::get(NetUid::from(netuid));
        Ok(U256::from(val.to_bits()))
    }

    #[precompile::public("getFeeGlobalAlpha(uint16)")]
    #[precompile::view]
    fn get_fee_global_alpha(_handle: &mut impl PrecompileHandle, netuid: u16) -> EvmResult<U256> {
        let val = pallet_subtensor_swap::FeeGlobalAlpha::<R>::get(NetUid::from(netuid));
        Ok(U256::from(val.to_bits()))
    }

    #[precompile::public("getTicks(uint16,int32)")]
    #[precompile::view]
    fn get_ticks(_handle: &mut impl PrecompileHandle, netuid: u16, tick_index: U256) -> EvmResult<(U256, U256, U256, U256)> {
        let tick_val = tick_index.0[0] as u32 as i32;
        use core::convert::TryFrom;
        if let Ok(tick_idx) = pallet_subtensor_swap::tick::TickIndex::try_from(tick_val) {
            let val = pallet_subtensor_swap::Ticks::<R>::get(NetUid::from(netuid), tick_idx).unwrap_or_default();
            return Ok((
                U256::from(val.liquidity_net as u128),
                U256::from(val.liquidity_gross),
                U256::from(val.fees_out_tao.to_bits() as u128),
                U256::from(val.fees_out_alpha.to_bits() as u128),
            ));
        }
        Ok((U256::zero(), U256::zero(), U256::zero(), U256::zero()))
    }

    #[precompile::public("getSwapV3Initialized(uint16)")]
    #[precompile::view]
    fn get_swap_v3_initialized(_handle: &mut impl PrecompileHandle, netuid: u16) -> EvmResult<bool> {
        let val = pallet_subtensor_swap::SwapV3Initialized::<R>::get(NetUid::from(netuid));
        Ok(val)
    }

    #[precompile::public("getAlphaSqrtPrice(uint16)")]
    #[precompile::view]
    fn get_alpha_sqrt_price(_handle: &mut impl PrecompileHandle, netuid: u16) -> EvmResult<U256> {
        let val = pallet_subtensor_swap::AlphaSqrtPrice::<R>::get(NetUid::from(netuid));
        Ok(U256::from(val.to_bits()))
    }

    #[precompile::public("getCurrentTick(uint16)")]
    #[precompile::view]
    fn get_current_tick(_handle: &mut impl PrecompileHandle, netuid: u16) -> EvmResult<U256> {
        let val = pallet_subtensor_swap::CurrentTick::<R>::get(NetUid::from(netuid));
        Ok(U256::from(val.get() as u32 as u128))
    }

    #[precompile::public("getCurrentLiquidity(uint16)")]
    #[precompile::view]
    fn get_current_liquidity(_handle: &mut impl PrecompileHandle, netuid: u16) -> EvmResult<u64> {
        let val = pallet_subtensor_swap::CurrentLiquidity::<R>::get(NetUid::from(netuid));
        Ok(val)
    }

    #[precompile::public("getEnabledUserLiquidity(uint16)")]
    #[precompile::view]
    fn get_enabled_user_liquidity(_handle: &mut impl PrecompileHandle, netuid: u16) -> EvmResult<bool> {
        let val = pallet_subtensor_swap::EnabledUserLiquidity::<R>::get(NetUid::from(netuid));
        Ok(val)
    }

    #[precompile::public("getPositions(uint16,bytes32,uint256)")]
    #[precompile::view]
    fn get_positions(_handle: &mut impl PrecompileHandle, netuid: u16, account: H256, position_id: U256) -> EvmResult<(U256, u16, U256, U256, u64, U256, U256)> {
        let acc = R::AccountId::from(account.0);
        let pid = pallet_subtensor_swap::position::PositionId::from(position_id.as_u128());
        if let Some(val) = pallet_subtensor_swap::Positions::<R>::get(&(NetUid::from(netuid), acc, pid)) {
            let id: u128 = val.id.into();
            return Ok((
                U256::from(id),
                val.netuid.into(),
                U256::from(val.tick_low.get() as u32 as u128),
                U256::from(val.tick_high.get() as u32 as u128),
                val.liquidity,
                U256::from(val.fees_tao.to_bits() as u128),
                U256::from(val.fees_alpha.to_bits() as u128),
            ));
        }
        Ok((U256::zero(), 0, U256::zero(), U256::zero(), 0, U256::zero(), U256::zero()))
    }

    #[precompile::public("getLastPositionId()")]
    #[precompile::view]
    fn get_last_position_id(_handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
        let val = pallet_subtensor_swap::LastPositionId::<R>::get();
        Ok(U256::from(val))
    }

    #[precompile::public("getTickIndexBitmapWords(uint16,uint8,uint32)")]
    #[precompile::view]
    fn get_tick_index_bitmap_words(_handle: &mut impl PrecompileHandle, netuid: u16, layer_idx: u8, word_index: u32) -> EvmResult<U256> {
        let layer = match layer_idx {
            0 => pallet_subtensor_swap::tick::LayerLevel::Top,
            1 => pallet_subtensor_swap::tick::LayerLevel::Middle,
            _ => pallet_subtensor_swap::tick::LayerLevel::Bottom,
        };
        let val = pallet_subtensor_swap::TickIndexBitmapWords::<R>::get(&(NetUid::from(netuid), layer, word_index));
        Ok(U256::from(val))
    }

    #[precompile::public("getScrapReservoirTao(uint16)")]
    #[precompile::view]
    fn get_scrap_reservoir_tao(_handle: &mut impl PrecompileHandle, netuid: u16) -> EvmResult<u64> {
        let val = pallet_subtensor_swap::ScrapReservoirTao::<R>::get(NetUid::from(netuid));
        Ok(val.into())
    }

    #[precompile::public("getScrapReservoirAlpha(uint16)")]
    #[precompile::view]
    fn get_scrap_reservoir_alpha(_handle: &mut impl PrecompileHandle, netuid: u16) -> EvmResult<u64> {
        let val = pallet_subtensor_swap::ScrapReservoirAlpha::<R>::get(NetUid::from(netuid));
        Ok(val.into())
    }

    #[precompile::public("getPalletVersion()")]
    #[precompile::view]
    fn get_pallet_version(_handle: &mut impl PrecompileHandle) -> EvmResult<u16> {
        Ok(<pallet_subtensor_swap::Pallet<R> as frame_support::traits::PalletInfoAccess>::crate_version().major as u16)
    }
}
