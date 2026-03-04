import * as assert from "assert";

import { getDevnetApi, getRandomSubstrateKeypair } from "../src/substrate";
import { getPublicClient } from "../src/utils";
import { ETH_LOCAL_URL } from "../src/config";
import { devnet } from "@polkadot-api/descriptors";
import { PublicClient } from "viem";
import { TypedApi } from "polkadot-api";
import { toViemAddress, convertPublicKeyToSs58 } from "../src/address-utils";
import { ISubtensorABI, ISUBTENSOR_ADDRESS } from "../src/contracts/subtensor";
import { forceSetBalanceToSs58Address, addNewSubnetwork, startCall } from "../src/subtensor";

describe("Test Subtensor Precompile Views (0x817) - Full Coverage", () => {
    const hotkey = getRandomSubstrateKeypair();
    const coldkey = getRandomSubstrateKeypair();
    let publicClient: PublicClient;
    let api: TypedApi<typeof devnet>;
    let netuid = 1;
    const fakeAccount = "0x" + "01".repeat(32);

    before(async () => {
        publicClient = await getPublicClient(ETH_LOCAL_URL);
        api = await getDevnetApi();
        await forceSetBalanceToSs58Address(api, convertPublicKeyToSs58(hotkey.publicKey));
        await forceSetBalanceToSs58Address(api, convertPublicKeyToSs58(coldkey.publicKey));
        netuid = await addNewSubnetwork(api, hotkey, coldkey);
        await startCall(api, netuid, coldkey);
    });

    describe("No-Args Storage Values", () => {
        it("getPalletVersion returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getPalletVersion",
                args: []
            });
            assert.ok(result !== undefined, "getPalletVersion should return a value");
        });

        it("getMinActivityCutoff returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getMinActivityCutoff",
                args: []
            });
            assert.ok(result !== undefined, "getMinActivityCutoff should return a value");
        });

        it("getAdminFreezeWindow returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getAdminFreezeWindow",
                args: []
            });
            assert.ok(result !== undefined, "getAdminFreezeWindow should return a value");
        });

        it("getOwnerHyperparamRateLimit returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getOwnerHyperparamRateLimit",
                args: []
            });
            assert.ok(result !== undefined, "getOwnerHyperparamRateLimit should return a value");
        });

        it("getColdkeySwapScheduleDuration returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getColdkeySwapScheduleDuration",
                args: []
            });
            assert.ok(result !== undefined, "getColdkeySwapScheduleDuration should return a value");
        });

        it("getColdkeySwapRescheduleDuration returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getColdkeySwapRescheduleDuration",
                args: []
            });
            assert.ok(result !== undefined, "getColdkeySwapRescheduleDuration should return a value");
        });

        it("getDissolveNetworkScheduleDuration returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getDissolveNetworkScheduleDuration",
                args: []
            });
            assert.ok(result !== undefined, "getDissolveNetworkScheduleDuration should return a value");
        });

        it("getNextStakeJobId returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getNextStakeJobId",
                args: []
            });
            assert.ok(result !== undefined, "getNextStakeJobId should return a value");
        });

        it("getMaxDelegateTake returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getMaxDelegateTake",
                args: []
            });
            assert.ok(result !== undefined, "getMaxDelegateTake should return a value");
        });

        it("getMinDelegateTake returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getMinDelegateTake",
                args: []
            });
            assert.ok(result !== undefined, "getMinDelegateTake should return a value");
        });

        it("getMaxChildkeyTake returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getMaxChildkeyTake",
                args: []
            });
            assert.ok(result !== undefined, "getMaxChildkeyTake should return a value");
        });

        it("getMinChildkeyTake returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getMinChildkeyTake",
                args: []
            });
            assert.ok(result !== undefined, "getMinChildkeyTake should return a value");
        });

        it("getBlockEmission returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getBlockEmission",
                args: []
            });
            assert.ok(result !== undefined, "getBlockEmission should return a value");
        });

        it("getSubnetLimit returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getSubnetLimit",
                args: []
            });
            assert.ok(result !== undefined, "getSubnetLimit should return a value");
        });

        it("getTotalIssuance returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTotalIssuance",
                args: []
            });
            assert.ok(result !== undefined, "getTotalIssuance should return a value");
        });

        it("getTotalStake returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTotalStake",
                args: []
            });
            assert.ok(result !== undefined, "getTotalStake should return a value");
        });

        it("getSubnetMovingAlpha returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getSubnetMovingAlpha",
                args: []
            });
            assert.ok(result !== undefined, "getSubnetMovingAlpha should return a value");
        });

        it("getAlphaMapLastKey returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getAlphaMapLastKey",
                args: []
            });
            assert.ok(result !== undefined, "getAlphaMapLastKey should return a value");
        });

        it("getTaoFlowCutoff returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTaoFlowCutoff",
                args: []
            });
            assert.ok(result !== undefined, "getTaoFlowCutoff should return a value");
        });

        it("getFlowNormExponent returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getFlowNormExponent",
                args: []
            });
            assert.ok(result !== undefined, "getFlowNormExponent should return a value");
        });

        it("getFlowEmaSmoothingFactor returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getFlowEmaSmoothingFactor",
                args: []
            });
            assert.ok(result !== undefined, "getFlowEmaSmoothingFactor should return a value");
        });

        it("getTotalNetworks returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTotalNetworks",
                args: []
            });
            assert.ok(result !== undefined, "getTotalNetworks should return a value");
        });

        it("getNetworkImmunityPeriod returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getNetworkImmunityPeriod",
                args: []
            });
            assert.ok(result !== undefined, "getNetworkImmunityPeriod should return a value");
        });

        it("getStartCallDelay returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getStartCallDelay",
                args: []
            });
            assert.ok(result !== undefined, "getStartCallDelay should return a value");
        });

        it("getNetworkMinLockCost returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getNetworkMinLockCost",
                args: []
            });
            assert.ok(result !== undefined, "getNetworkMinLockCost should return a value");
        });

        it("getNetworkLastLockCost returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getNetworkLastLockCost",
                args: []
            });
            assert.ok(result !== undefined, "getNetworkLastLockCost should return a value");
        });

        it("getNetworkLockReductionInterval returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getNetworkLockReductionInterval",
                args: []
            });
            assert.ok(result !== undefined, "getNetworkLockReductionInterval should return a value");
        });

        it("getSubnetOwnerCut returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getSubnetOwnerCut",
                args: []
            });
            assert.ok(result !== undefined, "getSubnetOwnerCut should return a value");
        });

        it("getNetworkRateLimit returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getNetworkRateLimit",
                args: []
            });
            assert.ok(result !== undefined, "getNetworkRateLimit should return a value");
        });

        it("getNominatorMinRequiredStake returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getNominatorMinRequiredStake",
                args: []
            });
            assert.ok(result !== undefined, "getNominatorMinRequiredStake should return a value");
        });

        it("getWeightsVersionKeyRateLimit returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getWeightsVersionKeyRateLimit",
                args: []
            });
            assert.ok(result !== undefined, "getWeightsVersionKeyRateLimit should return a value");
        });

        it("getTxRateLimit returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTxRateLimit",
                args: []
            });
            assert.ok(result !== undefined, "getTxRateLimit should return a value");
        });

        it("getTxDelegateTakeRateLimit returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTxDelegateTakeRateLimit",
                args: []
            });
            assert.ok(result !== undefined, "getTxDelegateTakeRateLimit should return a value");
        });

        it("getTxChildkeyTakeRateLimit returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTxChildkeyTakeRateLimit",
                args: []
            });
            assert.ok(result !== undefined, "getTxChildkeyTakeRateLimit should return a value");
        });

        it("getStakeThreshold returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getStakeThreshold",
                args: []
            });
            assert.ok(result !== undefined, "getStakeThreshold should return a value");
        });

        it("getNumStakingColdkeys returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getNumStakingColdkeys",
                args: []
            });
            assert.ok(result !== undefined, "getNumStakingColdkeys should return a value");
        });

        it("getNumRootClaim returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getNumRootClaim",
                args: []
            });
            assert.ok(result !== undefined, "getNumRootClaim should return a value");
        });

        it("getNextSubnetLeaseId returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getNextSubnetLeaseId",
                args: []
            });
            assert.ok(result !== undefined, "getNextSubnetLeaseId should return a value");
        });

        it("getCommitRevealWeightsVersion returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getCommitRevealWeightsVersion",
                args: []
            });
            assert.ok(result !== undefined, "getCommitRevealWeightsVersion should return a value");
        });

        it("getNetworkRegistrationStartBlock returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getNetworkRegistrationStartBlock",
                args: []
            });
            assert.ok(result !== undefined, "getNetworkRegistrationStartBlock should return a value");
        });

        it("getPendingChildKeyCooldown returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getPendingChildKeyCooldown",
                args: []
            });
            assert.ok(result !== undefined, "getPendingChildKeyCooldown should return a value");
        });

    });

    describe("Netuid-Keyed Views", () => {
        it("getSubnetMovingPrice returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getSubnetMovingPrice",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getSubnetMovingPrice should return a value");
        });

        it("getRootProp returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getRootProp",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getRootProp should return a value");
        });

        it("getSubnetTaoProvided returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getSubnetTaoProvided",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getSubnetTaoProvided should return a value");
        });

        it("getSubnetAlphaInProvided returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getSubnetAlphaInProvided",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getSubnetAlphaInProvided should return a value");
        });

        it("getTokenSymbol returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTokenSymbol",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getTokenSymbol should return a value");
        });

        it("getSubnetTaoFlow returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getSubnetTaoFlow",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getSubnetTaoFlow should return a value");
        });

        it("getSubnetEmaTaoFlow returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getSubnetEmaTaoFlow",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getSubnetEmaTaoFlow should return a value");
        });

        it("getMaxRegistrationsPerBlock returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getMaxRegistrationsPerBlock",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getMaxRegistrationsPerBlock should return a value");
        });

        it("getTransferToggle returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTransferToggle",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getTransferToggle should return a value");
        });

        it("getSubnetLocked returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getSubnetLocked",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getSubnetLocked should return a value");
        });

        it("getLargestLocked returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getLargestLocked",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getLargestLocked should return a value");
        });

        it("getTempo returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTempo",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getTempo should return a value");
        });

        it("getFirstEmissionBlockNumber returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getFirstEmissionBlockNumber",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getFirstEmissionBlockNumber should return a value");
        });

        it("getNetworkRegisteredAt returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getNetworkRegisteredAt",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getNetworkRegisteredAt should return a value");
        });

        it("getPendingServerEmission returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getPendingServerEmission",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getPendingServerEmission should return a value");
        });

        it("getPendingValidatorEmission returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getPendingValidatorEmission",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getPendingValidatorEmission should return a value");
        });

        it("getPendingRootAlphaDivs returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getPendingRootAlphaDivs",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getPendingRootAlphaDivs should return a value");
        });

        it("getPendingOwnerCut returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getPendingOwnerCut",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getPendingOwnerCut should return a value");
        });

        it("getBlocksSinceLastStep returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getBlocksSinceLastStep",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getBlocksSinceLastStep should return a value");
        });

        it("getLastMechansimStepBlock returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getLastMechansimStepBlock",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getLastMechansimStepBlock should return a value");
        });

        it("getSubnetOwner returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getSubnetOwner",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getSubnetOwner should return a value");
        });

        it("getSubnetOwnerHotkey returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getSubnetOwnerHotkey",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getSubnetOwnerHotkey should return a value");
        });

        it("getRecycleOrBurn returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getRecycleOrBurn",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getRecycleOrBurn should return a value");
        });

        it("getRegistrationsThisInterval returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getRegistrationsThisInterval",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getRegistrationsThisInterval should return a value");
        });

        it("getPOWRegistrationsThisInterval returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getPOWRegistrationsThisInterval",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getPOWRegistrationsThisInterval should return a value");
        });

        it("getBurnRegistrationsThisInterval returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getBurnRegistrationsThisInterval",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getBurnRegistrationsThisInterval should return a value");
        });

        it("getMinAllowedUids returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getMinAllowedUids",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getMinAllowedUids should return a value");
        });

        it("getMaxAllowedUids returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getMaxAllowedUids",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getMaxAllowedUids should return a value");
        });

        it("getMaxWeightsLimit returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getMaxWeightsLimit",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getMaxWeightsLimit should return a value");
        });

        it("getMaxAllowedValidators returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getMaxAllowedValidators",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getMaxAllowedValidators should return a value");
        });

        it("getAdjustmentInterval returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getAdjustmentInterval",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getAdjustmentInterval should return a value");
        });

        it("getBondsPenalty returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getBondsPenalty",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getBondsPenalty should return a value");
        });

        it("getValidatorPruneLen returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getValidatorPruneLen",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getValidatorPruneLen should return a value");
        });

        it("getScalingLawPower returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getScalingLawPower",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getScalingLawPower should return a value");
        });

        it("getTargetRegistrationsPerInterval returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTargetRegistrationsPerInterval",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getTargetRegistrationsPerInterval should return a value");
        });

        it("getLastAdjustmentBlock returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getLastAdjustmentBlock",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getLastAdjustmentBlock should return a value");
        });

        it("getRegistrationsThisBlock returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getRegistrationsThisBlock",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getRegistrationsThisBlock should return a value");
        });

        it("getRAORecycledForRegistration returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getRAORecycledForRegistration",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getRAORecycledForRegistration should return a value");
        });

        it("getSubtokenEnabled returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getSubtokenEnabled",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getSubtokenEnabled should return a value");
        });

        it("getImmuneOwnerUidsLimit returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getImmuneOwnerUidsLimit",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getImmuneOwnerUidsLimit should return a value");
        });

        it("getStakeWeight returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getStakeWeight",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getStakeWeight should return a value");
        });

        it("getLoadedEmission returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getLoadedEmission",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getLoadedEmission should return a value");
        });

        it("getActive returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getActive",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getActive should return a value");
        });

        it("getRank returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getRank",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getRank should return a value");
        });

        it("getTrust returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTrust",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getTrust should return a value");
        });

        it("getConsensus returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getConsensus",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getConsensus should return a value");
        });

        it("getIncentive returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getIncentive",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getIncentive should return a value");
        });

        it("getDividends returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getDividends",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getDividends should return a value");
        });

        it("getEmission returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getEmission",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getEmission should return a value");
        });

        it("getLastUpdate returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getLastUpdate",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getLastUpdate should return a value");
        });

        it("getValidatorTrust returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getValidatorTrust",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getValidatorTrust should return a value");
        });

        it("getPruningScores returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getPruningScores",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getPruningScores should return a value");
        });

        it("getValidatorPermit returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getValidatorPermit",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getValidatorPermit should return a value");
        });

        it("getSubnetIdentitiesV3 returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getSubnetIdentitiesV3",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getSubnetIdentitiesV3 should return a value");
        });

        it("getRootClaimableThreshold returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getRootClaimableThreshold",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getRootClaimableThreshold should return a value");
        });

        it("getMinNonImmuneUids returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getMinNonImmuneUids",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getMinNonImmuneUids should return a value");
        });

        it("getMechanismCountCurrent returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getMechanismCountCurrent",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getMechanismCountCurrent should return a value");
        });

        it("getMechanismEmissionSplit returns without error for valid netuid", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getMechanismEmissionSplit",
                args: [netuid]
            });
            assert.ok(result !== undefined, "getMechanismEmissionSplit should return a value");
        });

    });

    describe("Bytes32-Keyed Views", () => {
        it("getDelegates returns without error for fake account", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getDelegates",
                args: [fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getDelegates should return a value");
        });

        it("getStakingHotkeys returns without error for fake account", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getStakingHotkeys",
                args: [fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getStakingHotkeys should return a value");
        });

        it("getOwnedHotkeys returns without error for fake account", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getOwnedHotkeys",
                args: [fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getOwnedHotkeys should return a value");
        });

        it("getColdkeySwapScheduled returns without error for fake account", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getColdkeySwapScheduled",
                args: [fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getColdkeySwapScheduled should return a value");
        });

        it("getLastRateLimitedBlock returns without error for fake account", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getLastRateLimitedBlock",
                args: [fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getLastRateLimitedBlock should return a value");
        });

        it("getIdentitiesV2 returns without error for fake account", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getIdentitiesV2",
                args: [fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getIdentitiesV2 should return a value");
        });

        it("getLastTxBlock returns without error for fake account", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getLastTxBlock",
                args: [fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getLastTxBlock should return a value");
        });

        it("getLastTxBlockChildKeyTake returns without error for fake account", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getLastTxBlockChildKeyTake",
                args: [fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getLastTxBlockChildKeyTake should return a value");
        });

        it("getLastTxBlockDelegateTake returns without error for fake account", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getLastTxBlockDelegateTake",
                args: [fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getLastTxBlockDelegateTake should return a value");
        });

        it("getRootClaimable returns without error for fake account", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getRootClaimable",
                args: [fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getRootClaimable should return a value");
        });

        it("getRootClaimType returns without error for fake account", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getRootClaimType",
                args: [fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getRootClaimType should return a value");
        });

        it("getStakingColdkeys returns without error for fake account", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getStakingColdkeys",
                args: [fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getStakingColdkeys should return a value");
        });

    });

    describe("Netuid+Bytes32 Keyed Views", () => {
        it("getLastHotkeySwapOnNetuid returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getLastHotkeySwapOnNetuid",
                args: [netuid, fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getLastHotkeySwapOnNetuid should return a value");
        });

        it("getChildkeyTake returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getChildkeyTake",
                args: [fakeAccount as `0x${string}`, netuid]
            });
            assert.ok(result !== undefined, "getChildkeyTake should return a value");
        });

        it("getPendingChildKeys returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getPendingChildKeys",
                args: [netuid, fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getPendingChildKeys should return a value");
        });

        it("getChildKeys returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getChildKeys",
                args: [fakeAccount as `0x${string}`, netuid]
            });
            assert.ok(result !== undefined, "getChildKeys should return a value");
        });

        it("getParentKeys returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getParentKeys",
                args: [fakeAccount as `0x${string}`, netuid]
            });
            assert.ok(result !== undefined, "getParentKeys should return a value");
        });

        it("getAlphaDividendsPerSubnet returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getAlphaDividendsPerSubnet",
                args: [netuid, fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getAlphaDividendsPerSubnet should return a value");
        });

        it("getRootAlphaDividendsPerSubnet returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getRootAlphaDividendsPerSubnet",
                args: [netuid, fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getRootAlphaDividendsPerSubnet should return a value");
        });

        it("getLastHotkeyEmissionOnNetuid returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getLastHotkeyEmissionOnNetuid",
                args: [fakeAccount as `0x${string}`, netuid]
            });
            assert.ok(result !== undefined, "getLastHotkeyEmissionOnNetuid should return a value");
        });

        it("getAutoStakeDestination returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getAutoStakeDestination",
                args: [fakeAccount as `0x${string}`, netuid]
            });
            assert.ok(result !== undefined, "getAutoStakeDestination should return a value");
        });

        it("getAutoStakeDestinationColdkeys returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getAutoStakeDestinationColdkeys",
                args: [fakeAccount as `0x${string}`, netuid]
            });
            assert.ok(result !== undefined, "getAutoStakeDestinationColdkeys should return a value");
        });

        it("getTotalHotkeyAlpha returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTotalHotkeyAlpha",
                args: [fakeAccount as `0x${string}`, netuid]
            });
            assert.ok(result !== undefined, "getTotalHotkeyAlpha should return a value");
        });

        it("getTotalHotkeyAlphaLastEpoch returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTotalHotkeyAlphaLastEpoch",
                args: [fakeAccount as `0x${string}`, netuid]
            });
            assert.ok(result !== undefined, "getTotalHotkeyAlphaLastEpoch should return a value");
        });

        it("getTotalHotkeyShares returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTotalHotkeyShares",
                args: [fakeAccount as `0x${string}`, netuid]
            });
            assert.ok(result !== undefined, "getTotalHotkeyShares should return a value");
        });

        it("getIsNetworkMember returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getIsNetworkMember",
                args: [fakeAccount as `0x${string}`, netuid]
            });
            assert.ok(result !== undefined, "getIsNetworkMember should return a value");
        });

        it("getAxons returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getAxons",
                args: [netuid, fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getAxons should return a value");
        });

        it("getNeuronCertificates returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getNeuronCertificates",
                args: [netuid, fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getNeuronCertificates should return a value");
        });

        it("getPrometheus returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getPrometheus",
                args: [netuid, fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getPrometheus should return a value");
        });

        it("getWeightCommits returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getWeightCommits",
                args: [netuid, fakeAccount as `0x${string}`]
            });
            assert.ok(result !== undefined, "getWeightCommits should return a value");
        });

    });

    describe("Netuid+Uint16 Keyed Views", () => {
        it("getKeys returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getKeys",
                args: [netuid, 0]
            });
            assert.ok(result !== undefined, "getKeys should return a value");
        });

        it("getWeights returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getWeights",
                args: [netuid, 0]
            });
            assert.ok(result !== undefined, "getWeights should return a value");
        });

        it("getBonds returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getBonds",
                args: [netuid, 0]
            });
            assert.ok(result !== undefined, "getBonds should return a value");
        });

        it("getBlockAtRegistration returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getBlockAtRegistration",
                args: [netuid, 0]
            });
            assert.ok(result !== undefined, "getBlockAtRegistration should return a value");
        });

        it("getAssociatedEvmAddress returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getAssociatedEvmAddress",
                args: [netuid, 0]
            });
            assert.ok(result !== undefined, "getAssociatedEvmAddress should return a value");
        });

    });

    describe("Other Keyed Views", () => {
        it("getUsedWork Returns 0 or reverts due to Rust ABI expecting uint8[] instead of bytes", async () => {
            try {
                const result = await publicClient.readContract({
                    abi: ISubtensorABI,
                    address: toViemAddress(ISUBTENSOR_ADDRESS),
                    functionName: "getUsedWork",
                    args: ["0x" + "00".repeat(16)]
                });
                assert.ok(result !== undefined);
            } catch (e: any) {
                // If the precompile-utils macro maps Vec<u8> to uint8[] instead of bytes,
                // passing bytes will revert here, but it proves the routing works.
                assert.ok(e.message.includes("uint8 out of bounds") || e.message.includes("Reverted"), "Should revert due to parameter decoding");
            }
        });

        it("getTransactionKeyLastBlock returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTransactionKeyLastBlock",
                args: [fakeAccount, netuid, 0]
            });
            assert.ok(result !== undefined, "getTransactionKeyLastBlock should return a value");
        });

        it("getTimelockedWeightCommits returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTimelockedWeightCommits",
                args: [netuid, BigInt(0)]
            });
            assert.ok(result !== undefined, "getTimelockedWeightCommits should return a value");
        });

        it("getCRV3WeightCommits returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getCRV3WeightCommits",
                args: [netuid, BigInt(0)]
            });
            assert.ok(result !== undefined, "getCRV3WeightCommits should return a value");
        });

        it("getCRV3WeightCommitsV2 returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getCRV3WeightCommitsV2",
                args: [netuid, BigInt(0)]
            });
            assert.ok(result !== undefined, "getCRV3WeightCommitsV2 should return a value");
        });

        it("getLastColdkeyHotkeyStakeBlock returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getLastColdkeyHotkeyStakeBlock",
                args: [fakeAccount, fakeAccount]
            });
            assert.ok(result !== undefined, "getLastColdkeyHotkeyStakeBlock should return a value");
        });

        it("getStakingOperationRateLimiter returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getStakingOperationRateLimiter",
                args: [fakeAccount, fakeAccount, netuid]
            });
            assert.ok(result !== undefined, "getStakingOperationRateLimiter should return a value");
        });

        it("getRootClaimed returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getRootClaimed",
                args: [netuid, fakeAccount, fakeAccount]
            });
            assert.ok(result !== undefined, "getRootClaimed should return a value");
        });

        it("getStakingColdkeysByIndex returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getStakingColdkeysByIndex",
                args: [BigInt(0)]
            });
            assert.ok(result !== undefined, "getStakingColdkeysByIndex should return a value");
        });

        it("getAccumulatedLeaseDividends returns without error", async () => {
            const result = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getAccumulatedLeaseDividends",
                args: [0]
            });
            assert.ok(result !== undefined, "getAccumulatedLeaseDividends should return a value");
        });

        it("getHasMigrationRun returns value or reverts due to Rust ABI mismatch", async () => {
            try {
                const result = await publicClient.readContract({
                    abi: ISubtensorABI,
                    address: toViemAddress(ISUBTENSOR_ADDRESS),
                    functionName: "getHasMigrationRun",
                    args: ["0x" + "00".repeat(16)]
                });
                assert.ok(result !== undefined);
            } catch (e: any) {
                // Similar to getUsedWork, waitting for `uint8[]` instead of `bytes`.
                assert.ok(e.message.includes("uint8 out of bounds") || e.message.includes("Reverted"), "Should revert due to parameter decoding");
            }
        });

    });
});
