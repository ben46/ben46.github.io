# what is eigenlayer

很多区块链服务(比如预言机)的信任机制都是依赖于质押. eigenlayer就是利用了已经质押在beacon chain上面的eth, 做一次再质押.

支持多种质押方式
* 直接质押: 将质押在以太坊上的直接质押
* LSD质押: 质押在lido或者rocket pool的质押
* ETH LP质押: 质押在DEFI协议中的LP token质押过来
* LSD LP质押: 比如curve的stETH-ETH等LPToken质押过来

那么这里和其他所有的质押一样. 有eigenlayer, 有不同AVS服务节点运营商(validator/operator), 有staker(delegator)

# 为什么会出现
自从二层网络,模块化区块链越来越火之后诞生的.

以太坊的安全性只扩展到了智能合约这一层.

但是排序器,垮脸桥,DA层的安全无法保证.

eigenlayer通过restaking, 来给这些提供PoS安全机制提供保障.

# eigenlayer中AVS,operator分别是什么角色,互相有什么关系?

operator=validator

avs=DA服务之类的

avs: 

* altlayer: faster settlement层+DA层
* Blockless: 计算服务
* Lagrange: 并行zk证明
* near: settlement layer
 