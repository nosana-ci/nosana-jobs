mod instructions;
mod state;

use anchor_lang::declare_id;
use anchor_lang::prelude::*;
use instructions::*;
use nosana_common::rewards;
use state::*;

declare_id!(rewards::ID);

#[program]
pub mod nosana_rewards {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        init::handler(ctx)
    }

    pub fn enter(ctx: Context<Enter>) -> Result<()> {
        enter::handler(ctx)
    }

    pub fn add_fee(ctx: Context<AddFee>, amount: u64) -> Result<()> {
        add_fee::handler(ctx, amount)
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        claim::handler(ctx)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        close::handler(ctx)
    }
}
