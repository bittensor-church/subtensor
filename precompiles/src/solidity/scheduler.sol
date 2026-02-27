pragma solidity ^0.8.0;

address constant ISCHEDULER_ADDRESS = 0x0000000000000000000000000000000000000813;

/**
 * @title IScheduler
 * @dev Precompile at 0x813 providing typed views into Scheduler pallet storage.
 */
interface IScheduler {
    /// @dev Returns the active scheduler lookup key.
    function getLookup(bytes32 name) external view returns (uint64 blockNumber, uint32 taskIndex);

    /// @dev Returns the block number at which the agenda began incomplete execution.
    function getIncompleteSince() external view returns (uint64);

    /// @dev Returns the current pallet version from storage.
    function getPalletVersion() external view returns (uint16);
}
