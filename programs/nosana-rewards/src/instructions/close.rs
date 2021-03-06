use crate::*;
use nosana_common::{staking, NosanaError};
use nosana_staking::StakeAccount;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, seeds = [ b"stats" ], bump = stats.bump)]
    pub stats: Account<'info, StatsAccount>,
    #[account(mut, close = authority, constraint = staker.key() == reward.authority)]
    pub reward: Box<Account<'info, RewardAccount>>,
    #[account(owner = staking::ID, constraint = staker.key() == stake.authority)]
    pub stake: Account<'info, StakeAccount>,
    /// CHECK: this is the owner of the stake and reward, and is optionally the signer
    pub staker: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<Close>) -> Result<()> {
    let stake: &Account<StakeAccount> = &ctx.accounts.stake;
    let reward: &mut Box<Account<RewardAccount>> = &mut ctx.accounts.reward;

    // TODO: we should also close a reward if the corresponding stake does not
    // exist (after it's closed). is this possible?

    // if the stake is not unstaked yet, only the owner can close the reward
    if stake.time_unstake == 0_i64 {
        require!(
            reward.authority == *ctx.accounts.authority.key,
            NosanaError::Unauthorized
        );
    }

    // update stats
    let stats: &mut Account<StatsAccount> = &mut ctx.accounts.stats;
    stats.remove_rewards_account(reward.reflection, reward.xnos);

    // finish
    Ok(())
}
