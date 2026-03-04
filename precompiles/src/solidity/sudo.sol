pragma solidity ^0.8.0;

address constant ISUDO_ADDRESS = 0x0000000000000000000000000000000000000814;

/**
 * @title ISudo
 * @dev Precompile at 0x814 providing typed views into Sudo pallet storage.
 */
interface ISudo {
    /// @dev Returns the active sudo key.
    function getKey() external view returns (bytes32);

    /// @dev Returns the current pallet version from storage.
    function getPalletVersion() external view returns (uint16);
}
