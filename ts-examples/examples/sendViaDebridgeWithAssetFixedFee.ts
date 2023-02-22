/* eslint-disable no-console */
import { crypto, helpers } from "@debridge-finance/solana-utils";
import { AccountMeta } from "@solana/web3.js";

import {
  buildSendContextManual,
  buildSendContextWithClient,
  DefaultArgs,
  initAll,
  prepareDefaultParser,
  sendTransaction,
} from "./helpers";

function parseArgs() {
  const parser = prepareDefaultParser();
  const parsed = parser.parse_args();

  return parsed as DefaultArgs;
}

async function main() {
  const { connection, wallet, example, deBridge } = initAll();
  const parsed = parseArgs();

  const builder = example.methods.sendViaDebridgeWithAssetFixedFee(
    parsed.amount,
    Array.from(crypto.normalizeChainId(parsed.targetChain)),
    helpers.hexToBuffer(parsed.receiver),
  );
  let remainingAccounts: AccountMeta[];
  switch (parsed.mode) {
    case "manual": {
      remainingAccounts = await buildSendContextManual(
        deBridge,
        wallet.publicKey,
        parsed.tokenMint,
        parsed.targetChain,
        crypto.hashExternalCallBytes(),
      );
      break;
    }
    case "client": {
      remainingAccounts = await buildSendContextWithClient(
        deBridge,
        wallet.publicKey,
        parsed.tokenMint,
        parsed.receiver,
        parsed.targetChain,
        true,
      );
      break;
    }
    default: {
      throw new Error("unkown mode");
    }
  }
  builder.remainingAccounts(remainingAccounts);
  await sendTransaction(await builder.transaction(), connection, wallet);
}

main().catch(console.error);
