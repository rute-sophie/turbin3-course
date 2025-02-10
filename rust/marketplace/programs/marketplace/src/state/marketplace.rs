use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct Marketplace {
    pub admin: Pubkey, // 32 bytes
    pub fee: u16, // 2 bytes
    pub bump: u8,
    pub treasury_bump: u8,
    pub rewards_bump: u8,
    #[max_len(32)]
    pub name: String,
}

//alternative manual way
//impl Space for Marketplace {
    //const INIT_SPACE: usize = 8 + 32 + 2 + 1 + 1 + 1 + (4 + 32);
//}