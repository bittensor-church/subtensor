export const ITIMESTAMP_ADDRESS = "0x0000000000000000000000000000000000000812";
export const ITimestampABI = [
    {
        "type": "function",
        "name": "getNow",
        "inputs": [],
        "outputs": [{ "name": "", "type": "uint64", "internalType": "uint64" }],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "getDidUpdate",
        "inputs": [],
        "outputs": [{ "name": "", "type": "bool", "internalType": "bool" }],
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
