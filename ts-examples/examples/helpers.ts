import { DeBridgeSolanaClient } from "@debridge-finance/solana-contracts-client";
import { AnchorProvider, Program, Wallet as AnchorWallet } from "@coral-xyz/anchor";
import { AccountMeta, Connection, Keypair } from "@solana/web3.js";
import { helpers } from "@debridge-finance/solana-utils";
import { config } from "dotenv";

import { IDL } from "./idl";

config();

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

export function formatRemainingAccounts(
  deBridge: DeBridgeSolanaClient,
  context: Awaited<ReturnType<DeBridgeSolanaClient["buildSendContext"]>>,
) {
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
