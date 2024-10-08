use anchor_lang::prelude::*;

declare_id!("6YnirNoEnw8Tp7ZbaH1DV8KBwN7ScbHZNXQVeqaAZYct");

#[program]
pub mod so_supporter_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
