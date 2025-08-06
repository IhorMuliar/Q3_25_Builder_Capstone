use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(fee_rate: u16)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = MarketplaceConfig::SIZE,
        seeds = [MARKETPLACE_SEED],
        bump
    )]
    pub marketplace_config: Account<'info, MarketplaceConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// Treasury account where fees will be collected
    /// CHECK: This is just a pubkey for receiving fees
    pub treasury: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<Initialize>, fee_rate: u16) -> Result<()> {
    let marketplace_config = &mut ctx.accounts.marketplace_config;
    
    // Validate fee rate (max 10% = 1000 basis points)
    require!(fee_rate <= MAX_FEE_RATE, crate::error::MarketplaceError::InvalidFeeRate);
    
    marketplace_config.authority = ctx.accounts.authority.key();
    marketplace_config.fee_rate = fee_rate;
    marketplace_config.treasury = ctx.accounts.treasury.key();
    marketplace_config.total_trades = 0;
    marketplace_config.total_volume = 0;
    marketplace_config.bump = ctx.bumps.marketplace_config;
    
    msg!("Marketplace initialized with fee rate: {} basis points", fee_rate);
    msg!("Authority: {}", ctx.accounts.authority.key());
    msg!("Treasury: {}", ctx.accounts.treasury.key());
    
    Ok(())
}
