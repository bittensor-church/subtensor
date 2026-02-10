#!/usr/bin/env python3
"""
Extract all storage items from Subtensor pallets and generate a reference
with ready-to-use Solidity code for StorageMapQueryPrecompile.
"""

import os
import re
import json
from pathlib import Path

# ==================== Type Mappings ====================

HASHER_TO_ID = {
    "Identity": 0,
    "Twox64Concat": 1,
    "Blake2_128Concat": 2,
    "Twox128": 3,
    "Blake2_256": 4,
}

HASHER_TO_CONST = {
    "Identity": "HASH_IDENTITY",
    "Twox64Concat": "HASH_TWOX64_CONCAT",
    "Blake2_128Concat": "HASH_BLAKE2_128_CONCAT",
    "Twox128": "HASH_TWOX128",
    "Blake2_256": "HASH_BLAKE2_256",
}

# Map Rust types to ReturnType ID and Solidity type
RUST_TYPE_TO_RETURN = {
    "u8": (1, "RET_U8", "uint8"),
    "u16": (2, "RET_U16", "uint16"),
    "u32": (3, "RET_U32", "uint32"),
    "u64": (4, "RET_U64", "uint64"),
    "u128": (5, "RET_U128", "uint128"),
    "bool": (6, "RET_BOOL", "bool"),
    "T::AccountId": (7, "RET_BYTES32", "bytes32"),
    "AccountIdOf<T>": (7, "RET_BYTES32", "bytes32"),
}

# Map Rust key types to ScaleEncode helper
RUST_KEY_TO_ENCODE = {
    "u8": ("uint8 {name}", "abi.encodePacked(uint8({name}))"),
    "u16": ("uint16 {name}", "ScaleEncode.u16LE({name})"),
    "u32": ("uint32 {name}", "ScaleEncode.u32LE({name})"),
    "u64": ("uint64 {name}", "ScaleEncode.u64LE({name})"),
    "u128": ("uint128 {name}", "ScaleEncode.u64LE(uint64({name}))"),  # simplified
    "bool": ("bool {name}", "abi.encodePacked({name} ? uint8(1) : uint8(0))"),
    "T::AccountId": ("bytes32 {name}", "ScaleEncode.account({name})"),
    "AccountIdOf<T>": ("bytes32 {name}", "ScaleEncode.account({name})"),
}

# Common Substrate type aliases
TYPE_ALIASES = {
    "NetUid": "u16",
    "TaoCurrency": "u64",
    "AlphaCurrency": "u64",
    "Currency": "u64",
    "BalanceOf<T>": "u128",
    "Balance": "u64",
    "BlockNumberFor<T>": "u32",
    "T::BlockNumber": "u32",
    "Compact<u64>": "u64",
    "Compact<u128>": "u128",
}


def normalize_type(rust_type: str) -> str:
    """Normalize Rust type to a base type we understand."""
    t = rust_type.strip().rstrip(",>")
    # Check aliases
    if t in TYPE_ALIASES:
        return TYPE_ALIASES[t]
    # Check if it's a direct match
    if t in RUST_TYPE_TO_RETURN:
        return t
    if t in RUST_KEY_TO_ENCODE:
        return t
    # Try stripping generics
    base = re.sub(r'<.*>', '', t).strip()
    if base in TYPE_ALIASES:
        return TYPE_ALIASES[base]
    if base in RUST_TYPE_TO_RETURN:
        return base
    return t


def get_return_info(rust_type: str):
    """Get (ret_id, ret_const, sol_type) for a Rust value type."""
    t = normalize_type(rust_type)
    if t in RUST_TYPE_TO_RETURN:
        return RUST_TYPE_TO_RETURN[t]
    # Default: raw bytes
    return (0, "RET_BYTES", "bytes memory")


def get_key_encode(rust_type: str, var_name: str) -> tuple:
    """Get (sol_param, encode_call) for a Rust key type."""
    t = normalize_type(rust_type)
    if t in RUST_KEY_TO_ENCODE:
        param_tpl, encode_tpl = RUST_KEY_TO_ENCODE[t]
        return (param_tpl.format(name=var_name), encode_tpl.format(name=var_name))
    return (f"bytes memory {var_name}", var_name)


