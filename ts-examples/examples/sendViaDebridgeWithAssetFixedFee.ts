/* eslint-disable no-console */
import { crypto, helpers } from "@debridge-finance/solana-utils";

import { buildSendContext } from "./contextBuilding";
import { initAll, sendTransaction, getDefaultArgs } from "./helpers";

async function main() {
  const { connection, wallet, example, deBridge } = initAll();
  const parsed = getDefaultArgs();

  const builder = example.methods.sendViaDebridgeWithAssetFixedFee(
    parsed.amount,
    Array.from(crypto.normalizeChainId(parsed.targetChain)),
    helpers.hexToBuffer(parsed.receiver),
  );
  const remainingAccounts = await buildSendContext(
    deBridge,
    wallet.publicKey,
    parsed.tokenMint,
    parsed.targetChain,
    parsed.receiver,
    true,
    parsed.mode,
  );
  builder.remainingAccounts(remainingAccounts);
  await sendTransaction(await builder.transaction(), connection, wallet);
}

main().catch(console.error);
