---
title: smart contract overview
---
# smart contract overview

在节点级别，链堆栈通过智能合约调用将交易数据直接传输到L2和L1网络。系统使用二进制树结构存储状态，包括可验证的本地和全局退出根。

## 共识合约

在以太坊世界里，一组共识合约和它们提供的功能，帮助对节点级别的序列化和验证机制进行驱动，比如序列化器和聚合器。这些合约确定了链的类型，比如是 validium 还是非 validium，通常每个 CDK 链都会有一个合约，提供独特的定制功能。


## rollup 管理

PolygonRollupManager.sol 这个合约的工作就是负责建立、更新和验证 CDK Rollup 和 Validium 链。也就是说，它负责处理这些链的实时数据和更新，确保它们的运行正常和有效。

## 桥合约
这个叫做 PolygonZkEVMBridgeV2.sol 的统一桥梁合约负责连接和确认 L1 和 L2 链的活动。 在 L1 网络里，这座桥梁还管理着复杂的退出根机制，控制系统状态。而在 L2 网络里，有一个较简单的退出根机制，控制这一层的状态。

## exit root management 合约
系统状态整体存储在像树一样的结构里，数据和退出信息存储在树的最底层。

每当在最底层的地方有更新，就会触发整个树顶端的退出信息更新，然后这些信息会经过桥梁传递到其他地方，以确保每个地方的信息都是最新的。

PolygonZkEVMGlobalExitRootV2.sol 合约管理着在以太坊 L1 网络上多个网络的退出信息。而 L2 的退出信息管理合约，PolygonZkEVMGlobalExitRootL2.sol，拥有一个更简单的退出信息机制。


# Validium stacks¶ Validium 堆栈¶

CDK Validium 堆栈用的是 cdk-validium-contracts。这些合约稍微调整了行为。它们考虑到了 Validium 组件和 CDK 的特定需求。

CDK仓库是zkEVM主合约仓库的一个分支。所以，所有的合约都继承了通用接口。

