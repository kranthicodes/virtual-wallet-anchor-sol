use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod virtual_wallet {
    use super::*;

    pub fn initialize_wallet(ctx: Context<InitializeWallet>) -> Result<()> {
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
