# Subtensor Storage Items - Precompile Reference

> Auto-generated reference for using `ISubtensorStorage` precompile (address `0x809`).
> Import `ISubtensorStorage.sol` and use the `SUBTENSOR_STORAGE` constant.

## Quick Start

```solidity
import "./ISubtensorStorage.sol";

contract Example {
    // StorageValue: getTotalStake() -> uint64
    function totalStake() external view returns (uint64) {
        return uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "TotalStake", RET_U64));
    }
    
    // StorageMap: getBurn(netuid) -> uint64
    function burn(uint16 netuid) external view returns (uint64) {
        return uint64(SUBTENSOR_STORAGE.getMap(
            "SubtensorModule", "Burn",
            HASH_IDENTITY, ScaleEncode.u16LE(netuid), RET_U64
        ));
    }
}
```

## Constants

| Return Type | ID | Constant | Solidity Type |
|-------------|----|-----------|----|
| Raw bytes | 0 | `RET_BYTES` | `bytes memory` |
| uint8 | 1 | `RET_U8` | `uint8` |
| uint16 | 2 | `RET_U16` | `uint16` |
| uint32 | 3 | `RET_U32` | `uint32` |
| uint64 | 4 | `RET_U64` | `uint64` |
| uint128 | 5 | `RET_U128` | `uint128` |
| bool | 6 | `RET_BOOL` | `bool` |
| bytes32 | 7 | `RET_BYTES32` | `bytes32` |

| Hasher | ID | Constant |
|--------|----|----------|
| Identity | 0 | `HASH_IDENTITY` |
| Twox64Concat | 1 | `HASH_TWOX64_CONCAT` |
| Blake2_128Concat | 2 | `HASH_BLAKE2_128_CONCAT` |
| Twox128 | 3 | `HASH_TWOX128` |
| Blake2_256 | 4 | `HASH_BLAKE2_256` |

## Key Encoding (ScaleEncode library)

| Rust Type | Solidity | Encode |
|-----------|----------|--------|
| `u16` / `NetUid` | `uint16` | `ScaleEncode.u16LE(val)` |
| `u32` | `uint32` | `ScaleEncode.u32LE(val)` |
| `u64` | `uint64` | `ScaleEncode.u64LE(val)` |
| `T::AccountId` | `bytes32` | `ScaleEncode.account(val)` |

---


## `AdminUtils`

### `PrecompileEnable` — StorageMap[`PrecompileEnum`] → `bool` (bool)
Hasher: **Blake2_128Concat** (`HASH_BLAKE2_128_CONCAT`)

```solidity
// Param: bytes memory myKey
bool val = bool(SUBTENSOR_STORAGE.getMap(
    "AdminUtils", "PrecompileEnable",
    HASH_BLAKE2_128_CONCAT, myKey, RET_BOOL
));
```


## `Commitments`

### `TimelockedIndex` — StorageDoubleMap[`NetUid`, `T::AccountId`] → `Registration<BalanceOf<T` (bytes memory)
Hashers: **Identity** + **Twox64Concat**

```solidity
// Params: uint16 key1, bytes32 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "Commitments", "TimelockedIndex",
    HASH_IDENTITY, ScaleEncode.u16LE(key1),
    HASH_TWOX64_CONCAT, ScaleEncode.account(key2),
    RET_BYTES
));
```

### `UsedSpaceOf` — StorageDoubleMap[`NetUid`, `T::AccountId`] → `UsageTracker` (bytes memory)
Hashers: **Identity** + **Twox64Concat**

```solidity
// Params: uint16 key1, bytes32 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "Commitments", "UsedSpaceOf",
    HASH_IDENTITY, ScaleEncode.u16LE(key1),
    HASH_TWOX64_CONCAT, ScaleEncode.account(key2),
    RET_BYTES
));
```


## `Crowdloan`

### `Contributions` — StorageValue → `CrowdloanId` (bytes memory)

```solidity
bytes memory val = SUBTENSOR_STORAGE.getValueRaw("Crowdloan", "Contributions");
```

### `CurrencyOf` — StorageDoubleMap[`CrowdloanId`, `T::AccountId`] → `BalanceOf<T` (bytes memory)
Hashers: **Twox64Concat** + **Identity**

```solidity
// Params: bytes memory key1, bytes32 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "Crowdloan", "CurrencyOf",
    HASH_TWOX64_CONCAT, key1,
    HASH_IDENTITY, ScaleEncode.account(key2),
    RET_BYTES
));
```

### `NextCrowdloanId` — StorageMap[`BoundedVec<u8`] → `MigrationKeyMaxLen` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: bytes memory myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "Crowdloan", "NextCrowdloanId",
    HASH_IDENTITY, myKey
);
```

### `PassedCrowdloanId` — StorageValue → `CrowdloanId` (bytes memory)

```solidity
bytes memory val = SUBTENSOR_STORAGE.getValueRaw("Crowdloan", "PassedCrowdloanId");
```


## `Drand`

### `BeaconConfig` — StorageMap[`BoundedVec<u8`] → `MigrationKeyMaxLen` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: bytes memory myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "Drand", "BeaconConfig",
    HASH_IDENTITY, myKey
);
```

### `HasMigrationRun` — StorageValue → `RoundNumber` (bytes memory)

```solidity
bytes memory val = SUBTENSOR_STORAGE.getValueRaw("Drand", "HasMigrationRun");
```

### `OldestStoredRound` — StorageValue → `RoundNumber` (bytes memory)

```solidity
bytes memory val = SUBTENSOR_STORAGE.getValueRaw("Drand", "OldestStoredRound");
```

### `Pulses` — StorageMap[`RoundNumber`] → `Pulse` (bytes memory)
Hasher: **Blake2_128Concat** (`HASH_BLAKE2_128_CONCAT`)

```solidity
// Param: bytes memory myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "Drand", "Pulses",
    HASH_BLAKE2_128_CONCAT, myKey
);
```


