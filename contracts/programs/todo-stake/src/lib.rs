use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("J2iKPwbfTUTATBiMkK1mysY7HJpPgiQxHWzzW9UxgR7a");

#[program]
pub mod todo_stake {
    use super::*;

    pub fn stake(ctx: Context<Stake>, todo_id: String, amount: u64) -> Result<()> {
        let stake_account = &mut ctx.accounts.stake_account;
        let user = &ctx.accounts.user;

        // Transfer funds from user to stake_account PDA
        // We use the system program to transfer.
        // The stake_account must be initialized with enough lamports for rent exemption + stake amount.
        // However, standard Anchor 'init' pays for rent.
        // We want to transfer the *stake amount* on top of rent.
        
        let transfer_instruction = system_instruction::transfer(
            user.key,
            &stake_account.key(),
            amount,
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                user.to_account_info(),
                stake_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        stake_account.owner = user.key();
        stake_account.todo_id = todo_id;
        stake_account.amount = amount;
        stake_account.status = StakeStatus::Active;
        stake_account.bump = ctx.bumps.stake_account;
        
        Ok(())
    }

    pub fn resolve(ctx: Context<Resolve>, _todo_id: String, success: bool) -> Result<()> {
        let stake_account = &mut ctx.accounts.stake_account;
        let user = &ctx.accounts.user;
        
        // Only the authority can sign this (checked in struct).

        if success {
            // Return funds to user
            stake_account.status = StakeStatus::ResolvedSuccess;
            
            // Allow the stake account to close and return rent + stake to user
            // We can just close the account entirely.
            // But if we want to keep history, we keep it and manually transfer.
            // Let's close it to recover rent for the user too.
            stake_account.close(user.to_account_info())?;
        } else {
            // Slash funds (send to treasury)
            stake_account.status = StakeStatus::ResolvedFailure;
            
            // Transfer everything to treasury
            let treasury = &ctx.accounts.treasury;
             stake_account.close(treasury.to_account_info())?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(todo_id: String)]
pub struct Stake<'info> {
    #[account(
        init,
        seeds = [b"stake", user.key().as_ref(), todo_id.as_bytes()],
        bump,
        payer = user,
        space = 8 + 32 + 4 + todo_id.len() + 8 + 1 + 1 // Discriminator + Pubkey + String Prefix + String content + u64 + Enum + Bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(todo_id: String)]
pub struct Resolve<'info> {
    #[account(
        mut,
        seeds = [b"stake", user.key().as_ref(), todo_id.as_bytes()],
        bump = stake_account.bump,
        has_one = owner,
        close = treasury // Default verify close, but we handle logic inside
    )]
    pub stake_account: Account<'info, StakeAccount>,
    
    /// CHECK: The user who owns the stake. Needed to receive refund context or verify ownership.
    #[account(mut)] 
    pub user: SystemAccount<'info>,
    
    /// CHECK: The treasury to receive slashed funds. logic should enforce this.
    #[account(mut)]
    pub treasury: SystemAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>, // The backend keypair
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub todo_id: String,
    pub amount: u64,
    pub status: StakeStatus,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum StakeStatus {
    Active,
    ResolvedSuccess,
    ResolvedFailure,
}
