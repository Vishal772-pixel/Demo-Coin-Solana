use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, Burn, Transfer};

declare_id!("3tDrPCXZoEdg3qCQVoCJYevpRqeA5EdGXwaRZBoSXMMh");

#[program]
pub mod demo_coin {
    use super::*;

    pub fn initialize_mint(
        ctx: Context<InitializeMintCtx>,
        amount: u64,
        decimals: u8,
    ) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::InitializeMint {
                mint: ctx.accounts.mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        );

        token::initialize_mint(cpi_ctx, decimals, ctx.accounts.authority.key, None)?;

        let mint_to_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.staking_wallet.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );

        token::mint_to(mint_to_ctx, amount)?;

        Ok(())
    }

    pub fn burn_tokens(ctx: Context<BurnCtx>, amount: u64) -> Result<()> {
        let burn_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.from.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );

        token::burn(burn_ctx, amount)?;
        Ok(())
    }

    pub fn transfer_with_fee(ctx: Context<TransferWithFee>, amount: u64) -> Result<()> {
        let fee = (amount as u128 * 3 / 10_000) as u64;
        let actual = amount.checked_sub(fee).unwrap_or(0);

        // Main transfer
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            actual,
        )?;

        // Fee transfer
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.staking_wallet.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            fee,
        )?;

        Ok(())
    }
}



#[derive(Accounts)]
pub struct InitializeMintCtx<'info> {
    #[account(init, payer = authority, mint::decimals = 9, mint::authority = authority)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub staking_wallet: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnCtx<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub from: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferWithFee<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,

    #[account(mut)]
    pub to: Account<'info, TokenAccount>,

    #[account(mut)]
    pub staking_wallet: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