## `Proxy`

### `Announcements` — StorageMap[`T::AccountId`] → `(
            BoundedVec<Announcement<T::AccountId` (bytes memory)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: bytes32 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "Proxy", "Announcements",
    HASH_TWOX64_CONCAT, ScaleEncode.account(myKey)
);
```

### `BlockNumberFor` — StorageMap[`T::AccountId`] → `(
            BoundedVec<
                ProxyDefinition<T::AccountId` (bytes memory)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: bytes32 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "Proxy", "BlockNumberFor",
    HASH_TWOX64_CONCAT, ScaleEncode.account(myKey)
);
```

### `LastCallResult` — StorageMap[`T::AccountId`] → `DispatchResult` (bytes memory)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: bytes32 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "Proxy", "LastCallResult",
    HASH_TWOX64_CONCAT, ScaleEncode.account(myKey)
);
```


## `Shield`

### `CurrentKey` — StorageMap[`T::Hash`] → `Submission<T::AccountId` (bytes memory)
Hasher: **Blake2_128Concat** (`HASH_BLAKE2_128_CONCAT`)

```solidity
// Param: bytes memory myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "Shield", "CurrentKey",
    HASH_BLAKE2_128_CONCAT, myKey
);
```

### `KeyHashByBlock` — StorageMap[`BlockNumberFor<T>`] → `T::Hash` (bytes memory)
Hasher: **Blake2_128Concat** (`HASH_BLAKE2_128_CONCAT`)

```solidity
// Param: bytes memory myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "Shield", "KeyHashByBlock",
    HASH_BLAKE2_128_CONCAT, myKey
);
```

### `NextKey` — StorageValue → `BoundedVec<u8` (bytes memory)

```solidity
bytes memory val = SUBTENSOR_STORAGE.getValueRaw("Shield", "NextKey");
```


## `SubtensorModule`

### `AccumulatedLeaseDividends` — StorageValue → `u16` (uint16)

```solidity
uint16 val = uint16(SUBTENSOR_STORAGE.getValue("SubtensorModule", "AccumulatedLeaseDividends", RET_U16));
```

### `Active` — StorageMap[`NetUid`] → `Vec<bool` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "Active",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `ActivityCutoff` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "ActivityCutoff",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `AdjustmentAlpha` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "AdjustmentAlpha",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `AdjustmentInterval` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "AdjustmentInterval",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `AdminFreezeWindow` — StorageValue → `u16` (uint16)

```solidity
uint16 val = uint16(SUBTENSOR_STORAGE.getValue("SubtensorModule", "AdminFreezeWindow", RET_U16));
```

### `Alpha` — StorageDoubleMap[`T::AccountId`, `NetUid`] → `bool` (bool)
Hashers: **Blake2_128Concat** + **Identity**

```solidity
// Params: bytes32 key1, uint16 key2
bool val = bool(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "Alpha",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key1),
    HASH_IDENTITY, ScaleEncode.u16LE(key2),
    RET_BOOL
));
```

### `AlphaDividendsPerSubnet` — StorageDoubleMap[`NetUid`, `T::AccountId`] → `AlphaCurrency` (uint64)
Hashers: **Identity** + **Blake2_128Concat**

```solidity
// Params: uint16 key1, bytes32 key2
uint64 val = uint64(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "AlphaDividendsPerSubnet",
    HASH_IDENTITY, ScaleEncode.u16LE(key1),
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key2),
    RET_U64
));
```

### `AlphaSigmoidSteepness` — StorageMap[`NetUid`] → `i16` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "AlphaSigmoidSteepness",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `AlphaValues` — StorageMap[`NetUid`] → `(u16` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "AlphaValues",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `AssociatedEvmAddress` — StorageValue → `LeaseId` (bytes memory)

```solidity
bytes memory val = SUBTENSOR_STORAGE.getValueRaw("SubtensorModule", "AssociatedEvmAddress");
```

### `AutoStakeDestination` — StorageMap[`T::AccountId`] → `(BlockNumberFor<T` (bytes memory)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: bytes32 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "AutoStakeDestination",
    HASH_TWOX64_CONCAT, ScaleEncode.account(myKey)
);
```

### `AutoStakeDestinationColdkeys` — StorageDoubleMap[`T::AccountId`, `NetUid`] → `Vec<T::AccountId` (bytes memory)
Hashers: **Blake2_128Concat** + **Identity**

```solidity
// Params: bytes32 key1, uint16 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "AutoStakeDestinationColdkeys",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key1),
    HASH_IDENTITY, ScaleEncode.u16LE(key2),
    RET_BYTES
));
```

### `Axons` — StorageDoubleMap[`NetUid`, `T::AccountId`] → `AxonInfoOf` (bytes memory)
Hashers: **Identity** + **Blake2_128Concat**

```solidity
// Params: uint16 key1, bytes32 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "Axons",
    HASH_IDENTITY, ScaleEncode.u16LE(key1),
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key2),
    RET_BYTES
));
```

### `BlockAtRegistration` — StorageDoubleMap[`NetUid`, `u16`] → `u64` (uint64)
Hashers: **Identity** + **Identity**

```solidity
// Params: uint16 key1, uint16 key2
uint64 val = uint64(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "BlockAtRegistration",
    HASH_IDENTITY, ScaleEncode.u16LE(key1),
    HASH_IDENTITY, ScaleEncode.u16LE(key2),
    RET_U64
));
```

### `BlockEmission` — StorageDoubleMap[`T::AccountId`, `NetUid`] → `AlphaCurrency` (uint64)
Hashers: **Blake2_128Concat** + **Identity**

```solidity
// Params: bytes32 key1, uint16 key2
uint64 val = uint64(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "BlockEmission",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key1),
    HASH_IDENTITY, ScaleEncode.u16LE(key2),
    RET_U64
));
```

### `BlocksSinceLastStep` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "BlocksSinceLastStep",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `Bonds` — StorageDoubleMap[`NetUidStorageIndex`, `u16`] → `Vec<(u16` (bytes memory)
Hashers: **Identity** + **Identity**

```solidity
// Params: bytes memory key1, uint16 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "Bonds",
    HASH_IDENTITY, key1,
    HASH_IDENTITY, ScaleEncode.u16LE(key2),
    RET_BYTES
));
```

### `BondsMovingAverage` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "BondsMovingAverage",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `BondsPenalty` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "BondsPenalty",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `BondsResetOn` — StorageMap[`NetUid`] → `bool` (bool)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bool val = bool(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "BondsResetOn",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_BOOL
));
```

### `Burn` — StorageMap[`NetUid`] → `TaoCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "Burn",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `BurnRegistrationsThisInterval` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "BurnRegistrationsThisInterval",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `CKBurn` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "CKBurn", RET_U64));
```

### `CRV3WeightCommits` — StorageDoubleMap[`NetUidStorageIndex`, `u64`] → `// epoch key
        VecDeque<(
            T::AccountId` (bytes memory)
Hashers: **Twox64Concat** + **Twox64Concat**

```solidity
// Params: bytes memory key1, uint64 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "CRV3WeightCommits",
    HASH_TWOX64_CONCAT, key1,
    HASH_TWOX64_CONCAT, ScaleEncode.u64LE(key2),
    RET_BYTES
));
```

### `CRV3WeightCommitsV2` — StorageDoubleMap[`NetUidStorageIndex`, `u64`] → `// epoch key
        VecDeque<(
            T::AccountId` (bytes memory)
Hashers: **Twox64Concat** + **Twox64Concat**

```solidity
// Params: bytes memory key1, uint64 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "CRV3WeightCommitsV2",
    HASH_TWOX64_CONCAT, key1,
    HASH_TWOX64_CONCAT, ScaleEncode.u64LE(key2),
    RET_BYTES
));
```

### `ChildKeys` — StorageDoubleMap[`T::AccountId`, `NetUid`] → `Vec<(u64` (bytes memory)
Hashers: **Blake2_128Concat** + **Identity**

```solidity
// Params: bytes32 key1, uint16 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "ChildKeys",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key1),
    HASH_IDENTITY, ScaleEncode.u16LE(key2),
    RET_BYTES
));
```

### `ChildkeyTake` — StorageMap[`NetUid`] → `I96F32` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "ChildkeyTake",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `ColdkeySwapAnnouncementDelay` — StorageDoubleMap[`T::AccountId`, `NetUid`] → `AlphaCurrency` (uint64)
Hashers: **Blake2_128Concat** + **Identity**

```solidity
// Params: bytes32 key1, uint16 key2
uint64 val = uint64(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "ColdkeySwapAnnouncementDelay",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key1),
    HASH_IDENTITY, ScaleEncode.u16LE(key2),
    RET_U64
));
```

### `ColdkeySwapAnnouncements` — StorageValue → `Option<Vec<u8` (bytes memory)

```solidity
bytes memory val = SUBTENSOR_STORAGE.getValueRaw("SubtensorModule", "ColdkeySwapAnnouncements");
```

### `ColdkeySwapDisputes` — StorageMap[`T::AccountId`] → `BlockNumberFor<T` (bytes memory)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: bytes32 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "ColdkeySwapDisputes",
    HASH_TWOX64_CONCAT, ScaleEncode.account(myKey)
);
```

