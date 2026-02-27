pragma solidity ^0.8.0;

address constant ISUBTENSOR_BALANCE_TRANSFER_ADDRESS = 0x0000000000000000000000000000000000000800;

interface ISubtensorBalanceTransfer {
    function transfer(bytes32 data) external payable;

    /// @dev Returns the total issuance of the native token.
    function getTotalIssuance() external view returns (uint64);

    /// @dev Returns the free balance of an account.
    /// @param account The account ID (bytes32).
    function getFreeBalance(bytes32 account) external view returns (uint64);

    /// @dev Returns the current pallet version from storage.
    function getPalletVersion() external view returns (uint16);

    /// @dev Returns the total units of outstanding deactivated balance in the system.
    function getInactiveIssuance() external view returns (uint64);

    /// @dev Returns the balance of an account. Returns (free, reserved, frozen, flags).
    function getAccount(bytes32 account) external view returns (uint64, uint64, uint64, uint64);

    /// @dev Returns any liquidity locks on some account balances. Returns array of (id, amount, reasons).
    function getLocks(bytes32 account) external view returns (bytes32[] memory, uint64[] memory, uint8[] memory);

    /// @dev Returns named reserves on some account balances. Returns array of (id, amount).
    function getReserves(bytes32 account) external view returns (bytes32[] memory, uint64[] memory);

    /// @dev Returns holds on account balances. Returns array of (id, amount).
    function getHolds(bytes32 account) external view returns (bytes32[] memory, uint64[] memory);

    /// @dev Returns freeze locks on account balances. Returns array of (id, amount).
    function getFreezes(bytes32 account) external view returns (bytes32[] memory, uint64[] memory);
}