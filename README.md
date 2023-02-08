# debridge-solana-sdk

## About

Debridge-solana-sdk allows you to easily and quickly connect your Solana programs to the debridge infrastructure. Debridge
protocol enables decentralized transfers of arbitrary messages and value between various blockchains. See Debridge 
documentation for more information https://docs.debridge.finance/.

## Create simple message and value transfering to other chain

This example show how you can send some tokens and message to any supported chain using debridge-solana-sdk crate.

### Create new program with Anchor 

Debridge-solana-sdk doesn't depend on any solana framework like Anchor and can be connected to any Solana program.
But for simplifying example we will use example program built with Anchor

```bash
anchor init send-via-debridge
```

### Add debridge-solana-sdk crate

Next step is add debridge-solana-sdk crate to program dependency. Add to `./send-via-debridge/programs/Cargo.toml` 
in chapter dependency next row:

```toml
debridge-solana-sdk = { git = "ssh://git@github.com/debridge-finance/debridge-solana-sdk.git" }
```

### Create simple contract with using Debridge protocol

Add in `./send-via-debridge/programs/src/lib` code of simple smart contract that transfer tokens and message to other
chain with debridge solana program:
```rust
use anchor_lang::prelude::*;

declare_id!("3botMWU4s1Lcs4Q2wQBkZqsCW1vc3N9H9tY9SZYVs5vZ");

#[program]
pub mod send_via_debridge {

    use debridge_solana_sdk::sending::{invoke_debridge_send, SendIx, SendSubmissionParamsInput};

    use super::*;

    pub fn send_via_debridge(ctx: Context<SendViaDebridge>) -> Result<()> {
        let amount = 10000;
        let execution_fee = 1000;

        invoke_debridge_send(
            SendIx {
                target_chain_id: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 137,
                ],
                receiver: hex::decode("bd1e72155Ce24E57D0A026e0F7420D6559A7e651").unwrap(),
                is_use_asset_fee: false,
                amount,
                submission_params: Some(SendSubmissionParamsInput::execution_fee_only(
                    execution_fee,
                )),
                referral_code: None,
            },
            ctx.remaining_accounts,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendViaDebridge {}

```

### Build and deploy contract

Before build test program need to provide Debrdige and Settings program pubkeys through enviroment variables.
```bash
export SETTINGS_PROGRAM_PUBKEY=DeSetTwWhjZq6Pz9Kfdo1KoS5NqtsM6G8ERbX4SSCSft
export DEBRIDGE_PROGRAM_PUBKEY=DEbrdGj3HsRsAzx6uH4MKyREKxVAfBydijLUF3ygsFfh
```

Build program with anchor

```bash
anchor build
```

Deploy program
```bash
solana program deploy --program-id target/deploy/send_via_debridge-keypair.json ./target/deploy/send_via_debridge.so
```

### Write and run tests

Add file `./send-via-debridge/tests/send-via-debridge.ts` with next test case:

```typescript
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SendViaDebridge } from "../target/types/send_via_debridge";
import { AccountMeta, PublicKey, Signer } from "@solana/web3.js";

function accountsToMeta() {
  const result: AccountMeta[] = [
    {
      isSigner: false,
      isWritable: true,
      pubkey: new PublicKey("6SW1N9Rq2TqT3uQCD4F5zwtTTSFSarZmfyrk829SzsBX"),
    },
    {
      isSigner: false,
      isWritable: true,
      pubkey: new PublicKey("So11111111111111111111111111111111111111112"),
    },
    {
      isSigner: false,
      isWritable: true,
      pubkey: new PublicKey("8gjgVkHXTttCoSGGtzucFkJUWujQ8pgWuvnHCLSN7i3o"),
    },
    {
      isSigner: false,
      isWritable: false,
      pubkey: new PublicKey("7FmGdfJfDrrM6P68y7jijjj4xU9rH3hsUK2Kyp54iJUx"),
    },
    {
      isSigner: false,
      isWritable: false,
      pubkey: new PublicKey("8L81QZBfwA6Xi9zd49fyUfMRWJBCAxiUxd6jGHPnu1BQ"),
    },
    {
      isSigner: false,
      isWritable: false,
      pubkey: new PublicKey("DeSetTwWhjZq6Pz9Kfdo1KoS5NqtsM6G8ERbX4SSCSft"),
    },
    {
      isSigner: false,
      isWritable: false,
      pubkey: new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    },
    {
      isSigner: false,
      isWritable: true,
      pubkey: new PublicKey("CcjkxrCJvfXrmds78hwCnovkdmTgE12wqojiVLrtW1qn"),
    },
    {
      isSigner: false,
      isWritable: true,
      pubkey: new PublicKey("5MgAaNomDg4Y88v7gJ7LSWAyoLpDfcfbXZGQQnFddjJT"),
    },
    {
      isSigner: false,
      isWritable: true,
      pubkey: new PublicKey("2LKQceMRwfJNZovtSbsHmfszDYM5kTZHajFry2nqD2pi"),
    },
    {
      isSigner: false,
      isWritable: true,
      pubkey: new PublicKey("BzoSTqbp8vZ54Baq2K4LTwGnC8fYvKiEFQDNxdEDnosG"),
    },
    {
      isSigner: false,
      isWritable: false,
      pubkey: new PublicKey("11111111111111111111111111111111"),
    },
    {
      isSigner: false,
      isWritable: true,
      pubkey: new PublicKey("dPLMV1ky3H61yRGFfNC6AYmzBePhsdes9oNZ7chPbYW"),
    },
    {
      isSigner: false,
      isWritable: true,
      pubkey: new PublicKey("2cU8vjsMnRcusX1WdwZy1AwCLrUWbDw6frnk3XDz3VVK"),
    },
    {
      isSigner: true,
      isWritable: true,
      pubkey: new PublicKey("FsiBNh2KcPrjZFMF7EBCWpUpAo95DfrMXB2U2XrqSFWF"),
    },
    {
      isSigner: false,
      isWritable: false,
      pubkey: new PublicKey("4kQYWVy6Vu8YUXVp5BgQC12ZX1HLRUfkK3bLzBFFjnNW"),
    },
    {
      isSigner: false,
      isWritable: false,
      pubkey: new PublicKey("APMGxdbtubfWLQUACsN2yv2pxkvAgWwuxBe8ohFYoB37"),
    },
    {
      isSigner: false,
      isWritable: false,
      pubkey: new PublicKey("DEbrdGj3HsRsAzx6uH4MKyREKxVAfBydijLUF3ygsFfh"),
    },
  ];

  return result;
}

describe("send-via-debridge", async () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SendViaDebridge as Program<SendViaDebridge>;

  it("Send via Debridge!", async () => {
    const builder = await program.methods
      .sendViaDebridge()
      .signers([program.provider.wallet.payer]);
    builder.remainingAccounts([...accountsToMeta()]);

    let tx = await builder.rpc();

    console.log("Your transaction signature", tx);
  });
});
```

Then in `./send-via-debridge/Anchor.toml` change cluster to `mainnet` to use Debridge production contract:

```Toml
[provider]
cluster = "mainnet"
```

To run test use with build and deploy contract use:

```bash
anchor test
```

To test running only use:
```bash
anchor test --skip-build --skip-deploy
```

## Other examples:

Examples of sdk using you can find in example solana program by path `./send-via-debridge/exampleprogram` 