### `ColdkeySwapReannouncementDelay` — StorageValue → `BlockNumberFor<T` (bytes memory)

```solidity
bytes memory val = SUBTENSOR_STORAGE.getValueRaw("SubtensorModule", "ColdkeySwapReannouncementDelay");
```

### `ColdkeySwapRescheduleDuration` — StorageValue → `BlockNumberFor<T` (bytes memory)

```solidity
bytes memory val = SUBTENSOR_STORAGE.getValueRaw("SubtensorModule", "ColdkeySwapRescheduleDuration");
```

### `ColdkeySwapScheduleDuration` — StorageValue → `BlockNumberFor<T` (bytes memory)

```solidity
bytes memory val = SUBTENSOR_STORAGE.getValueRaw("SubtensorModule", "ColdkeySwapScheduleDuration");
```

### `CommitRevealWeightsEnabled` — StorageMap[`NetUid`] → `bool` (bool)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bool val = bool(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "CommitRevealWeightsEnabled",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_BOOL
));
```

### `CommitRevealWeightsVersion` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "CommitRevealWeightsVersion",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `Consensus` — StorageMap[`NetUid`] → `Vec<u16` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "Consensus",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `Delegates` — StorageMap[`T::AccountId`] → `u16` (uint16)
Hasher: **Blake2_128Concat** (`HASH_BLAKE2_128_CONCAT`)

```solidity
// Param: bytes32 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "Delegates",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(myKey), RET_U16
));
```

### `Difficulty` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "Difficulty",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `DissolveNetworkScheduleDuration` — StorageValue → `BlockNumberFor<T` (bytes memory)

```solidity
bytes memory val = SUBTENSOR_STORAGE.getValueRaw("SubtensorModule", "DissolveNetworkScheduleDuration");
```

### `Dividends` — StorageMap[`NetUid`] → `Vec<u16` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "Dividends",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `EMAPriceHalvingBlocks` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "EMAPriceHalvingBlocks",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `Emission` — StorageMap[`NetUid`] → `Vec<AlphaCurrency` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "Emission",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `FirstEmissionBlockNumber` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "FirstEmissionBlockNumber",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `FlowEmaSmoothingFactor` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "FlowEmaSmoothingFactor", RET_U64));
```

### `FlowNormExponent` — StorageValue → `U64F64` (bytes memory)

```solidity
bytes memory val = SUBTENSOR_STORAGE.getValueRaw("SubtensorModule", "FlowNormExponent");
```

