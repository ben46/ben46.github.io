---
title: 4-program-state-management
---
# 状态管理

### 测验

```
1. 程序状态存储在哪里？
   A. 在程序本身中
   B. 在 PDA 账户中
   C. 在系统程序中
   D. 在用户账户中

2. 创建一个新账户需要通过什么进行跨程序调用？
   A. invoke
   B. invoke_signed
   C. create_account
   D. system_program

3. 如何将数据序列化到账户中？
   A. 使用 BorshSerialize trait
   B. 使用 BorshDeserialize trait
   C. 使用 derive 属性宏
   D. 使用 try_from_slice_unchecked 函数
   
4. 什么是程序派生地址（PDA）？
   A. 从账户 ID 派生而来的地址
   B. 从程序 ID 和种子列表派生而来的地址
   C. 从系统程序派生而来的地址
   D. 从用户账户派生而来的地址
   
```

```
   Answer: B
   Citation: "Solana 通过使程序无状态（stateless）来保持速度、效率和可扩展性。程序不会将状态存储在程序本身上，而是使用 Solana 的账户模型从单独的 PDA 账户中读取状态并写入状态。"

   Answer: B
   Citation: "在我们的程序中创建一个新账户需要进行跨程序调用（Cross Program Invocation，CPI）。CPI 是一个程序调用另一个程序上的指令的过程。要在我们的程序中创建一个新账户，我们将在系统程序上调用 `create_account` 指令。CPI 可以使用 `invoke` 或 `invoke_signed` 来完成。"

   Answer: A
   Citation: "一旦表示账户数据的 Rust 实例已经使用适当的值进行更新，您就可以将数据“保存”在账户上。这是通过您创建的 Rust 类型实例上的 `serialize` 函数来完成的。"

   Answer: B
   Citation: "PDAs 是使用程序 ID（创建账户的程序的地址）和一个可选的“种子（seeds）”列表派生而来的。"
```


## 什么是solana PDA账户

答：PDA是从程序 ID 和一个可选的种子列表派生而来的地址，用于存储账户的状态。

## 一个program有几个PDA账户?

一个program可以有多个PDA账户。每个PDA账户是通过程序 ID 和一个可选的种子列表派生而来的，可以用于存储不同的状态数据。因此，一个program可以创建多个PDA账户来管理不同的状态信息。

## 什么是solana程序状态?

Solana程序状态的主要作用是帮助程序保持速度、效率和可扩展性。

通过将程序状态存储在单独的PDA账户中，程序可以更轻松地读取和写入状态数据，而不必在程序本身上存储大量数据。

这种无状态的设计使得Solana程序更加灵活和高效。

程序状态存储在其他账户中还有助于提高程序的安全性，因为只有拥有相应私钥的程序才能访问和修改PDA账户中的状态数据。

总的来说，Solana程序状态的使用可以简化程序的设计和开发过程，同时提高程序的性能和安全性。

## 程序状态存储哪里?

程序状态存储在其他账户中，而不是在程序本身中。

在Solana中，其他账户指的是Solana账户，每个Solana账户都有一个data字段，其中保存着一个字节数组。

程序状态存储在这些Solana账户中，而不是存储在程序本身上。 

Solana的账户模型使得程序可以从单独的PDA（程序派生地址）账户中读取状态并写入状态。


# CPI

q

```
1. 什么是CPI（Cross Program Invocation）？
   A. 一种加密货币
   B. 一个程序调用另一个程序上的指令的过程
   C. 一种网络协议
   D. 一种编程语言

2. CPI中的`invoke_signed`函数有什么作用？
   A. 使用私钥进行签名
   B. 使用可选的种子、增量种子和程序 ID 来派生一个 PDA 并签署一条指令
   C. 用于创建新账户
   D. 用于序列化数据

3. 在CPI中，为什么使用`invoke_signed`而不是`invoke`函数？
   A. `invoke_signed`更简单易用
   B. `invoke_signed`可以安全地签署交易
   C. `invoke`函数已经被弃用
   D. `invoke_signed`可以跳过系统程序的验证

4. CPI中的`invoke_signed`函数如何确保安全性？
   A. 使用私钥进行签名
   B. 使用可选的种子、增量种子和程序 ID 来派生一个 PDA 并签署一条指令
   C. 通过重新派生PDA并与账户进行比较来签署交易
   D. 通过直接发送指令来签署交易
```

---

ans

```
1. B. 一个程序调用另一个程序上的指令的过程
   - 解释：CPI是一种程序调用另一个程序上的指令的过程，用于在Solana程序中创建新账户等操作。

2. B. 使用可选的种子、增量种子和程序 ID 来派生一个 PDA 并签署一条指令
   - 解释：`invoke_signed`函数在CPI中的作用是使用可选的种子、增量种子和程序 ID 来派生一个PDA并签署一条指令。

3. B. `invoke_signed`可以安全地签署交易
   - 解释：`invoke_signed`函数相对于`invoke`函数的优势之一是可以安全地签署交易，通过重新派生PDA并与账户进行比较来确保安全性。

4. C. 通过重新派生PDA并与账户进行比较来签署交易
   - 解释：`invoke_signed`函数确保安全性的方式是通过重新派生PDA并与账户进行比较来签署交易，从而防止恶意程序生成与使用另一个程序 ID 派生的PDA 对应的匹配 PDA。

```