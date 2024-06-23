---
title: Exit roots
---

# Exit roots

下面的类图描述了退出根合约如何与桥和rollup管理器进行通信。
![sdfsd](https://docs.polygon.technology/img/cdk/high-level-architecture/exit-root-class-diagram.png)
退出树是一个二进制的、只追加的、稀疏的 Merkle 树（SMT），其叶节点存储桥接数据。退出树的深度为32。

退出树的 Merkle 根被称为退出树根，它是记录在退出树叶节点中的所有信息的指纹。

因此，L1 信息树的全局退出树根是整个网络的真实性来源。

# Rollup local exit trees¶
L2桥接合约管理着一种特殊的默克尔树，我们叫它本地退出树。每个参与桥接和索赔的网络都有这么一棵树。这个树是由PolygonZkEVMGlobalExitRootL2.sol合约来更新的。
![dfdf](https://docs.polygon.technology/img/cdk/high-level-architecture/local-exit-tree.png)
桥接资产和桥接信息的调用数据，都被存储在本地退出树的叶子节点上。

# Exit tree for rollups¶

L2的本地退出树，它们的根都连到一个单独的退出树。这个树负责管理所有参与的L2 rollup的状态。这个状态是存在L1领域里的，然后在排序的时候可以访问。
![sdf](https://docs.polygon.technology/img/cdk/high-level-architecture/exit-tree-for-rollups.png)
L2本地退出树的根，通过调用getRollupExitRoot()这个方法，可以在rollup管理器上访问到。

# L1 local exit tree

每次在L1以太坊层调用bridgeAsset()和bridgeMessage()时，
数据就会被存到L1本地退出树的一个叶子节点上。
![sdf](https://docs.polygon.technology/img/cdk/high-level-architecture/l1-ethereum-exit-tree.png)
# L1 info tree

L1信息树存储在PolygonZkEVMGlobalExitRootV2.sol合约里，这个合约也被称作全局退出根管理器。
所有子树的退出根都汇集到L1信息树的叶子上，这个树包含了全局退出根（GER）。
GER是所有树存储信息的指纹，代表了系统的全局状态。
![sdfsdf](https://docs.polygon.technology/img/cdk/high-level-architecture/l1-info-tree.png)

# Exit leaves
在桥接合约中，有两个常量定义了交易叶子的类型。
叶子中的数据包含了元数据的Keccak256哈希值（如果有的话，就是ABI编码的元数据），以及以下参数（根据桥接L1到L2文档中公开可见的交易数据匹配）：

# Updating system state¶

系统用一组退出树根来管理状态。树的叶子指向像上面详细描述的交易数据。

给树添加一片新叶子，就会更新退出树根，然后这个更新会传播到全局退出树根的更新。

使用Merkle树退出根，由桥接合约引用，并且PolygonRollupManager合约可以访问，桥接合约触发L1和L2之间的数据同步，包括在排序器和状态数据库层面。

使用两个不同的全局退出根管理合约来处理L1和L2，以及排序流程和桥接合约的独立逻辑，允许广泛的网络互操作性。同时，由于状态数据的可访问性，所有资产转移都可以由任何L1和L2节点验证。

退出根在两个关键流程中被修改：排序和桥接。

# Sequencing flow¶

PolygonZkEVM的GlobalExitRootV2合约，负责在序列化过程中更新退出根。
合约在序列化流程中调用GlobalExitRootManager的updateExitRoot(...)方法，
用来给相关的退出树添加一个退出叶子。

![sd](https://docs.polygon.technology/img/cdk/high-level-architecture/update-exit-roots-via-sequencing.png)

开始更新：PolygonZkEVMEtrog 通过调用 PolygonRollupBaseEtrog 的 updateExitRoots 来启动更新过程。
获取当前根：PolygonRollupBaseEtrog 从 PolygonZkEVMGlobalExitRootL2 和 PolygonZkEVMGlobalExitRootV2 分别获取当前的本地和全局退出根。
计算新退出根：PolygonRollupBaseEtrog 根据获取的本地和全局退出根来计算新的退出根。
更新本地退出根：PolygonRollupBaseEtrog 在 PolygonZkEVMGlobalExitRootL2 中更新本地退出根。
更新全局退出根：PolygonRollupBaseEtrog 在 PolygonZkEVMGlobalExitRootV2 中更新全局退出根。
验证更新后的退出根：PolygonRollupBaseEtrog 调用 PolygonRollupManager 的 getRollupExitRoot 来验证更新后的退出根。

# Bridging flow¶

当进行桥接时，如果forceUpdateGlobalExitRoot变量设置为真，就会更新全局退出根。
用户通过调用bridge()函数与PolygonZkEVMBridgeV2合约交互。

![sdf](https://docs.polygon.technology/img/cdk/high-level-architecture/update-exit-roots-via-bridging.png)
PolygonZkEVMBridgeV2在PolygonZkEVMGlobalExitRootL2上调用updateLocalExitRoot()，这会更新本地退出根。
如果forceUpdateGlobalExitRoot设置为真，PolygonZkEVMBridgeV2就在PolygonZkEVMGlobalExitRootV2上调用updateGlobalExitRoot()，这会更新全局退出根。












