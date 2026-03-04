import * as assert from "assert";

import { getDevnetApi } from "../src/substrate";
import { getPublicClient } from "../src/utils";
import { ETH_LOCAL_URL } from "../src/config";
import { devnet } from "@polkadot-api/descriptors";
import { PublicClient } from "viem";
import { TypedApi } from "polkadot-api";
import { toViemAddress } from "../src/address-utils";
import { ITIMESTAMP_ADDRESS, ITimestampABI } from "../src/contracts/timestamp";

describe("Test Timestamp Precompile (0x812)", () => {
    let publicClient: PublicClient;
    let api: TypedApi<typeof devnet>;

    before(async () => {
        publicClient = await getPublicClient(ETH_LOCAL_URL);
        api = await getDevnetApi();
    });

    describe("View Storage Functions", () => {
        it("getNow returns a positive timestamp matching on-chain value", async () => {
            const onChain = await api.query.Timestamp.Now.getValue();

            const fromContract = await publicClient.readContract({
                abi: ITimestampABI,
                address: toViemAddress(ITIMESTAMP_ADDRESS),
                functionName: "getNow",
                args: []
            });

            const expected = BigInt(onChain as any);
            const actual = BigInt(fromContract as any);
            const diff = actual > expected ? actual - expected : expected - actual;
            assert.ok(diff <= BigInt(5000), `Timestamp.Now should match closely (actual: ${actual}, expected: ${expected})`);
            assert.ok(BigInt(fromContract as any) > BigInt(0), "Timestamp should be positive");
        });

        it("getDidUpdate returns a boolean", async () => {
            const fromContract = await publicClient.readContract({
                abi: ITimestampABI,
                address: toViemAddress(ITIMESTAMP_ADDRESS),
                functionName: "getDidUpdate",
                args: []
            });

            assert.strictEqual(typeof fromContract, "boolean", "getDidUpdate should return a boolean");
        });

        it("getPalletVersion returns a valid version number", async () => {
            const fromContract = await publicClient.readContract({
                abi: ITimestampABI,
                address: toViemAddress(ITIMESTAMP_ADDRESS),
                functionName: "getPalletVersion",
                args: []
            });

            assert.ok(Number(fromContract) >= 0, "Pallet version should be a non-negative number");
        });
    });
});
