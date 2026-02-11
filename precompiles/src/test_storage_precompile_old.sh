#!/bin/bash
# ============================================================
# FULL Storage Precompile Test — Generated Script
# Precompile: 0x000000000000000000000000000000000000080F
# ============================================================

P="0x000000000000000000000000000000000000080F"
R="http://127.0.0.1:9944"
PASS=0; FAIL=0; SKIP=0

test_call() {
  local label="$1"; shift
  # Run cast call, capture output
  local result=$(cast call "$@" --rpc-url $R 2>&1)
  # Check return code or error message
  if [[ $? -ne 0 ]] || echo "$result" | grep -q "Error\|revert"; then
    echo "  ❌ $label: FAIL"
    # Optional: echo "$result" | head -n 1
    FAIL=$((FAIL+1))
  else
    # Extract just the return value if possible, or show success
    val=$(echo "$result" |  sed "s/^0x//")
    # If it looks like a number, show it? cast call usually returns decoded if signature provided
    echo "  ✅ $label = $result"
    PASS=$((PASS+1))
  fi
}

echo ""
echo "=== AdminUtils ==="
echo ""
# 1. PrecompileEnable Map[PrecompileEnum] -> bool\ntest_call "PrecompileEnable" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "AdminUtils" "PrecompileEnable" 2 "0x0100" 6\n\necho ""
echo "=== Commitments ==="
echo ""
# 2. TimelockedIndex DoubleMap[NetUid, T::AccountId] -> Registration<BalanceOf<T
test_call "TimelockedIndex" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "Commitments" "TimelockedIndex" 0 "0x0100" 1 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0

# 3. UsedSpaceOf DoubleMap[NetUid, T::AccountId] -> UsageTracker
test_call "UsedSpaceOf" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "Commitments" "UsedSpaceOf" 0 "0x0100" 1 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0

echo ""
echo "=== Crowdloan ==="
echo ""
# 4. Contributions -> CrowdloanId
test_call "Contributions" $P "getValueRaw(string,string)(bytes)" "Crowdloan" "Contributions"

# 5. CurrencyOf DoubleMap[CrowdloanId, T::AccountId] -> BalanceOf<T
test_call "CurrencyOf" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "Crowdloan" "CurrencyOf" 1 "0x0100" 0 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0

# 6. NextCrowdloanId [SKIPPED complex key: BoundedVec<u8]\necho "  ⏭️  NextCrowdloanId: SKIPPED"; SKIP=$((SKIP+1))\n\n# 7. PassedCrowdloanId -> CrowdloanId
test_call "PassedCrowdloanId" $P "getValueRaw(string,string)(bytes)" "Crowdloan" "PassedCrowdloanId"

echo ""
echo "=== Drand ==="
echo ""
# 8. BeaconConfig [SKIPPED complex key: BoundedVec<u8]\necho "  ⏭️  BeaconConfig: SKIPPED"; SKIP=$((SKIP+1))\n\n# 9. HasMigrationRun -> RoundNumber
test_call "HasMigrationRun" $P "getValueRaw(string,string)(bytes)" "Drand" "HasMigrationRun"

# 10. OldestStoredRound -> RoundNumber
test_call "OldestStoredRound" $P "getValueRaw(string,string)(bytes)" "Drand" "OldestStoredRound"

# 11. Pulses Map[RoundNumber] -> Pulse\ntest_call "Pulses" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "Drand" "Pulses" 2 "0x0100"\n\necho ""
echo "=== Proxy ==="
echo ""
# 12. Announcements Map[T::AccountId] -> ( BoundedVec<Announcement<T::AccountId\ntest_call "Announcements" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "Proxy" "Announcements" 1 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800"\n\n# 13. BlockNumberFor Map[T::AccountId] -> ( BoundedVec< ProxyDefinition<T::AccountId\ntest_call "BlockNumberFor" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "Proxy" "BlockNumberFor" 1 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800"\n\n# 14. LastCallResult Map[T::AccountId] -> DispatchResult\ntest_call "LastCallResult" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "Proxy" "LastCallResult" 1 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800"\n\necho ""
echo "=== Shield ==="
echo ""
# 15. CurrentKey Map[T::Hash] -> Submission<T::AccountId\ntest_call "CurrentKey" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "Shield" "CurrentKey" 2 "0x0100"\n\n# 16. KeyHashByBlock Map[BlockNumberFor<T>] -> T::Hash\ntest_call "KeyHashByBlock" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "Shield" "KeyHashByBlock" 2 "0x0100"\n\n# 17. NextKey -> BoundedVec<u8
test_call "NextKey" $P "getValueRaw(string,string)(bytes)" "Shield" "NextKey"

echo ""
echo "=== SubtensorModule ==="
echo ""
# 18. AccumulatedLeaseDividends -> u16
test_call "AccumulatedLeaseDividends" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "AccumulatedLeaseDividends" 2

