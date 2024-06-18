# event and topics
topics是一个特殊的数据结构, 代替了log数据的部分

一个主题只能hold一个简单的单词(32bytes, 256bit). 

如果indexed你用了引用类型, 将会存储该值的hash作为主题

所有的参数, 没有索引属性的, 将会被abi-encoded到log的数据部分

主题允许你搜索events

event的签名hash之后, 也是主题之一, 除非你定义了匿名

匿名的主题只能用合约地址来搜索

```solidity
event Swap (index_topic_1 address sender, index_topic_2 address recipient, int256 amount0, int256 amount1, uint160 sqrtPriceX96, uint128 liquidity, int24 tick)
```

这种就是三个topics

签名一个`keccak256("Swap(address,address,int256,int256,uint160,uint128,int24)")`

address分别两个
