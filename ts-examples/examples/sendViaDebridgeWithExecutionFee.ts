/* eslint-disable no-console */
import { BN } from "@coral-xyz/anchor";
import { crypto, helpers } from "@debridge-finance/solana-utils";

import { buildSendContext } from "./contextBuilding";
import { DefaultArgs, initAll, ParseBN, ParseBool, getDefaultParser, sendTransaction } from "./helpers";

function parseArgs() {
  const parser = getDefaultParser();
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

  const builder = example.methods.sendViaDebridgeWithExecutionFee(
    parsed.amount,
    Array.from(crypto.normalizeChainId(parsed.targetChain)),
    helpers.hexToBuffer(parsed.receiver),
    parsed.executionFee,
  );
  const remainingAccounts = await buildSendContext(
    deBridge,
    wallet.publicKey,
    parsed.tokenMint,
    parsed.targetChain,
    parsed.receiver,
    parsed.useAssetFee,
    parsed.mode,
  );
  builder.remainingAccounts(remainingAccounts);

  await sendTransaction(await builder.transaction(), connection, wallet);
}

main().catch(console.error);