# 19. Active Map[NetUid] -> Vec<bool\ntest_call "Active" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "Active" 0 "0x0100"\n\n# 20. ActivityCutoff Map[NetUid] -> u16\ntest_call "ActivityCutoff" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "ActivityCutoff" 0 "0x0100" 2\n\n# 21. AdjustmentAlpha Map[NetUid] -> u64\ntest_call "AdjustmentAlpha" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "AdjustmentAlpha" 0 "0x0100" 4\n\n# 22. AdjustmentInterval Map[NetUid] -> u16\ntest_call "AdjustmentInterval" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "AdjustmentInterval" 0 "0x0100" 2\n\n# 23. AdminFreezeWindow -> u16
test_call "AdminFreezeWindow" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "AdminFreezeWindow" 2

# 24. Alpha DoubleMap[T::AccountId, NetUid] -> bool
test_call "Alpha" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "Alpha" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0 "0x0100" 6

# 25. AlphaDividendsPerSubnet DoubleMap[NetUid, T::AccountId] -> AlphaCurrency
test_call "AlphaDividendsPerSubnet" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "AlphaDividendsPerSubnet" 0 "0x0100" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 4

# 26. AlphaSigmoidSteepness Map[NetUid] -> i16\ntest_call "AlphaSigmoidSteepness" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "AlphaSigmoidSteepness" 0 "0x0100"\n\n# 27. AlphaValues Map[NetUid] -> (u16\ntest_call "AlphaValues" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "AlphaValues" 0 "0x0100"\n\n# 28. AssociatedEvmAddress -> LeaseId
test_call "AssociatedEvmAddress" $P "getValueRaw(string,string)(bytes)" "SubtensorModule" "AssociatedEvmAddress"

# 29. AutoStakeDestination Map[T::AccountId] -> (BlockNumberFor<T\ntest_call "AutoStakeDestination" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "AutoStakeDestination" 1 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800"\n\n# 30. AutoStakeDestinationColdkeys DoubleMap[T::AccountId, NetUid] -> Vec<T::AccountId
test_call "AutoStakeDestinationColdkeys" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "AutoStakeDestinationColdkeys" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0 "0x0100" 0

# 31. Axons DoubleMap[NetUid, T::AccountId] -> AxonInfoOf
test_call "Axons" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "Axons" 0 "0x0100" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0

# 32. BlockAtRegistration DoubleMap[NetUid, u16] -> u64
test_call "BlockAtRegistration" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "BlockAtRegistration" 0 "0x0100" 0 "0x0100" 4

# 33. BlockEmission DoubleMap[T::AccountId, NetUid] -> AlphaCurrency
test_call "BlockEmission" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "BlockEmission" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0 "0x0100" 4

# 34. BlocksSinceLastStep Map[NetUid] -> u64\ntest_call "BlocksSinceLastStep" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "BlocksSinceLastStep" 0 "0x0100" 4\n\n# 35. Bonds DoubleMap[NetUidStorageIndex, u16] -> Vec<(u16
test_call "Bonds" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "Bonds" 0 "0x0100" 0 "0x0100" 0

# 36. BondsMovingAverage Map[NetUid] -> u64\ntest_call "BondsMovingAverage" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "BondsMovingAverage" 0 "0x0100" 4\n\n# 37. BondsPenalty Map[NetUid] -> u16\ntest_call "BondsPenalty" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "BondsPenalty" 0 "0x0100" 2\n\n# 38. BondsResetOn Map[NetUid] -> bool\ntest_call "BondsResetOn" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "BondsResetOn" 0 "0x0100" 6\n\n# 39. Burn Map[NetUid] -> TaoCurrency\ntest_call "Burn" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "Burn" 0 "0x0100" 4\n\n# 40. BurnRegistrationsThisInterval Map[NetUid] -> u16\ntest_call "BurnRegistrationsThisInterval" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "BurnRegistrationsThisInterval" 0 "0x0100" 2\n\n# 41. CKBurn -> u64
test_call "CKBurn" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "CKBurn" 4

# 42. CRV3WeightCommits DoubleMap[NetUidStorageIndex, u64] -> // epoch key VecDeque<( T::AccountId
test_call "CRV3WeightCommits" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "CRV3WeightCommits" 1 "0x0100" 1 "0x0100000000000000" 0

# 43. CRV3WeightCommitsV2 DoubleMap[NetUidStorageIndex, u64] -> // epoch key VecDeque<( T::AccountId
test_call "CRV3WeightCommitsV2" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "CRV3WeightCommitsV2" 1 "0x0100" 1 "0x0100000000000000" 0

# 44. ChildKeys DoubleMap[T::AccountId, NetUid] -> Vec<(u64
test_call "ChildKeys" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "ChildKeys" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0 "0x0100" 0

# 45. ChildkeyTake Map[NetUid] -> I96F32\ntest_call "ChildkeyTake" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "ChildkeyTake" 0 "0x0100"\n\n# 46. ColdkeySwapAnnouncementDelay DoubleMap[T::AccountId, NetUid] -> AlphaCurrency
test_call "ColdkeySwapAnnouncementDelay" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "ColdkeySwapAnnouncementDelay" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0 "0x0100" 4

