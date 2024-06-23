---
title: 4-详细介绍合约的四大模块
---

# Consensus contracts

以下合约管理共识机制。它们处理L1和L2网络之间的交易batch排序和验证。

**PolygonRollupBaseEtrog.sol**
这个合约是rollup和validium的基础合约。

**PolygonZkEVMEtrog.sol**
这个合约是继承自基础合约的rollup合约。

这个合约调用PolygonRollupManager.sol合约中的onSequenceBatches(...)函数。在成功排序后，通过sequenceBatches(...)调用触发验证机制。

**PolygonValidiumEtrog.sol**
这个合约是继承自基础合约的validium合约。

这个合约调用PolygonRollupManager.sol合约中的onSequenceBatches(...)函数。在成功排序后，通过sequenceBatchesValidium(...)调用触发验证机制。

# Rollup manager

PolygonRollupManager.sol 这个合约主要管理rollup。它验证batch，还通过存储哈希序列的数据来创建和更新rollup堆栈，当有新batch到达时。

它负责通过提供更新的`exit root`数据给 PolygonZkEVMGlobalExitRootV2.sol 合约来完成验证流程。

主要功能包括：

- 定义和添加包含共识实现细节和兼容性检查的rollup类型。
- 定义 RollupData 结构体。
- 初始化用于验证多个batch的可信聚合器流程。
- 通过计算所有rollup的本地`exit root`来获取`exit root`数据。
- 计算batch奖励。

# Bridge

PolygonZkEVMBridgeV2.sol 是 L1 和 L2 之间主要的通信机制。它负责管理资产和消息在不同环境之间的桥接和领取。

主要功能包括：
- 使用 `bridgeAsset(...)` 函数桥接资产。
- 使用不同场景的 `bridgeMessage(...)` 函数桥接消息。
- 使用 `claimAsset(...)` 函数领取资产。
- 使用 `claimMessage(...)` 函数领取消息。
- 验证状态并更新全局`exit root`。
- 通过 `IBasePolygonZkEVMGlobalExitRoot` 提供对全局`exit root`管理器的访问。
- 与 L2 空间中的 `PolygonZkEVMGlobalExitRootL2.sol` 合约交互，作为桥接功能的一部分。

# Exit roots

**PolygonZkEVMGlobalExitRootV2.sol**

合约负责管理L1信息树，代表系统当前状态，通过更新全局`exit root`来反映状态变化。这个任务跨多个网络和层级进行。

主要功能包括：
- 通过触发UpdateL1InfoTree(...)事件更新L1信息树。
- 更新`exit root`。
- 获取最新的`exit root`和叶子值。

**PolygonZkEVMGlobalExitRootL2.sol**

合约管理 L2 rollup信息树。

它是之前提到的全局`exit root`合约的轻量版。

主要功能：

- 在 globalExitRootMap 中存储每个全局`exit root`，键是全局`exit root`，值是时间戳。
- 在任何桥接调用时更新 lastRollupExitRoot。
- 使用 updateExitRoot(...) 函数更新 L2 网络和全局`exit root`。