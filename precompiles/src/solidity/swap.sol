// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

/// @dev Interface for the Swap precompile.
/// Address: 0x0000000000000000000000000000000000000816 (2070)
interface ISwap {
    /// @notice Returns the fee rate for a given subnet.
    function getFeeRate(uint16 netuid) external view returns (uint16);

    /// @notice Returns the global accrued fees in tao per subnet.
    function getFeeGlobalTao(uint16 netuid) external view returns (uint256);

    /// @notice Returns the global accrued fees in alpha per subnet.
    function getFeeGlobalAlpha(uint16 netuid) external view returns (uint256);

    /// @notice Returns tick information.
    function getTicks(uint16 netuid, int32 tick_index) external view returns (
        int256 liquidityNet,
        uint256 liquidityGross,
        int256 feesOutTao,
        int256 feesOutAlpha
    );

    /// @notice Returns whether Swap V3 is initialized for a specific subnet.
    function getSwapV3Initialized(uint16 netuid) external view returns (bool);

    /// @notice Returns the square root price of Alpha token for each subnet.
    function getAlphaSqrtPrice(uint16 netuid) external view returns (uint256);

    /// @notice Returns the current price tick.
    function getCurrentTick(uint16 netuid) external view returns (int256);

    /// @notice Returns the current liquidity amount for each subnet.
    function getCurrentLiquidity(uint16 netuid) external view returns (uint64);

    /// @notice Indicates whether a subnet has been switched to V3 swap from V2.
    function getEnabledUserLiquidity(uint16 netuid) external view returns (bool);

    /// @notice Returns user position details.
    function getPositions(uint16 netuid, bytes32 account, uint256 position_id) external view returns (
        uint256 id,
        uint16 n,
        int256 tickLow,
        int256 tickHigh,
        uint64 liquidity,
        int256 feesTao,
        int256 feesAlpha
    );

    /// @notice Returns the last position ID.
    function getLastPositionId() external view returns (uint256);

    /// @notice Returns the tick index bitmap words.
    function getTickIndexBitmapWords(uint16 netuid, uint8 layer_idx, uint32 word_index) external view returns (uint256);

    /// @notice Returns the scrap reservoir tao for a given subnet.
    function getScrapReservoirTao(uint16 netuid) external view returns (uint64);

    /// @notice Returns the scrap reservoir alpha for a given subnet.
    function getScrapReservoirAlpha(uint16 netuid) external view returns (uint64);

    /// @notice Returns the pallet version.
    function getPalletVersion() external view returns (uint16);
}
