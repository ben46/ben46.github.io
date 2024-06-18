---
title: 1-intro-to-custom-on-chain-programs
---
# 自定义程序介绍(非SPL)

https://decert.me/tutorial/sol-dev/zh-chs/intro-to-custom-on-chain-programs

## `AccountMeta` 类型的账户数组需要包含哪些信息？

* pubkey
* isSigner
* isWriteable

## 如何使用 TransactionInstruction 构造函数创建非原生指令？

`TransactionInstruction`有三个参数: keys, programId, data

* keys: AccountMeta类型的数组,  表示交易期间会被读取或者写入的账户
* programId: 程序地址
* data: 需要传递给程序的数据

比如:
```typescript
const instruction = new web3.TransactionInstruction({
    keys:[{
        pubkey: programDataAccount,
        isSigner: false,
        isWriteable: true
    }],
    programId,
})
```


## 如何在 Solana Explorer 中查看交易的详细信息？

1. 获取交易hash
2. 打开区块链浏览器, 输入hash, 就能查到
3. 如果你想看program logs, 滚动到底部, 里面会显示程序被调用次数