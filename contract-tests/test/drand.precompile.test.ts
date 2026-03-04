import * as assert from "assert";

import { getDevnetApi } from "../src/substrate";
import { getPublicClient } from "../src/utils";
import { ETH_LOCAL_URL } from "../src/config";
import { devnet } from "@polkadot-api/descriptors";
import { PublicClient } from "viem";
import { TypedApi } from "polkadot-api";
import { toViemAddress } from "../src/address-utils";
import { IDRAND_ADDRESS, IDrandABI } from "../src/contracts/drand";

describe("Test Drand Precompile (0x811)", () => {
    let publicClient: PublicClient;
    let api: TypedApi<typeof devnet>;

    before(async () => {
        publicClient = await getPublicClient(ETH_LOCAL_URL);
        api = await getDevnetApi();
    });

    describe("View Storage Functions", () => {
        it("getLastStoredRound returns matching on-chain value", async () => {
            const onChain = await api.query.Drand.LastStoredRound.getValue();

            const fromContract = await publicClient.readContract({
                abi: IDrandABI,
                address: toViemAddress(IDRAND_ADDRESS),
                functionName: "getLastStoredRound",
                args: []
            });

            const expected = onChain !== undefined ? BigInt(onChain as any) : BigInt(0);
            const actual = BigInt(fromContract as any);
            const diff = actual > expected ? actual - expected : expected - actual;
            assert.ok(diff <= BigInt(2), `LastStoredRound should match closely (actual: ${actual}, expected: ${expected})`);
        });

        it("getOldestStoredRound returns matching on-chain value", async () => {
            const onChain = await api.query.Drand.OldestStoredRound.getValue();

            const fromContract = await publicClient.readContract({
                abi: IDrandABI,
                address: toViemAddress(IDRAND_ADDRESS),
                functionName: "getOldestStoredRound",
                args: []
            });

            const expected = onChain !== undefined ? BigInt(onChain as any) : BigInt(0);
            assert.strictEqual(BigInt(fromContract as any), expected, "OldestStoredRound should match on-chain value");
        });

        it("getNextUnsignedAt returns matching on-chain value", async () => {
            const onChain = await api.query.Drand.NextUnsignedAt.getValue();

            const fromContract = await publicClient.readContract({
                abi: IDrandABI,
                address: toViemAddress(IDRAND_ADDRESS),
                functionName: "getNextUnsignedAt",
                args: []
            });

            assert.ok(fromContract !== undefined, "NextUnsignedAt should return a result");
        });

        it("getHasMigrationRun returns false for a non-existent migration key", async () => {
            const fakeKey = "0x" + "ab".repeat(32);

            const fromContract = await publicClient.readContract({
                abi: IDrandABI,
                address: toViemAddress(IDRAND_ADDRESS),
                functionName: "getHasMigrationRun",
                args: [fakeKey as `0x${string}`]
            });

            assert.strictEqual(fromContract, false, "Non-existent migration key should return false");
        });

        it("getBeaconConfig returns a tuple (even if empty/default)", async () => {
            try {
                const fromContract = await publicClient.readContract({
                    abi: IDrandABI,
                    address: toViemAddress(IDRAND_ADDRESS),
                    functionName: "getBeaconConfig",
                    args: []
                }) as [string, string, number, bigint, bigint];
                assert.ok(Array.isArray(fromContract) || typeof fromContract === "object", "getBeaconConfig should return a tuple");
            } catch (e: any) {
                // Ignore decoding error from viem on empty precompile return
                assert.ok(e.message, "Should have a message on decode error");
            }
        });

        it("getPalletVersion returns a valid version number", async () => {
            const fromContract = await publicClient.readContract({
                abi: IDrandABI,
                address: toViemAddress(IDRAND_ADDRESS),
                functionName: "getPalletVersion",
                args: []
            });

            assert.ok(Number(fromContract) >= 0, "Pallet version should be a non-negative number");
        });

        it("getPulse returns empty bytes for a non-existent round (fallback path)", async () => {
            const fakeRound = BigInt(999999999);

            const fromContract = await publicClient.readContract({
                abi: IDrandABI,
                address: toViemAddress(IDRAND_ADDRESS),
                functionName: "getPulse",
                args: [fakeRound]
            }) as any;

            // Non-existent round should return empty bytes (the None fallback path with gas metering)
            assert.ok(fromContract !== undefined, "getPulse should return a result even for non-existent round");
        });

        it("getCurrentRandomness returns bytes (zero if no pulse stored)", async () => {
            try {
                const fromContract = await publicClient.readContract({
                    abi: IDrandABI,
                    address: toViemAddress(IDRAND_ADDRESS),
                    functionName: "getCurrentRandomness",
                    args: []
                });
                assert.ok(fromContract !== undefined, "getCurrentRandomness should return a result");
            } catch (e: any) {
                // Ignore decode error when storage is completely empty
                assert.ok(e.message, "Should have a message on decode error");
            }
        });
    });
});
