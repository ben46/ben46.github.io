## 为什么bytes在函数传参的时候,必须定义calldata/memory

因为bytes是数组类型,也叫引用类型,占用空间大

## calldata和memory区别

calldata的数据不能更改, 一般用在函数入参的引用类型, 比如bytes[], uint[]

## memory赋值给memory, 如果我改变原变量, 新赋值的变量会变吗?

会, 因为是创建了引用

## solidity函数参数里面storage代表什么?

传入的是状态变量的指针, 可以直接对状态变量进行读写. 