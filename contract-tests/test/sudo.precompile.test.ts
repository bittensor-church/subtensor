import * as assert from "assert";

import { getAliceSigner, getDevnetApi } from "../src/substrate";
import { getPublicClient } from "../src/utils";
import { ETH_LOCAL_URL } from "../src/config";
import { devnet } from "@polkadot-api/descriptors";
import { PublicClient } from "viem";
import { TypedApi } from "polkadot-api";
import { toViemAddress } from "../src/address-utils";
import { ISUDO_ADDRESS, ISudoABI } from "../src/contracts/sudo";
import { decodeAddress } from "@polkadot/util-crypto";
import { u8aToHex } from "@polkadot/util";

describe("Test Sudo Precompile (0x814)", () => {
    let publicClient: PublicClient;
    let api: TypedApi<typeof devnet>;

    before(async () => {
        publicClient = await getPublicClient(ETH_LOCAL_URL);
        api = await getDevnetApi();
    });

    describe("View Storage Functions", () => {
        it("getKey returns the current sudo key matching on-chain state", async () => {
            const sudoKeyOnChain = await api.query.Sudo.Key.getValue();

            const fromContract = await publicClient.readContract({
                abi: ISudoABI,
                address: toViemAddress(ISUDO_ADDRESS),
                functionName: "getKey",
                args: []
            }) as `0x${string}`;

            if (sudoKeyOnChain) {
                const expectedBytes = u8aToHex(decodeAddress(sudoKeyOnChain));
                assert.strictEqual(fromContract.toLowerCase(), expectedBytes.toLowerCase(), "Sudo key should match the on-chain sudo key");
            } else {
                // No sudo key set — contract should return zero bytes32
                assert.strictEqual(fromContract, "0x" + "00".repeat(32), "Should return zero bytes32 when no sudo key");
            }
        });

        it("getPalletVersion returns a valid version number", async () => {
            const fromContract = await publicClient.readContract({
                abi: ISudoABI,
                address: toViemAddress(ISUDO_ADDRESS),
                functionName: "getPalletVersion",
                args: []
            });

            assert.ok(Number(fromContract) >= 0, "Pallet version should be a non-negative number");
        });
    });
});
