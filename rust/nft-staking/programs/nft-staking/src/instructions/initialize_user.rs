use anchor_lang::prelude::*;

use crate::state::UserAccount;

//The user will be able to create new User accounts
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump,
        space = 8 + UserAccount::INIT_SPACE,
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}
//user: Will be the person starting the user account. He will be a signer of the transaction, and we mark his account as mutable as we will be deducting lamports from this account
//user_account: Will be the state account that we will initialize and the user will be paying for the initialization of the account.
impl<'info> Initialize<'info> {
    pub fn initialize_user(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.user_account.set_inner(UserAccount {
            points: 0,
            amount_staked: 0,
            bump: bumps.user_account,
        });

        Ok(())
    }
}
