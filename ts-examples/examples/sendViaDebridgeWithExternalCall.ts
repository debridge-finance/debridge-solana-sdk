/* eslint-disable no-console */
import { crypto, helpers } from "@debridge-finance/solana-utils";

import { buildSendContext } from "./contextBuilding";
import { initAll, sendTransaction, getArgsForExtCall } from "./helpers";

async function main() {
  const { connection, wallet, example, deBridge } = initAll();
  const parsed = getArgsForExtCall();

  const builder = example.methods.sendViaDebridgeWithExternalCall(
    parsed.amount,
    Array.from(crypto.normalizeChainId(parsed.targetChain)),
    helpers.hexToBuffer(parsed.receiver),
    parsed.executionFee,
    helpers.hexToBuffer(parsed.fallbackAddress),
    Array.from({ length: 32 }).fill(0) as number[],
    parsed.data,
  );

  const remainingAccounts = await buildSendContext(
    deBridge,
    wallet.publicKey,
    parsed.tokenMint,
    parsed.targetChain,
    parsed.receiver,
    false,
    parsed.mode,
    {
      data: parsed.data,
      flags: 0,
      fallbackAddress: parsed.fallbackAddress,
    },
  );
  builder.remainingAccounts(remainingAccounts);
  await sendTransaction(await builder.transaction(), connection, wallet);
}

main().catch(console.error);
