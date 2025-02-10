use anchor_lang::prelude::*;

declare_id!("HsGagk4VxZEFe5FQN7G9FjjfCX28bTiBasyuBcBL8r6F");

pub mod context;
pub mod state;
pub mod errors;

pub use context::*;
pub use errors::*;
pub use state::*;

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.initialize(name, fee, &ctx.bumps)?;

        Ok(())
    }

/*     pub fn listing(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;

        ok(())
    }

    pub fn delist(ctx: Context<List>, price: u64) -> Result<()> {
        

        ok(())
    }

    pub fn purchase(ctx: Context<List>, price: u64) -> Result<()> {
    

        ok(())
    } */


}

