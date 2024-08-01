

```mermaid
graph LR
  A[Cosmos SDK 模块] --> B[模块]
  A --> C[模块组件]
  A --> D[核心模块]
  A --> E[推荐的文件结构]
  A --> F[错误]
  A --> H[概述]

  B --> B1[定义]
  B1 --> B1a["应用级别关注点的功能组件"]
  B1 --> B1b["更大状态机中的状态机"]

  C --> C1[接口]
  C1 --> C1a["AppModuleBasic"]
  C1 --> C1b["AppModule"]
  C1 --> C1c["AppModuleGenesis"]
  C --> C2[protobuf 服务]
  C2 --> C2a["Msg service"]
  C2 --> C2b["gRPC Query service"]
  C --> C3["Keeper.go"]
  C3 --> C3a["视为Controller状态控制器"]
  C3 --> C3b["更新和检查状态的方法"]
  C --> C4["命令行命令\nclient/cli"]

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


  H --> H1["模块作为状态机"]
  H --> H2["消息分解和路由"]
  H --> H3["核心功能"]
  H --> H4["自定义模块开发"]
  H --> H5["核心模块优势"]
  H --> H6["错误处理"]
  B1b --> node_2
  B1a --> node_1
  B1a --> node_3
  B --> node_8
  node_8 --> node_9
  node_9 --> node_10
  node_9 --> node_11
  node_9 --> node_12
  node_8 --> node_13
  node_9 --> node_14
  node_8 --> node_15
  node_8 --> node_16
  node_16 --> node_17
  node_16 --> node_18
  C1a --> node_19
  C1b --> node_20
  C1c --> node_21
  C2a --> node_22
  C2b --> node_23
  C2a --> node_24
  C2a --> node_25
  C2a --> node_26
  C2b --> node_27
  C2b --> node_28
  C2b --> node_29
  C2b --> node_30
  C4 --> node_31
  C4 --> node_32
  node_31 --> node_33
  node_32 --> node_34
  C3 --> node_35
  node_35 --> node_36
  E1a --> node_37
  E1b --> node_38
  E1c --> node_39
  E1d --> node_40
  E1e --> node_41
  E2a --> node_42
  E2b --> node_43
  E2c --> node_44
  E2d --> node_45
  E2e --> node_46
  E2f --> node_47
  E2g --> node_48
  node_48 --> node_49
  E2g --> node_50
  node_50 --> node_51
  E2g --> node_52
  node_52 --> node_53
  E2g --> node_54
  node_54 --> node_55
  E2g --> node_56
  node_56 --> node_57
  E2g --> node_58
  node_58 --> node_59
  E2g --> node_60
  node_60 --> node_61
  E2g --> node_62
  node_62 --> node_63
  E2g --> node_64
  node_64 --> node_65
  E2g --> node_66
  node_66 --> node_67
  E2b --> node_68
  node_2["它们包含存储布局或状态，\n以及状态转换函数（即消息方法）。"]
  node_1["定义了 Cosmos SDK 应用程序的大部分逻辑"]
  node_3["以便应用程序开发人员可以专注于其应用程序真正独特的方面。"]
  node_8["模块范围"]
  node_9["包括底层core功能"]
  node_10["与cometBFT使用ABCI进行通信"]
  node_11["通用的数据存储,multistore"]
  node_12["服务器和接口, 方便节点之间的通信"]
  node_13["实现大部分应用逻辑"]
  node_14["负责处理布线和基础设施问题，并允许模块组合成更高阶的模块"]
  node_15["定义与其他存在的模块的交互"]
  node_16["定义了整体状态的subset"]
  node_17["KVStore"]
  node_18["应用程序所需但尚不存在的消息类型的子集。\n"]
  node_19["实现模块的非依赖元素。"]
  node_20["模块中相互依赖的、专门的元素，是应用程序所独有的"]
  node_21["相互依赖的、模块的起源/初始化元素，\n在开始时建立区块链的初始状态。"]
  node_22["一组与 Protobuf 请求类型一一相关的 RPC 方法，用于处理消息。"]
  node_23[" gRPC查询服务，用于处理查询。"]
  node_24["Msg最佳实践是在文件中定义Protobuf 服务tx.proto。"]
  node_25["每个模块都应将该RegisterServices方法作为接口的一部分来实现AppModule。\n这样应用程序就可以知道模块可以处理哪些消息和查询。"]
  node_26["服务方法应该使用一个keeper，它封装了有关存储布局的知识并提供更新状态的方法。\n"]
  node_27["Query最佳实践是在文件中定义Protobuf 服务query.proto。\n"]
  node_28["它允许用户使用 gRPC 查询状态。\n"]
  node_29["每个 gRPC 端点对应一个服务方法，以rpcgRPCQuery服务内部的前缀命名。\n"]
  node_30["可以在grpc.enable和grpc.address字段下进行配置app.toml。\n"]
  node_31["tx.go"]
  node_32["query.go"]
  node_33["交易CLI命令"]
  node_34["查询CLI"]
  node_35["权限控制"]
  node_36["为了防止模块在运行时随机访问另一个模块，\n模块需要在构造时声明其使用另一个模块的意图。"]
  node_37["模块的常用消息类型定义。"]
  node_38["模块与事件相关的消息类型定义。"]
  node_39["与创世状态相关的模块的消息类型定义。\n"]
  node_40["模块的Query服务和相关消息类型定义。"]
  node_41["模块的Msg服务和相关消息类型定义。"]
  node_42["模块的 CLI 客户端功能实现和模块的集成测试套件。"]
  node_43["模块的导出类型 - 通常是接口类型（另请参阅以下注释）。"]
  node_44["模块Keeper和MsgServer实现。"]
  node_45["模块AppModule和AppModuleBasic实现。"]
  node_46["该模块的模拟包定义了区块链模拟器应用程序所使用的函数（simapp）。"]
  node_47["模块的规范文档，概述了重要概念、状态存储结构以及消息和事件类型定义。"]
  node_48["abci.go"]
  node_49["模块BeginBlocker和实现。仅当和/或需要定义时才EndBlocker需要此文件。BeginBlockerEndBlocker"]
  node_50["codec.go"]
  node_51["模块针对接口类型的注册方法。"]
  node_52["errors.go"]
  node_53["模块的标记错误。"]
  node_54["events.go"]
  node_55["模块的事件类型和构造函数。"]
  node_56["expected_keepers.go"]
  node_57["模块预期的其他守护接口。"]
  node_58["genesis.go"]
  node_59["模块的起源状态方法和辅助函数。"]
  node_60["keys.go"]
  node_61["模块的存储键和相关辅助函数。"]
  node_62["msgs.go"]
  node_63["模块的消息类型定义和相关方法。"]
  node_64["params.go"]
  node_65["模块的参数类型定义和相关方法。"]
  node_66["*.pb.go"]
  node_67["模块的类型定义由协议缓冲区生成，如相应的*.proto文件中所定义。"]
  node_68["如果模块依赖于另一个模块的 Keeper，则exported/代码元素希望将 Keeper 作为接口契约接收，以避免直接依赖实现 Keeper 的模块。但是，这些接口契约可以定义对实现 Keeper 的模块进行操作的方法（或返回特定于该模块的类型）。\n\n定义的接口类型exported/使用规范类型，允许模块通过文件接收接口契约expected_keepers.go。此模式允许代码保持DRY （打开新窗口）并缓解了进口周期混乱。"]
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
