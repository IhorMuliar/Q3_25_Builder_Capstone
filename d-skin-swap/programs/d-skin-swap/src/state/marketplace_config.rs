use anchor_lang::prelude::*;

#[account]
pub struct MarketplaceConfig {
    pub authority: Pubkey,
    pub fee_rate: u16, // Basis points (100 = 1%)
    pub treasury: Pubkey,
    pub total_trades: u64,
    pub total_volume: u64,
    pub bump: u8,
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