def extract_storage_items(base_path: str) -> list:
    """Extract storage definitions from Rust pallet files."""
    items = []
    pallets_dir = Path(base_path) / "pallets"

    storage_value_pattern = re.compile(
        r'pub type (\w+)<.*?>\s*=\s*StorageValue<.*?,\s*([^,>]+)',
        re.DOTALL
    )
    storage_map_pattern = re.compile(
        r'pub type (\w+)<.*?>\s*=\s*StorageMap<\s*_,\s*(\w+),\s*([^,]+),\s*([^,>]+)',
        re.DOTALL
    )
    storage_double_map_pattern = re.compile(
        r'pub type (\w+)<.*?>\s*=\s*StorageDoubleMap<\s*_,\s*(\w+),\s*([^,]+),\s*(\w+),\s*([^,]+),\s*([^,>]+)',
        re.DOTALL
    )

    # Detect pallet name from #[pallet::pallet] struct name or module name
    pallet_name_pattern = re.compile(r'#\[pallet::call\].*?impl.*?Pallet', re.DOTALL)

    for pallet_dir in sorted(pallets_dir.iterdir()):
        if not pallet_dir.is_dir():
            continue

        pallet_name = pallet_dir.name
        src_dir = pallet_dir / "src"
        if not src_dir.exists():
            continue

        # Try to find the real runtime pallet name from macro
        runtime_name = get_runtime_pallet_name(pallet_name, src_dir)

        for rs_file in src_dir.rglob("*.rs"):
            try:
                content = rs_file.read_text()
            except:
                continue

            # Extract StorageDoubleMap
            for match in storage_double_map_pattern.finditer(content):
                name, hasher1, key1, hasher2, key2, value = match.groups()
                if any(i["name"] == name.strip() and i["pallet_dir"] == pallet_name for i in items):
                    continue
                items.append({
                    "pallet_dir": pallet_name,
                    "pallet": runtime_name,
                    "name": name.strip(),
                    "type": "DoubleMap",
                    "hasher1": hasher1.strip(),
                    "key1": key1.strip(),
                    "hasher2": hasher2.strip(),
                    "key2": key2.strip(),
                    "value": value.strip().rstrip(',>'),
                    "file": str(rs_file.relative_to(base_path))
                })

            # Extract StorageMap
            for match in storage_map_pattern.finditer(content):
                name, hasher, key, value = match.groups()
                if any(i["name"] == name.strip() and i["pallet_dir"] == pallet_name for i in items):
                    continue
                items.append({
                    "pallet_dir": pallet_name,
                    "pallet": runtime_name,
                    "name": name.strip(),
                    "type": "Map",
                    "hasher": hasher.strip(),
                    "key": key.strip(),
                    "value": value.strip().rstrip(',>'),
                    "file": str(rs_file.relative_to(base_path))
                })

            # Extract StorageValue
            for match in storage_value_pattern.finditer(content):
                name, value = match.groups()
                if any(i["name"] == name.strip() and i["pallet_dir"] == pallet_name for i in items):
                    continue
                items.append({
                    "pallet_dir": pallet_name,
                    "pallet": runtime_name,
                    "name": name.strip(),
                    "type": "Value",
                    "value": value.strip().rstrip(',>'),
                    "file": str(rs_file.relative_to(base_path))
                })

    return items


def get_runtime_pallet_name(dir_name: str, src_dir: Path) -> str:
    """Try to detect the runtime pallet name used in storage keys."""
    # Known mappings
    known = {
        "subtensor": "SubtensorModule",
        "drand": "Drand",
        "admin-utils": "AdminUtils",
        "registry": "Registry",
        "commitments": "Commitments",
    }
    if dir_name in known:
        return known[dir_name]
    # Try to infer from directory name
    return "".join(word.capitalize() for word in dir_name.replace("-", "_").split("_"))


