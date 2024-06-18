# anchor-pda

## 情景1: 使用instruction_data作为seeds

```rust
#[derive(Accounts)]
#[instruction(instruction_data:String)] // 引入指令数据作为seeds
pub struct Example<'info> {
    #[account(
        seeds = [
            b"example_seed",
            user.key().as_ref(),
            instruction_data.as_ref() // instruction data 作为 seeds
        ],
        bump
    )]
    pub pda_account: Account<'info, AccountType>,

    #[account(mut)]
    pub user:Signer<'info>
}

#[account]
pub struct AccountType{
    pub data:u64
}
```

## 2. 使用init

```rust
#[derive(Accounts)]
pub struct InitializePda<'info> {
    #[account(
        init, // 增加init约束
        seed = [b"example_seed", user.key().as_ref()],
        bump,
        payer = user,//规定payer
        space = 8+8 // 必须一起使用, 指定初始化分配空间
    )]
    pub pda_account:Account<'info, AccountType>

    #[account(mut)]
    pub user:Signer<'info>

    pub system_program: Program<'info, System>, // 必须加入这个
}

#[account]
pub struct AccountType {
    pub data: u64,
}
```

## 3. init_if_needed

Cargo.toml里面启用`features` init-if-needed

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        payer=payer,
        associated_token::mint=mint,
        associated_token::authority=payer
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer:Signer<'info>,
    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
```
anchor检查token account是否存在, 如果不存在就初始化

## 4. realloc 重新分配空间

结合以下使用
* mut 账户必须可变
* realloc::payer 指定减少或者添加lam的账户, 根据realloc是减少还是增加账户空间
* realloc::zero 用户指定新内存是否应该进行zero初始化的bool值

```rust
#[derive(Accounts)]
#[instruction(instruction_data: String)]
pub struct ReallocExample<'info> {
    #[account(
        mut,// 可变
        seeds = [b"example_seed", user.key().as_ref()]
        bump,
        realloc = 8 + 4 + instruction_data.len(), 
        // 8: 账户鉴别器
        // 4: BORSH用于存储字符串长度的空间
        // 字符串本身的长度
        realloc::payer = user, // 如果账户数据长度的变化是增加的，lamports 将从 realloc::payer 转移到账户，以保持免租金。同样，如果变化是减少的，则 lamports 将从账户转移到 realloc::payer。
        realloc::zero = false, // 如果可能存在多次更新, 设置为true
    )]    
    pub pda_account: Account<'info, AccountType>,

    #[account(mut)]
    pub user: Signer<'info>
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AccountType{
    pub data: String // 因为空间不固定, 所以每个初始化的时候分配的空间不固定
}
```

## close 关闭

指令执行结束时将账户标记为关闭状态
```rust
pub fn close(ctx: Context<Close>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct Close<'info> {

    // 账户一旦关闭之后, 其他人无法重新初始化
    // 关闭之后的租金返回到receiver账户
    #[account(mut, close=receiver)]
    pub data_account: Account<'info, AccountType>,

    #[account(mut)]
    pub receiver: Signer<'info>
}
```

