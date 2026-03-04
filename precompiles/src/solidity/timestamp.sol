pragma solidity ^0.8.0;

address constant ITIMESTAMP_ADDRESS = 0x0000000000000000000000000000000000000812;

/**
 * @title ITimestamp
 * @dev Precompile at 0x812 providing the current chain timestamp.
 */
interface ITimestamp {
    /// @dev Returns the current chain timestamp in milliseconds.
    function getNow() external view returns (uint64);

    /// @dev Returns whether the timestamp has been updated in this block.
    function getDidUpdate() external view returns (bool);

    /// @dev Returns the current pallet version from storage.
    function getPalletVersion() external view returns (uint16);
}
