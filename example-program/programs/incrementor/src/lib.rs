// Because anchor
#![allow(clippy::result_large_err)]

use anchor_lang::{prelude::*, solana_program::sysvar};
use debridge_solana_sdk::check_claiming;

declare_id!("deincZzsdCg8DjeLsZjA9sTJw2qWP32Bty9aL44FNUk");

#[program]
pub mod incrementor {
    use super::*;

    #[derive(Accounts)]
    pub struct IncrementGlobalState<'info> {
        // NOTE: For `global_state` you can use constant `Pubkey` calculated beforehand
        // Pubkey (bs58): `266YXq5ZKDnFMfW4c4hqXK5RFSsU9QZxKWtvjXW6sRjt`
        #[account(
            init_if_needed,
            seeds = [
                b"STATE"
            ],
            bump,
            space = 8 + 8,
            payer = payer,
        )]
        global_state: Account<'info, GlobalState>,
        // NOTE: For `payer` you should use `AUTH_PLACEHOLDER`
        // AUTH_PLACEHOLDER (bs58): `2iBUASRfDHgEkuZ91Lvos5NxwnmiryHrNbWBfEVqHRQZ`
        #[account(mut)]
        payer: Signer<'info>,
        // NOTE: Constant (in bs58): `Sysvar1nstructions1111111111111111111111111`
        #[account(address = sysvar::instructions::ID)]
        instructions: AccountInfo<'info>,
        // NOTE: Constant (in bs58): `11111111111111111111111111111111`
        system_program: Program<'info, System>,
    }

    /// This instruction can be called by anyone from another network supported by Debridge
    ///
    /// This instruction increments the global state of this contract by one
    //
    // From client side this instruction looks like:
    // ```ron
    // Instruction {
    //     program_id: deincZzsdCg8DjeLsZjA9sTJw2qWP32Bty9aL44FNUk,
    //     accounts: [
    //         AccountMeta {
    //             pubkey: 266YXq5ZKDnFMfW4c4hqXK5RFSsU9QZxKWtvjXW6sRjt,
    //             is_signer: false,
    //             is_writable: true
    //         },
    //         AccountMeta {
    //             pubkey: 2iBUASRfDHgEkuZ91Lvos5NxwnmiryHrNbWBfEVqHRQZ,
    //             is_signer: true,
    //             is_writable: true
    //         },
    //         AccountMeta {
    //             pubkey: Sysvar1nstructions1111111111111111111111111,
    //             is_signer: false,
    //             is_writable: false
    //         },
    //         AccountMeta {
    //             pubkey: 11111111111111111111111111111111,
    //             is_signer: false,
    //             is_writable: false
    //         }
    //     ],
    //     data: [
    //         126, 21, 38, 168, 43, 51, 118, 133 // DISCRIMINATOR (constant)
    //     ]
    // }
    // ```
    // Expenses: 1002240 lamports
    // Reward must cover expenses + 5000 lamports
    pub fn increment_global_state(ctx: Context<IncrementGlobalState>) -> Result<()> {
        let validated_execute_ext_call =
            check_claiming::ValidatedExecuteExtCallIx::try_from_current_ix(
                &ctx.accounts.instructions,
            )?;

        validated_execute_ext_call
            .validate_submission_auth(ctx.accounts.payer.key)
            .map_err(|err| {
                msg!("Error while check signer == submission auth: {}", err);
                ProgramError::InvalidAccountData
            })?;

        ctx.accounts.global_state.value += 1;

        emit!(GlobalIncremented {
            value: ctx.accounts.global_state.value
        });

        Ok(())
    }

    #[derive(Accounts)]
    #[instruction(
        native_sender: [u8; 20],
    )]
    pub struct IncrementUserState<'info> {
        // NOTE: This pubkey use user pubkey as part of `Pubkey` seeds.
        //       Because of that, Substitution is needed:
        //
        // ```
        // AccountIndex: 0
        // PubkeySubstitution::BySeeds {
        //    program_id: deincZzsdCg8DjeLsZjA9sTJw2qWP32Bty9aL44FNUk,
        //    seeds: [
        //        SeedVariants::Arbitrary(b"USER"),
        //        SeedVariants::Arbitrary(native_sender),
        //    ],
        //    bump: None,
        // }
        // ```
        #[account(
            init_if_needed,
            seeds = [
                b"USER",
                native_sender.as_slice(),
            ],
            bump,
            space = 8 + 8,
            payer = payer,
        )]
        user_state: Account<'info, UserState>,
        // NOTE: For `payer` you should use `SUBMISSION_PLACEHOLDER` constant
        // NOTE: Constant (in bs58) `7cu34CRu47UZKLRHjt9kFPhuoYyHCzAafGiGWz83GNFs`
        submission: AccountInfo<'info>,
        #[account(mut)]
        // NOTE: For `payer` you should use `AUTH_PLACEHOLDER`
        // NOTE: Constant (in bs58) `2iBUASRfDHgEkuZ91Lvos5NxwnmiryHrNbWBfEVqHRQZ`
        payer: Signer<'info>,
        // NOTE: Constant (in bs58): `Sysvar1nstructions1111111111111111111111111`
        #[account(address = sysvar::instructions::ID)]
        instructions: AccountInfo<'info>,
        // NOTE: Constant (in bs58): `11111111111111111111111111111111`
        system_program: Program<'info, System>,
    }

    /// This instruction can be called by native_sender from another network supported by Debridge
    ///
    /// This instruction initialize if needed user state account &
    /// increments the user state of this contract by one
    ///
    // ```ron
    // Instruction {
    //     program_id: deincZzsdCg8DjeLsZjA9sTJw2qWP32Bty9aL44FNUk,
    //     accounts: [
    //         AccountMeta {
    //             pubkey: 11111111111111111111111111111111, // REPLACED BY SUBSITUTION
    //             is_signer: false,
    //             is_writable: true
    //         },
    //         AccountMeta {
    //             pubkey: 7cu34CRu47UZKLRHjt9kFPhuoYyHCzAafGiGWz83GNFs,
    //             is_signer: false,
    //             is_writable: false
    //         },
    //         AccountMeta {
    //             pubkey: 2iBUASRfDHgEkuZ91Lvos5NxwnmiryHrNbWBfEVqHRQZ,
    //             is_signer: true,
    //             is_writable: true
    //         },
    //         AccountMeta {
    //             pubkey: Sysvar1nstructions1111111111111111111111111,
    //             is_signer: false,
    //             is_writable: false
    //         },
    //         AccountMeta {
    //             pubkey: 11111111111111111111111111111111,
    //             is_signer: false,
    //             is_writable: false
    //         }
    //     ],
    //     data: [
    //         109, 53, 55, 244, 247, 63, 196, 165,  // DISCRIMINATOR (constant)
    //         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255 // NATIVE_SENDER
    //     ]
    // }
    // ```
    // Expenses: 1002240 lamports
    // Reward must cover expenses + 5000 lamports
    pub fn increment_user_state(
        ctx: Context<IncrementUserState>,
        native_sender: [u8; 20],
    ) -> Result<()> {
        let validated_execute_ext_call =
            check_claiming::ValidatedExecuteExtCallIx::try_from_current_ix(
                &ctx.accounts.instructions,
            )?;

        validated_execute_ext_call
            .validate_submission_auth(ctx.accounts.payer.key)
            .map_err(|err| {
                msg!("Error while check signer == submission auth: {}", err);
                ProgramError::InvalidAccountData
            })?;

        // Validate submission account & native sender
        validated_execute_ext_call
            .validate_submission_account(
                &ctx.accounts.submission,
                check_claiming::SubmissionAccountValidation {
                    native_sender_validation: Some(native_sender.to_vec()),

                    // You can validate either part of the context or the whole context
                    source_chain_id_validation: None,
                    claimer_validation: None,
                    receiver_validation: None,
                    fallback_address_validation: None,
                    token_mint_validation: None,
                },
            )
            .map_err(|err| {
                msg!("Error while check debridge execution context: {}", err);
                ProgramError::InvalidArgument
            })?;

        ctx.accounts.user_state.value += 1;

        emit!(UserIncremented {
            value: ctx.accounts.user_state.value
        });

        Ok(())
    }

    #[derive(Accounts)]
    #[instruction(
        native_sender: [u8; 20],
    )]
    pub struct MultiplyStates<'info> {
        // NOTE: This pubkey use user pubkey as part of `Pubkey` seeds.
        //       Because of that, Substitution is needed:
        //
        // ```
        // AccountIndex: 0
        // PubkeySubstitution::BySeeds {
        //    program_id: deincZzsdCg8DjeLsZjA9sTJw2qWP32Bty9aL44FNUk,
        //    seeds: [
        //        SeedVariants::Arbitrary(b"USER"),
        //        SeedVariants::Arbitrary(native_sender),
        //    ],
        //    bump: None,
        // }
        // ```
        #[account(
            seeds = [
                b"USER",
                native_sender.as_slice(),
            ],
            bump,
        )]
        user_state: Account<'info, UserState>,
        // NOTE: For `global_state` you can use constant `Pubkey` calculated beforehand
        // Pubkey (bs58): `266YXq5ZKDnFMfW4c4hqXK5RFSsU9QZxKWtvjXW6sRjt`
        #[account(
            seeds = [
                b"STATE"
            ],
            bump,
        )]
        global_state: Account<'info, GlobalState>,
    }

    /// This instruction can be called by native_sender from another network supported by Debridge
    ///
    /// This instruction initialize if needed user state account &
    /// increments the user state of this contract by one
    ///
    // ```ron
    // Instruction {
    //     program_id: deincZzsdCg8DjeLsZjA9sTJw2qWP32Bty9aL44FNUk,
    //     accounts: [
    //         AccountMeta {
    //             pubkey: 11111111111111111111111111111111, // REPLACED BY SUBSITUTION
    //             is_signer: false,
    //             is_writable: false
    //         },
    //         AccountMeta {
    //             pubkey: 266YXq5ZKDnFMfW4c4hqXK5RFSsU9QZxKWtvjXW6sRjt,
    //             is_signer: false,
    //             is_writable: false
    //         }
    //     ],
    //     data: [
    //         109, 53, 55, 244, 247, 63, 196, 165, // DISCRIMINATOR (constant)
    //         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255 // NATIVE_SENDER
    //     ]
    // }
    // ```
    // Expenses: 0 lamports
    // Reward must cover 5000 lamports
    pub fn multiply_states(ctx: Context<MultiplyStates>) -> Result<()> {
        let global = ctx.accounts.global_state.value;
        let user = ctx.accounts.user_state.value;

        emit!(Multiplied {
            multiplication: global * user,
            global,
            user,
        });

        Ok(())
    }
}

#[event]
struct GlobalIncremented {
    value: u64,
}

#[event]
struct UserIncremented {
    value: u64,
}

#[event]
struct Multiplied {
    global: u64,
    user: u64,
    multiplication: u64,
}

#[account]
pub struct GlobalState {
    value: u64,
}

#[account]
pub struct UserState {
    value: u64,
}
