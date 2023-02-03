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
use debridge_solana_sdk::{
    sending::{
        add_all_fees, invoke_debridge_send, invoke_init_external_call, ,
        is_chain_supported, SendIx, SendSubmissionParamsInput,
    },
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod send_via_debridge {
    use super::*;

    pub fn send_via_debridge(
        ctx: Context<SendViaDebridge>,
        exact_amount: u64,
        target_chain_id: [u8; 32],
        execution_fee: u64,
        fallback_address: Vec<u8>,
    ) -> Result<()> {
        let reciever = vec![];
        let external_call = vec![]; 
        
        is_chain_supported(ctx.remaining_accounts, target_chain_id)
            .expect("Failed to parse chain support info")
            .then_some(())
            .expect("Chain info not supported");

        let final_amount = add_all_fees(
            ctx.remaining_accounts,
            target_chain_id,
            exect_amount,
            execution_fee,
            is_use_asset_fee,
        )
        .expect("Failed to add fee to amount");

        invoke_init_external_call(external_call.as_slice(), ctx.remaining_accounts)
            .map_err(AnchorError::from)?;

        let send_ix = SendIx {
            target_chain_id,
            receiver,
            is_use_asset_fee,
            final_amount,
            submission_params: Some(SendSubmissionParamsInput::with_external_call(
                external_call,
                execution_fee,
                fallback_address,
            )),
            referral_code: None,
        };

        invoke_debridge_send(send_ix, ctx.remaining_accounts).map_err(|err| err.into())
    }
}

#[derive(Accounts)]
pub struct SendViaDebridge {}
```

### What happens in this program


### Build and deploy contract

### Write and run tests

