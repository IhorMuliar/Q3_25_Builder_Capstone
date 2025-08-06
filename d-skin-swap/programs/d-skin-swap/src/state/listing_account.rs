use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ListingStatus {
    Active,
    Sold,
    Cancelled,
    InEscrow,
}

#[account]
pub struct ListingAccount {
    pub nft_mint: Pubkey,
    pub seller: Pubkey,
    pub price: u64, // Price in USDC (with decimals)
    pub status: ListingStatus,
    pub created_at: i64,
    pub bump: u8,
}

impl ListingAccount {
    pub const SIZE: usize = 8 + // discriminator
        32 + // nft_mint
        32 + // seller
        8 + // price
        1 + // status enum
        8 + // created_at
        1; // bump
}