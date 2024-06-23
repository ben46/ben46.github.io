---
title: Consensus contracts - sequencing
---

# Consensus contracts - sequencing

交易在系统中流动，通过两种合约调用用例之一进入智能合约环境：

1. 从节点的排序器组件来的批处理请求。
2. 从节点的聚合器组件来的批处理请求。

这一部分重点讲解排序工作流程。

下图展示了适用于汇总（非验证）堆栈的排序工作流程，调用了 sequenceBatches(...) 和 onSequenceBatches(...)。

![sdfsdf](https://docs.polygon.technology/img/cdk/high-level-architecture/sequencing-flow.png)

### sequenceBatches(batches, maxSequenceTs, initSequenceBatch, l2Coinbase)¶

这个函数是在 PolygonZkEVMEtrog.sol 合约上调用的。

Rollup 排序器组件调用了 PolygonZkEVMEtrog.sol 合约上的 sequenceBatches 函数，这个函数继承自 PolygonRollupBaseEtrog.sol 合约。

这个函数接收来自一个共识合约的 BatchData 结构体数组。每个结构体包含 L2 以太坊交易数据和一些强制状态信息。

```solidity
struct BatchData {
    bytes transactions;
    bytes32 forcedGlobalExitRoot;
    uint64 forcedTimestamp;
    bytes32 forcedBlockHashL1;
}
```

函数验证参数，检查并整理批次数据，并按照正确的顺序附加，同时计算一个累积哈希。

最后，函数触发一个 SequenceBatches 事件，在 onSequenceBatches(...) 函数成功返回后，发送一个新排序的交易批次到 PolygonRollupManager.sol 合约。

函数的步骤如下：

1. 验证参数。
2. 通过调用 globalExitRootManager.updateExitRoot(L1LocalExitRoot) 告诉桥更新全局退出根，这会使用最新的 L1 本地退出根创建一个新的全局退出根。
3. 获取 L1 信息根和计算所需的其他变量。
4. 通过 keccak(batch.transaction) 和 keccak(accInputHash, txHash, l1InfoRoot, maxSequenceTs, l2Coinbase, bytes32(0)) 计算累积哈希，遍历批次。
5. 存储累积哈希。
6. 调用者用 POL 支付 Rollup 管理器。
7. 调用 PolygonRollupManager.onSequenceBatches(...) 函数，等待 OnSequenceBatches(...) 事件回调。
8. 触发 SequenceBatches(...) 事件。

```solidity
onSequenceBatches(newSequencedBatches, newAccInputHash)
```

这个函数是在 PolygonRollupManager.sol 合约上调用的。

它接收来自调用合约的已排序批次和累积哈希，将批次添加到正确的堆栈，并更新批次数。

函数的步骤如下：

1. 验证参数和调用合约。
2. 更新 totalSequencedBatches 存储变量。
3. 更新 lastBatchSequenced，并为调用 sequenceBatches 的 Rollup 添加一个新的 SequencedBatchData 结构体。
4. 通过更新 lastVerifiedBatch、batchNumToStateRoot[] 和 lastLocalExitRoot 状态

### sequenceBatchesValidium(batches, l2Coinbase, dataAvailabilityMessage)¶

这个功能是在 PolygonValidiumEtrog.sol 合约中调用的。

其顺序逻辑几乎与 rollup 的 sequenceBatches(...) 函数相同，只是这个函数接受 ValidiumBatchData[] 数组而不是 BatchData[]。这意味着结构体传递的是交易的哈希而不是实际的交易数据。

ValidiumBatchData 结构体包括 transactionsHash、forcedGlobalExitRoot、forcedTimestamp、forcedBlockHashL1。

它还有一个 dataAvailabilityMessage 参数，而不是序列信息。该参数包含了委员会所有成员按升序排列的 ECDSA 地址签名，用于验证。

该函数的逐步执行逻辑与 PolygonRollupBaseEtrog.sequenceBatches(...) 函数几乎相同，除了以下步骤：

- 使用 ValidiumBatchData 替代 BatchData。
- 将 txHash 累积到 accumulatedNonForcedTransactionHash。
- 添加了一个有效性检查，使用 dataAvailabilityProtocol.verifyMessage(accumulatedNonForcedTransactionHash, dataAvailabilityMessage)。