# 47. ColdkeySwapAnnouncements -> Option<Vec<u8
test_call "ColdkeySwapAnnouncements" $P "getValueRaw(string,string)(bytes)" "SubtensorModule" "ColdkeySwapAnnouncements"

# 48. ColdkeySwapDisputes Map[T::AccountId] -> BlockNumberFor<T\ntest_call "ColdkeySwapDisputes" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "ColdkeySwapDisputes" 1 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800"\n\n# 49. ColdkeySwapReannouncementDelay -> BlockNumberFor<T
test_call "ColdkeySwapReannouncementDelay" $P "getValueRaw(string,string)(bytes)" "SubtensorModule" "ColdkeySwapReannouncementDelay"

# 50. ColdkeySwapRescheduleDuration -> BlockNumberFor<T
test_call "ColdkeySwapRescheduleDuration" $P "getValueRaw(string,string)(bytes)" "SubtensorModule" "ColdkeySwapRescheduleDuration"

# 51. ColdkeySwapScheduleDuration -> BlockNumberFor<T
test_call "ColdkeySwapScheduleDuration" $P "getValueRaw(string,string)(bytes)" "SubtensorModule" "ColdkeySwapScheduleDuration"

# 52. CommitRevealWeightsEnabled Map[NetUid] -> bool\ntest_call "CommitRevealWeightsEnabled" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "CommitRevealWeightsEnabled" 0 "0x0100" 6\n\n# 53. CommitRevealWeightsVersion Map[NetUid] -> u16\ntest_call "CommitRevealWeightsVersion" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "CommitRevealWeightsVersion" 0 "0x0100" 2\n\n# 54. Consensus Map[NetUid] -> Vec<u16\ntest_call "Consensus" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "Consensus" 0 "0x0100"\n\n# 55. Delegates Map[T::AccountId] -> u16\ntest_call "Delegates" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "Delegates" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 2\n\n# 56. Difficulty Map[NetUid] -> u64\ntest_call "Difficulty" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "Difficulty" 0 "0x0100" 4\n\n# 57. DissolveNetworkScheduleDuration -> BlockNumberFor<T
test_call "DissolveNetworkScheduleDuration" $P "getValueRaw(string,string)(bytes)" "SubtensorModule" "DissolveNetworkScheduleDuration"

# 58. Dividends Map[NetUid] -> Vec<u16\ntest_call "Dividends" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "Dividends" 0 "0x0100"\n\n# 59. EMAPriceHalvingBlocks Map[NetUid] -> u64\ntest_call "EMAPriceHalvingBlocks" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "EMAPriceHalvingBlocks" 0 "0x0100" 4\n\n# 60. Emission Map[NetUid] -> Vec<AlphaCurrency\ntest_call "Emission" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "Emission" 0 "0x0100"\n\n# 61. FirstEmissionBlockNumber Map[NetUid] -> u64\ntest_call "FirstEmissionBlockNumber" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "FirstEmissionBlockNumber" 0 "0x0100" 4\n\n# 62. FlowEmaSmoothingFactor -> u64
test_call "FlowEmaSmoothingFactor" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "FlowEmaSmoothingFactor" 4

# 63. FlowNormExponent -> U64F64
test_call "FlowNormExponent" $P "getValueRaw(string,string)(bytes)" "SubtensorModule" "FlowNormExponent"

# 64. HasMigrationRun [SKIPPED complex key: Vec<u8>]\necho "  ⏭️  HasMigrationRun: SKIPPED"; SKIP=$((SKIP+1))\n\n# 65. IdentitiesV2 DoubleMap[NetUidStorageIndex, T::AccountId] -> VecDeque<(H256
test_call "IdentitiesV2" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "IdentitiesV2" 1 "0x0100" 1 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0

# 66. ImmuneOwnerUidsLimit Map[NetUid] -> u16\ntest_call "ImmuneOwnerUidsLimit" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "ImmuneOwnerUidsLimit" 0 "0x0100" 2\n\n# 67. ImmunityPeriod Map[NetUid] -> u16\ntest_call "ImmunityPeriod" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "ImmunityPeriod" 0 "0x0100" 2\n\n# 68. Incentive Map[NetUidStorageIndex] -> Vec<u16\ntest_call "Incentive" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "Incentive" 0 "0x0100"\n\n# 69. IsNetworkMember Map[NetUid] -> bool\ntest_call "IsNetworkMember" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "IsNetworkMember" 0 "0x0100" 6\n\n# 70. Kappa Map[NetUid] -> u16\ntest_call "Kappa" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "Kappa" 0 "0x0100" 2\n\n# 71. Keys DoubleMap[NetUid, u16] -> T::AccountId
test_call "Keys" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "Keys" 0 "0x0100" 0 "0x0100" 7

# 72. LargestLocked Map[NetUid] -> u64\ntest_call "LargestLocked" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "LargestLocked" 0 "0x0100" 4\n\n# 73. LastAdjustmentBlock Map[NetUid] -> u64\ntest_call "LastAdjustmentBlock" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "LastAdjustmentBlock" 0 "0x0100" 4\n\n# 74. LastColdkeyHotkeyStakeBlock Map[NetUid] -> I96F32\ntest_call "LastColdkeyHotkeyStakeBlock" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "LastColdkeyHotkeyStakeBlock" 2 "0x0100"\n\n# 75. LastHotkeyEmissionOnNetuid -> u16
test_call "LastHotkeyEmissionOnNetuid" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "LastHotkeyEmissionOnNetuid" 2

