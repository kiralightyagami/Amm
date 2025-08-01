use anchor_lang::prelude::*;

declare_id!("");


pub mod instructions;
pub use instructions::*;
pub mod errors;
pub mod constant;

pub mod states;

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.initialize(seed, fee, authority, &ctx.bumps)
    }

    pub fn deposite(ctx: Context<Deposite>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposite(amount, max_x, max_y)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64, min_x: u64, min_y: u64) -> Result<()> {
        ctx.accounts.withdraw(amount, min_x, min_y)
    }

    pub fn swap(ctx: Context<Swap>, is_x: bool, amount_in: u64, min_amount_out: u64) -> Result<()> {
        ctx.accounts.swap(is_x, amount_in, min_amount_out)
    }
}
