call和delegatecall的区别主要是后者不改变msg的上下文（context）
```solidity
pragma solidity ^0.8.10;

contract Called {
  event callEvent(address sender, address origin, address from);
  function callMe() public {
    emit callEvent(msg.sender, tx.origin, address(this)); // tx.origin 是全局变量
// 意思是交易的发送者    sender of the transaction (full call chain)
  }
}

contract Caller {
  function makeCalls(address _contractAddress) public {   
    address(_contractAddress).call(abi.encodeWithSignature("callMe()"));
    address(_contractAddress).delegatecall(abi.encodeWithSignature("callMe()"));
  }
}
```
js调用
```javascript
const hre = require('hardhat');
  const main = async () => {
    ...
    const tx = await caller.makeCalls(called.address);
    const res = await tx.wait();
    ...
    const iface = new hre.ethers.utils.Interface(eventAbi);
    ...
    console.log(`EOA address: ${signer.address}`);
    console.log(`Caller contract address: ${caller.address}`);
    console.log(`Called contract address: ${called.address}`);

    res.events.map(event => console.log(iface.parseLog(event)));
  };

  main();
```

ouput
```
EOA address: 0xf39F...2266
Caller contract address: 0xe7f1...512
Called contract address: 0x9fE4...6e0

sender: '0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512',
origin: '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266',
from: '0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0'

sender: '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266',
origin: '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266',
from: '0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512'
```
### call
1. sender是Caller合约。
2. origin是发送交易执行的帐户Caller.makeCalls。（一般也就是用户的账户）
3. from是Called合约。

### delegatecall
1. sender是EOA！
2. origin也是EOA！
3. from是Caller合约，而不是Called（实际发出事件的合约）。
