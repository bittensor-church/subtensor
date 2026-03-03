import * as assert from "assert";

import { getDevnetApi } from "../src/substrate";
import { getPublicClient } from "../src/utils";
import { ETH_LOCAL_URL } from "../src/config";
import { devnet } from "@polkadot-api/descriptors";
import { PublicClient } from "viem";
import { TypedApi } from "polkadot-api";
import { toViemAddress } from "../src/address-utils";
import { ISCHEDULER_ADDRESS, ISchedulerABI } from "../src/contracts/scheduler";

describe("Test Scheduler Precompile (0x813)", () => {
    let publicClient: PublicClient;
    let api: TypedApi<typeof devnet>;

    before(async () => {
        publicClient = await getPublicClient(ETH_LOCAL_URL);
        api = await getDevnetApi();
    });

    describe("View Storage Functions", () => {
        it("getIncompleteSince returns 0 when no incomplete agenda", async () => {
            const onChain = await api.query.Scheduler.IncompleteSince.getValue();

            const fromContract = await publicClient.readContract({
                abi: ISchedulerABI,
                address: toViemAddress(ISCHEDULER_ADDRESS),
                functionName: "getIncompleteSince",
                args: []
            });

            // When no incomplete agenda exists, both should resolve to 0
            const expectedValue = onChain !== undefined ? BigInt(onChain as any) : BigInt(0);
            assert.strictEqual(BigInt(fromContract as any), expectedValue, "IncompleteSince should match on-chain value");
        });

        it("getLookup returns (0, 0) for a non-existent lookup key", async () => {
            const fakeName = "0x" + "00".repeat(32);

            const fromContract = await publicClient.readContract({
                abi: ISchedulerABI,
                address: toViemAddress(ISCHEDULER_ADDRESS),
                functionName: "getLookup",
                args: [fakeName as `0x${string}`]
            }) as [bigint, number];

            assert.strictEqual(fromContract[0], BigInt(0), "block_number should be 0 for non-existent lookup");
            assert.strictEqual(fromContract[1], 0, "task_index should be 0 for non-existent lookup");
        });

        it("getPalletVersion returns a valid version number", async () => {
            const fromContract = await publicClient.readContract({
                abi: ISchedulerABI,
                address: toViemAddress(ISCHEDULER_ADDRESS),
                functionName: "getPalletVersion",
                args: []
            });

            assert.ok(Number(fromContract) >= 0, "Pallet version should be a non-negative number");
        });
    });
});
