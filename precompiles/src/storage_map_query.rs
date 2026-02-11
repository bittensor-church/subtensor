use alloc::vec::Vec;
use core::marker::PhantomData;

use fp_evm::{ExitError, PrecompileFailure};
use frame_support::dispatch::{GetDispatchInfo, PostDispatchInfo};
use pallet_evm::{ExitSucceed, Precompile, PrecompileHandle, PrecompileOutput};
use sp_runtime::traits::{Dispatchable, StaticLookup};
use sp_io::hashing;

use crate::PrecompileExt;

/// Hasher types matching Substrate storage hashers
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StorageHasher {
    Identity = 0,
    Twox64Concat = 1,
    Blake2_128Concat = 2,
    Twox128 = 3,
    Blake2_256 = 4,
}

impl TryFrom<u8> for StorageHasher {
    type Error = PrecompileFailure;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(StorageHasher::Identity),
            1 => Ok(StorageHasher::Twox64Concat),
            2 => Ok(StorageHasher::Blake2_128Concat),
            3 => Ok(StorageHasher::Twox128),
            4 => Ok(StorageHasher::Blake2_256),
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::InvalidRange,
            }),
        }
    }
}

/// Return type for SCALE decoding
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ReturnType {
    /// Raw bytes (no decoding)
    Bytes = 0,
    /// uint8
    U8 = 1,
    /// uint16 (little-endian)
    U16 = 2,
    /// uint32 (little-endian)
    U32 = 3,
    /// uint64 (little-endian)
    U64 = 4,
    /// uint128 (little-endian)
    U128 = 5,
    /// bool
    Bool = 6,
    /// bytes32 (AccountId / H256)
    Bytes32 = 7,
}

impl TryFrom<u8> for ReturnType {
    type Error = PrecompileFailure;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ReturnType::Bytes),
            1 => Ok(ReturnType::U8),
            2 => Ok(ReturnType::U16),
            3 => Ok(ReturnType::U32),
            4 => Ok(ReturnType::U64),
            5 => Ok(ReturnType::U128),
            6 => Ok(ReturnType::Bool),
            7 => Ok(ReturnType::Bytes32),
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::InvalidRange,
            }),
        }
    }
}

pub struct StorageMapQueryPrecompile<R>(PhantomData<R>);

// ==================== Function Selectors (keccak256) ====================
// getValue(string,string,uint8) => 0x311959ba
const SEL_GET_VALUE: [u8; 4] = [0x31, 0x19, 0x59, 0xba];
// getMap(string,string,uint8,bytes,uint8) => 0x647bb2f3
const SEL_GET_MAP: [u8; 4] = [0x64, 0x7b, 0xb2, 0xf3];
// getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8) => 0xcc1273cc
const SEL_GET_DOUBLE_MAP: [u8; 4] = [0xcc, 0x12, 0x73, 0xcc];
// getValueRaw(string,string) => 0x1cb67b10
const SEL_GET_VALUE_RAW: [u8; 4] = [0x1c, 0xb6, 0x7b, 0x10];
// getMapRaw(string,string,uint8,bytes) => 0x06940235
const SEL_GET_MAP_RAW: [u8; 4] = [0x06, 0x94, 0x02, 0x35];

impl<R> PrecompileExt<R::AccountId> for StorageMapQueryPrecompile<R>
where
    R: frame_system::Config + pallet_subtensor::Config + pallet_evm::Config,
    R::AccountId: From<[u8; 32]>,
    <R as frame_system::Config>::RuntimeCall:
    GetDispatchInfo + Dispatchable<PostInfo = PostDispatchInfo>,
    <R as frame_system::Config>::RuntimeCall: From<pallet_subtensor::Call<R>>
    + GetDispatchInfo
    + Dispatchable<PostInfo = PostDispatchInfo>,
    <<R as frame_system::Config>::Lookup as StaticLookup>::Source: From<R::AccountId>,
{
    const INDEX: u64 = 2063; // 0x80F
}

