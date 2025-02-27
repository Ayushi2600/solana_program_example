use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_instruction, program::invoke};

declare_id!("6EM75XzUKSKfvogQg8X6XHV6yqHDG7VgYmKmbDWKZPWS");

#[program]
pub mod example {
    use super::*;

    pub fn transfer_many(ctx: Context<TransferMany>, recipients: Vec<Recipient>) -> Result<()> {
        let from = &ctx.accounts.from;

        let to_lamports: Vec<(Pubkey, u64)> = recipients.iter()
            .map(|r| (r.to, r.amount))
            .collect();

        let instructions = system_instruction::transfer_many(&from.key(), &to_lamports);

        for ix in instructions {
            invoke(
                &ix,
                &[
                    from.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferMany<'info> {
    #[account(mut)]
    pub from: Signer<'info>, 
    pub system_program: Program<'info, System>,
}

// Define a struct instead of using a tuple
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Recipient {
    pub to: Pubkey,
    pub amount: u64,
}
