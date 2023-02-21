import { BN } from "@coral-xyz/anchor";
import { crypto, helpers, WRAPPED_SOL_MINT } from "@debridge-finance/solana-utils";

import { formatRemainingAccounts, initAll } from "./helpers";

async function main() {
  const { connection, wallet, example, deBridge } = initAll();

  const amount = 100;
  const receiver = "";
  const targetChain = 137;
  const sendingContext = await deBridge.buildSendContext(
    wallet.publicKey,
    null,
    WRAPPED_SOL_MINT,
    receiver,
    targetChain,
    true,
    receiver,
  );

  const builder = example.methods.sendViaDebridgeWithAssetFixedFee(
    new BN(amount),
    Array.from(crypto.normalizeChainId(targetChain)),
    helpers.hexToBuffer(receiver),
  );
  builder.remainingAccounts(formatRemainingAccounts(deBridge, sendingContext));
  const tx = await builder.transaction();
  const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash("finalized");
  tx.recentBlockhash = blockhash;
  tx.lastValidBlockHeight = lastValidBlockHeight;
  await wallet.signTransaction(tx);

  const txId = await connection.sendRawTransaction(tx.serialize());
  console.log(`Sent tx: ${txId}`);
}

main().catch(console.error);
