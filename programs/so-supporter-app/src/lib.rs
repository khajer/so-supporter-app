use anchor_lang::prelude::*;

declare_id!("6YnirNoEnw8Tp7ZbaH1DV8KBwN7ScbHZNXQVeqaAZYct");

#[program]
pub mod so_supporter_app {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }
    pub fn create_post(
        ctx: Context<CreatePost>,
        title: String,
        img_url: String,
        desc: String,
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.title = title;
        user_account.img_url = img_url;
        user_account.desc = desc;

        Ok(())
    }
}

#[account]
pub struct PostAccount {
    title: String,
    img_url: String,
    desc: String,
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init, payer = user, space = 8 + std::mem::size_of::<PostAccount>() )]
    user_account: Account<'info, PostAccount>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>,
}
