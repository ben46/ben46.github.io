---
title: 8-共识合约 - 验证
---

# 共识合约 - 验证

系统里的交易，在两种合约调用场景中的一种后，进入智能合约环境：

1. 序列器组件在节点中发出的序列批次请求。
2. 聚合器组件在节点中发出的验证批次请求。
这部分重点讲验证流程。

下面的序列图显示了rollup堆栈和/或AggLayer调用rollup管理器上的verifyBatchesTrustedAggregator(...)函数的验证流程。
![f](https://docs.polygon.technology/img/cdk/high-level-architecture/verification-flow.png)
Polygon Solidity共识验证流程

### verifyBatchesTrustedAggregator(rollupID, pendingStateNum, initNumBatch, finalNewBatch, newLocalExitRoot, newStateRoot, beneficiary, proof)

这个函数在PolygonRollupManager合约上调用。

zkEVM节点聚合器，或者AggLayer，在PolygonRollupManager.sol合约上调用verifyBatchesTrustedAggregator函数。

这个函数创建一个rollup数据存储对象，用调用者提供的数据，首先通过发送到辅助函数_verifyAndRewardBatches来验证。

这个内部函数计算inputSnark字节值，是一个（SHA256 % "RFIELD"）计算在输入数据上，然后用这个值通过调用rollup.verifier.verifyProof(proof, [inputSnark])在IVerifierRollup接口实现上验证证明。

接下来，在_verifyAndRewardBatches辅助函数中，如果证明没有成功验证，代码会回滚。否则，代码向受益人支付POL奖励。

然后，函数更新状态，并调用rollup.rollupContract.onVerifyBatches(...)，这是一个回调到rollup共识合约，允许自定义行为。onVerifyBatches(...)接受新的状态根和最终批次，并发出一个VerifyBatches(...)事件。

命令流程返回到verifyBatchesTrustedAggregator函数，它整合并更新状态为newStateRoot和newLocalExitRoot，并在GlobalExitRootManager合约上调用updateExitRoot(...)函数，传递rollup的更新后的退出根。