use anchor_lang::prelude::*;
// to import all the library of anchor 

use anchor_spl::token::{ self,Mint,TokenAccount,Token,MintTo, IntializeMint};
// Helper module from Anchor to interact with Solana Program Library's SPL Token Program 

declare_id!("Fg6PaFpoGXkYsidMpWxTWqgGUkJdftFb3EMabA7xh1gE");
//It is unique solana program address on chain

#[program]
pub mod demo_coin {
    use super::*;

    pub fn intialize_mint(
        ctx:Context<IntilaizeMintCtx>,
        decimels:u8,
    )->Result<()>{
        let cpi_accounts = IntilaizeMint{
            mint:ctx.accounts.mint.to_account_info(),
            rent:ctx.accounts.rent.to_account_info(),


            let cpi_program =ctx.accounts.token_program.to_account_info();


            token::intialize_mint(
                CpiContext::new(cpi_program,cpi_accounts),
                decimels,
                ctx.accounts.authority.key,
                Some(ctx.accounts.authority.key),
            )?;
            Ok(())
        }




        pub fn mint_tokens(
            ctx:Context<MintTokenCtx>,
            amount:u64,
        )->Result<()>{
            let cpi_accounts =MintoTo{
                mint:ctx.accounts.mint.to_account_info(),
                to:ctx.accounts.recipent.to_account_info(),
                authority:ctx.accounts.authority.to_account_info(),
            };




            let cpi_program = ctx.accounts.token_program.to_account_info();

            token::mint_to(
                cpiContext::new(cpi_program,cpi_accounts),
                amount,
            )?;

            Ok(())
        }

    })
    






    #[derive(Accounts)]

    pub struct IntializeMMintCtx<'info>{
        #[account(
            init,
            payer=authority,
            space=82,
            mint::decimels=6,
            mint::authority=authority
        )]
        pub mint:Account<'info,Mint>,


        #[account(mut)]
        pub authority:Signer<'info>,


        pub rent:Sysvar<'info,Rent>
        pub system_program:Program<'info,System>,
        pub token_program :Program<'info,Token>,


    }


    #[derive(Accounts)]
    pub struct MintTokenCtx<'info>{


     pub mint:Account<'info,Mint>,

     pub recipient: Account<'info, TokenAccount>,

    pub authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

        
    }












}