# 76. LastHotkeySwapOnNetuid -> u64
test_call "LastHotkeySwapOnNetuid" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "LastHotkeySwapOnNetuid" 4

# 77. LastMechansimStepBlock Map[NetUid] -> u64\ntest_call "LastMechansimStepBlock" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "LastMechansimStepBlock" 0 "0x0100" 4\n\n# 78. LastRateLimitedBlock -> u64
test_call "LastRateLimitedBlock" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "LastRateLimitedBlock" 4

# 79. LastTxBlockChildKeyTake Map[T::AccountId] -> u64\ntest_call "LastTxBlockChildKeyTake" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "LastTxBlockChildKeyTake" 0 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 4\n\n# 80. LastTxBlockDelegateTake Map[T::AccountId] -> u64\ntest_call "LastTxBlockDelegateTake" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "LastTxBlockDelegateTake" 0 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 4\n\n# 81. LastUpdate Map[NetUidStorageIndex] -> Vec<u64\ntest_call "LastUpdate" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "LastUpdate" 0 "0x0100"\n\n# 82. LiquidAlphaOn -> u64
test_call "LiquidAlphaOn" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "LiquidAlphaOn" 4

# 83. LoadedEmission DoubleMap[NetUidStorageIndex, u16] -> Vec<(u16
test_call "LoadedEmission" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "LoadedEmission" 0 "0x0100" 0 "0x0100" 0

# 84. MaxAllowedUids Map[NetUid] -> u16\ntest_call "MaxAllowedUids" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "MaxAllowedUids" 0 "0x0100" 2\n\n# 85. MaxAllowedValidators Map[NetUid] -> u16\ntest_call "MaxAllowedValidators" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "MaxAllowedValidators" 0 "0x0100" 2\n\n# 86. MaxBurn Map[NetUid] -> TaoCurrency\ntest_call "MaxBurn" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "MaxBurn" 0 "0x0100" 4\n\n# 87. MaxChildkeyTake -> u16
test_call "MaxChildkeyTake" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "MaxChildkeyTake" 2

# 88. MaxDelegateTake -> u16
test_call "MaxDelegateTake" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "MaxDelegateTake" 2

# 89. MaxDifficulty Map[NetUid] -> u64\ntest_call "MaxDifficulty" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "MaxDifficulty" 0 "0x0100" 4\n\n# 90. MaxMechanismCount Map[NetUid] -> MechId\ntest_call "MaxMechanismCount" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "MaxMechanismCount" 1 "0x0100"\n\n# 91. MaxRegistrationsPerBlock Map[NetUid] -> u16\ntest_call "MaxRegistrationsPerBlock" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "MaxRegistrationsPerBlock" 0 "0x0100" 2\n\n# 92. MaxWeightsLimit Map[NetUid] -> u16\ntest_call "MaxWeightsLimit" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "MaxWeightsLimit" 0 "0x0100" 2\n\n# 93. MechanismCountCurrent -> u64
test_call "MechanismCountCurrent" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "MechanismCountCurrent" 4

# 94. MechanismEmissionSplit Map[NetUid] -> Vec<u16\ntest_call "MechanismEmissionSplit" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "MechanismEmissionSplit" 1 "0x0100"\n\n# 95. MinAllowedUids Map[NetUid] -> u16\ntest_call "MinAllowedUids" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "MinAllowedUids" 0 "0x0100" 2\n\n# 96. MinAllowedWeights Map[NetUid] -> u16\ntest_call "MinAllowedWeights" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "MinAllowedWeights" 0 "0x0100" 2\n\n# 97. MinBurn Map[NetUid] -> TaoCurrency\ntest_call "MinBurn" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "MinBurn" 0 "0x0100" 4\n\n# 98. MinChildkeyTake -> u16
test_call "MinChildkeyTake" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "MinChildkeyTake" 2

# 99. MinDelegateTake -> u16
test_call "MinDelegateTake" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "MinDelegateTake" 2

# 100. MinDifficulty Map[NetUid] -> u64\ntest_call "MinDifficulty" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "MinDifficulty" 0 "0x0100" 4\n\n# 101. MinNonImmuneUids -> MechId
test_call "MinNonImmuneUids" $P "getValueRaw(string,string)(bytes)" "SubtensorModule" "MinNonImmuneUids"

# 102. NetworkImmunityPeriod -> u64
test_call "NetworkImmunityPeriod" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "NetworkImmunityPeriod" 4

# 103. NetworkLastLockCost -> TaoCurrency
test_call "NetworkLastLockCost" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "NetworkLastLockCost" 4

# 104. NetworkLockReductionInterval -> u64
test_call "NetworkLockReductionInterval" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "NetworkLockReductionInterval" 4

