import { DeBridgeSolanaClient } from "@debridge-finance/solana-contracts-client";
import { constants, crypto, findAssociatedTokenAddress, TOKEN_PROGRAM_ID } from "@debridge-finance/solana-utils";
import { AccountMeta, PublicKey, SystemProgram } from "@solana/web3.js";

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

export async function buildSendContext(
  deBridge: DeBridgeSolanaClient,
  sender: PublicKey,
  tokenMint: PublicKey,
  chainTo: number,
  receiver: string,
  useAssetFee: boolean,
  mode: "manual" | "client",
  extCall?: {
    data: Buffer;
    flags: number;
    fallbackAddress: string;
  },
) {
  let remainingAccounts: AccountMeta[];
  switch (mode) {
    case "manual": {
      remainingAccounts = await buildSendContextManual(
        deBridge,
        sender,
        tokenMint,
        chainTo,
        crypto.hashExternalCallBytes(extCall?.data),
      );
      break;
    }
    case "client": {
      remainingAccounts = await buildSendContextWithClient(
        deBridge,
        sender,
        tokenMint,
        receiver,
        chainTo,
        useAssetFee,
        extCall,
      );
      break;
    }
    default: {
      throw new Error("unkown mode");
    }
  }

  return remainingAccounts;
}
