import { BN } from "@coral-xyz/anchor";
import { DeBridgeSolanaClient } from "@debridge-finance/solana-contracts-client";
import {
  constants,
  crypto,
  findAssociatedTokenAddress,
  helpers,
  TOKEN_PROGRAM_ID,
  WRAPPED_SOL_MINT,
} from "@debridge-finance/solana-utils";
import { AccountMeta, PublicKey, SystemProgram } from "@solana/web3.js";

import { formatRemainingAccounts, initAll } from "./helpers";

async function buildSendContextManual(
  deBridge: DeBridgeSolanaClient,
  sender: PublicKey,
  tokenMint: PublicKey,
  chainTo: number,
  extCallShortcut: Buffer,
) {
  const [bridge] = deBridge.accountsResolver.getBridgeAddress(tokenMint);
  const [mintAuthority] = deBridge.accountsResolver.getMintAuthorityAddress(bridge);
  const [stakingWallet] = findAssociatedTokenAddress(mintAuthority, tokenMint);
  const [stateAddress] = deBridge.accountsResolver.getStateAddress();
  const [chainSupportInfo] = deBridge.accountsResolver.getChainSupportInfoAddress(chainTo);
  const state = await deBridge.getStateSafe();
  const feeBeneficiary = state.feeBeneficiary;
  const [storage] = deBridge.accountsResolver.getExternalCallStorageAddress(
    extCallShortcut,
    sender,
    constants.SOLANA_CHAIN_ID,
  );
  const [meta] = deBridge.accountsResolver.getExternalCallMetaAddress(storage);
  const [sendFromWallet] = findAssociatedTokenAddress(sender, tokenMint);
  const [nonceAddress] = deBridge.accountsResolver.getNonceAddress();
  let [discount] = deBridge.accountsResolver.getDiscountInfoAddress(sender);
  if ((await deBridge.getDiscountInfoSafe(discount)) === null) {
    [discount] = deBridge.accountsResolver.getNoDiscountAddress();
  }
  let [bridgeFee] = deBridge.accountsResolver.getBridgeFeeAddress(bridge, chainTo);
  try {
    const feeInfo = await deBridge.getBridgeFeeSafe(bridgeFee);
  } catch (e) {
    [bridgeFee] = deBridge.accountsResolver.getNoBridgeFeeAddress();
  }

  const remainingAccounts: AccountMeta[] = [
    { isSigner: false, isWritable: true, pubkey: bridge },
    { isSigner: false, isWritable: true, pubkey: tokenMint },
    {
      isSigner: false,
      isWritable: true,
      pubkey: stakingWallet,
    },
    { isSigner: false, isWritable: false, pubkey: mintAuthority },
    {
      isSigner: false,
      isWritable: false,
      pubkey: chainSupportInfo,
    },
    { isSigner: false, isWritable: false, pubkey: deBridge.settingsProgram.programId },
    { isSigner: false, isWritable: false, pubkey: TOKEN_PROGRAM_ID },
    { isSigner: false, isWritable: true, pubkey: stateAddress },
    {
      isSigner: false,
      isWritable: true,
      pubkey: feeBeneficiary,
    },
    {
      isSigner: false,
      isWritable: true,
      pubkey: nonceAddress,
    },
    {
      isSigner: false,
      isWritable: true,
      pubkey: sendFromWallet,
    },
    { isSigner: false, isWritable: false, pubkey: SystemProgram.programId },
    { isSigner: false, isWritable: true, pubkey: storage },
    {
      isSigner: false,
      isWritable: true,
      pubkey: meta,
    },
    { isSigner: true, isWritable: true, pubkey: sender },
    { isSigner: false, isWritable: false, pubkey: discount },
    { isSigner: false, isWritable: false, pubkey: bridgeFee },
    { isSigner: false, isWritable: false, pubkey: deBridge.program.programId },
  ];

  return remainingAccounts;
}

async function main() {
  const { connection, wallet, example, deBridge } = initAll();

  const amount = 100;
  const receiver = "";
  const targetChain = 137;
  const executionFee = 123;
  const sendingContext = await deBridge.buildSendContext(
    wallet.publicKey,
    null,
    WRAPPED_SOL_MINT,
    receiver,
    targetChain,
    true,
    receiver,
  );

  const builder = example.methods.sendViaDebridgeWithExactAmount(
    new BN(amount),
    Array.from(crypto.normalizeChainId(targetChain)),
    helpers.hexToBuffer(receiver),
    new BN(executionFee),
    false,
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
