use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub wallet: Pubkey,
    pub steam_id: String,
    pub total_trades: u64,
    pub created_at: i64,
    pub bump: u8,
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

#[account]
pub struct MarketplaceConfig {
    pub authority: Pubkey,
    pub fee_rate: u16, // Basis points (100 = 1%)
    pub treasury: Pubkey,
    pub total_trades: u64,
    pub total_volume: u64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ListingStatus {
    Active,
    Sold,
    Cancelled,
    InEscrow,
}

impl UserAccount {
    pub const SIZE: usize = 8 + // discriminator
        32 + // wallet
        32 + 4 + // steam_id (max 32 chars + length)
        8 + // total_trades
        8 + // created_at
        1; // bump
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

impl MarketplaceConfig {
    pub const SIZE: usize = 8 + // discriminator
        32 + // authority
        2 + // fee_rate
        32 + // treasury
        8 + // total_trades
        8 + // total_volume
        1; // bump
}