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
    // init substrate part
    const hotkey = getRandomSubstrateKeypair();
    const coldkey = getRandomSubstrateKeypair();
    let publicClient: PublicClient;
    let api: TypedApi<typeof devnet>;
    let netuid = 1; // default new subnet from script

    before(async () => {
        publicClient = await getPublicClient(ETH_LOCAL_URL)
        api = await getDevnetApi()

        await forceSetBalanceToSs58Address(api, convertPublicKeyToSs58(hotkey.publicKey))
        await forceSetBalanceToSs58Address(api, convertPublicKeyToSs58(coldkey.publicKey))

        netuid = await addNewSubnetwork(api, hotkey, coldkey)
        await startCall(api, netuid, coldkey)
    })

    describe("Value Storage Maps", () => {
        it("getLpad10 returns correctly by Swap Pallet configured via Substrate API", async () => {
            const lpadOnChain = await api.query.Swap.Lpad10.getValue();

            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getLpad10",
                args: [] // no arguments for StorageValue
            });

            assert.strictEqual(BigInt(fromContract as any), lpadOnChain, "Lpad10 should match the value on chain");
        });
    });

    describe("Map Storage (Single Key)", () => {
        it("getAlphaSqrtPrice returns correctly by Netuid", async () => {
            const sqrtPriceOnChain = await api.query.Swap.AlphaSqrtPrice.getValue(netuid);

            const fromContract = await publicClient.readContract({
                abi: ISwapABI,
                address: toViemAddress(ISWAP_ADDRESS),
                functionName: "getAlphaSqrtPrice",
                args: [netuid]
            });

            assert.strictEqual(BigInt(fromContract as any), BigInt(sqrtPriceOnChain as any), "Alpha Sqrt Price should match on chain");
        });
    });
});
