use anchor_lang::prelude::*;

/* 
u8  = 1
u16 = 2
u32 = 4
u64 = 8
u128 = 16

i8 = 1

vec<u8> and string needs to be initialized with 4 bytes
 */
#[account]
pub struct Listing {
    pub maker: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub bump: u8,
}

impl Space for Listing {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1;
}