impl<R> Precompile for StorageMapQueryPrecompile<R>
where
    R: frame_system::Config + pallet_subtensor::Config + pallet_evm::Config,
    R::AccountId: From<[u8; 32]>,
    <R as frame_system::Config>::RuntimeCall:
    GetDispatchInfo + Dispatchable<PostInfo = PostDispatchInfo>,
    <R as frame_system::Config>::RuntimeCall: From<pallet_subtensor::Call<R>>
    + GetDispatchInfo
    + Dispatchable<PostInfo = PostDispatchInfo>,
    <<R as frame_system::Config>::Lookup as StaticLookup>::Source: From<R::AccountId>,
{
    fn execute(handle: &mut impl PrecompileHandle) -> fp_evm::PrecompileResult {
        let input = handle.input();

        if input.len() < 4 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Input too short".into()),
            });
        }

        let selector: [u8; 4] = input[0..4].try_into().unwrap();
        let data = &input[4..];

        match selector {
            SEL_GET_VALUE => Self::handle_get_value_typed(data),
            SEL_GET_MAP => Self::handle_get_map_typed(data),
            SEL_GET_DOUBLE_MAP => Self::handle_get_double_map_typed(data),
            SEL_GET_VALUE_RAW => Self::handle_get_value_raw(data),
            SEL_GET_MAP_RAW => Self::handle_get_map_raw(data),
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Unknown function selector".into()),
            }),
        }
    }
}

