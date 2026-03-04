export const IBALANCE_TRANSFER_ADDRESS = "0x0000000000000000000000000000000000000800";
export const IBalanceTransferABI = [
    {
        "type": "function",
        "name": "getPalletVersion",
        "inputs": [],
        "outputs": [{ "name": "", "type": "uint16", "internalType": "uint16" }],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getInactiveIssuance",
        "inputs": [],
        "outputs": [{ "name": "", "type": "uint64", "internalType": "uint64" }],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getTotalIssuance",
        "inputs": [],
        "outputs": [{ "name": "", "type": "uint64", "internalType": "uint64" }],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getFreeBalance",
        "inputs": [{ "name": "account", "type": "bytes32", "internalType": "bytes32" }],
        "outputs": [{ "name": "", "type": "uint64", "internalType": "uint64" }],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getAccount",
        "inputs": [{ "name": "account", "type": "bytes32", "internalType": "bytes32" }],
        "outputs": [
            { "name": "free", "type": "uint64", "internalType": "uint64" },
            { "name": "reserved", "type": "uint64", "internalType": "uint64" },
            { "name": "frozen", "type": "uint64", "internalType": "uint64" },
            { "name": "flags", "type": "uint64", "internalType": "uint64" }
        ],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getLocks",
        "inputs": [{ "name": "account", "type": "bytes32", "internalType": "bytes32" }],
        "outputs": [
            {
                "name": "",
                "type": "tuple[]",
                "internalType": "struct BalanceLock[]",
                "components": [
                    { "name": "id", "type": "bytes32", "internalType": "bytes32" },
                    { "name": "amount", "type": "uint64", "internalType": "uint64" },
                    { "name": "reasons", "type": "uint8", "internalType": "uint8" }
                ]
            }
        ],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getReserves",
        "inputs": [{ "name": "account", "type": "bytes32", "internalType": "bytes32" }],
        "outputs": [
            {
                "name": "",
                "type": "tuple[]",
                "internalType": "struct ReserveData[]",
                "components": [
                    { "name": "id", "type": "bytes32", "internalType": "bytes32" },
                    { "name": "amount", "type": "uint64", "internalType": "uint64" }
                ]
            }
        ],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getHolds",
        "inputs": [{ "name": "account", "type": "bytes32", "internalType": "bytes32" }],
        "outputs": [
            {
                "name": "",
                "type": "tuple[]",
                "internalType": "struct IdAmount[]",
                "components": [
                    { "name": "id", "type": "bytes32", "internalType": "bytes32" },
                    { "name": "amount", "type": "uint64", "internalType": "uint64" }
                ]
            }
        ],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getFreezes",
        "inputs": [{ "name": "account", "type": "bytes32", "internalType": "bytes32" }],
        "outputs": [
            {
                "name": "",
                "type": "tuple[]",
                "internalType": "struct IdAmount[]",
                "components": [
                    { "name": "id", "type": "bytes32", "internalType": "bytes32" },
                    { "name": "amount", "type": "uint64", "internalType": "uint64" }
                ]
            }
        ],
        "stateMutability": "view"
    }
];
