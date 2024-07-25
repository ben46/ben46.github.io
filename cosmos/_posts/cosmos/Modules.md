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

```mermaid
graph LR
  A[Cosmos SDK Modules] --> B[Modules]  
  A --> C[Module Components]  
  A --> D[Core Modules]  
  A --> E[Recommended Folder Structure]  
  A --> F[Errors]  
  A --> G[Code Example]  
  A --> H[Summary]  

  B --> B1[Definition]  
  B1 --> B1a["Functional components for application-level concerns"]  
  B1 --> B1b["State machines within the larger state machine"]  
  B --> B2[Core Functionality]  
  B2 --> B2a["ABCI implementation"]  
  B2 --> B2b["Multistore"]  
  B2 --> B2c["Server and interfaces"]  
  B --> B3[Scope]  
  B3 --> B3a["KVStore"]  
  B3 --> B3b["Message types"]  
  B3 --> B3c["Interactions with other modules"]  

  C --> C1[Interfaces]  
  C1 --> C1a["AppModuleBasic"]  
  C1 --> C1b["AppModule"]  
  C1 --> C1c["AppModuleGenesis"]  
  C --> C2[Protobuf Services]  
  C2 --> C2a["Msg"]  
  C2 --> C2b["Query"]  
  C --> C3[Keeper]  
  C3 --> C3a["Controller for state"]  
  C3 --> C3b["Methods for updating and inspecting state"]  
  C --> C4[Command-line Commands]  
  C4 --> C4a["CLI commands for transactions and queries"]  

  D --> D1[Advantages]  
  D1 --> D1a["Standardization"]  
  D1 --> D1b["Reduced duplication of effort"]  
  D1 --> D1c["Working examples"]  

  E --> E1[Proto Files]  
  E1 --> E1a["{module_name}.proto"]  
  E1 --> E1b["event.proto"]  
  E1 --> E1c["genesis.proto"]  
  E1 --> E1d["query.proto"]  
  E1 --> E1e["tx.proto"]  
  E --> E2[Code Elements]  
  E2 --> E2a["client/"]  
  E2 --> E2b["exported/"]  
  E2 --> E2c["keeper/"]  
  E2 --> E2d["module/"]  
  E2 --> E2e["simulation/"]  
  E2 --> E2f["spec/"]  
  E2 --> E2g["Root directory files"]  

  F --> F1["ABCIInfo API"]  
  F --> F2["ResponseCheckTx"]  
  F --> F3["ResponseDeliverTx"]  

  G --> G1[Checkers Module]  
  G1 --> G1a["Game wager"]  
  G1 --> G1b["Wager payment"]  

  H --> H1["Modules as state machines"]  
  H --> H2["Message decomposition and routing"]  
  H --> H3["Core functionalities"]  
  H --> H4["Custom module development"]  
  H --> H5["Core modules benefits"]  
  H --> H6["Error handling"]
```