use anchor_lang::prelude::*;

declare_id!("CyLY1NifD82iGpMdXpssvSUKUM6RUHYhXf4w5GQKiuri");

#[program]
pub mod contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
