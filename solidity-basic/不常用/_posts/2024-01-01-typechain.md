# 什么是typechain

Ethereum智能合约的TypeScript绑定
TypeChain是一个代码生成器-提供ABI文件和区块链访问库的名称（ethers/truffle/web3.js），您将获得与给定库兼容的TypeScript类型。

静态类型-您将不再调用不存在的方法

IDE支持-适用于任何支持Typescript的IDE

可扩展-使用多种不同的工具：ether.js、hardhat、truffle、Web3.js或您可以创建自己的目标

无摩擦-适用于简单的JSON ABI文件以及Truffle/Hardhat工件

---

需要添加tsconfig.json

```JSON
{
  "compilerOptions": {
    "target": "es5",
    "module": "commonjs",
    "strict": true,
    "esModuleInterop": true,
    "outDir": "dist",
    "resolveJsonModule": true
  },
  "include": ["./scripts", "./test", "./typechain-types"],
  "files": ["./hardhat.config.ts"]
}
```




需要在hardhat.config.ts里面添加

```TS
typechain: {
    target: 'truffle-v5',
  }
  ```

添加依赖
```
"devDependencies": {
    "@nomiclabs/hardhat-etherscan": "^2.1.1",
    "@nomiclabs/hardhat-truffle5": "^2.0.0",
    "@nomiclabs/hardhat-web3": "^2.0.0",
    "@typechain/hardhat": "workspace:^6.1.4",
    "@typechain/truffle-v5": "workspace:^8.0.2",
    "@types/chai": "^4.2.15",
    "@types/chai-as-promised": "^7.1.3",
    "@types/mocha": "^8.2.0",
    "@types/node": "^14",
    "chai": "^4.3.0",
    "chai-as-promised": "^7.1.1",
    "chai-bn": "^0.2.1",
    "dotenv": "^8.2.0",
    "hardhat": "^2.9.9",
    "ts-generator": "0.0.8",
    "ts-node": "^10.7.0",
    "typechain": "workspace:^8.1.1",
    "typescript": "^4.6",
    "web3": "^1.3.4",
    "web3-core": "^1",
    "web3-eth-contract": "^1",
    "web3-core-helpers": "^1.2.1",
    "web3-core-promievent": "^1.2.1",
    "web3-eth-abi": "^1.2.1",
    "web3-utils": "^1.2.1",
    "bn.js": "^4.11.0",
    "@types/bn.js": "^4.11.6"
  }
```

然后运行

```
pnpm install # it will automatically run TypeChain types generation
# pnpm generate-types to manually regenerate them
pnpm test
```

会自动生成typechain-types文件夹

然后在test文件中
```ts
import {xxxcontract, xxxInstance} from '../typechain-types'
```





