---
title: 2-interact-with-wallets
---
# 与钱包互动

https://decert.me/tutorial/sol-dev/zh-chs/interact-with-wallets



## Solana 的钱包适配器库的作用是什么？它简化了什么过程？

方便连接浏览器钱包插件, @solana/wallet-adapter-base 和 @solana/wallet-adapter-react

比如获取用户钱包地址

对交易签名

发送交易

开发人员不用从头去开发各种签名过程

## @solana/wallet-adapter-base 是什么包?

链接钱包, 处理交易

## @solana/wallet-adapter-react是什么包?

提供了钩子和上下文provider, 用于持久保存和获取钱包连接状态, 以及处理交易状态

## 3. 如何连接钱包到现有的 React 应用程序中？

安装三个包

* @solana/wallet-adapter-base
* @solana/wallet-adapter-react 
* @solana/wallet-adapter-react-ui

### 第一步, 使用 ConnectionProvider 和 WalletProvider 包装整个应用程序，确保 ConnectionProvider 需要一个 endpoint 属性，WalletProvider 需要一个 wallets 属性。

```typescript
import { NextPage } from "next";
import { FC, ReactNode, useMemo } from "react";
import { ConnectionProvider, WalletProvider } from "@solana/wallet-adapter-react";
import { WalletModalProvider, WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import * as web3 from "@solana/web3.js";

const Home: NextPage = (props) => {
    const endpoint = web3.clusterApiUrl("devnet");
    const wallets = useMemo(() => [], []);

    return (
        <ConnectionProvider endpoint={endpoint}>
            <WalletProvider wallets={wallets}>
                <WalletModalProvider>
                    <WalletMultiButton />
                    <p>Put the rest of your app here</p>
                </WalletModalProvider>
            </WalletProvider>
        </ConnectionProvider>
    );
};

export default Home;
```

### 第二步, 在应用程序中使用 `useWallet` 和 `useConnection` 钩子来持久保存和访问钱包连接状态。

```typescript
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { FC, useEffect, useState } from "react";

export const BalanceDisplay: FC = () => {
    const [balance, setBalance] = useState(0);
    const { connection } = useConnection();
    const { publicKey } = useWallet();

    useEffect(() => {
        if (!connection || !publicKey) {
            return;
        }

        connection.onAccountChange(
            publicKey,
            (updatedAccountInfo) => {
                setBalance(updatedAccountInfo.lamports / LAMPORTS_PER_SOL);
            },
            "confirmed"
        );

        connection.getAccountInfo(publicKey).then((info) => {
            setBalance(info.lamports);
        });
    }, [connection, publicKey]);

    return (
        <div>
            <p>{publicKey ? `Balance: ${balance / LAMPORTS_PER_SOL} SOL` : ""}</p>
        </div>
    );
};
```

### 第三步, 在用户点击连接按钮时，使用 `wallet.connect()` 进行连接，这将提示用户是否有权查看其公钥并请求批准交易。

```typescript
import { useWallet, useConnection } from "@solana/wallet-adapter-react";
import * as web3 from "@solana/web3.js";
import { FC } from "react";

export const ConnectButton: FC = () => {
    const { connection } = useConnection();
    const { publicKey, wallet } = useWallet();

    const handleConnect = () => {
        if (!connection || !publicKey) {
            return;
        }

        wallet.connect().then(() => {
            console.log("Wallet connected successfully");
        });
    };

    return (
        <div>
            <button onClick={handleConnect}>Connect Wallet</button>
        </div>
    );
};
```

## 5. 在实现一个应用程序中，如何使用钱包适配器发送交易并请求用户批准？

1. 安装库
2. 使用useWallet获取钱包上下文状态和sendTransaction函数
3. 构建交易指令.  
4. 调用sendTranaction

