use anchor_lang::prelude::*;
use instructions::deposit::*;
use instructions::withdraw::*;
use instructions::withdraw_closed_feed::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("AgfK2C4koo8Qa6jR2zqvWfvmwHYsR13xUgJGL23zhit3");

#[program]
mod burry_escrow {

    use super::*;
    // Deposit funds into the escrow account, storing the amount and unlock price
    pub fn deposit(ctx: Context<Deposit>, escrow_amt: u64, unlock_price: u64) -> Result<()> {
        deposit_handler(ctx, escrow_amt, unlock_price)
    }
    // Withdraw funds from the escrow account based on Switchboard feed data
    pub fn withdraw(ctx: Context<Withdraw>, params: WithdrawParams) -> Result<()> {
        withdraw_handler(ctx, params)
    }
    // Withdraw funds from the escrow account when the feed account has been closed (lamports = 0)
    pub fn withdraw_closed_feed(ctx: Context<WithdrawClosedFeed>) -> Result<()> {
        withdraw_closed_feed_handler(ctx)
    }
}
