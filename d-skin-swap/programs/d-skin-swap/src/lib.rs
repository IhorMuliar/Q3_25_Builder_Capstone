#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2enVEguZByDdcQu6n2dYgBXdHcwDP2cMh8r5td2bPARc");

#[program]
pub mod d_skin_swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee_rate: u16) -> Result<()> {
        initialize::handler(ctx, fee_rate)
    }
}