### `HasMigrationRun` — StorageMap[`Vec<u8>`] → `bool` (bool)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: bytes memory myKey
bool val = bool(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "HasMigrationRun",
    HASH_IDENTITY, myKey, RET_BOOL
));
```

### `IdentitiesV2` — StorageDoubleMap[`NetUidStorageIndex`, `T::AccountId`] → `VecDeque<(H256` (bytes memory)
Hashers: **Twox64Concat** + **Twox64Concat**

```solidity
// Params: bytes memory key1, bytes32 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "IdentitiesV2",
    HASH_TWOX64_CONCAT, key1,
    HASH_TWOX64_CONCAT, ScaleEncode.account(key2),
    RET_BYTES
));
```

### `ImmuneOwnerUidsLimit` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "ImmuneOwnerUidsLimit",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `ImmunityPeriod` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "ImmunityPeriod",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `Incentive` — StorageMap[`NetUidStorageIndex`] → `Vec<u16` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: bytes memory myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "Incentive",
    HASH_IDENTITY, myKey
);
```

### `IsNetworkMember` — StorageMap[`NetUid`] → `bool` (bool)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bool val = bool(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "IsNetworkMember",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_BOOL
));
```

### `Kappa` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "Kappa",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `Keys` — StorageDoubleMap[`NetUid`, `u16`] → `T::AccountId` (bytes32)
Hashers: **Identity** + **Identity**

```solidity
// Params: uint16 key1, uint16 key2
bytes32 val = bytes32(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "Keys",
    HASH_IDENTITY, ScaleEncode.u16LE(key1),
    HASH_IDENTITY, ScaleEncode.u16LE(key2),
    RET_BYTES32
));
```

### `LargestLocked` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "LargestLocked",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `LastAdjustmentBlock` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "LastAdjustmentBlock",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `LastColdkeyHotkeyStakeBlock` — StorageMap[`NetUid`] → `I96F32` (bytes memory)
Hasher: **Blake2_128Concat** (`HASH_BLAKE2_128_CONCAT`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "LastColdkeyHotkeyStakeBlock",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.u16LE(myKey)
);
```

### `LastHotkeyEmissionOnNetuid` — StorageValue → `u16` (uint16)

```solidity
uint16 val = uint16(SUBTENSOR_STORAGE.getValue("SubtensorModule", "LastHotkeyEmissionOnNetuid", RET_U16));
```

### `LastHotkeySwapOnNetuid` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "LastHotkeySwapOnNetuid", RET_U64));
```

### `LastMechansimStepBlock` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "LastMechansimStepBlock",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `LastRateLimitedBlock` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "LastRateLimitedBlock", RET_U64));
```

### `LastTxBlockChildKeyTake` — StorageMap[`T::AccountId`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: bytes32 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "LastTxBlockChildKeyTake",
    HASH_IDENTITY, ScaleEncode.account(myKey), RET_U64
));
```

### `LastTxBlockDelegateTake` — StorageMap[`T::AccountId`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: bytes32 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "LastTxBlockDelegateTake",
    HASH_IDENTITY, ScaleEncode.account(myKey), RET_U64
));
```

### `LastUpdate` — StorageMap[`NetUidStorageIndex`] → `Vec<u64` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: bytes memory myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "LastUpdate",
    HASH_IDENTITY, myKey
);
```

### `LiquidAlphaOn` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "LiquidAlphaOn", RET_U64));
```

### `LoadedEmission` — StorageDoubleMap[`NetUidStorageIndex`, `u16`] → `Vec<(u16` (bytes memory)
Hashers: **Identity** + **Identity**

```solidity
// Params: bytes memory key1, uint16 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "LoadedEmission",
    HASH_IDENTITY, key1,
    HASH_IDENTITY, ScaleEncode.u16LE(key2),
    RET_BYTES
));
```

### `MaxAllowedUids` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "MaxAllowedUids",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `MaxAllowedValidators` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "MaxAllowedValidators",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `MaxBurn` — StorageMap[`NetUid`] → `TaoCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "MaxBurn",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `MaxChildkeyTake` — StorageValue → `u16` (uint16)

```solidity
uint16 val = uint16(SUBTENSOR_STORAGE.getValue("SubtensorModule", "MaxChildkeyTake", RET_U16));
```

### `MaxDelegateTake` — StorageValue → `u16` (uint16)

```solidity
uint16 val = uint16(SUBTENSOR_STORAGE.getValue("SubtensorModule", "MaxDelegateTake", RET_U16));
```

### `MaxDifficulty` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "MaxDifficulty",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `MaxMechanismCount` — StorageMap[`NetUid`] → `MechId` (bytes memory)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "MaxMechanismCount",
    HASH_TWOX64_CONCAT, ScaleEncode.u16LE(myKey)
);
```

### `MaxRegistrationsPerBlock` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "MaxRegistrationsPerBlock",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `MaxWeightsLimit` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "MaxWeightsLimit",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `MechanismCountCurrent` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "MechanismCountCurrent", RET_U64));
```

### `MechanismEmissionSplit` — StorageMap[`NetUid`] → `Vec<u16` (bytes memory)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "MechanismEmissionSplit",
    HASH_TWOX64_CONCAT, ScaleEncode.u16LE(myKey)
);
```

### `MinAllowedUids` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "MinAllowedUids",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `MinAllowedWeights` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "MinAllowedWeights",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `MinBurn` — StorageMap[`NetUid`] → `TaoCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "MinBurn",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `MinChildkeyTake` — StorageValue → `u16` (uint16)

```solidity
uint16 val = uint16(SUBTENSOR_STORAGE.getValue("SubtensorModule", "MinChildkeyTake", RET_U16));
```

### `MinDelegateTake` — StorageValue → `u16` (uint16)

