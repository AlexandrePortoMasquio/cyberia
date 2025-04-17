use anchor_lang::prelude::*;

declare_id!("83WQ78rDZprgM6zo2YEcvFMJTwSFWB7bRteebwefpgnB");

#[program]
pub mod cyberia {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
