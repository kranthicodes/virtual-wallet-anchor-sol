use anchor_lang::prelude::*;

declare_id!("DVM4rzaJpyxCmKav7u2D87RyqYibaoWfFtUvMaQiz6Xm");

#[program]
pub mod virtual_wallet {
    use super::*;

    pub fn initialize_wallet(ctx: Context<InitializeWallet>) -> Result<()> {
        ctx.accounts.cash.amount = 0;

        ctx.accounts.credit_card.credit_limit = 5000;
        ctx.accounts.credit_card.credit_used = 0;

        ctx.accounts.debit_card.savings = 0;
        ctx.accounts.debit_card.checking = 0;

        msg!("Virtual wallet initialized!");
        msg!(
            "Credit card limit: {}",
            ctx.accounts.credit_card.credit_limit
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeWallet<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        seeds = [user.key().as_ref(), b"cash"],
        bump,
        payer = user,
        space = 8 + 8
    )]
    pub cash: Account<'info, Cash>,
    #[account(init, seeds = [user.key().as_ref(), b"credit_card"], bump, payer = user, space = 8 + 16)]
    pub credit_card: Account<'info, CreditCard>,
    #[account(init, seeds = [user.key().as_ref(), b"debit_card"], bump, payer = user, space = 8 + 16)]
    pub debit_card: Account<'info, DebitCard>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Cash {
    pub amount: u64,
}

#[account]
pub struct CreditCard {
    pub credit_used: u64,
    pub credit_limit: u64,
}

#[account]
pub struct DebitCard {
    pub checking: u64,
    pub savings: u64,
}
