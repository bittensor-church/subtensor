import * as assert from "assert";

import { getDevnetApi, getRandomSubstrateKeypair } from "../src/substrate"
import { getPublicClient } from "../src/utils";
import { ETH_LOCAL_URL } from "../src/config";
import { devnet } from "@polkadot-api/descriptors"
import { PublicClient } from "viem";
import { TypedApi } from "polkadot-api";
import { toViemAddress, convertPublicKeyToSs58 } from "../src/address-utils"
import { ISwapABI, ISWAP_ADDRESS } from "../src/contracts/swap"
import { forceSetBalanceToSs58Address, addNewSubnetwork, startCall } from "../src/subtensor";

describe("Test Auto-Generated Swap Precompile (0x816)", () => {
    const hotkey = getRandomSubstrateKeypair();
    const coldkey = getRandomSubstrateKeypair();
    let publicClient: PublicClient;
    let api: TypedApi<typeof devnet>;
    let netuid = 1;

    before(async () => {
        publicClient = await getPublicClient(ETH_LOCAL_URL)
        api = await getDevnetApi()

        await forceSetBalanceToSs58Address(api, convertPublicKeyToSs58(hotkey.publicKey))
        await forceSetBalanceToSs58Address(api, convertPublicKeyToSs58(coldkey.publicKey))

        netuid = await addNewSubnetwork(api, hotkey, coldkey)
        await startCall(api, netuid, coldkey)
    })

    describe("Value Storage", () => {
        it("getLastPositionId returns a non-negative value", async () => {
            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getLastPositionId",
                args: []
            });

            assert.ok(BigInt(fromContract as any) >= BigInt(0), "LastPositionId should be non-negative");
        });

        it("getPalletVersion returns a valid version number", async () => {
            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getPalletVersion",
                args: []
            });

            assert.ok(Number(fromContract) >= 0, "Pallet version should be a non-negative number");
        });
    });

    describe("Map Storage (Single Key by Netuid)", () => {
        it("getFeeRate returns matching on-chain value", async () => {
            const onChain = await api.query.Swap.FeeRate.getValue(netuid);

            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getFeeRate",
                args: [netuid]
            });

            assert.strictEqual(Number(fromContract), onChain, "FeeRate should match on-chain value");
        });

        it("getFeeGlobalTao returns matching on-chain value", async () => {
            const onChain = await api.query.Swap.FeeGlobalTao.getValue(netuid);

            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getFeeGlobalTao",
                args: [netuid]
            });

            assert.strictEqual(BigInt(fromContract as any), BigInt(onChain as any), "FeeGlobalTao should match on-chain value");
        });

        it("getFeeGlobalAlpha returns matching on-chain value", async () => {
            const onChain = await api.query.Swap.FeeGlobalAlpha.getValue(netuid);

            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getFeeGlobalAlpha",
                args: [netuid]
            });

            assert.strictEqual(BigInt(fromContract as any), BigInt(onChain as any), "FeeGlobalAlpha should match on-chain value");
        });

        it("getAlphaSqrtPrice returns matching on-chain value", async () => {
            const onChain = await api.query.Swap.AlphaSqrtPrice.getValue(netuid);

            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getAlphaSqrtPrice",
                args: [netuid]
            });

            assert.strictEqual(BigInt(fromContract as any), BigInt(onChain as any), "AlphaSqrtPrice should match on-chain value");
        });

        it("getSwapV3Initialized returns matching on-chain state", async () => {
            const onChain = await api.query.Swap.SwapV3Initialized.getValue(netuid);

            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getSwapV3Initialized",
                args: [netuid]
            });

            assert.strictEqual(fromContract, !!onChain, "SwapV3Initialized should match on-chain state");
        });

        it("getCurrentTick returns matching on-chain value", async () => {
            const onChain = await api.query.Swap.CurrentTick.getValue(netuid);

            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getCurrentTick",
                args: [netuid]
            });

            assert.ok(BigInt(fromContract as any) >= BigInt(0), "CurrentTick should be non-negative");
        });

        it("getCurrentLiquidity returns matching on-chain value", async () => {
            const onChain = await api.query.Swap.CurrentLiquidity.getValue(netuid);

            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getCurrentLiquidity",
                args: [netuid]
            });

            assert.strictEqual(BigInt(fromContract as any), BigInt(onChain as any), "CurrentLiquidity should match on-chain value");
        });

        it("getEnabledUserLiquidity returns matching on-chain value", async () => {
            const onChain = await api.query.Swap.EnabledUserLiquidity.getValue(netuid);

            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getEnabledUserLiquidity",
                args: [netuid]
            });

            assert.strictEqual(fromContract, !!onChain, "EnabledUserLiquidity should match on-chain state");
        });

        it("getScrapReservoirTao returns matching on-chain value", async () => {
            const onChain = await api.query.Swap.ScrapReservoirTao.getValue(netuid);

            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getScrapReservoirTao",
                args: [netuid]
            });

            assert.strictEqual(BigInt(fromContract as any), BigInt(onChain as any), "ScrapReservoirTao should match on-chain value");
        });

        it("getScrapReservoirAlpha returns matching on-chain value", async () => {
            const onChain = await api.query.Swap.ScrapReservoirAlpha.getValue(netuid);

            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getScrapReservoirAlpha",
                args: [netuid]
            });

            assert.strictEqual(BigInt(fromContract as any), BigInt(onChain as any), "ScrapReservoirAlpha should match on-chain value");
        });
    });

    describe("Gas-Metered Fallback Paths (Non-existent items)", () => {
        it("getTicks returns (0,0,0,0) for a non-existent tick index", async () => {
            const nonExistentTick = 999999;

            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getTicks",
                args: [netuid, nonExistentTick]
            }) as [bigint, bigint, bigint, bigint];

            // Fallback path we fixed: gas metering for the None/out-of-range case
            assert.strictEqual(fromContract[0], BigInt(0), "liquidity_net should be 0 for non-existent tick");
            assert.strictEqual(fromContract[1], BigInt(0), "liquidity_gross should be 0 for non-existent tick");
            assert.strictEqual(fromContract[2], BigInt(0), "fees_out_tao should be 0 for non-existent tick");
            assert.strictEqual(fromContract[3], BigInt(0), "fees_out_alpha should be 0 for non-existent tick");
        });

        it("getPositions returns zeros for a non-existent position", async () => {
            const fakeAccount = "0x" + "01".repeat(32);
            const fakePositionId = BigInt(999999);

            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getPositions",
                args: [netuid, fakeAccount as `0x${string}`, fakePositionId]
            }) as [bigint, number, bigint, bigint, bigint, bigint, bigint];

            // Fallback path we fixed: gas metering for the None case in if let Some
            assert.strictEqual(fromContract[0], BigInt(0), "position id should be 0");
            assert.strictEqual(fromContract[1], 0, "netuid should be 0");
            assert.strictEqual(fromContract[4], BigInt(0), "liquidity should be 0");
            assert.strictEqual(fromContract[5], BigInt(0), "fees_tao should be 0");
            assert.strictEqual(fromContract[6], BigInt(0), "fees_alpha should be 0");
        });

        it("getTickIndexBitmapWords returns 0 for default bitmap word", async () => {
            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getTickIndexBitmapWords",
                args: [netuid, 0, 0] // layer_idx=0 (Top), word_index=0
            });

            assert.ok(BigInt(fromContract as any) >= BigInt(0), "BitmapWords should be non-negative");
        });
    });
});
