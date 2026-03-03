import * as assert from "assert";

import { getDevnetApi } from "../src/substrate";
import { getPublicClient } from "../src/utils";
import { ETH_LOCAL_URL } from "../src/config";
import { devnet } from "@polkadot-api/descriptors";
import { PublicClient } from "viem";
import { TypedApi } from "polkadot-api";
import { toViemAddress } from "../src/address-utils";
import { IMULTISIG_ADDRESS, IMultisigABI } from "../src/contracts/multisig";

describe("Test Multisig Precompile (0x815)", () => {
    let publicClient: PublicClient;
    let api: TypedApi<typeof devnet>;

    before(async () => {
        publicClient = await getPublicClient(ETH_LOCAL_URL);
        api = await getDevnetApi();
    });

    describe("View Storage Functions", () => {
        it("getMultisig returns (0, [], 0) for a non-existent multisig", async () => {
            const fakeAccount = "0x" + "01".repeat(32);
            const fakeCallHash = "0x" + "02".repeat(32);

            const fromContract = await publicClient.readContract({
                abi: IMultisigABI,
                address: toViemAddress(IMULTISIG_ADDRESS),
                functionName: "getMultisig",
                args: [fakeAccount as `0x${string}`, fakeCallHash as `0x${string}`]
            }) as [bigint, `0x${string}`[], number];

            assert.strictEqual(fromContract[0], BigInt(0), "deposit should be 0 for non-existent multisig");
            assert.strictEqual(fromContract[1].length, 0, "approvals list should be empty for non-existent multisig");
            assert.strictEqual(fromContract[2], 0, "approvals_len should be 0 for non-existent multisig");
        });

        it("getPalletVersion returns a valid version number", async () => {
            const fromContract = await publicClient.readContract({
                abi: IMultisigABI,
                address: toViemAddress(IMULTISIG_ADDRESS),
                functionName: "getPalletVersion",
                args: []
            });

            assert.ok(Number(fromContract) >= 0, "Pallet version should be a non-negative number");
        });
    });
});
