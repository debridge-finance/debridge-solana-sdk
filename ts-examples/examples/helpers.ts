/* eslint-disable no-console */
import { DeBridgeSolanaClient } from "@debridge-finance/solana-contracts-client";
import { AnchorProvider, BN, Program, Wallet as AnchorWallet } from "@coral-xyz/anchor";
import { AccountMeta, Connection, Keypair, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import {
  constants,
  findAssociatedTokenAddress,
  helpers,
  TOKEN_PROGRAM_ID,
  interfaces,
} from "@debridge-finance/solana-utils";
import { config } from "dotenv";
import { Action, ArgumentParser, Namespace } from "argparse";

import { IDL } from "./idl";

config();

export class ParsePubkey extends Action {
  call(parser: ArgumentParser, namespace: Namespace, values: string | string[], optionString: string | null) {
    namespace[this.dest] = new PublicKey(values as string);
  }
}

export class ParseBN extends Action {
  call(parser: ArgumentParser, namespace: Namespace, values: string | string[], optionString: string | null) {
    namespace[this.dest] = new BN(values as string);
  }
}

export class ParseHex extends Action {
  call(parser: ArgumentParser, namespace: Namespace, values: string | string[], optionString: string | null) {
    namespace[this.dest] = helpers.hexToBuffer(values as string);
  }
}

export class ParseBool extends Action {
  call(parser: ArgumentParser, namespace: Namespace, values: string | string[], optionString: string | null) {
    namespace[this.dest] = (values as string) === "true";
  }
}

export function prepareDefaultParser(): ArgumentParser {
  const parser = new ArgumentParser();
  parser.add_argument("-amount", "--amount", { required: true, action: ParseBN });
  parser.add_argument("-receiver", "--receiver", { required: true, type: "str" });
  parser.add_argument("-targetChain", "--chain", { required: true, type: "int", dest: "targetChain" });
  parser.add_argument("-tokenMint", "--mint", {
    required: true,
    action: ParsePubkey,
    dest: "tokenMint",
  });
  parser.add_argument("-mode", "--mode", { required: false, default: "client", choices: ["client", "manual"] });

  return parser;
}

export type DefaultArgs = {
  amount: BN;
  receiver: string;
  targetChain: number;
  tokenMint: PublicKey;
  mode: "manual" | "client";
};

export async function sendTransaction(tx: Transaction, connection: Connection, wallet: interfaces.IWallet) {
  const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash("finalized");
  tx.recentBlockhash = blockhash;
  tx.lastValidBlockHeight = lastValidBlockHeight;
  await wallet.signTransaction(tx);
  const serialized = tx.serialize();
  console.log(serialized.toString("base64"));

  return "";
  const txId = await connection.sendRawTransaction(tx.serialize());
  console.log(`Sent tx: ${txId}`);
}

export function initAll() {
  config();
  const requiredFields = ["DEBRIDGE", "SETTINGS", "EXAMPLE", "WALLET", "RPC"];
  for (const field of requiredFields) {
    if (process.env[field] === undefined) throw new Error(`Field ${field} expected to be in .env`);
  }
  const rawKp = helpers.hexToBuffer(process.env.WALLET!);
  const kp = Keypair.fromSecretKey(rawKp);
  const wallet = new helpers.Wallet(kp);
  const connection = new Connection(process.env.RPC!);
  const program = new Program(
    IDL,
    process.env.EXAMPLE!,
    new AnchorProvider(connection, {} as unknown as AnchorWallet, {}),
  );
  const deBridge = new DeBridgeSolanaClient(connection, wallet, {
    programId: process.env.DEBRIDGE!,
    settingsProgramId: process.env.SETTINGS!,
  });

  return { example: program, deBridge, connection, wallet };
}

/**
 * Builds context for CPI send. *Will not perform ANY checks*
 * @param deBridge client
 * @param sender address of sender, required for PDA calculation
 * @param tokenMint token to send
 * @param chainTo destination chainId, required for BridgeFeeInfo and ChainSupportInfo PDA
 * @param extCallShortcut hash of external call data
 * @returns accounts meta for deBridge CPI send call
 */
export async function buildSendContextManual(
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
  const discountInfo = await deBridge.getDiscountInfoSafe(discount);
  if (discountInfo === undefined) {
    [discount] = deBridge.accountsResolver.getNoDiscountAddress();
  }
  let [bridgeFee] = deBridge.accountsResolver.getBridgeFeeAddress(bridge, chainTo);
  try {
    await deBridge.getBridgeFeeSafe(bridgeFee);
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

/**
 * @param deBridge client
 * @param sender address of sender, required for PDA calculation
 * @param tokenMint token to send
 * @param receiver receiver address in target chain.
 * Client will check if address length is correct for the provided chain
 * @param chainTo destination chainId, required for BridgeFeeInfo and ChainSupportInfo PDA
 * @param useAssetFee client will check if it's possible to use asset fee for provided chain, if not will throw an error
 * @param externalCall external call information, required if external call exists
 * @param externalCall.flags external call flags, client will only check if `SEND_HASHED_DATA` flag is set
 * If true externalCall.data param will be threated as hash, else client will hash external call data
 * @param externalCall.data external call data or external call data hash if `SEND_HASHED_DATA` flag is true
 * @param externalCall.fallbackAddress authority in targetChain that can cancel external call execution
 * @returns accounts meta for deBridge CPI send call
 */
export async function buildSendContextWithClient(
  deBridge: DeBridgeSolanaClient,
  sender: PublicKey,
  tokenMint: PublicKey,
  receiver: string,
  chainTo: number,
  useAssetFee: boolean,
  externalCall?: {
    flags: number;
    data: Buffer;
    fallbackAddress: string;
  },
) {
  const context = await deBridge.buildSendContext(
    sender,
    null,
    tokenMint,
    receiver,
    chainTo,
    useAssetFee,
    externalCall?.fallbackAddress || receiver,
    externalCall?.flags,
    externalCall?.data,
  );
  const accounts = context.asIdl;
  const remainingAccounts: AccountMeta[] = [
    { isSigner: false, isWritable: true, pubkey: accounts.bridgeCtx.bridge },
    { isSigner: false, isWritable: true, pubkey: accounts.bridgeCtx.tokenMint },
    {
      isSigner: false,
      isWritable: true,
      pubkey: accounts.bridgeCtx.stakingWallet,
    },
    {
      isSigner: false,
      isWritable: false,
      pubkey: accounts.bridgeCtx.mintAuthority,
    },
    {
      isSigner: false,
      isWritable: false,
      pubkey: accounts.bridgeCtx.chainSupportInfo,
    },
    {
      isSigner: false,
      isWritable: false,
      pubkey: accounts.bridgeCtx.settingsProgram,
    },
    {
      isSigner: false,
      isWritable: false,
      pubkey: accounts.bridgeCtx.splTokenProgram,
    },
    { isSigner: false, isWritable: true, pubkey: accounts.stateCtx.state },
    {
      isSigner: false,
      isWritable: true,
      pubkey: accounts.stateCtx.feeBeneficiary,
    },
    { isSigner: false, isWritable: true, pubkey: accounts.nonceStorage },
    { isSigner: false, isWritable: true, pubkey: accounts.sendFromWallet },
    { isSigner: false, isWritable: false, pubkey: accounts.systemProgram },
    { isSigner: false, isWritable: true, pubkey: accounts.externalCallStorage },
    { isSigner: false, isWritable: true, pubkey: accounts.externalCallMeta },
    { isSigner: true, isWritable: true, pubkey: accounts.sendFrom },
    { isSigner: false, isWritable: false, pubkey: accounts.discount },
    { isSigner: false, isWritable: false, pubkey: accounts.bridgeFee },
    { isSigner: false, isWritable: false, pubkey: deBridge.program.programId },
  ];

  return remainingAccounts;
}
