## example #1
```solidity
contract C {
    struct S { uint16 a; uint16 b; uint256 c; }
    uint x;
    mapping(uint => mapping(uint => S)) data;
}
 ```
 让我们计算data[4][9].c的存储位置。
 
 mapping本身的位置是1（前面有32个字节的变量x）。
 
 这意味着data[4]存储在`keccak256（uint256（4）.unt256（1））`。 
 
 data[4]的类型也是mapping， data[4][9]的数据从slot `keccak256（uint256（9）.kecak256（duint256（4）.unt256（1））`开始。
 
 结构S内成员c的slot偏移量为1，因为a和b被压缩在一个slot中。
  
 c是`keccak256（uint256（9）.kecak256（duint256（4）.uint256（1））+1`
 
 值的类型是uint256，因此它使用单个插slot。
 
## example #2 一维数组
```solidity
contract C {
    uint256 y;
    uint8[] x;
}
```

x[2]的储存位置应该是`keccak256(keccak256(1) + 2)`.

假设取到的slot值为v,

那元素则为`v >> 0 & type(uint8).max`,也就是`v >> 0`,也就是`v`

## example #3 二维数组
```solidity
contract C {
    uint256 y;
    uint24[][] x;
}
```

slot0储存x的长度, 因为元素长度为3字节(不超过16字节), 因此共享存储曹.

x[i][j]的储存位置应该是`keccak256(keccak256(2) + i) + floor(j / floor(256 / 24))`.

假设取到的slot值为v,

那元素则为`(v >> ((j % floor(256 / 24)) * 24)) & type(uint24).max`.

