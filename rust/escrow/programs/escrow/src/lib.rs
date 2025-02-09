use anchor_lang::prelude::*;

declare_id!("HnQye9Gt7Vo1QxnyzzKkA3hCByqEghva1KGJiJUjbEZD");

mod instructions;
mod state;
pub use instructions::*;
pub use state::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive_amount: u64, send_amount: u64) -> Result<()> {
        ctx.accounts.make(seed, receive_amount, &ctx.bumps)?;
        ctx.accounts.deposit(send_amount)? ;
        Ok(())
    }
    
    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.release()?;
        ctx.accounts.close()?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund()?;
        ctx.accounts.close()?;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize {}
