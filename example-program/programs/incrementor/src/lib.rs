use anchor_lang::prelude::*;

declare_id!("RsJUECkXgzsAkFxXgjuaHwxwzZZ1rbckyWnu8xYwZN4");

#[program]
pub mod incrementor {
    use super::*;

    pub fn increment_global_state(ctx: Context<InrecementGlobalState>) -> Result<()> {
        Ok(())
    }

    pub fn increment_user_state(ctx: Context<IncrementUserState>) -> Result<()> {
        Ok(())
    }

    pub fn multiply_states(ctx: Context<MultiplyStates>) -> Result<()> {
        Ok(())
    }
}

#[account]
pub struct GlobalState {
    value: u64,
}

#[account]
pub struct UserState {
    value: u64,
}

#[derive(Accounts)]
pub struct InrecementGlobalState {}

#[derive(Accounts)]
pub struct IncrementUserState {}

#[derive(Accounts)]
pub struct MultiplyStates {}
