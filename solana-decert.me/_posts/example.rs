// 初始化一个新账户
// 使用传入指令的数据更新账户上的数据字段

use anchor_lang::prelude::*;

declare_id!("F8Q1Z");

// 定义program入口
#[program]
mod program_modeul_name {
    use super::*;

    // 初始化一个新账户
    pub fn initialize_account(ctx: Context<InitializeAccount>, data: u64) -> Result<()> {
        ctx.accounts.account_name.data = data;
        Ok(())
    }

}


// 定义账户
#[derive(Accounts)]
pub struct InsAccounts{

    #[account(init, payer = user, space = 8 + 8)]
    pub account_name: Account<'info, AccountStruct>,

    #[account(mut)]
    pub user:Signer<'info>,

    pub system_program:Program<'info, System>,
}


// 定义结构体
#[account] // 序列化用
pub struct AccountStruct {
      data: u64,
}