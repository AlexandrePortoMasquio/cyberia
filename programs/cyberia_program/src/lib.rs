use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, AuthorityType, InitializeMint, MintTo, SetAuthority, Transfer as TokenTransfer},
};

declare_id!("83WQ78rDZprgM6zo2YEcvFMJTwSFWB7bRteebwefpgnB");

#[program]
pub mod cyberia_program {
    use super::*;

    /// Inicializa o mint com supply inicial e torna a supply imutável
    pub fn initialize(ctx: Context<Initialize>, initial_supply: u64, decimals: u8) -> Result<()> {
        // 1) Cria o mint on‑chain
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            decimals,
            &ctx.accounts.mint_authority.key(),
            None, // sem freeze authority
        )?;

        // 2) Cunha o supply inicial para a conta do payer
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            initial_supply,
        )?;

        // 3) Remove permanentemente autoridade de mint (supply imutável)
        token::set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SetAuthority {
                    account_or_mint: ctx.accounts.mint.to_account_info(),
                    current_authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            AuthorityType::MintTokens,
            None,
        )?;

        Ok(())
    }

    /// Transfere tokens entre duas Associated Token Accounts
    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TokenTransfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }
}

/// Contexto para `initialize`
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// Mint account (PDA ou Keypair)
    #[account(
        init,
        payer = payer,
        space = 8 + token::Mint::LEN,
        mint::decimals = 0,
        mint::authority = mint_authority,
        mint::freeze_authority = None
    )]
    pub mint: Account<'info, token::Mint>,

    /// Associated Token Account do payer para receber supply inicial
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub token_account: Account<'info, token::TokenAccount>,

    /// Authority temporária usada apenas na initialize
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    /// Pagador de taxas e que recebe o supply inicial
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

/// Contexto para `transfer`
#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut, constraint = from.amount >= amount @ ErrorCode::InsufficientFunds)]
    pub from: Account<'info, token::TokenAccount>,

    #[account(mut, constraint = to.mint == from.mint)]
    pub to: Account<'info, token::TokenAccount>,

    /// A autoridade que assina a transferência
    pub authority: Signer<'info>,

    pub token_program: Program<'info, token::Token>,
}

/// Erros customizados
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds for the operation")]
    InsufficientFunds,
}
