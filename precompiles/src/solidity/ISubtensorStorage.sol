// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @title ISubtensorStorage - Generic Substrate storage access with automatic SCALE decoding
/// @notice Specify return type and the precompile handles all encoding/decoding
/// @dev Precompile address: 0x000000000000000000000000000000000000080F
///
/// Return types:
///   0 = bytes   (raw SCALE, no decoding)
///   1 = uint8
///   2 = uint16
///   3 = uint32
///   4 = uint64
///   5 = uint128
///   6 = bool
///   7 = bytes32
///
/// Hasher types:
///   0 = Identity        (no hash)
///   1 = Twox64Concat    (twox_64 ++ key)
///   2 = Blake2_128Concat (blake2_128 ++ key)
///   3 = Twox128
///   4 = Blake2_256
interface ISubtensorStorage {

    /// @notice Query StorageValue with typed return
    /// @param pallet   Pallet name, e.g. "SubtensorModule"
    /// @param storage  Storage item name, e.g. "TotalStake"
    /// @param retType  Return type (0-7, see above)
    /// @return ABI-encoded value of the requested type
    function getValue(string calldata pallet, string calldata storage, uint8 retType) 
        external view returns (uint256);
    
    /// @notice Query StorageMap with typed return
    /// @param pallet   Pallet name
    /// @param storage  Storage item name
    /// @param hasher   Hasher type (0-4)
    /// @param key      SCALE-encoded key (use abi.encodePacked for LE encoding)
    /// @param retType  Return type (0-7)
    /// @return ABI-encoded value of the requested type
    function getMap(
        string calldata pallet, 
        string calldata storage, 
        uint8 hasher, 
        bytes calldata key, 
        uint8 retType
    ) external view returns (uint256);
    
    /// @notice Query StorageDoubleMap with typed return
    function getDoubleMap(
        string calldata pallet,
        string calldata storage,
        uint8 hasher1, bytes calldata key1,
        uint8 hasher2, bytes calldata key2,
        uint8 retType
    ) external view returns (uint256);
    
    /// @notice Query StorageValue - raw bytes (no decoding)
    function getValueRaw(string calldata pallet, string calldata storage) 
        external view returns (bytes memory);
    
    /// @notice Query StorageMap - raw bytes (no decoding)
    function getMapRaw(
        string calldata pallet, 
        string calldata storage, 
        uint8 hasher, 
        bytes calldata key
    ) external view returns (bytes memory);
}

// ==================== Constants ====================

ISubtensorStorage constant SUBTENSOR_STORAGE = ISubtensorStorage(0x000000000000000000000000000000000000080F);

// Return types
uint8 constant RET_BYTES   = 0;
uint8 constant RET_U8      = 1;
uint8 constant RET_U16     = 2;
uint8 constant RET_U32     = 3;
uint8 constant RET_U64     = 4;
uint8 constant RET_U128    = 5;
uint8 constant RET_BOOL    = 6;
uint8 constant RET_BYTES32 = 7;

// Hasher types
uint8 constant HASH_IDENTITY         = 0;
uint8 constant HASH_TWOX64_CONCAT    = 1;
uint8 constant HASH_BLAKE2_128_CONCAT = 2;
uint8 constant HASH_TWOX128          = 3;
uint8 constant HASH_BLAKE2_256       = 4;

/// @title ScaleEncode - Minimal SCALE encoding helpers (only key encoding needed in Solidity)
library ScaleEncode {
    function u16LE(uint16 v) internal pure returns (bytes memory) {
        return abi.encodePacked(uint8(v & 0xFF), uint8(v >> 8));
    }
    
    function u32LE(uint32 v) internal pure returns (bytes memory) {
        return abi.encodePacked(
            uint8(v & 0xFF), uint8((v >> 8) & 0xFF),
            uint8((v >> 16) & 0xFF), uint8(v >> 24)
        );
    }
    
    function u64LE(uint64 v) internal pure returns (bytes memory) {
        bytes memory r = new bytes(8);
        for (uint i = 0; i < 8; i++) {
            r[i] = bytes1(uint8(v >> (i * 8)));
        }
        return r;
    }
    
    function account(bytes32 v) internal pure returns (bytes memory) {
        return abi.encodePacked(v);
    }
}
