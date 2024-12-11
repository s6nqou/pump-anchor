use crate::states::Global;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

pub const SEED_GLOBAL: &[u8] = b"global";
pub const SEED_BONDING_CURVE: &[u8] = b"bonding-curve";

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        seeds = [SEED_GLOBAL],
        bump,
    )]
    pub global: Account<'info, Global>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct SetParams<'info> {
    #[account(
        mut,
        seeds = [SEED_GLOBAL],
        bump,
    )]
    pub global: Account<'info, Global>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
        mut,
        mint::authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,

    pub mint_authority: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [SEED_BONDING_CURVE, &mint.key().to_bytes()],
        bump,
    )]
    pub bonding_curve: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = bonding_curve,
    )]
    pub associated_bonding_curve: Account<'info, TokenAccount>,

    #[account(
        seeds = [SEED_GLOBAL],
        bump,
    )]
    pub global: Account<'info, Global>,

    /// CHECK: checked
    #[account(
        executable,
        address = mpl_token_metadata::ID,
    )]
    pub mpl_token_metadata: AccountInfo<'info>,

    /// CHECK: checked
    #[account(
        mut,
        owner = mpl_token_metadata::ID,
    )]
    pub metadata: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(
        seeds = [SEED_GLOBAL],
        bump,
    )]
    pub global: Account<'info, Global>,

    #[account(
        mut,
        address = global.fee_recipient,
    )]
    pub fee_recipient: SystemAccount<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [SEED_BONDING_CURVE, &mint.key().to_bytes()],
        bump,
    )]
    pub bonding_curve: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = bonding_curve,
    )]
    pub associated_bonding_curve: Account<'info, TokenAccount>,

    #[account(mut)]
    pub associated_user: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(
        seeds = [SEED_GLOBAL],
        bump,
    )]
    pub global: Account<'info, Global>,

    #[account(
        mut,
        address = global.fee_recipient,
    )]
    pub fee_recipient: SystemAccount<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [SEED_BONDING_CURVE, &mint.key().to_bytes()],
        bump,
    )]
    pub bonding_curve: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = bonding_curve,
    )]
    pub associated_bonding_curve: Account<'info, TokenAccount>,

    #[account(mut)]
    pub associated_user: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        seeds = [SEED_GLOBAL],
        bump,
    )]
    pub global: Account<'info, Global>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [SEED_BONDING_CURVE, &mint.key().to_bytes()],
        bump,
    )]
    pub bonding_curve: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = bonding_curve,
    )]
    pub associated_bonding_curve: Account<'info, TokenAccount>,

    #[account(mut)]
    pub associated_user: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}
