# tendermint & cometBFT

tendermint=共识算法

1. cosmos sdk: 状态机
2. CometBFT: 网络层和共识层

## CometBFT 软件

1. 共识引擎 pos
2. 网络层

封装了网络层和共识层, 并且给应用层提供了接口-Application Blockchain Interface (ABCI)

## cosmos sdk
app layer开发
### CheckTx
检查交易
### DeliverTx
处理交易
### BeginBlock and EndBlock

继承baseApp就行了