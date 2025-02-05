use anchor_lang::prelude::*;

declare_id!("HnQye9Gt7Vo1QxnyzzKkA3hCByqEghva1KGJiJUjbEZD");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
