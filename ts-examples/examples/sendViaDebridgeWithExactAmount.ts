/* eslint-disable no-console */
import { BN } from "@coral-xyz/anchor";
import { crypto, helpers } from "@debridge-finance/solana-utils";
import { AccountMeta } from "@solana/web3.js";

import {
  buildSendContextManual,
  buildSendContextWithClient,
  DefaultArgs,
  initAll,
  ParseBN,
  ParseBool,
  prepareDefaultParser,
  sendTransaction,
} from "./helpers";

function parseArgs() {
  const parser = prepareDefaultParser();
  parser.add_argument("-useAssetFee", "--assetFee", {
    required: false,
    default: false,
    choices: ["false", "true"],
    action: ParseBool,
  });
  parser.add_argument("-executionFee", "--execFee", { required: true, action: ParseBN });
  const parsed = parser.parse_args();
  type ParsedType = DefaultArgs & {
    useAssetFee: boolean;
    executionFee: BN;
  };

  return parsed as ParsedType;
}

async function main() {
  const { connection, wallet, example, deBridge } = initAll();
  const parsed = parseArgs();

  const builder = example.methods.sendViaDebridgeWithExactAmount(
    parsed.amount,
    Array.from(crypto.normalizeChainId(parsed.targetChain)),
    helpers.hexToBuffer(parsed.receiver),
    parsed.executionFee,
    parsed.useAssetFee,
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
        parsed.useAssetFee,
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
