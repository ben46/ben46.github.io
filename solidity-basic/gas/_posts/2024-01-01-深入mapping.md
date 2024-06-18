![image](https://user-images.githubusercontent.com/1460432/206158045-7a3ce804-5183-4944-91a1-f0d042195447.png)

数据存在了keccake256(key.(v的插槽)), 这个算出来的数字是16进制的, 我们把他转换成十进制, 就是slot的位置(最大2^256).

![image](https://user-images.githubusercontent.com/1460432/206159277-9351bfb3-c956-45ed-b12b-ce62a6c354e3.png)

和其他类型一样, 使用public, 自动生成mapping getter

找到具体的slot地址
```solidity
function findMapLocation(uint256 slot, uint256 key) public pure returns (uint256) {
    return uint256(keccak256(abi.encode(key, slot)));
}
```

mapping作为函数传参
 * 只能在私有方法使用
 * 必须定义为storage