# 105. NetworkMinLockCost -> TaoCurrency
test_call "NetworkMinLockCost" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "NetworkMinLockCost" 4

# 106. NetworkPowRegistrationAllowed Map[NetUid] -> bool\ntest_call "NetworkPowRegistrationAllowed" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "NetworkPowRegistrationAllowed" 0 "0x0100" 6\n\n# 107. NetworkRateLimit -> u64
test_call "NetworkRateLimit" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "NetworkRateLimit" 4

# 108. NetworkRegisteredAt Map[NetUid] -> u64\ntest_call "NetworkRegisteredAt" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "NetworkRegisteredAt" 0 "0x0100" 4\n\n# 109. NetworkRegistrationAllowed DoubleMap[NetUid, T::AccountId] -> u64
test_call "NetworkRegistrationAllowed" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "NetworkRegistrationAllowed" 0 "0x0100" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 4

# 110. NetworkRegistrationStartBlock -> u64
test_call "NetworkRegistrationStartBlock" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "NetworkRegistrationStartBlock" 4

# 111. NetworksAdded Map[NetUid] -> bool\ntest_call "NetworksAdded" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "NetworksAdded" 0 "0x0100" 6\n\n# 112. NeuronCertificates DoubleMap[NetUid, T::AccountId] -> NeuronCertificateOf
test_call "NeuronCertificates" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "NeuronCertificates" 0 "0x0100" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0

# 113. NextStakeJobId DoubleMap[NetUid, T::AccountId] -> (Vec<(u64
test_call "NextStakeJobId" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "NextStakeJobId" 0 "0x0100" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0

# 114. NextSubnetLeaseId [SKIPPED complex key: LeaseId]\necho "  ⏭️  NextSubnetLeaseId: SKIPPED"; SKIP=$((SKIP+1))\n\n# 115. NominatorMinRequiredStake -> u64
test_call "NominatorMinRequiredStake" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "NominatorMinRequiredStake" 4

# 116. NumRootClaim -> u64
test_call "NumRootClaim" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "NumRootClaim" 4

# 117. NumStakingColdkeys [SKIPPED complex key: LeaseId]\necho "  ⏭️  NumStakingColdkeys: SKIPPED"; SKIP=$((SKIP+1))\n\n# 118. OwnedHotkeys Map[T::AccountId] -> Vec<T::AccountId\ntest_call "OwnedHotkeys" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "OwnedHotkeys" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800"\n\n# 119. Owner -> u64
test_call "Owner" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "Owner" 4

# 120. OwnerHyperparamRateLimit -> u16
test_call "OwnerHyperparamRateLimit" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "OwnerHyperparamRateLimit" 2

# 121. POWRegistrationsThisInterval Map[NetUid] -> u16\ntest_call "POWRegistrationsThisInterval" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "POWRegistrationsThisInterval" 0 "0x0100" 2\n\n# 122. PalletsOriginOf DoubleMap[NetUid, T::AccountId] -> u64
test_call "PalletsOriginOf" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "PalletsOriginOf" 0 "0x0100" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 4

# 123. ParentKeys DoubleMap[T::AccountId, NetUid] -> Vec<(u64
test_call "ParentKeys" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "ParentKeys" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0 "0x0100" 0

# 124. PendingOwnerCut Map[NetUid] -> AlphaCurrency\ntest_call "PendingOwnerCut" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "PendingOwnerCut" 0 "0x0100" 4\n\n# 125. PendingRootAlphaDivs Map[NetUid] -> AlphaCurrency\ntest_call "PendingRootAlphaDivs" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "PendingRootAlphaDivs" 0 "0x0100" 4\n\n# 126. PendingServerEmission Map[NetUid] -> AlphaCurrency\ntest_call "PendingServerEmission" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "PendingServerEmission" 0 "0x0100" 4\n\n# 127. PendingValidatorEmission Map[NetUid] -> AlphaCurrency\ntest_call "PendingValidatorEmission" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "PendingValidatorEmission" 0 "0x0100" 4\n\n# 128. Prometheus DoubleMap[NetUid, T::AccountId] -> PrometheusInfoOf
test_call "Prometheus" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "Prometheus" 0 "0x0100" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0

# 129. PruningScores Map[NetUid] -> Vec<u16\ntest_call "PruningScores" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "PruningScores" 0 "0x0100"\n\n# 130. RAORecycledForRegistration Map[NetUid] -> TaoCurrency\ntest_call "RAORecycledForRegistration" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "RAORecycledForRegistration" 0 "0x0100" 4\n\n# 131. Rank Map[NetUid] -> Vec<u16\ntest_call "Rank" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "Rank" 0 "0x0100"\n\n# 132. RecycleOrBurn Map[NetUid] -> RecycleOrBurnEnum\ntest_call "RecycleOrBurn" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "RecycleOrBurn" 0 "0x0100"\n\n# 133. RegistrationsThisBlock Map[NetUid] -> u16\ntest_call "RegistrationsThisBlock" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "RegistrationsThisBlock" 0 "0x0100" 2\n\n# 134. RegistrationsThisInterval Map[NetUid] -> u16\ntest_call "RegistrationsThisInterval" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "RegistrationsThisInterval" 0 "0x0100" 2\n\n# 135. RevealPeriodEpochs DoubleMap[T::AccountId, T::AccountId] -> u64
test_call "RevealPeriodEpochs" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "RevealPeriodEpochs" 1 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 1 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 4