```solidity
uint16 val = uint16(SUBTENSOR_STORAGE.getValue("SubtensorModule", "MinDelegateTake", RET_U16));
```

### `MinDifficulty` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "MinDifficulty",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `MinNonImmuneUids` — StorageValue → `MechId` (bytes memory)

```solidity
bytes memory val = SUBTENSOR_STORAGE.getValueRaw("SubtensorModule", "MinNonImmuneUids");
```

### `NetworkImmunityPeriod` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "NetworkImmunityPeriod", RET_U64));
```

### `NetworkLastLockCost` — StorageValue → `TaoCurrency` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "NetworkLastLockCost", RET_U64));
```

### `NetworkLockReductionInterval` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "NetworkLockReductionInterval", RET_U64));
```

### `NetworkMinLockCost` — StorageValue → `TaoCurrency` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "NetworkMinLockCost", RET_U64));
```

### `NetworkPowRegistrationAllowed` — StorageMap[`NetUid`] → `bool` (bool)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bool val = bool(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "NetworkPowRegistrationAllowed",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_BOOL
));
```

### `NetworkRateLimit` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "NetworkRateLimit", RET_U64));
```

### `NetworkRegisteredAt` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "NetworkRegisteredAt",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `NetworkRegistrationAllowed` — StorageDoubleMap[`NetUid`, `T::AccountId`] → `u64` (uint64)
Hashers: **Identity** + **Blake2_128Concat**

```solidity
// Params: uint16 key1, bytes32 key2
uint64 val = uint64(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "NetworkRegistrationAllowed",
    HASH_IDENTITY, ScaleEncode.u16LE(key1),
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key2),
    RET_U64
));
```

### `NetworkRegistrationStartBlock` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "NetworkRegistrationStartBlock", RET_U64));
```

### `NetworksAdded` — StorageMap[`NetUid`] → `bool` (bool)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bool val = bool(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "NetworksAdded",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_BOOL
));
```

### `NeuronCertificates` — StorageDoubleMap[`NetUid`, `T::AccountId`] → `NeuronCertificateOf` (bytes memory)
Hashers: **Identity** + **Blake2_128Concat**

```solidity
// Params: uint16 key1, bytes32 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "NeuronCertificates",
    HASH_IDENTITY, ScaleEncode.u16LE(key1),
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key2),
    RET_BYTES
));
```

### `NextStakeJobId` — StorageDoubleMap[`NetUid`, `T::AccountId`] → `(Vec<(u64` (bytes memory)
Hashers: **Identity** + **Blake2_128Concat**

```solidity
// Params: uint16 key1, bytes32 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "NextStakeJobId",
    HASH_IDENTITY, ScaleEncode.u16LE(key1),
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key2),
    RET_BYTES
));
```

### `NextSubnetLeaseId` — StorageMap[`LeaseId`] → `AlphaCurrency` (uint64)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: bytes memory myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "NextSubnetLeaseId",
    HASH_TWOX64_CONCAT, myKey, RET_U64
));
```

### `NominatorMinRequiredStake` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "NominatorMinRequiredStake", RET_U64));
```

### `NumRootClaim` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "NumRootClaim", RET_U64));
```

### `NumStakingColdkeys` — StorageMap[`LeaseId`] → `SubnetLeaseOf<T` (bytes memory)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: bytes memory myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "NumStakingColdkeys",
    HASH_TWOX64_CONCAT, myKey
);
```

### `OwnedHotkeys` — StorageMap[`T::AccountId`] → `Vec<T::AccountId` (bytes memory)
Hasher: **Blake2_128Concat** (`HASH_BLAKE2_128_CONCAT`)

```solidity
// Param: bytes32 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "OwnedHotkeys",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(myKey)
);
```

### `Owner` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "Owner", RET_U64));
```

### `OwnerHyperparamRateLimit` — StorageValue → `u16` (uint16)

```solidity
uint16 val = uint16(SUBTENSOR_STORAGE.getValue("SubtensorModule", "OwnerHyperparamRateLimit", RET_U16));
```

### `POWRegistrationsThisInterval` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "POWRegistrationsThisInterval",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `PalletsOriginOf` — StorageDoubleMap[`NetUid`, `T::AccountId`] → `u64` (uint64)
Hashers: **Identity** + **Blake2_128Concat**

```solidity
// Params: uint16 key1, bytes32 key2
uint64 val = uint64(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "PalletsOriginOf",
    HASH_IDENTITY, ScaleEncode.u16LE(key1),
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key2),
    RET_U64
));
```

### `ParentKeys` — StorageDoubleMap[`T::AccountId`, `NetUid`] → `Vec<(u64` (bytes memory)
Hashers: **Blake2_128Concat** + **Identity**

```solidity
// Params: bytes32 key1, uint16 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "ParentKeys",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key1),
    HASH_IDENTITY, ScaleEncode.u16LE(key2),
    RET_BYTES
));
```

### `PendingOwnerCut` — StorageMap[`NetUid`] → `AlphaCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "PendingOwnerCut",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `PendingRootAlphaDivs` — StorageMap[`NetUid`] → `AlphaCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "PendingRootAlphaDivs",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `PendingServerEmission` — StorageMap[`NetUid`] → `AlphaCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "PendingServerEmission",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `PendingValidatorEmission` — StorageMap[`NetUid`] → `AlphaCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "PendingValidatorEmission",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `Prometheus` — StorageDoubleMap[`NetUid`, `T::AccountId`] → `PrometheusInfoOf` (bytes memory)
Hashers: **Identity** + **Blake2_128Concat**

```solidity
// Params: uint16 key1, bytes32 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "Prometheus",
    HASH_IDENTITY, ScaleEncode.u16LE(key1),
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key2),
    RET_BYTES
));
```

### `PruningScores` — StorageMap[`NetUid`] → `Vec<u16` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "PruningScores",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `RAORecycledForRegistration` — StorageMap[`NetUid`] → `TaoCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "RAORecycledForRegistration",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `Rank` — StorageMap[`NetUid`] → `Vec<u16` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "Rank",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `RecycleOrBurn` — StorageMap[`NetUid`] → `RecycleOrBurnEnum` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "RecycleOrBurn",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `RegistrationsThisBlock` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "RegistrationsThisBlock",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `RegistrationsThisInterval` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "RegistrationsThisInterval",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `RevealPeriodEpochs` — StorageDoubleMap[`T::AccountId`, `T::AccountId`] → `u64` (uint64)
Hashers: **Twox64Concat** + **Twox64Concat**

