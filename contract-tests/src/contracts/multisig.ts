export const IMULTISIG_ADDRESS = "0x0000000000000000000000000000000000000815";
export const IMultisigABI = [
    {
        "type": "function",
        "name": "getMultisig",
        "inputs": [
            { "name": "account", "type": "bytes32", "internalType": "bytes32" },
            { "name": "call_hash", "type": "bytes32", "internalType": "bytes32" }
        ],
        "outputs": [
            { "name": "", "type": "uint64", "internalType": "uint64" },
            { "name": "", "type": "bytes32[]", "internalType": "bytes32[]" },
            { "name": "", "type": "uint16", "internalType": "uint16" }
        ],
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
