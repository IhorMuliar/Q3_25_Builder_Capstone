use anchor_lang::prelude::*;

#[account]
pub struct EscrowAccount {
    pub marketplace: Pubkey,
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub nft_mint: Pubkey,
    pub amount: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl EscrowAccount {
    pub const SIZE: usize = 8 + // discriminator
        32 + // marketplace
        32 + // buyer
        32 + // seller
        32 + // nft_mint
        8 + // amount
        8 + // created_at
        1; // bump
}