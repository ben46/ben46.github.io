#[program]
mod program_module_name {
    use super::*;
    pub fn instruction_one(ctx: Context<InstructionAccounts>, instruction_data: u64) -> Result<()> {
        ...
        Ok(())
    }
}

// Deserializes this info
pub struct AccountInfo<'a> {
    pub key: &'a Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
    pub lamports: Rc<RefCell<&'a mut u64>>,
    pub data: Rc<RefCell<&'a mut [u8]>>,    // <---- deserializes account data
    pub owner: &'a Pubkey,    // <---- checks owner program
    pub executable: bool,
    pub rent_epoch: u64,
}

// 围绕着AccountInfo的包装器
// 验证这个程序的所有权
#[derive(Accounts)]
pub struct InstructionAccounts {
    // 实现辨别器, 所有者
    #[account(init, payer = user, space = 8 + 8)]

    // 把AccountInfo.data反序列化为AccountStruct
    // 生成的账户归这个程序所有
    pub account_name: Account<'info, AccountStruct>,

    #[account(mut)]
    // signer类型指定了user账户必须是指令的签名者
    // 会自动执行Signer.info.is_signer == true
    pub user: Signer<'info>,

    // 验证账户是否是某个特定程序
    // 会执行以下验证: 
    // account_info.key == expected_program
    // account_info.executable == true
    pub system_program: Program<'info, System>,
}

