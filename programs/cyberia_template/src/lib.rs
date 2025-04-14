use anchor_lang::prelude::*;

declare_id!("HaesmfbnNYXgL7JGEPuAEPuAZcmELoHrHA355i5T5Nui");

#[program]
pub mod cyberia_template {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
