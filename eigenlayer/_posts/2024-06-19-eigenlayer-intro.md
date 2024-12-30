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

# 什么是 EigenLayer 中的 Node Operator？




---


1. **什么是EigenLayer？**
   - EigenLayer是一个以太坊重质押协议，旨在通过利用ETH的安全性来支持新兴的区块链服务，降低进入Web3服务的门槛。

2. **什么是主动验证服务（AVS）？**
   - 主动验证服务（AVS）是任何基于区块链的系统，利用EigenLayer的重质押机制在以太坊网络上支持独特的验证方法。

3. **AVS如何解决Web3开发中的安全碎片化问题？**
   - AVS通过共享安全层，允许以太坊质押者重质押ETH或流动质押代币（LSTs），消除了新服务从零开始建立安全性的需求。

4. **AVS的主要参与者有哪些？**
   - AVS的主要参与者包括重质押者、运营商、主动验证服务和AVS用户。

5. **AVS如何运作？**
   - AVS由智能合约管理其功能，包括运营商的注册和安全质押的金额。它们结合了链下执行和链上强制执行。

6. **EigenLayer如何实现“集中安全”？**
   - EigenLayer通过重质押机制实现集中安全，结合所有主动验证服务的以太坊安全性，增加攻击成本并提高效率。

7. **AVS的高影响力用例有哪些？**
   - 高影响力用例包括事件驱动激活、超大规模数据可用性层和快速模式的跨链桥接。

8. **EigenLayer支持哪些商业模型？**
   - EigenLayer支持多种商业模型，包括服务提供者模型、基于代币的协议模型、原生代币模型和双重质押模型。

9. **目前有哪些已存在的AVS？**
   - 已存在的AVS包括Ava Protocol、EigenDA、Hyperlane、AltLayer、Lagrange等。

10. **EigenLayer的未来展望是什么？**
    - 随着更多AVS在EigenLayer上推出，Web3将快速发展，跨链互操作性和去中心化计算的创新将变得可行，可能会改变开发者和用户对区块链基础设施和服务开发的看法。
