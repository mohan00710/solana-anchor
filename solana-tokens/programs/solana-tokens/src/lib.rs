use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

declare_id!("DLpjkUTN6z4uC9xgSJXErhePopKxoyitnNoCNHeq5HGt");
pub mod token_sol;
#[program]
pub mod solana_tokens {
    use super::*;
    //function to create and mint amount to a token
    pub fn create_and_mint(ctx: Context<Initialize>, amount: u64) -> ProgramResult {
        ctx.accounts.process_mint(amount)
    }

    pub fn burn_mint(ctx: Context<Burn>, amount: u64) -> ProgramResult {
        ctx.accounts.process_burn(amount)
    }

    pub fn transfer(ctx: Context<TransferToken>, amount: u64) -> ProgramResult {
        ctx.accounts.process_transfer(amount)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        mint::decimals = 6,
        mint::authority = mint
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        token::mint = mint,
        token::authority = payer
    )]
    pub mint_to: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account( init_if_needed,
        payer = payer,
        token::mint = mint,
        token::authority = destination_owner)]
    pub destination: Account<'info, TokenAccount>,
    #[account(mut)]
    pub destination_owner: AccountInfo<'info>,
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
