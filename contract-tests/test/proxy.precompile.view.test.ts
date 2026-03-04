import * as assert from "assert";

import { getDevnetApi, getRandomSubstrateKeypair } from "../src/substrate";
import { getPublicClient } from "../src/utils";
import { ETH_LOCAL_URL } from "../src/config";
import { devnet } from "@polkadot-api/descriptors";
import { PublicClient } from "viem";
import { TypedApi } from "polkadot-api";
import { toViemAddress, convertPublicKeyToSs58 } from "../src/address-utils";
import { IPROXY_ADDRESS, IProxyABI } from "../src/contracts/proxy";
import { forceSetBalanceToSs58Address } from "../src/subtensor";

describe("Test Proxy Precompile Views (0x80b)", () => {
    const keypair = getRandomSubstrateKeypair();
    let publicClient: PublicClient;
    let api: TypedApi<typeof devnet>;
    let accountBytes: `0x${string}`;

    before(async () => {
        publicClient = await getPublicClient(ETH_LOCAL_URL);
        api = await getDevnetApi();
        await forceSetBalanceToSs58Address(api, convertPublicKeyToSs58(keypair.publicKey));
        accountBytes = ("0x" + Buffer.from(keypair.publicKey).toString("hex")) as `0x${string}`;
    });

    describe("View Storage Functions", () => {
        it("getProxies returns empty array for account with no proxies (gas-metered fix)", async () => {
            const fromContract = await publicClient.readContract({
                abi: IProxyABI,
                address: toViemAddress(IPROXY_ADDRESS),
                functionName: "getProxies",
                args: [accountBytes]
            }) as any[];

            assert.ok(Array.isArray(fromContract), "getProxies should return an array");
            assert.strictEqual(fromContract.length, 0, "Should have no proxies for a fresh account");
        });

        it("getLastCallResult returns (false, false) for account with no proxy calls", async () => {
            const fromContract = await publicClient.readContract({
                abi: IProxyABI,
                address: toViemAddress(IPROXY_ADDRESS),
                functionName: "getLastCallResult",
                args: [accountBytes]
            }) as [boolean, boolean];

            // No proxy calls have been made, so exists=false, success=false
            assert.strictEqual(fromContract[0], false, "exists should be false for account with no proxy calls");
            assert.strictEqual(fromContract[1], false, "success should be false for account with no proxy calls");
        });

        it("getAnnouncements returns empty array for account with no announcements", async () => {
            const fromContract = await publicClient.readContract({
                abi: IProxyABI,
                address: toViemAddress(IPROXY_ADDRESS),
                functionName: "getAnnouncements",
                args: [accountBytes]
            }) as any[];

            assert.ok(Array.isArray(fromContract), "getAnnouncements should return an array");
            assert.strictEqual(fromContract.length, 0, "Should have no announcements for a fresh account");
        });

        it("getPalletVersion returns a valid version number", async () => {
            const fromContract = await publicClient.readContract({
                abi: IProxyABI,
                address: toViemAddress(IPROXY_ADDRESS),
                functionName: "getPalletVersion",
                args: []
            });

            assert.ok(Number(fromContract) >= 0, "Pallet version should be non-negative");
        });
    });
});
