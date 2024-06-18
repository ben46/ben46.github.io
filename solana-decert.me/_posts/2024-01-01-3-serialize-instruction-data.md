---
title: 3-serialize-instruction-data
---
# 序列化指令数据

anchor里面不需要, 但是我们先看看原生的为什么需要

简单的说, 就是和solidity中的`abi.encode(xxx, yyy)`一样.

## 序列化

```
1. 交易中的指令数据是如何传递到服务器端的？
   A. 序列化为字节缓冲区
   B. 以 JSON 格式传输
   C. 通过 URL 参数传递
   D. 以 XML 格式传输

2. 每个指令包含哪些内容？
   A. 执行目标程序的程序 ID、账户数组、指令数据的字节缓冲区
   B. 执行目标程序的程序 ID、交易哈希、签名
   C. 账户数组、指令数据的字节缓冲区、交易哈希
   D. 账户数组、签名、交易哈希
  

3. 什么工具是Solana中最常用的序列化工具？
   A. JSON
   B. XML
   C. Borsh
   D. YAML

4. 如何将数据编码到缓冲区中？
   A. 使用 Borsh 库
   B. 使用 JSON 序列化
   C. 使用 XML 编码
   D. 使用 YAML 序列化
```
---

```
1. 交易中的指令数据是如何传递到服务器端的？
   A. 序列化为字节缓冲区
   B. 以 JSON 格式传输
   C. 通过 URL 参数传递
   D. 以 XML 格式传输
   Answer: A
   Citation: "为了将指令数据从客户端传递到服务器端，它必须被序列化为字节缓冲区。"

2. 每个指令包含哪些内容？
   A. 执行目标程序的程序 ID、账户数组、指令数据的字节缓冲区
   B. 执行目标程序的程序 ID、交易哈希、签名
   C. 账户数组、指令数据的字节缓冲区、交易哈希
   D. 账户数组、签名、交易哈希
   Answer: A
   Citation: "每个指令（instruction）包含以下内容：执行目标程序的程序 ID（公钥）、列出在执行过程中将被读取或写入的每个账户的数组、指令数据的字节缓冲区（byte buffer）。"

3. 什么工具是Solana中最常用的序列化工具？
   A. JSON
   B. XML
   C. Borsh
   D. YAML
   Answer: C
   Citation: "在Solana中最常用的序列化工具是 Borsh。"

4. 如何将数据编码到缓冲区中？
   A. 使用 Borsh 库
   B. 使用 JSON 序列化
   C. 使用 XML 编码
   D. 使用 YAML 序列化
   Answer: A
   Citation: "接下来，您可以使用这个模式（schema）和encode方法来编码数据。"

```

## 反序列化

```
1. 在 Solana 中，程序将数据存储在哪里？
   A. PDAs 中
   B. 普通账户中
   C. 私钥中
   D. 公钥中
2. 为了存储和定位数据，可以使用什么方法派生一个 PDA？
   A. `findProgramAddress(seeds, programid)`
   B. `getProgramAccounts(programId)`
   C. `createProgramAddress(seeds, programid)`
   D. `deriveProgramAddress(seeds, programid)`

3. PDAs 是否具备相应的私钥？
   A. 是
   B. 否
   C. 可能有
   D. 未知

4. 什么方法可以用于获取由程序创建的所有账户？
   A. `connection.getProgramAccounts(programId)`
   B. `findProgramAddress(seeds, programid)`
   C. `getProgramAccounts(programId)`
   D. `createProgramAddress(seeds, programid)`
```

---

```
1. 在 Solana 中，程序将数据存储在哪里？
   A. PDAs 中
   B. 普通账户中
   C. 私钥中
   D. 公钥中
   Answer: A
   Citation: "程序将数据分开存储，存储在 PDAs 中，即**程序衍生地址**（Program Derived Address）。"

2. 为了存储和定位数据，可以使用什么方法派生一个 PDA？
   A. `findProgramAddress(seeds, programid)`
   B. `getProgramAccounts(programId)`
   C. `createProgramAddress(seeds, programid)`
   D. `deriveProgramAddress(seeds, programid)`
   Answer: A
   Citation: "为了存储和定位数据，可以使用 `findProgramAddress(seeds, programid)` 方法派生一个 PDA。"

3. PDAs 是否具备相应的私钥？
   A. 是
   B. 否
   C. 可能有
   D. 未知
   Answer: B
   Citation: "PDAs 不具备相应的私钥。"

4. 什么方法可以用于获取由程序创建的所有账户？
   A. `connection.getProgramAccounts(programId)`
   B. `findProgramAddress(seeds, programid)`
   C. `getProgramAccounts(programId)`
   D. `createProgramAddress(seeds, programid)`
   Answer: A
   Citation: "除了派生地址之外，您还可以使用 `connection.getProgramAccounts(programId)` 获取由程序创建的所有账户。"
```

## 处理指令数据

```
1. 在 Rust 中，什么关键字用于定义不可变变量？
   A. let
   B. mut
   C. const
   D. var

2. Rust 中的枚举类型允许您通过列举可能的什么来定义一个类型？
   A. 函数
   B. 结构体
   C. 字段
   D. 变体

3. 在 Rust 中，哪个关键字用于为类型添加实现（implementations）？
   A. derive
   B. trait
   C. impl
   D. struct

4. 在处理指令数据时，程序通常会创建一个什么类型来表示指令？
   A. 结构体
   B. 函数
   C. 枚举
   D. 字段

```
---
```
1. 在 Rust 中，什么关键字用于定义不可变变量？
   A. let
   B. mut
   C. const
   D. var
   Answer: A
   Citation: "在 Rust 中，变量（variable）赋值使用 `let` 关键字。"

2. Rust 中的枚举类型允许您通过列举可能的什么来定义一个类型？
   A. 函数
   B. 结构体
   C. 字段
   D. 变体
   Answer: D
   Citation: "枚举类型（Enumerations，或简称枚举，Enums）是一种数据结构，允许您通过列举其可能的变体（variants）来定义一个类型。"

3. 在 Rust 中，哪个关键字用于为类型添加实现（implementations）？
   A. derive
   B. trait
   C. impl
   D. struct
   Answer: C
   Citation: "在 Rust 中，使用 `impl` 关键字来定义类型的实现（implementations）。"

4. 在处理指令数据时，程序通常会创建一个什么类型来表示指令？
   A. 结构体
   B. 函数
   C. 枚举
   D. 字段
   Answer: C
   Citation: "由于指令数据以字节数组的形式提供给程序的入口点，通常会创建一个 Rust 数据类型来表示指令，以便在整个代码中更方便地使用。"
```