```solidity
// Params: bytes32 key1, bytes32 key2
uint64 val = uint64(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "RevealPeriodEpochs",
    HASH_TWOX64_CONCAT, ScaleEncode.account(key1),
    HASH_TWOX64_CONCAT, ScaleEncode.account(key2),
    RET_U64
));
```

### `Rho` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "Rho",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `RootAlphaDividendsPerSubnet` — StorageDoubleMap[`NetUid`, `T::AccountId`] → `AlphaCurrency` (uint64)
Hashers: **Identity** + **Blake2_128Concat**

```solidity
// Params: uint16 key1, bytes32 key2
uint64 val = uint64(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "RootAlphaDividendsPerSubnet",
    HASH_IDENTITY, ScaleEncode.u16LE(key1),
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key2),
    RET_U64
));
```

### `RootClaimable` — StorageMap[`T::AccountId`] → `BTreeMap<NetUid` (bytes memory)
Hasher: **Blake2_128Concat** (`HASH_BLAKE2_128_CONCAT`)

```solidity
// Param: bytes32 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "RootClaimable",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(myKey)
);
```

### `RootClaimed` — StorageMap[`T::AccountId`] → `RootClaimTypeEnum` (bytes memory)
Hasher: **Blake2_128Concat** (`HASH_BLAKE2_128_CONCAT`)

```solidity
// Param: bytes32 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "RootClaimed",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(myKey)
);
```

### `RootProp` — StorageMap[`NetUid`] → `U96F32` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "RootProp",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `ScalingLawPower` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "ScalingLawPower",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `ServingRateLimit` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "ServingRateLimit",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `StakeThreshold` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "StakeThreshold",
    HASH_TWOX64_CONCAT, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `StakeWeight` — StorageMap[`NetUid`] → `Vec<u16` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "StakeWeight",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `StakingColdkeys` — StorageMap[`T::AccountId`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: bytes32 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "StakingColdkeys",
    HASH_IDENTITY, ScaleEncode.account(myKey), RET_U64
));
```

### `StakingColdkeysByIndex` — StorageMap[`u64`] → `T::AccountId` (bytes32)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint64 myKey
bytes32 val = bytes32(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "StakingColdkeysByIndex",
    HASH_IDENTITY, ScaleEncode.u64LE(myKey), RET_BYTES32
));
```

### `StakingHotkeys` — StorageMap[`T::AccountId`] → `Vec<T::AccountId` (bytes memory)
Hasher: **Blake2_128Concat** (`HASH_BLAKE2_128_CONCAT`)

```solidity
// Param: bytes32 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "StakingHotkeys",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(myKey)
);
```

### `StakingOperationRateLimiter` — StorageDoubleMap[`NetUid`, `u16`] → `(H160` (bytes memory)
Hashers: **Twox64Concat** + **Twox64Concat**

```solidity
// Params: uint16 key1, uint16 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "StakingOperationRateLimiter",
    HASH_TWOX64_CONCAT, ScaleEncode.u16LE(key1),
    HASH_TWOX64_CONCAT, ScaleEncode.u16LE(key2),
    RET_BYTES
));
```

### `StartCallDelay` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "StartCallDelay", RET_U64));
```

### `SubnetAlphaIn` — StorageMap[`NetUid`] → `AlphaCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "SubnetAlphaIn",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `SubnetAlphaInEmission` — StorageMap[`NetUid`] → `AlphaCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "SubnetAlphaInEmission",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `SubnetAlphaInProvided` — StorageMap[`NetUid`] → `AlphaCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "SubnetAlphaInProvided",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `SubnetAlphaOut` — StorageMap[`NetUid`] → `AlphaCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "SubnetAlphaOut",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `SubnetAlphaOutEmission` — StorageMap[`NetUid`] → `AlphaCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "SubnetAlphaOutEmission",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `SubnetEmaTaoFlow` — StorageMap[`NetUid`] → `(u64` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "SubnetEmaTaoFlow",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `SubnetIdentitiesV3` — StorageMap[`NetUid`] → `SubnetIdentityOfV3` (bytes memory)
Hasher: **Blake2_128Concat** (`HASH_BLAKE2_128_CONCAT`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "SubnetIdentitiesV3",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.u16LE(myKey)
);
```

### `SubnetLeaseShares` — StorageMap[`NetUid`] → `LeaseId` (bytes memory)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "SubnetLeaseShares",
    HASH_TWOX64_CONCAT, ScaleEncode.u16LE(myKey)
);
```

### `SubnetLeases` — StorageDoubleMap[`LeaseId`, `T::AccountId`] → `U64F64` (bytes memory)
Hashers: **Twox64Concat** + **Identity**

```solidity
// Params: bytes memory key1, bytes32 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "SubnetLeases",
    HASH_TWOX64_CONCAT, key1,
    HASH_IDENTITY, ScaleEncode.account(key2),
    RET_BYTES
));
```

### `SubnetLimit` — StorageDoubleMap[`T::AccountId`, `NetUid`] → `T::AccountId` (bytes32)
Hashers: **Blake2_128Concat** + **Identity**

```solidity
// Params: bytes32 key1, uint16 key2
bytes32 val = bytes32(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "SubnetLimit",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key1),
    HASH_IDENTITY, ScaleEncode.u16LE(key2),
    RET_BYTES32
));
```

