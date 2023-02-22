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
  ParseHex,
  prepareDefaultParser,
  sendTransaction,
} from "./helpers";

function parseArgs() {
  const parser = prepareDefaultParser();
  parser.add_argument("-executionFee", "--execFee", { required: true, action: ParseBN, dest: "executionFee" });
  parser.add_argument("-data", "--data", { required: true, action: ParseHex });
  parser.add_argument("-fallbackAddress", "--fallback", { required: true, type: "str", dest: "fallbackAddress" });
  const parsed = parser.parse_args();
  type ParsedType = DefaultArgs & {
    executionFee: BN;
    data: Buffer;
    fallbackAddress: string;
  };

  return parsed as ParsedType;
}

async function main() {
  const { connection, wallet, example, deBridge } = initAll();
  const parsed = parseArgs();

  const builder = example.methods.sendViaDebridgeWithExternalCall(
    parsed.amount,
    Array.from(crypto.normalizeChainId(parsed.targetChain)),
    helpers.hexToBuffer(parsed.receiver),
    parsed.executionFee,
    helpers.hexToBuffer(parsed.fallbackAddress),
    parsed.data,
  );

  let remainingAccounts: AccountMeta[];
  switch (parsed.mode) {
    case "manual": {
      remainingAccounts = await buildSendContextManual(
        deBridge,
        wallet.publicKey,
        parsed.tokenMint,
        parsed.targetChain,
        crypto.hashExternalCallBytes(parsed.data),
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
        false,
        {
          data: parsed.data,
          flags: 0,
          fallbackAddress: parsed.fallbackAddress,
        },
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
