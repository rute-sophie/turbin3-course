use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct Marketplace {
    pub admin: Pubkey,     // 32 bytes
    pub fee: u16,          // 2 bytes
    pub bump: u8,          // 1 byte
    pub treasury_bump: u8, // 1 byte
    pub rewards_bump: u8,  // 1 byte
    #[max_len(32)]
    pub name: String,      // 4 len + 32 bytes
}

//alternative manual way
//impl Space for Marketplace {
//const INIT_SPACE: usize = 8 + 32 + 2 + 1 + 1 + 1 + (4 + 32);
//}