def gen_solidity_example(item: dict) -> str:
    """Generate a ready-to-use Solidity code snippet for a storage item."""
    pallet = item["pallet"]
    name = item["name"]

    if item["type"] == "Value":
        ret_id, ret_const, sol_type = get_return_info(item["value"])
        if ret_const == "RET_BYTES":
            return (
                f'bytes memory val = SUBTENSOR_STORAGE.getValueRaw("{pallet}", "{name}");\n'
            )
        return (
            f'{sol_type} val = {sol_type}(SUBTENSOR_STORAGE.getValue("{pallet}", "{name}", {ret_const}));\n'
        )

    elif item["type"] == "Map":
        hasher = item.get("hasher", "Identity")
        hasher_const = HASHER_TO_CONST.get(hasher, "HASH_IDENTITY")
        ret_id, ret_const, sol_type = get_return_info(item["value"])
        key_param, key_encode = get_key_encode(item.get("key", ""), "myKey")

        if ret_const == "RET_BYTES":
            return (
                f'// Param: {key_param}\n'
                f'bytes memory val = SUBTENSOR_STORAGE.getMapRaw(\n'
                f'    "{pallet}", "{name}",\n'
                f'    {hasher_const}, {key_encode}\n'
                f');\n'
            )
        return (
            f'// Param: {key_param}\n'
            f'{sol_type} val = {sol_type}(SUBTENSOR_STORAGE.getMap(\n'
            f'    "{pallet}", "{name}",\n'
            f'    {hasher_const}, {key_encode}, {ret_const}\n'
            f'));\n'
        )

    elif item["type"] == "DoubleMap":
        hasher1 = item.get("hasher1", "Identity")
        hasher2 = item.get("hasher2", "Identity")
        hasher1_const = HASHER_TO_CONST.get(hasher1, "HASH_IDENTITY")
        hasher2_const = HASHER_TO_CONST.get(hasher2, "HASH_IDENTITY")
        ret_id, ret_const, sol_type = get_return_info(item["value"])
        key1_param, key1_encode = get_key_encode(item.get("key1", ""), "key1")
        key2_param, key2_encode = get_key_encode(item.get("key2", ""), "key2")

        return (
            f'// Params: {key1_param}, {key2_param}\n'
            f'{sol_type} val = {sol_type}(SUBTENSOR_STORAGE.getDoubleMap(\n'
            f'    "{pallet}", "{name}",\n'
            f'    {hasher1_const}, {key1_encode},\n'
            f'    {hasher2_const}, {key2_encode},\n'
            f'    {ret_const}\n'
            f'));\n'
        )

    return "// Unknown storage type\n"


def main():
    base_path = os.environ.get("SUBTENSOR_PATH", "/Users/pablo_vm/RustroverProjects/subtensor_our")

    items = extract_storage_items(base_path)
    items.sort(key=lambda x: (x["pallet"], x["name"]))

    # ==================== Generate Markdown ====================
    output = """# Subtensor Storage Items - Precompile Reference

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

"""

    current_pallet = None
    counts = {"Value": 0, "Map": 0, "DoubleMap": 0}

    for item in items:
        counts[item["type"]] = counts.get(item["type"], 0) + 1

        if item["pallet"] != current_pallet:
            current_pallet = item["pallet"]
            output += f"\n## `{current_pallet}`\n\n"

        sol_example = gen_solidity_example(item)

        if item["type"] == "Value":
            ret_id, ret_const, sol_type = get_return_info(item["value"])
            output += f"### `{item['name']}` — StorageValue → `{item['value']}` ({sol_type})\n\n"
            output += f"```solidity\n{sol_example}```\n\n"

        elif item["type"] == "Map":
            hasher = item.get("hasher", "?")
            ret_id, ret_const, sol_type = get_return_info(item["value"])
            output += f"### `{item['name']}` — StorageMap[`{item.get('key','?')}`] → `{item['value']}` ({sol_type})\n"
            output += f"Hasher: **{hasher}** (`{HASHER_TO_CONST.get(hasher, '?')}`)\n\n"
            output += f"```solidity\n{sol_example}```\n\n"

        elif item["type"] == "DoubleMap":
            h1, h2 = item.get("hasher1", "?"), item.get("hasher2", "?")
            ret_id, ret_const, sol_type = get_return_info(item["value"])
            output += f"### `{item['name']}` — StorageDoubleMap[`{item.get('key1','?')}`, `{item.get('key2','?')}`] → `{item['value']}` ({sol_type})\n"
            output += f"Hashers: **{h1}** + **{h2}**\n\n"
            output += f"```solidity\n{sol_example}```\n\n"

    # Summary
    output += f"\n---\n\n**Total:** {len(items)} storage items "
    output += f"({counts.get('Value',0)} Values, {counts.get('Map',0)} Maps, {counts.get('DoubleMap',0)} DoubleMaps)\n"

    # Write
    output_path = Path(base_path) / "precompiles" / "src" / "solidity" / "STORAGE_ITEMS_REFERENCE.md"
    output_path.parent.mkdir(exist_ok=True)
    output_path.write_text(output)
    print(f"✅ Generated {len(items)} storage items → {output_path}")

    # JSON
    json_path = Path(base_path) / "precompiles" / "src" / "solidity" / "storage_items.json"
    json_path.write_text(json.dumps(items, indent=2))
    print(f"✅ JSON output → {json_path}")


if __name__ == "__main__":
    main()
