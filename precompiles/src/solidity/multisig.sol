pragma solidity ^0.8.0;

address constant IMULTISIG_ADDRESS = 0x0000000000000000000000000000000000000815;

/**
 * @title IMultisig
 * @dev Precompile at 0x815 providing typed views into Multisig pallet storage.
 */
interface IMultisig {
    /// @dev Returns multisig info for a given account and call hash.
    function getMultisig(bytes32 account, bytes32 callHash) external view returns (uint64 deposit, bytes32[] memory approvals, uint16 approvalsCount);

    /// @dev Returns the current pallet version from storage.
    function getPalletVersion() external view returns (uint16);
}
