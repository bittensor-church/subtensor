// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

/// @dev Interface for the Subtensor precompile.
/// Address: 0x0000000000000000000000000000000000000817 (2071)
interface ISubtensor {
    /// @notice Returns the pallet version.
    function getPalletVersion() external view returns (uint16);

    function getMinActivityCutoff() external view returns (uint16);

    function getAdminFreezeWindow() external view returns (uint16);

    function getOwnerHyperparamRateLimit() external view returns (uint16);

    function getColdkeySwapScheduleDuration() external view returns (uint256);

    function getColdkeySwapRescheduleDuration() external view returns (uint256);

    function getDissolveNetworkScheduleDuration() external view returns (uint256);

    function getLastHotkeySwapOnNetuid(uint16 key1, bytes32 key2) external view returns (uint256);

    function getNextStakeJobId() external view returns (uint256);

    function getMaxDelegateTake() external view returns (uint16);

    function getMinDelegateTake() external view returns (uint16);

    function getMaxChildkeyTake() external view returns (uint16);

    function getMinChildkeyTake() external view returns (uint16);

    function getDelegates(bytes32 key1) external view returns (uint16);

    function getChildkeyTake(bytes32 key1, uint16 key2) external view returns (bytes);

    function getPendingChildKeys(uint16 key1, bytes32 key2) external view returns (bytes);

    function getChildKeys(bytes32 key1, uint16 key2) external view returns (bytes);

    function getParentKeys(bytes32 key1, uint16 key2) external view returns (bytes);

    function getAlphaDividendsPerSubnet(uint16 key1, bytes32 key2) external view returns (uint256);

    function getRootAlphaDividendsPerSubnet(uint16 key1, bytes32 key2) external view returns (uint256);

    function getBlockEmission() external view returns (uint256);

    function getLastHotkeyEmissionOnNetuid(bytes32 key1, uint16 key2) external view returns (uint256);

    function getSubnetLimit() external view returns (uint16);

    function getTotalIssuance() external view returns (uint256);

    function getTotalStake() external view returns (uint256);

    function getSubnetMovingAlpha() external view returns (uint256);

    function getSubnetMovingPrice(uint16 key1) external view returns (uint256);

    function getRootProp(uint16 key1) external view returns (uint256);

    function getSubnetTaoProvided(uint16 key1) external view returns (uint256);

    function getSubnetAlphaInProvided(uint16 key1) external view returns (uint256);

    function getStakingHotkeys(bytes32 key1) external view returns (bytes);

    function getOwnedHotkeys(bytes32 key1) external view returns (bytes);

    function getAutoStakeDestination(bytes32 key1, uint16 key2) external view returns (bytes32);

    function getAutoStakeDestinationColdkeys(bytes32 key1, uint16 key2) external view returns (bytes);

    function getColdkeySwapScheduled(bytes32 key1) external view returns (bytes);

    function getTotalHotkeyAlpha(bytes32 key1, uint16 key2) external view returns (uint256);

    function getTotalHotkeyAlphaLastEpoch(bytes32 key1, uint16 key2) external view returns (uint256);

    function getTotalHotkeyShares(bytes32 key1, uint16 key2) external view returns (bytes);

    function getAlphaMapLastKey() external view returns (bytes);

    function getTokenSymbol(uint16 key1) external view returns (bytes);

    function getSubnetTaoFlow(uint16 key1) external view returns (bytes);

    function getSubnetEmaTaoFlow(uint16 key1) external view returns (bytes);

    function getTaoFlowCutoff() external view returns (bytes);

    function getFlowNormExponent() external view returns (bytes);

    function getFlowEmaSmoothingFactor() external view returns (uint256);

    function getUsedWork(bytes key1) external view returns (uint256);

    function getMaxRegistrationsPerBlock(uint16 key1) external view returns (uint16);

    function getTotalNetworks() external view returns (uint16);

    function getNetworkImmunityPeriod() external view returns (uint256);

    function getStartCallDelay() external view returns (uint256);

    function getNetworkMinLockCost() external view returns (uint256);

    function getNetworkLastLockCost() external view returns (uint256);

    function getNetworkLockReductionInterval() external view returns (uint256);

    function getSubnetOwnerCut() external view returns (uint16);

    function getNetworkRateLimit() external view returns (uint256);

    function getNominatorMinRequiredStake() external view returns (uint256);

    function getWeightsVersionKeyRateLimit() external view returns (uint256);

    function getLastRateLimitedBlock(bytes32 key1) external view returns (uint256);

    function getTransferToggle(uint16 key1) external view returns (bool);

    function getSubnetLocked(uint16 key1) external view returns (uint256);

    function getLargestLocked(uint16 key1) external view returns (uint256);

    function getTempo(uint16 key1) external view returns (uint16);

    function getFirstEmissionBlockNumber(uint16 key1) external view returns (uint256);

    function getIsNetworkMember(bytes32 key1, uint16 key2) external view returns (bool);

    function getNetworkRegisteredAt(uint16 key1) external view returns (uint256);

    function getPendingServerEmission(uint16 key1) external view returns (uint256);

    function getPendingValidatorEmission(uint16 key1) external view returns (uint256);

    function getPendingRootAlphaDivs(uint16 key1) external view returns (uint256);

    function getPendingOwnerCut(uint16 key1) external view returns (uint256);

    function getBlocksSinceLastStep(uint16 key1) external view returns (uint256);

    function getLastMechansimStepBlock(uint16 key1) external view returns (uint256);

    function getSubnetOwner(uint16 key1) external view returns (bytes32);

    function getSubnetOwnerHotkey(uint16 key1) external view returns (bytes32);

