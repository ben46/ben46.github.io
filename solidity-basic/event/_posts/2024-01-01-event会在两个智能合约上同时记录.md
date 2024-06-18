```
Function: getPair(address, address)

tx hash
0x248861ac30d61a449fa97ce1298e1e790ce9fb99277a5e5db0fa249e9cdea794 

MethodID: 0xe6a43905
[0]:  000000000000000000000000          0x92fF563cE14fC62A5A87961CaBf1f98748fbBaEe
[1]:  000000000000000000000000         c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2

tokenA
92ff563ce14fc62a5a87961cabf1f98748fbbaee

tokenB
c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2

pair 
0x7Db49a4f1ff13Bc855d4ec9B2C13d519290F1F30

tx 3
# 1
0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2
topics
0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef
0x7Db49a4f1ff13Bc855d4ec9B2C13d519290F1F30 -> 0x777D0dCC4615ccfE3E575C20219fdF0BBe8251C7 wad 5918541755971593

#3
0x7db49a4f1ff13bc855d4ec9b2c13d519290f1f30
0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822
0x777D0dCC4615ccfE3E575C20219fdF0BBe8251C7 -> 0x777D0dCC4615ccfE3E575C20219fdF0BBe8251C7
```




下面两个方法, 分别查询两个智能合约, 同一个地址发出的transfer方法

最后结果显示, 同一个transfer log, 会同时记录在两个智能合约上
```js
import { ethers } from "https://cdn-cors.ethers.io/lib/ethers-5.6.9.esm.min.js";
const INFURA_ID = ''
const provider = new ethers.providers.JsonRpcProvider(`https://mainnet.infura.io/v3/${INFURA_ID}`)
  
async function func1(){
    const abiWETH = [
        "event Transfer(address indexed from, address indexed to, uint amount)"
    ];
    const addressWETH = '92ff563ce14fc62a5a87961cabf1f98748fbbaee'
    const contract = new ethers.Contract(addressWETH, abiWETH, provider)
    const ret = await contract.filters.Transfer('0x7Db49a4f1ff13Bc855d4ec9B2C13d519290F1F30')
    console.log(ret)

}

async function func3(){
    const abiWETH = [
        "event Transfer(address indexed from, address indexed to, uint amount)"
    ];
    const addressWETH = '0x7Db49a4f1ff13Bc855d4ec9B2C13d519290F1F30'
    const contract = new ethers.Contract(addressWETH, abiWETH, provider)
    const ret = await contract.filters.Transfer('0x7Db49a4f1ff13Bc855d4ec9B2C13d519290F1F30')
    console.log(ret)
}

func1()
func3()

```

```output
(2) {address: "92ff563ce14fc62a5a87961ca...}
address
:
"92ff563ce14fc62a5a87961cabf1f98748fbbaee"
topics
:
(2) ["0xddf252ad1be2c89b69c2b068fc378daa...]
0
:
"0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
1
:
"0x0000000000000000000000007db49a4f1ff13bc855d4ec9b2c13d519290f1f30"
(2) {address: "0x7Db49a4f1ff13Bc855d4ec9...}
address
:
"0x7Db49a4f1ff13Bc855d4ec9B2C13d519290F1F30"
topics
:
(2) ["0xddf252ad1be2c89b69c2b068fc378daa...]
0
:
"0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
1
:
"0x0000000000000000000000007db49a4f1ff13bc855d4ec9b2c13d519290f1f30"
```
