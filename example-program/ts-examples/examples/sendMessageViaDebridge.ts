/* eslint-disable no-console */
import { BN } from "@coral-xyz/anchor";
import { crypto, helpers } from "@debridge-finance/solana-utils";
import { PublicKey } from "@solana/web3.js";
import { ArgumentParser } from "argparse";

import { buildSendContext } from "./contextBuilding";
import { initAll, ParseBN, ParseHex, ParsePubkey, sendTransaction } from "./helpers";

function parseArgs() {
  const parser = new ArgumentParser();
  parser.add_argument("-receiver", "--receiver", { required: true, type: "str" });
  parser.add_argument("-targetChain", "--chain", { required: true, type: "int", dest: "targetChain" });
  parser.add_argument("-tokenMint", "--mint", {
    required: true,
    action: ParsePubkey,
    dest: "tokenMint",
  });
  parser.add_argument("-mode", "--mode", { required: false, default: "client", choices: ["client", "manual"] });
  parser.add_argument("-executionFee", "--execFee", { required: true, action: ParseBN, dest: "executionFee" });
  parser.add_argument("-data", "--data", { required: true, action: ParseHex });
  parser.add_argument("-fallbackAddress", "--fallback", { required: true, type: "str", dest: "fallbackAddress" });

  type Parsed = {
    receiver: string;
    targetChain: number;
    tokenMint: PublicKey;
    mode: "client" | "manual";
    data: Buffer;
    executionFee: BN;
    fallbackAddress: string;
  };

  return parser.parse_args() as Parsed;
}

async function main() {
  const { connection, wallet, example, deBridge } = initAll();
  const parsed = parseArgs();

  const builder = example.methods.sendMessageViaDebridge(
    Array.from(crypto.normalizeChainId(parsed.targetChain)),
    helpers.hexToBuffer(parsed.receiver),
    parsed.executionFee,
    helpers.hexToBuffer(parsed.fallbackAddress),
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
