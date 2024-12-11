use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");

#[program]
pub mod pump {
    use super::*;

    /// Creates the global state.
    #[allow(unused_variables)]
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    /// Sets the global state parameters.
    #[allow(unused_variables)]
    pub fn set_params(
        ctx: Context<SetParams>,
        fee_recipient: Pubkey,
        initial_virtual_token_reserves: u64,
        initial_virtual_sol_reserves: u64,
        initial_real_token_reserves: u64,
        token_total_suply: u64,
        fee_basis_points: u64,
    ) -> Result<()> {
        Ok(())
    }

    /// Creates a new coin and bonding curve.
    #[allow(unused_variables)]
    pub fn create(ctx: Context<Create>, name: String, symbol: String, uri: String) -> Result<()> {
        Ok(())
    }

    /// Buys tokens from a bonding curve.
    #[allow(unused_variables)]
    pub fn buy(ctx: Context<Buy>, amount: u64, max_sol_cost: u64) -> Result<()> {
        Ok(())
    }

    /// Sells tokens into a bonding curve.
    #[allow(unused_variables)]
    pub fn sell(ctx: Context<Sell>, amount: u64, min_sol_output: u64) -> Result<()> {
        Ok(())
    }

    /// Allows the admin to withdraw liquidity for a migration once the bonding curve completes
    #[allow(unused_variables)]
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        Ok(())
    }
}
