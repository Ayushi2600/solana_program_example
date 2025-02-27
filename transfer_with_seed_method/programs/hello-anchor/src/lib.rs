use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::invoke_signed,
    system_instruction::transfer_with_seed,
};

declare_id!("DGWcHDGhTLKm9PwMtjhiRYEk7mxeUs17pxh4H1Me5qBN");

#[program]
mod hello_anchor {
    use super::*;

    pub fn sol_transfer(ctx: Context<TransferData>, amount: u64, seed: String) -> Result<()> {
        let from = ctx.accounts.from.to_account_info();
        let base = ctx.accounts.base.to_account_info();
        let to = ctx.accounts.to.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        let transfer_instruction = transfer_with_seed(
            &from.key(), 
            &base.key(), 
            seed.clone(), // Pass the seed as a String
            &system_program.key(), 
            &to.key(), 
            amount
        );

        invoke_signed(
            &transfer_instruction,
            &[
                from.to_account_info(),
                base.to_account_info(),
                to.to_account_info(),
                system_program.to_account_info(),
            ], 
            &[&[seed.as_bytes()]]
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(seed: String)]
pub struct TransferData<'info> {
    #[account(mut)]
    from: Signer<'info>,
    #[account(mut)]
    base: Signer<'info>,
    #[account(mut, seeds = [seed.as_bytes(), base.key().as_ref()], bump)]
    to: SystemAccount<'info>,
    system_program: Program<'info, System>,
}