# 136. Rho Map[NetUid] -> u16\ntest_call "Rho" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "Rho" 0 "0x0100" 2\n\n# 137. RootAlphaDividendsPerSubnet DoubleMap[NetUid, T::AccountId] -> AlphaCurrency
test_call "RootAlphaDividendsPerSubnet" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "RootAlphaDividendsPerSubnet" 0 "0x0100" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 4

# 138. RootClaimable Map[T::AccountId] -> BTreeMap<NetUid\ntest_call "RootClaimable" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "RootClaimable" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800"\n\n# 139. RootClaimed Map[T::AccountId] -> RootClaimTypeEnum\ntest_call "RootClaimed" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "RootClaimed" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800"\n\n# 140. RootProp Map[NetUid] -> U96F32\ntest_call "RootProp" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "RootProp" 0 "0x0100"\n\n# 141. ScalingLawPower Map[NetUid] -> u16\ntest_call "ScalingLawPower" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "ScalingLawPower" 0 "0x0100" 2\n\n# 142. ServingRateLimit Map[NetUid] -> u64\ntest_call "ServingRateLimit" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "ServingRateLimit" 0 "0x0100" 4\n\n# 143. StakeThreshold Map[NetUid] -> u64\ntest_call "StakeThreshold" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "StakeThreshold" 1 "0x0100" 4\n\n# 144. StakeWeight Map[NetUid] -> Vec<u16\ntest_call "StakeWeight" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "StakeWeight" 0 "0x0100"\n\n# 145. StakingColdkeys Map[T::AccountId] -> u64\ntest_call "StakingColdkeys" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "StakingColdkeys" 0 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 4\n\n# 146. StakingColdkeysByIndex Map[u64] -> T::AccountId\ntest_call "StakingColdkeysByIndex" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "StakingColdkeysByIndex" 0 "0x0100000000000000" 7\n\n# 147. StakingHotkeys Map[T::AccountId] -> Vec<T::AccountId\ntest_call "StakingHotkeys" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "StakingHotkeys" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800"\n\n# 148. StakingOperationRateLimiter DoubleMap[NetUid, u16] -> (H160
test_call "StakingOperationRateLimiter" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "StakingOperationRateLimiter" 1 "0x0100" 1 "0x0100" 0

# 149. StartCallDelay -> u64
test_call "StartCallDelay" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "StartCallDelay" 4

# 150. SubnetAlphaIn Map[NetUid] -> AlphaCurrency\ntest_call "SubnetAlphaIn" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubnetAlphaIn" 0 "0x0100" 4\n\n# 151. SubnetAlphaInEmission Map[NetUid] -> AlphaCurrency\ntest_call "SubnetAlphaInEmission" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubnetAlphaInEmission" 0 "0x0100" 4\n\n# 152. SubnetAlphaInProvided Map[NetUid] -> AlphaCurrency\ntest_call "SubnetAlphaInProvided" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubnetAlphaInProvided" 0 "0x0100" 4\n\n# 153. SubnetAlphaOut Map[NetUid] -> AlphaCurrency\ntest_call "SubnetAlphaOut" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubnetAlphaOut" 0 "0x0100" 4\n\n# 154. SubnetAlphaOutEmission Map[NetUid] -> AlphaCurrency\ntest_call "SubnetAlphaOutEmission" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubnetAlphaOutEmission" 0 "0x0100" 4\n\n# 155. SubnetEmaTaoFlow Map[NetUid] -> (u64\ntest_call "SubnetEmaTaoFlow" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "SubnetEmaTaoFlow" 0 "0x0100"\n\n# 156. SubnetIdentitiesV3 Map[NetUid] -> SubnetIdentityOfV3\ntest_call "SubnetIdentitiesV3" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "SubnetIdentitiesV3" 2 "0x0100"\n\n# 157. SubnetLeaseShares Map[NetUid] -> LeaseId\ntest_call "SubnetLeaseShares" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "SubnetLeaseShares" 1 "0x0100"\n\n# 158. SubnetLeases [SKIPPED complex keys: LeaseId, T::AccountId]
echo "  ⏭️  SubnetLeases: SKIPPED"; SKIP=$((SKIP+1))

# 159. SubnetLimit DoubleMap[T::AccountId, NetUid] -> T::AccountId
test_call "SubnetLimit" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubnetLimit" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0 "0x0100" 7

# 160. SubnetLocked Map[NetUid] -> TaoCurrency\ntest_call "SubnetLocked" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubnetLocked" 0 "0x0100" 4\n\n# 161. SubnetMechanism Map[NetUid] -> u16\ntest_call "SubnetMechanism" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubnetMechanism" 0 "0x0100" 2\n\n# 162. SubnetMovingAlpha -> I96F32
test_call "SubnetMovingAlpha" $P "getValueRaw(string,string)(bytes)" "SubtensorModule" "SubnetMovingAlpha"

