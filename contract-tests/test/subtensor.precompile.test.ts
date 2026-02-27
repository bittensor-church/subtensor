import * as assert from "assert";

import { getDevnetApi, getRandomSubstrateKeypair } from "../src/substrate"
import { getPublicClient } from "../src/utils";
import { ETH_LOCAL_URL } from "../src/config";
import { devnet } from "@polkadot-api/descriptors"
import { PublicClient } from "viem";
import { TypedApi } from "polkadot-api";
import { toViemAddress, convertPublicKeyToSs58 } from "../src/address-utils"
import { ISubtensorABI, ISUBTENSOR_ADDRESS } from "../src/contracts/subtensor"
import { forceSetBalanceToSs58Address, addNewSubnetwork, startCall, setMaxChildkeyTake, setTempo } from "../src/subtensor";

describe("Test Auto-Generated Subtensor Precompile (0x817)", () => {
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

        await setMaxChildkeyTake(api, 257)
        await setTempo(api, netuid, 899)
    })

    describe("Value Storage Maps", () => {
        it("getMaxChildkeyTake returns correct value configured via Substrate API", async () => {
            const maxChildkeyTakeOnChain = await api.query.SubtensorModule.MaxChildkeyTake.getValue();

            const maxFromContract = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getMaxChildkeyTake",
                args: [] // no arguments for StorageValue
            });

            // Need to handle BigInt conversions for U16 depending on abi.
            assert.strictEqual(Number(maxFromContract), maxChildkeyTakeOnChain, "MaxChildkeyTake should match the updated value on chain (257)");
        });

        it("getTxRateLimit returns correctly", async () => {
            const txRateLimitOnChain = await api.query.SubtensorModule.TxRateLimit.getValue();

            const txRateLimitContract = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTxRateLimit",
                args: []
            });

            assert.strictEqual(BigInt(txRateLimitContract as any), txRateLimitOnChain, "Tx rate limit should match");
        });
    });

    describe("Map Storage (Single Key)", () => {
        it("getTempo returns correctly by Netuid", async () => {
            const tempoOnChain = await api.query.SubtensorModule.Tempo.getValue(netuid);

            const tempoContract = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTempo",
                args: [netuid]
            });

            assert.strictEqual(Number(tempoContract), tempoOnChain, "Tempo should match configured value");
        });

        it("getMaxAllowedUids returns correctly by Netuid", async () => {
            const maxOnChain = await api.query.SubtensorModule.MaxAllowedUids.getValue(netuid);

            const maxContract = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getMaxAllowedUids",
                args: [netuid]
            });

            assert.strictEqual(Number(maxContract), maxOnChain, "Max allowed UIDs should match on chain");
        });
    });

    describe("Double Map Storage (Two Keys)", () => {
        it("getTotalHotkeyAlpha returns correct initial alpha by hotkey + netuid", async () => {
            const hotkeyAddr = convertPublicKeyToSs58(hotkey.publicKey);
            const totalAlphaOnChain = await api.query.SubtensorModule.TotalHotkeyAlpha.getValue(hotkeyAddr, netuid);

            // Our Typed precompile mapped AccountIds to H256 (bytes32). 
            // In reality, it should be the raw 32 bytes of the Ed25519/Sr25519 public key
            const hotkeyBytes = "0x" + Buffer.from(hotkey.publicKey).toString("hex");

            const alphaContract = await publicClient.readContract({
                abi: ISubtensorABI,
                address: toViemAddress(ISUBTENSOR_ADDRESS),
                functionName: "getTotalHotkeyAlpha",
                args: [hotkeyBytes, netuid]
            });

            assert.strictEqual(BigInt(alphaContract as any), totalAlphaOnChain, "Total hotkey alpha should match");
        });
    });
});
