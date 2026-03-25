use anchor_lang::prelude::*;

declare_id!("7QW2bjKzk8RVXb8Kq8BiJJqBZ9haBf2yAPfhrm7cURKr");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
