use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, InitializeMint, Mint, MintTo, SetAuthority, Token, TokenAccount};
use spl_token::instruction::AuthorityType;

declare_id!("83WQ78rDZprgM6zo2YEcvFMJTwSFWB7bRteebwefpgnB");

// Number of decimal places for the token
pub const DECIMALS: u8 = 9;
// Total fixed supply to mint exactly once
pub const INITIAL_SUPPLY: u64 = 1_000_000_000 * 10u64.pow(DECIMALS as u32); // 1 billion tokens with 9 decimal places
                                                                            // The mint authority is set to None, which means no one can mint new tokens.

#[program]
pub mod cyberia {
    use super::*;

    /// Initializes the mint, mints the initial supply into the payer’s ATA,
    /// and then removes the mint authority permanently.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // 1) Create the mint with payer as temporary authority
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            DECIMALS,
            &ctx.accounts.payer.key(),
            None,
        )?;

        // 2) Mint the fixed initial supply to the payer’s associated token account
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            INITIAL_SUPPLY,
        )?;

        // 3) Remove mint authority permanently
        token::set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SetAuthority {
                    account_or_mint: ctx.accounts.mint.to_account_info(),
                    current_authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            AuthorityType::MintTokens,
            None,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = DECIMALS,
        mint::authority = payer,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