# 163. SubnetMovingPrice -> BlockNumberFor<T
test_call "SubnetMovingPrice" $P "getValueRaw(string,string)(bytes)" "SubtensorModule" "SubnetMovingPrice"

# 164. SubnetOwner Map[NetUid] -> T::AccountId\ntest_call "SubnetOwner" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubnetOwner" 0 "0x0100" 7\n\n# 165. SubnetOwnerCut -> u16
test_call "SubnetOwnerCut" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "SubnetOwnerCut" 2

# 166. SubnetOwnerHotkey Map[NetUid] -> T::AccountId\ntest_call "SubnetOwnerHotkey" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubnetOwnerHotkey" 0 "0x0100" 7\n\n# 167. SubnetTAO Map[NetUid] -> TaoCurrency\ntest_call "SubnetTAO" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubnetTAO" 0 "0x0100" 4\n\n# 168. SubnetTaoFlow Map[NetUid] -> i64\ntest_call "SubnetTaoFlow" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubnetTaoFlow" 0 "0x0100" 4\n\n# 169. SubnetTaoInEmission Map[NetUid] -> TaoCurrency\ntest_call "SubnetTaoInEmission" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubnetTaoInEmission" 0 "0x0100" 4\n\n# 170. SubnetTaoProvided Map[NetUid] -> TaoCurrency\ntest_call "SubnetTaoProvided" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubnetTaoProvided" 0 "0x0100" 4\n\n# 171. SubnetVolume Map[NetUid] -> u128\ntest_call "SubnetVolume" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubnetVolume" 0 "0x0100" 5\n\n# 172. SubnetworkN Map[NetUid] -> u16\ntest_call "SubnetworkN" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubnetworkN" 0 "0x0100" 2\n\n# 173. SubtokenEnabled Map[NetUid] -> bool\ntest_call "SubtokenEnabled" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "SubtokenEnabled" 0 "0x0100" 6\n\n# 174. TaoFlowCutoff [SKIPPED complex key: Vec<u8>]\necho "  ⏭️  TaoFlowCutoff: SKIPPED"; SKIP=$((SKIP+1))\n\n# 175. TaoWeight -> u64
test_call "TaoWeight" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "TaoWeight" 4

# 176. TargetRegistrationsPerInterval Map[NetUid] -> u16\ntest_call "TargetRegistrationsPerInterval" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "TargetRegistrationsPerInterval" 0 "0x0100" 2\n\n# 177. Tempo Map[NetUid] -> u16\ntest_call "Tempo" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "Tempo" 0 "0x0100" 2\n\n# 178. TimelockedWeightCommits DoubleMap[NetUidStorageIndex, u64] -> // epoch key VecDeque<( T::AccountId
test_call "TimelockedWeightCommits" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "TimelockedWeightCommits" 1 "0x0100" 1 "0x0100000000000000" 0

# 179. TokenSymbol -> I64F64
test_call "TokenSymbol" $P "getValueRaw(string,string)(bytes)" "SubtensorModule" "TokenSymbol"

# 180. TotalHotkeyAlpha Map[NetUid] -> Vec<u8\ntest_call "TotalHotkeyAlpha" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "TotalHotkeyAlpha" 0 "0x0100"\n\n# 181. TotalHotkeyAlphaLastEpoch DoubleMap[T::AccountId, NetUid] -> AlphaCurrency
test_call "TotalHotkeyAlphaLastEpoch" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "TotalHotkeyAlphaLastEpoch" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0 "0x0100" 4

# 182. TotalHotkeyShares DoubleMap[T::AccountId, NetUid] -> U64F64
test_call "TotalHotkeyShares" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "TotalHotkeyShares" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 0 "0x0100" 0

# 183. TotalIssuance -> TaoCurrency
test_call "TotalIssuance" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "TotalIssuance" 4

# 184. TotalNetworks [SKIPPED complex key: RateLimitKey<T::AccountId>]\necho "  ⏭️  TotalNetworks: SKIPPED"; SKIP=$((SKIP+1))\n\n# 185. TotalStake -> TaoCurrency
test_call "TotalStake" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "TotalStake" 4

# 186. TransactionKeyLastBlock Map[T::AccountId] -> u64\ntest_call "TransactionKeyLastBlock" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "TransactionKeyLastBlock" 0 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 4\n\n# 187. TransferToggle Map[NetUid] -> bool\ntest_call "TransferToggle" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "TransferToggle" 0 "0x0100" 6\n\n# 188. Trust Map[NetUid] -> Vec<u16\ntest_call "Trust" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "Trust" 0 "0x0100"\n\n# 189. TxChildkeyTakeRateLimit -> u64
test_call "TxChildkeyTakeRateLimit" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "TxChildkeyTakeRateLimit" 4

# 190. TxDelegateTakeRateLimit -> u64
test_call "TxDelegateTakeRateLimit" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "TxDelegateTakeRateLimit" 4

