import { Connection, PublicKey, TransactionInstruction, clusterApiUrl } from "@solana/web3.js";
import * as ec from "@debridge-finance/debridge-external-call";
import { ArgumentParser } from "argparse";
import {
  createTransferInstruction,
  createAssociatedTokenAccountIdempotentInstruction,
  getAssociatedTokenAddressSync,
  ACCOUNT_SIZE,
} from "@solana/spl-token";

import { ParsePubkey } from "./helpers";

/**
 * Since we don't know exact claim amount we can replace instruction
 * data at `offset` with wallet balance (subtracting `subtraction` amount)
 * encoded in little or big endian (depends on `is_big_endian`)
 */
type AmountSubstitution = {
  account_index: number;
  is_big_endian: boolean;
  offset: number;
  subtraction: number;
};

/**
 * Since we don't know some accounts (submissionAuth, submissionWallet, etc)
 * at the moment of ext call data preparation we can replace accounts in the
 * instruction keys field at `index` with `ATA(submissionAuth, tokenMint)`
 */
type WalletSubstitution = {
  token_mint: string;
  index: number;
};

/**
 * Format of external instruction that debridge-external-call consumes
 * to produce external call data.
 * Main difference from TransactionInstruction is stringified `PublicKey`s
 */
interface IExtIx {
  keys: {
    pubkey: string;
    isSigner: boolean;
    isWritable: boolean;
  }[];
  data: Buffer;
  programId: string;
}

/**
 * Transform @solana/web3.js TransactionInstruction into form required by debridge-external-call
 * @param ix
 * @returns
 */
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
  parser.add_argument("-dest", "--destinationWallets", { nargs: "*", dest: "destinationWallets", action: ParsePubkey });
  const args = parser.parse_args();

  return args as {
    claimMint: PublicKey;
    destinationWallets: PublicKey[];
  };
}

export async function buildTransferExtCall(claimMint: PublicKey, destinationWallets: PublicKey[]) {
  const connection = new Connection(clusterApiUrl("mainnet-beta"));
  /**
   * The External call executed in Solana is an ExternalInstruction set.
   * Each such instance includes a Solana instruction as well as service fields.
   */
  const extIx: ec.ExternalInstructionWrapper[] = [];
  /**
   * SubmissionAuth is a temporary address that controls spl assets in the case of an external call.
   * Since it cannot be known at the moment of creation
   * (hash of external call is part of submission id), placeholder addresses are used.
   */
  const submissionAuthPlaceholder = new PublicKey(ec.auth_placeholder());
  /**
   * not a real account, needed to traverse flow, will be replaced using WalletSubstitution
   */
  const submissionAuthATA = getAssociatedTokenAddressSync(claimMint, submissionAuthPlaceholder);

  for (const destinationWallet of destinationWallets) {
    /**
     * Address of the actual recipient
     */
    const destinationATA = getAssociatedTokenAddressSync(claimMint, destinationWallet);

    /**
     * We create a normal solana instruction by creating an ATA for the destionation pubkey
     * on behalf of the submission auth (by specifying an explicit placeholder).
     *
     * This will be paid from the amount of SOLs specified in the Expenses,
     * and the executor will receive a Reward for it, also specified in the external instruction below.
     */
    const initAtaIx = createAssociatedTokenAccountIdempotentInstruction(
      submissionAuthPlaceholder,
      destinationATA,
      destinationWallet,
      claimMint,
    );
    extIx.push(
      new ec.ExternalInstructionWrapper(
        /**
         * reward - how much of the transfer amount will be received by the person who executes this instruction.
         * If this amount will not be covered by the transfer,
         * then the instruction can only be executed by the fallback address
         * (the submission field specified when generating the transfer)
         * this should cover the execution field
         * This value could be calculated with `deBridgeClient.calculateExecutionFee`
         * based on expenses field and claim token and native token cost
         */
        100n,
        // expenses - cost of execution of instruction in native tokens (lamports)
        BigInt(5000 + (await connection.getMinimumBalanceForRentExemption(ACCOUNT_SIZE))),
        false,
        [],
        // replace account at index 1 with ATA(realSubmissionAuth, claimMint) during ext call execution
        [{ index: 1, token_mint: claimMint.toBase58() } as WalletSubstitution],
        // convert TransactionInstruction into required format (stringified pubkeys)
        instructionToExternalInstruction(initAtaIx),
      ),
    );

    /**
     * We're transferring tokens from submissionAuthATA to destination ATA where submissionAuthPlaceholder is an owner.
     * - SubmissionAuthATA will be replaced later using WalletSubstitution mechanism with `ATA(accounts[index], token_mint)`
     *   because submissionAuth is a placeholder and there's no sense in calculating ATA from placeholder
     * - Amount (1) will be replaced using AmountSubstitution as u64 at offset `offset` in ix data
     *   encoded as little-endian or big-endian (depends on is_big_endian flag)
     *   (since solana TokenProgram encodes amount with little-endian we set is_big_endian to false)
     *   with new amount = `splWalletBalance(accounts[account_index]) - subtraction`
     * - submissionAuthPlaceholder will be replaced with real submissionAuth as raw placeholder
     */
    const transferIx = createTransferInstruction(submissionAuthATA, destinationATA, submissionAuthPlaceholder, 1n);
    extIx.push(
      new ec.ExternalInstructionWrapper(
        10n, // reward
        5000n, // expenses, just solana signatureFee
        false,
        // replace amount at offset with `splWalletBalance(accounts[account_index]) - subtraction` during ext call execution
        [{ is_big_endian: false, offset: 1, account_index: 0, subtraction: 0 }] as AmountSubstitution[],
        // replace account at index 0 with ATA(realSubmissionAuth, claimMint) during ext call execution
        [{ index: 0, token_mint: claimMint.toBase58() }] as WalletSubstitution[],
        instructionToExternalInstruction(transferIx),
      ),
    );
  }
  const extCallData = Buffer.concat(extIx.map((eix) => eix.serialize()));

  return extCallData;
}

async function main() {
  const args = parseArgs();

  const { claimMint, destinationWallets } = args;

  const extCallData = await buildTransferExtCall(claimMint, destinationWallets);
  /* eslint-disable no-console */
  console.log(`Serialized: ${extCallData.toString("hex")}`);
}

main().catch(console.error);
