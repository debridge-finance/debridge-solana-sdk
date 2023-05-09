import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import * as ec from "@debridge-finance/debridge-external-call";
import { ArgumentParser } from "argparse";
import {
  createTransferInstruction,
  createAssociatedTokenAccountIdempotentInstruction,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";

import { ParsePubkey } from "./helpers";

type AmountSubstitution = {
  account_index: number;
  is_big_endian: boolean;
  offset: number;
  subtraction: number;
};

type WalletSubstitution = {
  token_mint: string;
  index: number;
};

interface IExtIx {
  keys: {
    pubkey: string;
    isSigner: boolean;
    isWritable: boolean;
  }[];
  data: Buffer;
  programId: string;
}

function instructionToExternalInstruction(ix: TransactionInstruction): IExtIx {
  return {
    data: ix.data,
    programId: ix.programId.toBase58(),
    keys: ix.keys.map(({ isSigner, isWritable, pubkey }) => ({ isSigner, isWritable, pubkey: pubkey.toBase58() })),
  };
}

function parseArgs() {
  const parser = new ArgumentParser();
  parser.add_argument("-mint", "--claimTokenMint", { dest: "claimMint", action: ParsePubkey });
  parser.add_argument("-dest", "--destinationWallet", { dest: "destinationWallet", action: ParsePubkey });
  const args = parser.parse_args();

  return args as {
    claimMint: PublicKey;
    destinationWallet: PublicKey;
  };
}

export function buildTransferExtCall(claimMint: PublicKey, destinationWallet: PublicKey) {
  const extIx: ec.ExternalInstructionWrapper[] = [];
  const submissionAuthPlaceholder = new PublicKey(ec.auth_placeholder());
  const submissionAuthATA = getAssociatedTokenAddressSync(claimMint, submissionAuthPlaceholder);
  const destinationATA = getAssociatedTokenAddressSync(claimMint, destinationWallet);

  const initAtaIx = createAssociatedTokenAccountIdempotentInstruction(
    submissionAuthPlaceholder,
    submissionAuthATA,
    submissionAuthPlaceholder,
    claimMint,
  );
  extIx.push(
    new ec.ExternalInstructionWrapper(
      undefined,
      undefined,
      false,
      [],
      // replace account at index 1 to ATA(realSubmissionAuth, claimMint) during ext call execution
      [{ index: 1, token_mint: claimMint.toBase58() } as WalletSubstitution],
      instructionToExternalInstruction(initAtaIx),
    ),
  );

  const transferIx = createTransferInstruction(submissionAuthATA, destinationATA, submissionAuthPlaceholder, 1n);
  extIx.push(
    new ec.ExternalInstructionWrapper(
      undefined,
      undefined,
      false,
      // replace amount at offfset with splWalletBalance(accounts[account_index]) - subtraction during ext call execution
      [{ is_big_endian: false, offset: 1, account_index: 0, subtraction: 0 }] as AmountSubstitution[],
      // replace account at index 1 with ATA(realSubmissionAuth, claimMint) during ext call execution
      [{ index: 0, token_mint: claimMint.toBase58() }] as WalletSubstitution[],
      instructionToExternalInstruction(transferIx),
    ),
  );
  const extCallData = Buffer.concat(extIx.map((eix) => eix.serialize()));

  return extCallData;
}

function main() {
  const args = parseArgs();

  const { claimMint, destinationWallet } = args;

  const extCallData = buildTransferExtCall(claimMint, destinationWallet);
  console.log(`Serialized: ${extCallData.toString("hex")}`);
}

main();
