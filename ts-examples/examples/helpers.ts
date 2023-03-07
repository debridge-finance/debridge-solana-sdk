/* eslint-disable no-console */
import { DeBridgeSolanaClient } from "@debridge-finance/solana-contracts-client";
import { AnchorProvider, BN, Program, Wallet as AnchorWallet } from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { helpers, interfaces } from "@debridge-finance/solana-utils";
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

export function getDefaultParser(): ArgumentParser {
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

export function getParserForExternalCall(): ArgumentParser {
  const parser = getDefaultParser();
  parser.add_argument("-executionFee", "--execFee", { required: true, action: ParseBN, dest: "executionFee" });
  parser.add_argument("-data", "--data", { required: true, action: ParseHex });
  parser.add_argument("-fallbackAddress", "--fallback", { required: true, type: "str", dest: "fallbackAddress" });

  return parser;
}

export function getDefaultArgs(): DefaultArgs {
  return getDefaultParser().parse_args() as DefaultArgs;
}

export function getArgsForExtCall(): DefaultArgs & ExtCallArgs {
  return getParserForExternalCall().parse_args() as DefaultArgs & ExtCallArgs;
}

export type DefaultArgs = {
  amount: BN;
  receiver: string;
  targetChain: number;
  tokenMint: PublicKey;
  mode: "manual" | "client";
};

export type ExtCallArgs = {
  executionFee: BN;
  data: Buffer;
  fallbackAddress: string;
};

export async function sendTransaction(tx: Transaction, connection: Connection, wallet: interfaces.IWallet) {
  const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash("finalized");
  tx.recentBlockhash = blockhash;
  tx.lastValidBlockHeight = lastValidBlockHeight;
  await wallet.signTransaction(tx);
  const serialized = tx.serialize();
  console.log(serialized.toString("base64"));

  const txId = await connection.sendRawTransaction(tx.serialize());
  console.log(`Sent tx: ${txId}`);
}

export function initAll() {
  config();
  const requiredFields = ["DEBRIDGE", "SETTINGS", "EXAMPLE_ID", "WALLET", "RPC"];
  for (const field of requiredFields) {
    if (process.env[field] === undefined) throw new Error(`Field ${field} expected to be in .env`);
  }
  const rawKp = helpers.hexToBuffer(process.env.WALLET!);
  const kp = Keypair.fromSecretKey(rawKp);
  const wallet = new helpers.Wallet(kp);
  const connection = new Connection(process.env.RPC!);
  const program = new Program(
    IDL,
    process.env.EXAMPLE_ID!,
    new AnchorProvider(connection, {} as unknown as AnchorWallet, {}),
  );
  const deBridge = new DeBridgeSolanaClient(connection, wallet, {
    programId: process.env.DEBRIDGE!,
    settingsProgramId: process.env.SETTINGS!,
  });

  return { example: program, deBridge, connection, wallet };
}
