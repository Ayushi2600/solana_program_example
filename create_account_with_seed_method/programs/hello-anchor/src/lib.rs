use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::{invoke, invoke_signed},
    system_instruction::create_account_with_seed,
};

declare_id!("KaTdv7GnQMugKNYkC6z8Aq3FJ7ZanM16JghhzWiysNH");

#[program]
mod hello_anchor {
    use super::*;

    pub fn sol_create_account_with_seed(
        ctx: Context<CreateAccWithSeed>,
        seed: String,
        amount: u64,
        space: u64,
    ) -> Result<()> {
        let from = &ctx.accounts.from;
        let to = &ctx.accounts.to;
        let base = &ctx.accounts.base;
        let system_program = &ctx.accounts.system_program;

        let bump = ctx.bumps.to; // Get bump seed from context

        let ix = create_account_with_seed(
            &from.key(),
            &to.key(),
            &base.key(),
            &seed,
            amount,
            space,
            &ctx.program_id,
        );

        msg!(
            "Creating account {} with seed '{}' and {} lamports",
            to.key(),
            seed,
            amount
        );

        invoke_signed(
            &ix,
            &[
                from.to_account_info(),
                to.to_account_info(),
                base.to_account_info(),
                system_program.to_account_info(),
            ],
            &[&[seed.as_bytes(), base.key().as_ref(), &[bump]]], // PDA seeds
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(seed: String)]
pub struct CreateAccWithSeed<'info> {
    /// The account that will fund the new account
    #[account(mut)]
    pub from: Signer<'info>,
    
    /// The base account used for creating the account with a seed
    #[account(mut)]
    pub base: Signer<'info>,

    /// The new account to be created
    #[account(
        mut,
        seeds = [seed.as_bytes(), base.key().as_ref()],
        bump,
    )]
    pub to: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}
