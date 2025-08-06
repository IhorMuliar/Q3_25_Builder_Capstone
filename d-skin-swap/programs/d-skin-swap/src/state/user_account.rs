use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub wallet: Pubkey,
    pub steam_id: String,
    pub total_trades: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl UserAccount {
    pub const SIZE: usize = 8 + // discriminator
        32 + // wallet
        32 + 4 + // steam_id (max 32 chars + length)
        8 + // total_trades
        8 + // created_at
        1; // bump
}