

```mermaid
graph LR
  A[Cosmos SDK 模块] --> B[模块]
  A --> C[模块组件]
  A --> D[核心模块]
  A --> E[推荐的文件结构]
  A --> F[错误]
  A --> G[示例代码]
  A --> H[概述]

  B --> B1[定义]
  B1 --> B1a["应用级别关注点的功能组件"]
  B1 --> B1b["更大状态机中的状态机"]
  B --> B2[核心功能]
  B2 --> B2a["ABCI 实现"]
  B2 --> B2b["多存储"]
  B2 --> B2c["服务器和接口"]
  B --> B3[范围]
  B3 --> B3a["KV 存储"]
  B3 --> B3b["消息类型"]
  B3 --> B3c["与其他模块的交互"]

  C --> C1[接口]
  C1 --> C1a["AppModuleBasic"]
  C1 --> C1b["AppModule"]
  C1 --> C1c["AppModuleGenesis"]
  C --> C2[protobuf 服务]
  C2 --> C2a["Msg"]
  C2 --> C2b["Query"]
  C --> C3[Keeper]
  C3 --> C3a["状态控制器"]
  C3 --> C3b["更新和检查状态的方法"]
  C --> C4[命令行命令]
  C4 --> C4a["交易和查询的CLI 命令"]

  D --> D1[优势]
  D1 --> D1a["标准化"]
  D1 --> D1b["减少重复劳动"]
  D1 --> D1c["工作示例"]

  E --> E1[proto 文件]
  E1 --> E1a["{module_name}.proto"]
  E1 --> E1b["event.proto"]
  E1 --> E1c["genesis.proto"]
  E1 --> E1d["query.proto"]
  E1 --> E1e["tx.proto"]
  E --> E2[代码元素]
  E2 --> E2a["client/"]
  E2 --> E2b["exported/"]
  E2 --> E2c["keeper/"]
  E2 --> E2d["module/"]
  E2 --> E2e["simulation/"]
  E2 --> E2f["spec/"]
  E2 --> E2g["根目录文件"]

  F --> F1["ABCIInfo API"]
  F --> F2["ResponseCheckTx"]
  F --> F3["ResponseDeliverTx"]

  G --> G1[检查器模块]
  G1 --> G1a["游戏赌注"]
  G1 --> G1b["赔付"]

  H --> H1["模块作为状态机"]
  H --> H2["消息分解和路由"]
  H --> H3["核心功能"]
  H --> H4["自定义模块开发"]
  H --> H5["核心模块优势"]
  H --> H6["错误处理"]
```


TODO-把这段也整理到脑图里面去:
当一个交易从底层的CometBFT共识引擎传递过来时，它会把这个交易分解成多个部分，然后将这些部分路由到合适的模块进行处理。 

当模块消息处理器接收到这些信息后，才开始解释和执行。

module包括:

1. 一个用于应用链接口的标准实现(ABCI),与CometBFT进行通信。
2. 一个通用的数据存储，负责保存模块状态。
3. 一台服务器和接口，让你能够跟节点进行互动。

core负责连接各个模块并处理基础设施问题，而Modules则承担大部分应用逻辑，能够被组合成更高级别的Modules。

一个Modules定义了整个状态的一个子集，用来：

1. 一到多个键值存储。
2. application需要但还没有的msg类型。

模块也定义了与已经存在的其他模块进行交互。

模块实现了几个元素：

1. interface: 接口促进模块之间的通信，并将多个模块组合成连贯的应用程序。
2. Protobuf: Protobuf 提供了一个 Msg 服务来处理消息，以及一个 gRPC 查询服务来处理查询。
3. Keeper: 一个 Keeper 是一个控制器，定义状态并提供更新和检查状态的方法。