# 191. TxRateLimit Map[NetUid] -> bool\ntest_call "TxRateLimit" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "TxRateLimit" 2 "0x0100" 6\n\n# 192. Uids Map[NetUid] -> Vec<(T::AccountId\ntest_call "Uids" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "Uids" 0 "0x0100"\n\n# 193. UsedWork -> u16
test_call "UsedWork" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "UsedWork" 2

# 194. ValidatorPermit Map[NetUid] -> Vec<bool\ntest_call "ValidatorPermit" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "ValidatorPermit" 0 "0x0100"\n\n# 195. ValidatorPruneLen Map[NetUid] -> u64\ntest_call "ValidatorPruneLen" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "ValidatorPruneLen" 0 "0x0100" 4\n\n# 196. ValidatorTrust Map[NetUid] -> Vec<u16\ntest_call "ValidatorTrust" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "ValidatorTrust" 0 "0x0100"\n\n# 197. VotingPower Map[NetUid] -> bool\ntest_call "VotingPower" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "VotingPower" 0 "0x0100" 6\n\n# 198. VotingPowerDisableAtBlock Map[NetUid] -> u64\ntest_call "VotingPowerDisableAtBlock" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "VotingPowerDisableAtBlock" 0 "0x0100" 4\n\n# 199. VotingPowerEmaAlpha Map[NetUid] -> u64\ntest_call "VotingPowerEmaAlpha" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "VotingPowerEmaAlpha" 0 "0x0100" 4\n\n# 200. VotingPowerTrackingEnabled DoubleMap[NetUid, T::AccountId] -> u16
test_call "VotingPowerTrackingEnabled" $P "getDoubleMap(string,string,uint8,bytes,uint8,bytes,uint8)(uint256)" "SubtensorModule" "VotingPowerTrackingEnabled" 0 "0x0100" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800" 2

# 201. WeightCommits -> u64
test_call "WeightCommits" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "WeightCommits" 4

# 202. Weights Map[T::AccountId] -> ChainIdentityOfV2\ntest_call "Weights" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "SubtensorModule" "Weights" 2 "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a568aa17fa52800"\n\n# 203. WeightsSetRateLimit Map[NetUid] -> u64\ntest_call "WeightsSetRateLimit" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "WeightsSetRateLimit" 0 "0x0100" 4\n\n# 204. WeightsVersionKey Map[NetUid] -> u64\ntest_call "WeightsVersionKey" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "WeightsVersionKey" 0 "0x0100" 4\n\n# 205. WeightsVersionKeyRateLimit -> u64
test_call "WeightsVersionKeyRateLimit" $P "getValue(string,string,uint8)(uint256)" "SubtensorModule" "WeightsVersionKeyRateLimit" 4

# 206. Yuma3On Map[NetUid] -> bool\ntest_call "Yuma3On" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "SubtensorModule" "Yuma3On" 2 "0x0100" 6\n\necho ""
echo "=== Swap ==="
echo ""
# 207. AlphaSqrtPrice Map[NetUid] -> U64F64\ntest_call "AlphaSqrtPrice" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "Swap" "AlphaSqrtPrice" 1 "0x0100"\n\n# 208. CurrentLiquidity Map[NetUid] -> u64\ntest_call "CurrentLiquidity" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "Swap" "CurrentLiquidity" 1 "0x0100" 4\n\n# 209. CurrentTick Map[NetUid] -> TickIndex\ntest_call "CurrentTick" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "Swap" "CurrentTick" 1 "0x0100"\n\n# 210. EnabledUserLiquidity Map[NetUid] -> bool\ntest_call "EnabledUserLiquidity" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "Swap" "EnabledUserLiquidity" 1 "0x0100" 6\n\n# 211. FeeGlobalAlpha Map[NetUid] -> U64F64\ntest_call "FeeGlobalAlpha" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "Swap" "FeeGlobalAlpha" 1 "0x0100"\n\n# 212. FeeGlobalTao Map[NetUid] -> U64F64\ntest_call "FeeGlobalTao" $P "getMapRaw(string,string,uint8,bytes)(bytes)" "Swap" "FeeGlobalTao" 1 "0x0100"\n\n# 213. FeeRate [SKIPPED complex keys: NetUid, TickIndex]
echo "  ⏭️  FeeRate: SKIPPED"; SKIP=$((SKIP+1))

# 214. Positions Map[NetUid] -> TaoCurrency\ntest_call "Positions" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "Swap" "Positions" 1 "0x0100" 4\n\n# 215. ScrapReservoirAlpha Map[NetUid] -> AlphaCurrency\ntest_call "ScrapReservoirAlpha" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "Swap" "ScrapReservoirAlpha" 1 "0x0100" 4\n\n# 216. Ticks Map[NetUid] -> bool\ntest_call "Ticks" $P "getMap(string,string,uint8,bytes,uint8)(uint256)" "Swap" "Ticks" 1 "0x0100" 6\n\necho ""
echo "============================================"
echo "SUMMARY: $PASS passed, $FAIL failed, $SKIP skipped"
echo "============================================"
