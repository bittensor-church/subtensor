export const ISCHEDULER_ADDRESS = "0x0000000000000000000000000000000000000813";
export const ISchedulerABI = [
    {
        "type": "function",
        "name": "getLookup",
        "inputs": [{ "name": "name", "type": "bytes32", "internalType": "bytes32" }],
        "outputs": [
            { "name": "", "type": "uint64", "internalType": "uint64" },
            { "name": "", "type": "uint32", "internalType": "uint32" }
        ],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getIncompleteSince",
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
