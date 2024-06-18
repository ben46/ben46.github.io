原文: https://hackmd.io/@totomanov/gas-optimization-loops
#### 总结:
* 不要使用<= and >=（备注，0.8.17版本测试，好像并没有区别，反而使用小于等于更节省gas）
* for循环的i++这种可以使用unchecked, 从for那行拿到最下面
* 一些简单的函数尽量用Yul(也就是assembly) [如何使用Yul](https://docs.soliditylang.org/en/latest/yul.html)
* for循环中, 每次都要使用length, 可以把length存成变量, 放入stack中
* 除了storage类型, 其他情况uint8并不会比uint256更省,反而还要多做一次cast
* 在定义状态变量的时候，如果两个变量正好在同一个函数里面修改，那么这两个变量如果属于同一个slot将会节省gas

大于等于和小于等于如何使用比较省gas
-------------------------
```solidity
function ddd()external view{//23312
    require(ccc >= 0);
}
```
```solidity
function eee()external view{//23363
    require(ccc > 0);
}
```
```solidity
function bbb()external view{//23519
    require(ccc+1 > 0);
}
```

结论：上个版本可能有这个区别，但是这个版本好像没有这个区别了

附上另一个测试
```solidity
contract AAA{

    function loop_lte() external pure returns (uint256 sum) {//58308
        for(uint256 n = 0; n <= 99; n++) {
            sum += n;
        }
    }

    function loop_lt() external pure returns (uint256 sum) {//58633
        for(uint256 n = 0; n < 100; n++) {
            sum += n;
        }
    }

}
```


for循环怎么写比较节省gas
---------------------------
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

contract AAA{

    function loop_unchecked_plusplus() external pure returns (uint256 sum) {//46633
        for(uint256 n = 0; n < 100;) {
            sum += n;
            unchecked {
                n++;
            }
        }
    }

    function loop_checked_plusplus() external pure returns (uint256 sum) {//58611
        for(uint256 n = 0; n < 100;n++) {
            sum += n; 
        }
    }

    function loop_assembly() external pure returns (uint256) {//27752
        assembly {
            let sum := 0
            for {let n := 0} lt(n, 100) {n := add(n, 1)} {
                sum := add(sum, n)
            }
            mstore(0, sum)
            return(0, 32)
        }
    }

}
```

for循环中，储存length，节省gas
----------------------------------
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

contract AAA{

    function loopArray(uint256[] calldata ns) external pure returns (uint256 sum) {//26534
        for(uint256 i = 0; i < ns.length;) {
            sum += ns[i];
            unchecked {
                i++;
            }
        }
    }

    function loopArray_cached(uint256[] calldata ns) external pure returns (uint256 sum) {//26425
        uint256 length = ns.length;
        for(uint256 i = 0; i < length;) {
            sum += ns[i];
            unchecked {
                i++;
            }
        }
    }

    function loopArray_assembly(uint256[] calldata ns) external pure returns(uint256 sum) {//24218
        assembly {
            let guard := add(1, calldatasize())
            for {let offset := ns.offset} 
                lt(offset, guard) 
                {offset := add(offset, 32)} 

                {sum := add(sum, calldataload(offset))}
        }
    }

}
    
```

批量转账的时候
----------
可以用yul把函数选择器存入内存



把数组转成数字
---------
```
原来数组[1, 6, 13, 22, 24, 25, 30, 33, 94, 215]
假设每个数字都是8个字节, 那么最大也就是2^8=255
那么我们可以把这串数组转换为一个uint256
每个数字是两个16hex
转换完成后0x00000000000000000000000000000000000000000000d75e211e1918160d0601
从右开始读01,06,0d,16,…,d7. 

```

原来的函数
```solidity
function loopArray_cached(uint256[] calldata ns) public returns (uint256 sum) {
    uint256 length = ns.length;
    for(uint256 i = 0; i < length;) {
        sum += ns[i];
        unchecked {
            i++;
        }
    }
}
```

```solidity
function loop_arr_magic(uint256 encoded_ns, uint256 length) public returns (uint256 sum) {
    for(uint256 i = 0; i < length;) {
        sum += encoded_ns & 0xff;
        encoded_ns >>= 8; // mask移动8, 8 = 2个字节
        unchecked {
            i++;
        }
    }
}
```
# 把数组转成数字实战
比如gmx里面有一个需求, 每隔几秒钟, 就把链下的交易标记价格更新到链上, 本来的实现方式
```solidity
function setPrices(address[] memory _tokens, uint256[] memory _prices) external onlyAdmin {
    for (uint256 i = 0; i < _tokens.length; i++) {
        address token = _tokens[i];
        prices[token] = _prices[i];
        if (fastPriceEvents != address(0)) {
          IFastPriceEvents(fastPriceEvents).emitPriceEvent(token, _prices[i]);
        }
    }
    lastUpdatedAt = block.timestamp;
}
```
#### after
我们修改了传参方式和数据存储, 大幅的降低了gas(note: 16进制中, 我们吧uint256要移动4位, 才是移动16进制1位, 因为移动是2进制的算法, 2^4=16哦!)
```solidity
contract FastPriceFeed{ 
    // 我们这里12个hex算作一组数据, 64个hex一共一次可以传4个资产的价格数据, 由于是美元计价, 我们设计三个参数
    // 1. MUL表示数字是否要乘以/除以一堆0
    // 2. 0的个数
    // 3. 10个代表价格的数字
    uint256 constant BTC_MUL_MASK      = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0; // prettier-ignore
    uint256 constant BTC_MUL_START_BIT_POSITION = 0;

    uint256 constant BTC_DECIMALS_MASK = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF00F; // prettier-ignore
    uint256 constant BTC_DECIMALS_START_BIT_POSITION = 4;
    uint256 constant BTC_PRICE_MASK    = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF000000000FFF; // prettier-ignore
    uint256 constant BTC_PRICE_START_BIT_POSITION = 12; // 这个12是因为3个hex = 3*4=12

    uint256 constant ETH_MUL_MASK      = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0FFFFFFFFFFFF; // prettier-ignore
    uint256 constant ETH_MUL_START_BIT_POSITION = 0 + 12*4;

    uint256 constant ETH_DECIMALS_MASK = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF00FFFFFFFFFFFFF; // prettier-ignore
    uint256 constant ETH_DECIMALS_START_BIT_POSITION = 4 + 12*4;

    uint256 constant ETH_PRICE_MASK    = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF000000000FFFFFFFFFFFFFFF; // prettier-ignore
    uint256 constant ETH_PRICE_START_BIT_POSITION = 12 + 12*4; 

    uint256 public constant PRICE_PRECISION = 10 ** 6; // 我们保留美元的6位小数,足够了

    uint256 public priceMap;

    function setPricesMap1(uint256 _map1) external{
        priceMap = _map1;
    }

    function setBTCPrice(uint256 _price)external {
        priceMap = (priceMap & BTC_PRICE_MASK) | (_price << BTC_PRICE_START_BIT_POSITION);
    } 

    function setBTCDecimals(uint256 _decimals)external {
        priceMap = (priceMap & BTC_DECIMALS_MASK) | (_decimals << BTC_DECIMALS_START_BIT_POSITION);
    } 

    function setBTCMul(bool _mul)external {
        priceMap = (priceMap & BTC_MUL_MASK) | (uint256(_mul ? 1 : 0) << BTC_MUL_START_BIT_POSITION);
    }

    function getBTCPrice()public view returns(uint256){
        return (priceMap & ~BTC_PRICE_MASK) >> BTC_PRICE_START_BIT_POSITION;
    }

    function getBTCDecimals()public view returns(uint256){
        return (priceMap & ~BTC_DECIMALS_MASK) >> BTC_DECIMALS_START_BIT_POSITION;
    }

    function getBTCMul()public view returns(bool){
        return ((priceMap & ~BTC_MUL_MASK) >> BTC_MUL_START_BIT_POSITION) & 1 != 0;
    } 
    
    function setETHPrice(uint256 _price)external {
        priceMap = (priceMap & ETH_PRICE_MASK) | (_price << ETH_PRICE_START_BIT_POSITION);
    } 

    function setETHDecimals(uint256 _decimals)external {
        priceMap = (priceMap & ETH_DECIMALS_MASK) | (_decimals << ETH_DECIMALS_START_BIT_POSITION);
    } 

    function setETHMul(bool _mul)external {
        priceMap = (priceMap & ETH_MUL_MASK) | (uint256(_mul ? 1 : 0) << ETH_MUL_START_BIT_POSITION);
    }

    function getETHPrice()public view returns(uint256){
        return (priceMap & ~ETH_PRICE_MASK) >> ETH_PRICE_START_BIT_POSITION;
    }

    function getETHDecimals()public view returns(uint256){
        return (priceMap & ~ETH_DECIMALS_MASK) >> ETH_DECIMALS_START_BIT_POSITION;
    }

    function getETHMul()public view returns(bool){
        return ((priceMap & ~ETH_MUL_MASK) >> ETH_MUL_START_BIT_POSITION) & 1 != 0;
    } 
    
    function getBTCPriceWithDecimals()external view returns(uint256){
        if(getBTCMul()) {
            return getBTCPrice() * (10 ** getBTCDecimals()) * PRICE_PRECISION;
        } else {
            return getBTCPrice() / (10 ** getBTCDecimals())*  PRICE_PRECISION;
        }
    } 
}
```