### `SubnetLocked` — StorageMap[`NetUid`] → `TaoCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "SubnetLocked",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `SubnetMechanism` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "SubnetMechanism",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `SubnetMovingAlpha` — StorageValue → `I96F32` (bytes memory)

```solidity
bytes memory val = SUBTENSOR_STORAGE.getValueRaw("SubtensorModule", "SubnetMovingAlpha");
```

### `SubnetMovingPrice` — StorageValue → `BlockNumberFor<T` (bytes memory)

```solidity
bytes memory val = SUBTENSOR_STORAGE.getValueRaw("SubtensorModule", "SubnetMovingPrice");
```

### `SubnetOwner` — StorageMap[`NetUid`] → `T::AccountId` (bytes32)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes32 val = bytes32(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "SubnetOwner",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_BYTES32
));
```

### `SubnetOwnerCut` — StorageValue → `u16` (uint16)

```solidity
uint16 val = uint16(SUBTENSOR_STORAGE.getValue("SubtensorModule", "SubnetOwnerCut", RET_U16));
```

### `SubnetOwnerHotkey` — StorageMap[`NetUid`] → `T::AccountId` (bytes32)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes32 val = bytes32(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "SubnetOwnerHotkey",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_BYTES32
));
```

### `SubnetTAO` — StorageMap[`NetUid`] → `TaoCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "SubnetTAO",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `SubnetTaoFlow` — StorageMap[`NetUid`] → `i64` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "SubnetTaoFlow",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `SubnetTaoInEmission` — StorageMap[`NetUid`] → `TaoCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "SubnetTaoInEmission",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `SubnetTaoProvided` — StorageMap[`NetUid`] → `TaoCurrency` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "SubnetTaoProvided",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `SubnetVolume` — StorageMap[`NetUid`] → `u128` (uint128)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint128 val = uint128(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "SubnetVolume",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U128
));
```

### `SubnetworkN` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "SubnetworkN",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `SubtokenEnabled` — StorageMap[`NetUid`] → `bool` (bool)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bool val = bool(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "SubtokenEnabled",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_BOOL
));
```

### `TaoFlowCutoff` — StorageMap[`Vec<u8>`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: bytes memory myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "TaoFlowCutoff",
    HASH_IDENTITY, myKey, RET_U64
));
```

### `TaoWeight` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "TaoWeight", RET_U64));
```

### `TargetRegistrationsPerInterval` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "TargetRegistrationsPerInterval",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `Tempo` — StorageMap[`NetUid`] → `u16` (uint16)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint16 val = uint16(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "Tempo",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U16
));
```

### `TimelockedWeightCommits` — StorageDoubleMap[`NetUidStorageIndex`, `u64`] → `// epoch key
        VecDeque<(
            T::AccountId` (bytes memory)
Hashers: **Twox64Concat** + **Twox64Concat**

```solidity
// Params: bytes memory key1, uint64 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "TimelockedWeightCommits",
    HASH_TWOX64_CONCAT, key1,
    HASH_TWOX64_CONCAT, ScaleEncode.u64LE(key2),
    RET_BYTES
));
```

### `TokenSymbol` — StorageValue → `I64F64` (bytes memory)

```solidity
bytes memory val = SUBTENSOR_STORAGE.getValueRaw("SubtensorModule", "TokenSymbol");
```

### `TotalHotkeyAlpha` — StorageMap[`NetUid`] → `Vec<u8` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "TotalHotkeyAlpha",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `TotalHotkeyAlphaLastEpoch` — StorageDoubleMap[`T::AccountId`, `NetUid`] → `AlphaCurrency` (uint64)
Hashers: **Blake2_128Concat** + **Identity**

```solidity
// Params: bytes32 key1, uint16 key2
uint64 val = uint64(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "TotalHotkeyAlphaLastEpoch",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key1),
    HASH_IDENTITY, ScaleEncode.u16LE(key2),
    RET_U64
));
```

### `TotalHotkeyShares` — StorageDoubleMap[`T::AccountId`, `NetUid`] → `U64F64` (bytes memory)
Hashers: **Blake2_128Concat** + **Identity**

```solidity
// Params: bytes32 key1, uint16 key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "TotalHotkeyShares",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key1),
    HASH_IDENTITY, ScaleEncode.u16LE(key2),
    RET_BYTES
));
```

### `TotalIssuance` — StorageValue → `TaoCurrency` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "TotalIssuance", RET_U64));
```

### `TotalNetworks` — StorageMap[`RateLimitKey<T::AccountId>`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: bytes memory myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "TotalNetworks",
    HASH_IDENTITY, myKey, RET_U64
));
```

### `TotalStake` — StorageValue → `TaoCurrency` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "TotalStake", RET_U64));
```

### `TransactionKeyLastBlock` — StorageMap[`T::AccountId`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: bytes32 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "TransactionKeyLastBlock",
    HASH_IDENTITY, ScaleEncode.account(myKey), RET_U64
));
```

### `TransferToggle` — StorageMap[`NetUid`] → `bool` (bool)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bool val = bool(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "TransferToggle",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_BOOL
));
```

### `Trust` — StorageMap[`NetUid`] → `Vec<u16` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "Trust",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `TxChildkeyTakeRateLimit` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "TxChildkeyTakeRateLimit", RET_U64));
```

### `TxDelegateTakeRateLimit` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "TxDelegateTakeRateLimit", RET_U64));
```

### `TxRateLimit` — StorageMap[`NetUid`] → `bool` (bool)
Hasher: **Blake2_128Concat** (`HASH_BLAKE2_128_CONCAT`)

```solidity
// Param: uint16 myKey
bool val = bool(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "TxRateLimit",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.u16LE(myKey), RET_BOOL
));
```

### `Uids` — StorageMap[`NetUid`] → `Vec<(T::AccountId` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "Uids",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `UsedWork` — StorageValue → `u16` (uint16)

