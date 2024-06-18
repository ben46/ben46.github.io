# 如何减小合约size

# 1.效果拔群

#### 分拆合约
* 尽量分拆合约
* 每个合约都搞一个interface, 这样引入的时候就不需要引入整个合约, 可以减少合约size

#### 使用代理合约

# 2. 效果中等
#### 移除函数
* 移除不必要的view函数
* 只被调用一次的private函数, 尽量合并, 使用{}内联代码


#### 减少不必要的变量
```solidity
function get(uint id) returns (address,address) {
    MyStruct memory myStruct = myStructs[id]; // 这里没必要, 直接合并成1行
    return (myStruct.addr1, myStruct.addr2);
}

```


#### 缩短require报错信息


# 3. 轻微影响


#### 函数可见性
* 如果两个合约在一个文件, 合约之间的调用应该为external,而不是public(节省size)
* 内部调用声明为private或者internal, 而不是public
#### 不要把结构体作为参数传给函数
```solidity
function get(uint id) returns (address,address) {
    return _get(myStruct);
}

// 合约文件会变大
function _get(MyStruct memory myStruct) private view returns(address,address) {
    return (myStruct.addr1, myStruct.addr2);
}

```
修改后
```solidity
function get(uint id) returns(address,address) {
    return _get(myStructs[id].addr1, myStructs[id].addr2);
}

function _get(address addr1, address addr2) private view returns(address,address) {
    return (addr1, addr2);
}

```





#### modifier不要频繁使用
