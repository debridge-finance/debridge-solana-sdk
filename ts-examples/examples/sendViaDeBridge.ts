/* eslint-disable no-console */
import { crypto, helpers } from "@debridge-finance/solana-utils";

import { buildSendContext } from "./contextBuilding";
import { getDefaultArgs, initAll, sendTransaction } from "./helpers";

async function main() {
  const args = getDefaultArgs(true);
  const { connection, wallet, example, deBridge } = initAll();
  const builder = example.methods.sendViaDebridge(
    args.amount,
    Array.from(crypto.normalizeChainId(args.targetChain)),
    helpers.hexToBuffer(args.receiver),
    args.useAssetFee!,
  );
  const remainingAccounts = await buildSendContext(
    deBridge,
    wallet.publicKey,
    args.tokenMint,
    args.targetChain,
    args.receiver,
    args.useAssetFee!,
    args.mode,
  );
  builder.remainingAccounts(remainingAccounts);
  await sendTransaction(await builder.transaction(), connection, wallet);
}

main().catch(console.error);
