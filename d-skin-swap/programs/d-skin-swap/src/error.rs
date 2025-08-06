use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Invalid fee rate - must be between 0 and 1000 basis points")]
    InvalidFeeRate,
    
    #[msg("Insufficient funds")]
    InsufficientFunds,
    
    #[msg("Invalid listing")]
    InvalidListing,
    
    #[msg("Listing not active")]
    ListingNotActive,
    
    #[msg("Unauthorized")]
    Unauthorized,
    
    #[msg("NFT not found")]
    NftNotFound,
    
    #[msg("Invalid price")]
    InvalidPrice,
    
    #[msg("Self purchase not allowed")]
    SelfPurchase,
    
    #[msg("Escrow account mismatch")]
    EscrowMismatch,
    
    #[msg("Steam ID too long")]
    SteamIdTooLong,
}
