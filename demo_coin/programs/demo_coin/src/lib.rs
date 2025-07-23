use anchor_lang::prelude::*;

declare_id!("6pnANYUXptMb5fj2uVyWiLCpfKFYBmzCjRGETD3usyC");

#[program]
pub mod demo_coin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