impl<R> StorageMapQueryPrecompile<R>
where
    R: frame_system::Config + pallet_subtensor::Config,
{
    // ==================== Typed Handlers ====================

    /// getValue(string pallet, string storage, uint8 returnType)
    fn handle_get_value_typed(data: &[u8]) -> fp_evm::PrecompileResult {
        if data.len() < 96 {
            return Err(Self::err("Invalid ABI data for getValue"));
        }

        let pallet_offset = Self::read_u256_as_usize(data, 0)?;
        let storage_offset = Self::read_u256_as_usize(data, 32)?;
        let return_type = ReturnType::try_from(data[95])?; // uint8 at slot 2 (bytes 64..96)

        let pallet = Self::get_slice_at(data, pallet_offset)?;
        let storage = Self::get_slice_at(data, storage_offset)?;

        let key = Self::build_storage_value_key(pallet, storage);
        let result = sp_io::storage::get(&key).unwrap_or_default();

        Self::encode_result(&result, return_type)
    }

    /// getMap(string pallet, string storage, uint8 hasher, bytes key, uint8 returnType)
    fn handle_get_map_typed(data: &[u8]) -> fp_evm::PrecompileResult {
        if data.len() < 160 {
            return Err(Self::err("Invalid ABI data for getMap"));
        }

        let pallet_offset = Self::read_u256_as_usize(data, 0)?;
        let storage_offset = Self::read_u256_as_usize(data, 32)?;
        let hasher_val = data[95];              // uint8 at slot 2
        let key_offset = Self::read_u256_as_usize(data, 96)?;
        let return_type = ReturnType::try_from(data[159])?; // uint8 at slot 4

        let pallet = Self::get_slice_at(data, pallet_offset)?;
        let storage = Self::get_slice_at(data, storage_offset)?;
        let map_key = Self::get_slice_at(data, key_offset)?;
        let hasher = StorageHasher::try_from(hasher_val)?;

        let full_key = Self::build_storage_map_key(pallet, storage, hasher, map_key);
        let result = sp_io::storage::get(&full_key).unwrap_or_default();

        Self::encode_result(&result, return_type)
    }

    /// getDoubleMap(string pallet, string storage, uint8 hasher1, bytes key1, uint8 hasher2, bytes key2, uint8 returnType)
    fn handle_get_double_map_typed(data: &[u8]) -> fp_evm::PrecompileResult {
        if data.len() < 224 {
            return Err(Self::err("Invalid ABI data for getDoubleMap"));
        }

        let pallet_offset = Self::read_u256_as_usize(data, 0)?;
        let storage_offset = Self::read_u256_as_usize(data, 32)?;
        let hasher1_val = data[95];             // uint8 at slot 2
        let key1_offset = Self::read_u256_as_usize(data, 96)?;
        let hasher2_val = data[159];            // uint8 at slot 4
        let key2_offset = Self::read_u256_as_usize(data, 160)?;
        let return_type = ReturnType::try_from(data[223])?; // uint8 at slot 6

        let pallet = Self::get_slice_at(data, pallet_offset)?;
        let storage = Self::get_slice_at(data, storage_offset)?;
        let key1 = Self::get_slice_at(data, key1_offset)?;
        let key2 = Self::get_slice_at(data, key2_offset)?;
        let hasher1 = StorageHasher::try_from(hasher1_val)?;
        let hasher2 = StorageHasher::try_from(hasher2_val)?;

        // Pre-allocate buffer to avoid re-allocation during append
        let mut full_key = Vec::with_capacity(128);
        full_key.extend_from_slice(&hashing::twox_128(pallet));
        full_key.extend_from_slice(&hashing::twox_128(storage));
        Self::append_hashed_key(&mut full_key, hasher1, key1);
        Self::append_hashed_key(&mut full_key, hasher2, key2);

        let result = sp_io::storage::get(&full_key).unwrap_or_default();

        Self::encode_result(&result, return_type)
    }

    // ==================== Raw Handlers (return bytes) ====================

    /// getValueRaw(string pallet, string storage) -> bytes
    fn handle_get_value_raw(data: &[u8]) -> fp_evm::PrecompileResult {
        if data.len() < 64 {
            return Err(Self::err("Invalid ABI data"));
        }
        let (pallet, storage) = Self::decode_two_slices(data)?;
        let key = Self::build_storage_value_key(pallet, storage);
        let result = sp_io::storage::get(&key).unwrap_or_default();
        Self::encode_result(&result, ReturnType::Bytes)
    }

    /// getMapRaw(string pallet, string storage, uint8 hasher, bytes key) -> bytes
    fn handle_get_map_raw(data: &[u8]) -> fp_evm::PrecompileResult {
        if data.len() < 128 {
            return Err(Self::err("Invalid ABI data"));
        }
        let pallet_offset = Self::read_u256_as_usize(data, 0)?;
        let storage_offset = Self::read_u256_as_usize(data, 32)?;
        let hasher_val = data[95];
        let key_offset = Self::read_u256_as_usize(data, 96)?;

        let pallet = Self::get_slice_at(data, pallet_offset)?;
        let storage = Self::get_slice_at(data, storage_offset)?;
        let map_key = Self::get_slice_at(data, key_offset)?;
        let hasher = StorageHasher::try_from(hasher_val)?;

        let full_key = Self::build_storage_map_key(pallet, storage, hasher, map_key);
        let result = sp_io::storage::get(&full_key).unwrap_or_default();
        Self::encode_result(&result, ReturnType::Bytes)
    }

    // ==================== Result Encoding ====================

    /// SCALE-decode the raw storage bytes and ABI-encode for the requested return type
    fn encode_result(raw: &[u8], return_type: ReturnType) -> fp_evm::PrecompileResult {
        let output = match return_type {
            ReturnType::Bytes => Self::abi_encode_bytes(raw),
            ReturnType::U8 => {
                let v = if raw.is_empty() { 0u8 } else { raw[0] };
                Self::abi_encode_u256_from_u8(v)
            }
            ReturnType::U16 => {
                let v = if raw.len() >= 2 {
                    u16::from_le_bytes([raw[0], raw[1]])
                } else { 0 };
                Self::abi_encode_u256_from_u16(v)
            }
            ReturnType::U32 => {
                let v = if raw.len() >= 4 {
                    u32::from_le_bytes(raw[0..4].try_into().unwrap())
                } else { 0 };
                Self::abi_encode_u256_from_u32(v)
            }
            ReturnType::U64 => {
                let v = if raw.len() >= 8 {
                    u64::from_le_bytes(raw[0..8].try_into().unwrap())
                } else { 0 };
                Self::abi_encode_u256_from_u64(v)
            }
            ReturnType::U128 => {
                let v = if raw.len() >= 16 {
                    u128::from_le_bytes(raw[0..16].try_into().unwrap())
                } else { 0 };
                Self::abi_encode_u256_from_u128(v)
            }
            ReturnType::Bool => {
                let v = !raw.is_empty() && raw[0] != 0;
                let mut out = [0u8; 32];
                if v { out[31] = 1; }
                out.to_vec()
            }
            ReturnType::Bytes32 => {
                let mut out = [0u8; 32];
                let copy_len = core::cmp::min(raw.len(), 32);
                out[..copy_len].copy_from_slice(&raw[..copy_len]);
                out.to_vec()
            }
        };

        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output,
        })
    }

    // ==================== ABI Encoding Helpers ====================

    fn abi_encode_u256_from_u8(v: u8) -> Vec<u8> {
        let mut out = [0u8; 32];
        out[31] = v;
        out.to_vec()
    }

    fn abi_encode_u256_from_u16(v: u16) -> Vec<u8> {
        let mut out = [0u8; 32];
        out[30..32].copy_from_slice(&v.to_be_bytes());
        out.to_vec()
    }

    fn abi_encode_u256_from_u32(v: u32) -> Vec<u8> {
        let mut out = [0u8; 32];
        out[28..32].copy_from_slice(&v.to_be_bytes());
        out.to_vec()
    }

    fn abi_encode_u256_from_u64(v: u64) -> Vec<u8> {
        let mut out = [0u8; 32];
        out[24..32].copy_from_slice(&v.to_be_bytes());
        out.to_vec()
    }

    fn abi_encode_u256_from_u128(v: u128) -> Vec<u8> {
        let mut out = [0u8; 32];
        out[16..32].copy_from_slice(&v.to_be_bytes());
        out.to_vec()
    }

    fn abi_encode_bytes(data: &[u8]) -> Vec<u8> {
        let padded_len = data.len().div_ceil(32) * 32;
        let mut result = Vec::with_capacity(64 + padded_len);
        // Offset
        let mut offset = [0u8; 32];
        offset[31] = 32;
        result.extend_from_slice(&offset);
        // Length
        let mut len_bytes = [0u8; 32];
        len_bytes[24..32].copy_from_slice(&(data.len() as u64).to_be_bytes());
        result.extend_from_slice(&len_bytes);
        // Data padded
        result.extend_from_slice(data);
        result.resize(64 + padded_len, 0);
        result
    }

    // ==================== ABI Decoding Helpers ====================

    fn err(msg: &'static str) -> PrecompileFailure {
        PrecompileFailure::Error {
            exit_status: ExitError::Other(msg.into()),
        }
    }

    fn read_u256_as_usize(data: &[u8], offset: usize) -> Result<usize, PrecompileFailure> {
        if offset + 32 > data.len() {
            return Err(Self::err("Out of bounds"));
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&data[offset + 24..offset + 32]);
        Ok(u64::from_be_bytes(bytes) as usize)
    }

    fn decode_two_slices(data: &[u8]) -> Result<(&[u8], &[u8]), PrecompileFailure> {
        let o1 = Self::read_u256_as_usize(data, 0)?;
        let o2 = Self::read_u256_as_usize(data, 32)?;
        Ok((Self::get_slice_at(data, o1)?, Self::get_slice_at(data, o2)?))
    }

    fn get_slice_at(data: &[u8], offset: usize) -> Result<&[u8], PrecompileFailure> {
        if offset + 32 > data.len() {
            return Err(Self::err("Invalid offset"));
        }
        let len = Self::read_u256_as_usize(data, offset)?;
        let start = offset + 32;
        let end = start + len;
        if end > data.len() {
            return Err(Self::err("Data out of bounds"));
        }
        Ok(&data[start..end])
    }

    // ==================== Storage Key Building ====================

    fn build_storage_value_key(pallet: &[u8], storage: &[u8]) -> Vec<u8> {
        let mut key = Vec::with_capacity(32);
        key.extend_from_slice(&hashing::twox_128(pallet));
        key.extend_from_slice(&hashing::twox_128(storage));
        key
    }

    fn build_storage_map_key(pallet: &[u8], storage: &[u8], hasher: StorageHasher, item_key: &[u8]) -> Vec<u8> {
        // Pre-allocate buffer to avoid re-allocation during append
        let mut key = Vec::with_capacity(128);
        key.extend_from_slice(&hashing::twox_128(pallet));
        key.extend_from_slice(&hashing::twox_128(storage));
        Self::append_hashed_key(&mut key, hasher, item_key);
        key
    }

    fn append_hashed_key(dest: &mut Vec<u8>, hasher: StorageHasher, key: &[u8]) {
        match hasher {
            StorageHasher::Identity => dest.extend_from_slice(key),
            StorageHasher::Twox64Concat => {
                dest.extend_from_slice(&hashing::twox_64(key));
                dest.extend_from_slice(key);
            }
            StorageHasher::Blake2_128Concat => {
                dest.extend_from_slice(&hashing::blake2_128(key));
                dest.extend_from_slice(key);
            }
            StorageHasher::Twox128 => dest.extend_from_slice(&hashing::twox_128(key)),
            StorageHasher::Blake2_256 => dest.extend_from_slice(&hashing::blake2_256(key)),
        }
    }
}