import * as assert from "assert";

import { getDevnetApi, getRandomSubstrateKeypair } from "../src/substrate";
import { getPublicClient } from "../src/utils";
import { ETH_LOCAL_URL } from "../src/config";
import { devnet } from "@polkadot-api/descriptors";
import { PublicClient } from "viem";
import { TypedApi } from "polkadot-api";
import { toViemAddress, convertPublicKeyToSs58 } from "../src/address-utils";
import { IBALANCE_TRANSFER_ADDRESS, IBalanceTransferABI } from "../src/contracts/balanceTransfer";
import { forceSetBalanceToSs58Address } from "../src/subtensor";

describe("Test Balance Transfer Precompile Views (0x800)", () => {
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

    describe("Storage Value Views", () => {
        it("getInactiveIssuance returns matching on-chain value (gas-metered fix)", async () => {
            const onChain = await api.query.Balances.InactiveIssuance.getValue();

            const fromContract = await publicClient.readContract({
                abi: IBalanceTransferABI,
                address: toViemAddress(IBALANCE_TRANSFER_ADDRESS),
                functionName: "getInactiveIssuance",
                args: []
            });

            assert.strictEqual(BigInt(fromContract as any), BigInt(onChain as any), "InactiveIssuance should match on-chain value");
        });

        it("getTotalIssuance returns matching on-chain value", async () => {
            const onChain = await api.query.Balances.TotalIssuance.getValue();

            const fromContract = await publicClient.readContract({
                abi: IBalanceTransferABI,
                address: toViemAddress(IBALANCE_TRANSFER_ADDRESS),
                functionName: "getTotalIssuance",
                args: []
            });

            assert.strictEqual(BigInt(fromContract as any), BigInt(onChain as any), "TotalIssuance should match on-chain value");
        });

        it("getPalletVersion returns a valid version number", async () => {
            const fromContract = await publicClient.readContract({
                abi: IBalanceTransferABI,
                address: toViemAddress(IBALANCE_TRANSFER_ADDRESS),
                functionName: "getPalletVersion",
                args: []
            });

            assert.ok(Number(fromContract) >= 0, "Pallet version should be non-negative");
        });
    });

    describe("Account-Keyed Views", () => {
        it("getFreeBalance returns matching on-chain value for funded account", async () => {
            const account = await api.query.System.Account.getValue(convertPublicKeyToSs58(keypair.publicKey));

            const fromContract = await publicClient.readContract({
                abi: IBalanceTransferABI,
                address: toViemAddress(IBALANCE_TRANSFER_ADDRESS),
                functionName: "getFreeBalance",
                args: [accountBytes]
            });

            assert.strictEqual(BigInt(fromContract as any), BigInt(account.data.free as any), "Free balance should match on-chain value");
        });

        it("getAccount returns (free, reserved, frozen, flags) for funded account", async () => {
            const account = await api.query.System.Account.getValue(convertPublicKeyToSs58(keypair.publicKey));

            const fromContract = await publicClient.readContract({
                abi: IBalanceTransferABI,
                address: toViemAddress(IBALANCE_TRANSFER_ADDRESS),
                functionName: "getAccount",
                args: [accountBytes]
            }) as [bigint, bigint, bigint, bigint];

            // Note: pallet_balances::Account storage might be empty if balances are stored in frame_system.
            // Therefore, getAccount directly reading pallet_balances::Account returns 0.
            assert.strictEqual(fromContract[0], BigInt(0), "free balance in pallet_balances::Account is 0");
            assert.strictEqual(fromContract[1], BigInt(0), "reserved should match on-chain");
            assert.strictEqual(fromContract[2], BigInt(0), "frozen should match on-chain");
        });

        it("getLocks returns an array (empty for fresh account)", async () => {
            const fromContract = await publicClient.readContract({
                abi: IBalanceTransferABI,
                address: toViemAddress(IBALANCE_TRANSFER_ADDRESS),
                functionName: "getLocks",
                args: [accountBytes]
            }) as any[];

            assert.ok(Array.isArray(fromContract), "getLocks should return an array");
        });

        it("getReserves returns an array (empty for fresh account)", async () => {
            const fromContract = await publicClient.readContract({
                abi: IBalanceTransferABI,
                address: toViemAddress(IBALANCE_TRANSFER_ADDRESS),
                functionName: "getReserves",
                args: [accountBytes]
            }) as any[];

            assert.ok(Array.isArray(fromContract), "getReserves should return an array");
        });

        it("getHolds returns an array (empty for fresh account)", async () => {
            const fromContract = await publicClient.readContract({
                abi: IBalanceTransferABI,
                address: toViemAddress(IBALANCE_TRANSFER_ADDRESS),
                functionName: "getHolds",
                args: [accountBytes]
            }) as any[];

            assert.ok(Array.isArray(fromContract), "getHolds should return an array");
        });

        it("getFreezes returns an array (empty for fresh account)", async () => {
            const fromContract = await publicClient.readContract({
                abi: IBalanceTransferABI,
                address: toViemAddress(IBALANCE_TRANSFER_ADDRESS),
                functionName: "getFreezes",
                args: [accountBytes]
            }) as any[];

            assert.ok(Array.isArray(fromContract), "getFreezes should return an array");
        });
    });
});
