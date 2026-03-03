export const IDRAND_ADDRESS = "0x0000000000000000000000000000000000000811";
export const IDrandABI = [
    {
        "type": "function",
        "name": "getLastStoredRound",
        "inputs": [],
        "outputs": [{ "name": "", "type": "uint64", "internalType": "uint64" }],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getOldestStoredRound",
        "inputs": [],
        "outputs": [{ "name": "", "type": "uint64", "internalType": "uint64" }],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getPulse",
        "inputs": [{ "name": "round", "type": "uint64", "internalType": "uint64" }],
        "outputs": [
            { "name": "", "type": "uint64", "internalType": "uint64" },
            { "name": "", "type": "bytes", "internalType": "bytes" },
            { "name": "", "type": "bytes", "internalType": "bytes" }
        ],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getCurrentRandomness",
        "inputs": [],
        "outputs": [{ "name": "", "type": "bytes", "internalType": "bytes" }],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getBeaconConfig",
        "inputs": [],
        "outputs": [
            { "name": "", "type": "bytes", "internalType": "bytes" },
            { "name": "", "type": "bytes", "internalType": "bytes" },
            { "name": "", "type": "uint8", "internalType": "uint8" },
            { "name": "", "type": "uint64", "internalType": "uint64" },
            { "name": "", "type": "uint64", "internalType": "uint64" }
        ],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getHasMigrationRun",
        "inputs": [{ "name": "key", "type": "bytes32", "internalType": "bytes32" }],
        "outputs": [{ "name": "", "type": "bool", "internalType": "bool" }],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getNextUnsignedAt",
        "inputs": [],
        "outputs": [{ "name": "", "type": "uint64", "internalType": "uint64" }],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getPalletVersion",
        "inputs": [],
        "outputs": [{ "name": "", "type": "uint16", "internalType": "uint16" }],
        "stateMutability": "view"
    }
];