```solidity
uint16 val = uint16(SUBTENSOR_STORAGE.getValue("SubtensorModule", "UsedWork", RET_U16));
```

### `ValidatorPermit` — StorageMap[`NetUid`] → `Vec<bool` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "ValidatorPermit",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `ValidatorPruneLen` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "ValidatorPruneLen",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `ValidatorTrust` — StorageMap[`NetUid`] → `Vec<u16` (bytes memory)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "ValidatorTrust",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey)
);
```

### `VotingPower` — StorageMap[`NetUid`] → `bool` (bool)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
bool val = bool(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "VotingPower",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_BOOL
));
```

### `VotingPowerDisableAtBlock` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "VotingPowerDisableAtBlock",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `VotingPowerEmaAlpha` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "VotingPowerEmaAlpha",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `VotingPowerTrackingEnabled` — StorageDoubleMap[`NetUid`, `T::AccountId`] → `u16` (uint16)
Hashers: **Identity** + **Blake2_128Concat**

```solidity
// Params: uint16 key1, bytes32 key2
uint16 val = uint16(SUBTENSOR_STORAGE.getDoubleMap(
    "SubtensorModule", "VotingPowerTrackingEnabled",
    HASH_IDENTITY, ScaleEncode.u16LE(key1),
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(key2),
    RET_U16
));
```

### `WeightCommits` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "WeightCommits", RET_U64));
```

### `Weights` — StorageMap[`T::AccountId`] → `ChainIdentityOfV2` (bytes memory)
Hasher: **Blake2_128Concat** (`HASH_BLAKE2_128_CONCAT`)

```solidity
// Param: bytes32 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "SubtensorModule", "Weights",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.account(myKey)
);
```

### `WeightsSetRateLimit` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "WeightsSetRateLimit",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `WeightsVersionKey` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Identity** (`HASH_IDENTITY`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "WeightsVersionKey",
    HASH_IDENTITY, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `WeightsVersionKeyRateLimit` — StorageValue → `u64` (uint64)

```solidity
uint64 val = uint64(SUBTENSOR_STORAGE.getValue("SubtensorModule", "WeightsVersionKeyRateLimit", RET_U64));
```

### `Yuma3On` — StorageMap[`NetUid`] → `bool` (bool)
Hasher: **Blake2_128Concat** (`HASH_BLAKE2_128_CONCAT`)

```solidity
// Param: uint16 myKey
bool val = bool(SUBTENSOR_STORAGE.getMap(
    "SubtensorModule", "Yuma3On",
    HASH_BLAKE2_128_CONCAT, ScaleEncode.u16LE(myKey), RET_BOOL
));
```


## `Swap`

### `AlphaSqrtPrice` — StorageMap[`NetUid`] → `U64F64` (bytes memory)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "Swap", "AlphaSqrtPrice",
    HASH_TWOX64_CONCAT, ScaleEncode.u16LE(myKey)
);
```

### `CurrentLiquidity` — StorageMap[`NetUid`] → `u64` (uint64)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "Swap", "CurrentLiquidity",
    HASH_TWOX64_CONCAT, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `CurrentTick` — StorageMap[`NetUid`] → `TickIndex` (bytes memory)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "Swap", "CurrentTick",
    HASH_TWOX64_CONCAT, ScaleEncode.u16LE(myKey)
);
```

### `EnabledUserLiquidity` — StorageMap[`NetUid`] → `bool` (bool)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: uint16 myKey
bool val = bool(SUBTENSOR_STORAGE.getMap(
    "Swap", "EnabledUserLiquidity",
    HASH_TWOX64_CONCAT, ScaleEncode.u16LE(myKey), RET_BOOL
));
```

### `FeeGlobalAlpha` — StorageMap[`NetUid`] → `U64F64` (bytes memory)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "Swap", "FeeGlobalAlpha",
    HASH_TWOX64_CONCAT, ScaleEncode.u16LE(myKey)
);
```

### `FeeGlobalTao` — StorageMap[`NetUid`] → `U64F64` (bytes memory)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: uint16 myKey
bytes memory val = SUBTENSOR_STORAGE.getMapRaw(
    "Swap", "FeeGlobalTao",
    HASH_TWOX64_CONCAT, ScaleEncode.u16LE(myKey)
);
```

### `FeeRate` — StorageDoubleMap[`NetUid`, `TickIndex`] → `Tick` (bytes memory)
Hashers: **Twox64Concat** + **Twox64Concat**

```solidity
// Params: uint16 key1, bytes memory key2
bytes memory val = bytes memory(SUBTENSOR_STORAGE.getDoubleMap(
    "Swap", "FeeRate",
    HASH_TWOX64_CONCAT, ScaleEncode.u16LE(key1),
    HASH_TWOX64_CONCAT, key2,
    RET_BYTES
));
```

### `Positions` — StorageMap[`NetUid`] → `TaoCurrency` (uint64)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "Swap", "Positions",
    HASH_TWOX64_CONCAT, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `ScrapReservoirAlpha` — StorageMap[`NetUid`] → `AlphaCurrency` (uint64)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: uint16 myKey
uint64 val = uint64(SUBTENSOR_STORAGE.getMap(
    "Swap", "ScrapReservoirAlpha",
    HASH_TWOX64_CONCAT, ScaleEncode.u16LE(myKey), RET_U64
));
```

### `Ticks` — StorageMap[`NetUid`] → `bool` (bool)
Hasher: **Twox64Concat** (`HASH_TWOX64_CONCAT`)

```solidity
// Param: uint16 myKey
bool val = bool(SUBTENSOR_STORAGE.getMap(
    "Swap", "Ticks",
    HASH_TWOX64_CONCAT, ScaleEncode.u16LE(myKey), RET_BOOL
));
```


---

**Total:** 216 storage items (49 Values, 134 Maps, 33 DoubleMaps)