    function getRecycleOrBurn(uint16 key1) external view returns (uint8);

    function getRegistrationsThisInterval(uint16 key1) external view returns (uint16);

    function getPOWRegistrationsThisInterval(uint16 key1) external view returns (uint16);

    function getBurnRegistrationsThisInterval(uint16 key1) external view returns (uint16);

    function getMinAllowedUids(uint16 key1) external view returns (uint16);

    function getMaxAllowedUids(uint16 key1) external view returns (uint16);

    function getMaxWeightsLimit(uint16 key1) external view returns (uint16);

    function getMaxAllowedValidators(uint16 key1) external view returns (uint16);

    function getAdjustmentInterval(uint16 key1) external view returns (uint16);

    function getBondsPenalty(uint16 key1) external view returns (uint16);

    function getValidatorPruneLen(uint16 key1) external view returns (uint256);

    function getScalingLawPower(uint16 key1) external view returns (uint16);

    function getTargetRegistrationsPerInterval(uint16 key1) external view returns (uint16);

    function getLastAdjustmentBlock(uint16 key1) external view returns (uint256);

    function getRegistrationsThisBlock(uint16 key1) external view returns (uint16);

    function getRAORecycledForRegistration(uint16 key1) external view returns (uint256);

    function getTxRateLimit() external view returns (uint256);

    function getTxDelegateTakeRateLimit() external view returns (uint256);

    function getTxChildkeyTakeRateLimit() external view returns (uint256);

    function getSubtokenEnabled(uint16 key1) external view returns (bool);

    function getImmuneOwnerUidsLimit(uint16 key1) external view returns (uint16);

    function getStakeWeight(uint16 key1) external view returns (bytes);

    function getKeys(uint16 key1, uint16 key2) external view returns (bytes32);

    function getLoadedEmission(uint16 key1) external view returns (bytes);

    function getActive(uint16 key1) external view returns (bytes);

    function getRank(uint16 key1) external view returns (bytes);

    function getTrust(uint16 key1) external view returns (bytes);

    function getConsensus(uint16 key1) external view returns (bytes);

    function getIncentive(uint16 key1) external view returns (bytes);

    function getDividends(uint16 key1) external view returns (bytes);

    function getEmission(uint16 key1) external view returns (bytes);

    function getLastUpdate(uint16 key1) external view returns (bytes);

    function getValidatorTrust(uint16 key1) external view returns (bytes);

    function getPruningScores(uint16 key1) external view returns (bytes);

    function getValidatorPermit(uint16 key1) external view returns (bytes);

    function getWeights(uint16 key1, uint16 key2) external view returns (bytes);

    function getBonds(uint16 key1, uint16 key2) external view returns (bytes);

    function getBlockAtRegistration(uint16 key1, uint16 key2) external view returns (uint256);

    function getAxons(uint16 key1, bytes32 key2) external view returns (bytes);

    function getNeuronCertificates(uint16 key1, bytes32 key2) external view returns (bytes);

    function getPrometheus(uint16 key1, bytes32 key2) external view returns (bytes);

    function getIdentitiesV2(bytes32 key1) external view returns (bytes);

    function getSubnetIdentitiesV3(uint16 key1) external view returns (bytes);

    function getTransactionKeyLastBlock(bytes32 key1, uint16 key2, uint16 key3) external view returns (uint256);

    function getLastTxBlock(bytes32 key1) external view returns (uint256);

    function getLastTxBlockChildKeyTake(bytes32 key1) external view returns (uint256);

    function getLastTxBlockDelegateTake(bytes32 key1) external view returns (uint256);

    function getStakeThreshold() external view returns (uint256);

    function getWeightCommits(uint16 key1, bytes32 key2) external view returns (bytes);

    function getTimelockedWeightCommits(uint16 key1, uint256 key2) external view returns (bytes);

    function getCRV3WeightCommits(uint16 key1, uint256 key2) external view returns (bytes);

    function getCRV3WeightCommitsV2(uint16 key1, uint256 key2) external view returns (bytes);

    function getLastColdkeyHotkeyStakeBlock(bytes32 key1, bytes32 key2) external view returns (uint256);

    function getStakingOperationRateLimiter(bytes32 key1, bytes32 key2, uint16 key3) external view returns (bool);

    function getRootClaimableThreshold(uint16 key1) external view returns (uint256);

    function getRootClaimable(bytes32 key1) external view returns (bytes);

    function getRootClaimed(uint16 key1, bytes32 key2, bytes32 key3) external view returns (uint256);

    function getRootClaimType(bytes32 key1) external view returns (bytes);

    function getStakingColdkeysByIndex(uint256 key1) external view returns (bytes32);

    function getStakingColdkeys(bytes32 key1) external view returns (uint256);

    function getNumStakingColdkeys() external view returns (uint256);

    function getNumRootClaim() external view returns (uint256);

    function getAssociatedEvmAddress(uint16 key1, uint16 key2) external view returns (bytes);

    function getNextSubnetLeaseId() external view returns (uint32);

    function getAccumulatedLeaseDividends(uint32 key1) external view returns (uint256);

    function getCommitRevealWeightsVersion() external view returns (uint16);

    function getNetworkRegistrationStartBlock() external view returns (uint256);

    function getMinNonImmuneUids(uint16 key1) external view returns (uint16);

    function getMechanismCountCurrent(uint16 key1) external view returns (uint16);

    function getMechanismEmissionSplit(uint16 key1) external view returns (bytes);

    function getHasMigrationRun(bytes key1) external view returns (bool);

    function getPendingChildKeyCooldown() external view returns (uint256);

}
