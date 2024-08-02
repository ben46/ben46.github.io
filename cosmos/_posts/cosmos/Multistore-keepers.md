# Multistore and Keepers

```mermaid
graph LR
    A[文章结构目录]  
    node_1["keeper"]
    node_2["storeKeys"]
    node_3["cdc"]
    A --> B[Keepers]  
    B --> B1[定义]  
    B --> B2[参数]  
    A --> C[Store types]  
    C --> C1[KVStore and Multistore in the interchain] 

    C --> C2[CacheMultistore]  
    C --> C3[Transient store]  
    C --> C4[Additional KVStore wrappers]  
    C4 --> C4a[GasKv store]  
    C4 --> C4b[TraceKv store]  
    C4 --> C4c[Prefix store]  
    B2 --> node_1
    B2 --> node_2
    B2 --> node_3
```

```mermaid
graph LR  
    B2[Parameters]  
        node_1["keeper"]
    node_2["storeKeys"]
    node_3["cdc"]
   node_4["cdc 是用于将结构体进行编解码的工具，能够将其转换为 []byte"]
    node_5["storeKeys 授予访问由模块管理的 multistore 中所有存储对象的权限。\n这些权限应始终保持对外部模块不可见。"]
    node_6["预期的 keeper 是指模块外部的 keeper，\n它是该模块内部 keeper 所依赖的。\n外部 keeper 的相关信息会以接口的形式列在内部 keeper 的类型定义中。\n这些接口在模块文件夹的根目录下的 expected_keepers.go 文件中进行定义。\n引入接口的目的是为了减少依赖关系，方便模块的维护。"]
     B2 --> node_1
    B2 --> node_2
    B2 --> node_3
    node_3 --> node_4
    node_2 --> node_5
    node_1 --> node_6
```

```mermaid
graph TD
    C1[KVStore and Multistore in the interchain] 
    nodeeeL[rootMulti.Store: 实现 CommitMultiStore 接口]  

    C1 --> nodee1
    C1 --> nodeeF
    C1 --> nodeeeL
    C1 --> nodeeJ
        nodee1[每个 Cosmos SDK 应用] --> nodee2[状态: Multistore]  
            nodeeF[基本 KVStore 和 Multistore 实现] --> nodeeG[扩展提供专门行为]  

    nodeeG --> nodeeH[CommitMultistore: 带有 committer 的 Multistore]  
    nodeeH --> nodeeI[用于 Cosmos SDK 的主要 Multistore]  
    nodeeJ[底层 KVStore] --> nodeeK[限制对 committer 的访问]  
        nodeeeL --> nodeeeM[基于数据库的基础层 multistore]  
    nodee2 --> nodee3[分为多个模块管理的隔离区]  
    nodee2 --> nodee4[包含 KVStores 的存储]  
    nodee4 --> nodee5[遵循 Multistore 接口]  


    nodeeeM --> nodeeeN[上面可以挂载多个 KVStores]  
    nodeeeM --> nodeeeO[BaseApp 中使用的默认 multistore]

```
# CommitMultiStore
`CommitMultiStore` 是 Cosmos SDK 中的一种 `Multistore` 实现，它带有一个提交器（committer）。这是 Cosmos SDK 中使用的主要多存储类型。`CommitMultiStore` 允许对多个 `KVStore` 进行管理，并在每个区块结束时提交状态变化。它的设计旨在确保在处理交易时能够有效地管理和维护应用程序的状态。

```mermaid
graph LR  
    A[CommitMultiStore] --> B[Multistore]  
    A --> C[Committer]  
    B --> D[KVStore]  
    B --> E[Substores]  
    C --> F[Transaction Management]  
    C --> G[State Management]
```
# CacheMultistore & TransientStore
```mermaid
graph TD
    C2[CacheMultistore]  
 

    node_7["分支存储"]
    node_8["虚拟存储创建"]
    node_9["状态隔离"]
    node_10["查询缓存"]
    node_11["临时数据存储"]
    node_12["内存数据库实现"]
    node_13["自动内存管理"]
    node_14["简化开发流程"]
    node_15["提升性能"]
  
    

    C3[Transient store]  
   
    C2 --> node_7
    C2 --> node_8
    C2 --> node_9
    C2 --> node_10
    C3 --> node_11
    C3 --> node_12
    C3 --> node_13
    C3 --> node_14
    C3 --> node_15
  
 
 
    
    
```

分支存储（Branch Store）是一种存储设计模式，主要用于管理和维护数据状态的不同版本或者变体。其核心思想是在不直接修改主存储的情况下，创建一个或多个独立的存储分支，以便对数据进行实验性或临时的变更。这在以下几种场景中非常有用：

# GasKV
```mermaid
graph LR  
  node_16["自动gas消耗"]
    node_17["每次对存储读取或者写入的操作\n自动消耗相应的gas"]
    node_18["存储使用跟踪"]
    node_19["该存储解决方案使得开发者能够准确追踪存储资源的使用，在操作时会根据Store.gasConfig自动调整消耗的gas数量，确保对资源使用的合理控制。"]
    node_20["默认包装机制"]
    node_21["所有KVStores在调用时都会默认被包装在GasKv.Store中，这个过程是在上下文的KVStore()方法中完成的，简化了开发工作，并提供了统一的gas管理接口。"]
    node_22["灵活的配置"]
    node_23["通过gasConfig，开发者可以根据实际需求调整默认的gas配置，灵活适应不同场景下的资源消耗和管理，提升了系统的可定制性。"]
    C4a[GasKv store]  
      C4a --> node_16
    node_16 --> node_17
    C4a --> node_18
    node_18 --> node_19
    C4a --> node_20
    node_20 --> node_21
    C4a --> node_22
    node_22 --> node_23
```

# traceKV

```mermaid
graph LR  
    node_24["操作跟踪：tracekv.Store是一个键值存储（KVStore）的包装器，提供操作跟踪功能，能够记录对底层KVStore的所有操作，以便后续分析和调试。"]
    node_25["自动应用：当在父MultiStore上启用了跟踪功能时，Cosmos SDK会自动对所有KVStores应用tracekv.Store，这使得开发者无需额外配置即可获得操作跟踪特性。\n\n"]
    node_26["日志记录：每当调用KVStore的方法时，tracekv.Store会自动记录traceOperation到Store.writer中，这为开发者提供了详细的操作日志，有助于后续分析和监控。\n\n"]
    node_27["元数据填充：在记录traceOperation时，traceOperation.Metadata会在Store.context不为nil时被填充，允许开发者附加上下文信息，例如请求编号或用户标识，以便更好地理解操作背后的背景。\n\n"]
    node_28["上下文灵活性：TraceContext 是一个map[string]interface{}，这意味着开发者可以灵活地存储各种类型的元数据，根据需要定制跟踪信息，提升了系统的可调试性和可分析性。\n\n"]


    C4b["traceKV"]
       C4b --> node_24
    C4b --> node_25
    C4b --> node_26
    C4b --> node_27
    C4b --> node_28

```
# prefix store

* **键前缀化**：prefix.Store是一个KVStore的包装器，提供自动的键前缀功能，允许开发者为存储中的键添加统一的前缀，以便更好地组织和管理数据。

* **自动转发操作**：当调用Store.{Get, Set}()方法时，prefix.Store会自动将请求转发到其父KVStore，同时在键的前面添加Store.prefix，这样可以简化对存储的操作并保持键的一致性。

* **避免键冲突**：通过使用前缀，prefix.Store能够有效避免不同模块或组件之间的键冲突，从而帮助开发者在同一存储中管理多个子存储。

* **迭代器的特殊处理**：在调用Store.Iterator()时，prefix.Store不简单地在父存储的基础上添加前缀，因为这种方式可能导致遍历时包含一些不以该前缀开头的元素，从而导致意外结果。因此，prefix.Store在实现迭代器时采取了更复杂的处理策略，以确保仅遍历相应的前缀元素。

* **灵活的数据组织**：prefix.Store提供了一种灵活的数据组织方式，使得应用程序能够高效地管理和检索关联数据，特别适用于需要在同一存储中处理多种数据类型或来源的场景。



# AnteHandler

```mermaid
graph LR  

D[AnteHandler]  
    node_34["AnteHandler定义：AnteHandler是一个特殊的处理器，实现了AnteHandler接口，用于在处理交易的内部消息之前对交易进行身份验证。\n\n"]
    node_35["可选性与重要性：AnteHandler在理论上是可选的，但对于公共区块链网络仍然是非常重要的组成部分。\n\n"]
    node_36["防垃圾邮件：它是防止垃圾邮件的第一道防线，也是防止交易重放的第二道防线（在mempool之后），通过费用扣除和序列检查来实现。\n\n"]
    node_37["初步有效性检查：AnteHandler执行初步的状态有效性检查，例如确保签名有效，或验证发送者是否有足够资金支付费用。\n\n"]
    node_38["激励角色：它通过收集交易费用在激励利益相关者方面发挥作用。\n\n"]
    node_39["BaseApp集成：BaseApp将AnteHandler作为一个参数，在应用程序的构造函数中进行初始化，最广泛使用的AnteHandler是auth模块。\n\n"]

D --> node_34
    D --> node_35
    D --> node_36
    D --> node_37
    D --> node_38
    D --> node_39
```

# 总结


```mermaid
graph LR  
    F["总结"]
node_40["每个keeper负责管理对区块链状态中特定模块状态的访问，<br>这一管理方式是Cosmos SDK基于对象能力的方法的核心，旨在防止应用程序遭受不必要的跨模块互动。"]
    node_41["每个keeper拥有一个storeKey，这个密钥赋予其对模块数据的无限访问权限，<br>并规定了如何读取和写入任何存储。因此，当一个模块要与另一个模块进行交互时，它必须遵循目标模块keeper的方法。"]
    node_42["所有Cosmos SDK应用都包含一个Multistore根状态，这个状态被划分为由各个模块管理的多个区块，<br>负责存储应用中所有模块的KVStores。"]
    node_43["建议在处理内部消息之前，使用AnteHandler组件来验证交易的身份。<br>该组件能够防止垃圾交易和其他不必要的交易事件，进行初步的状态有效性检查，并负责收取交易费用。"]
    F --> node_40
    F --> node_41
    F --> node_42
    F --> node_43
```

# 伪代码

我们现在有一个checker游戏, 现在我们要设计怎么存放游戏数据, 使用cosmos-sdk提供的KV存储

一个初步的想法是: 为每局游戏分配一个ID，并根据这个 ID 来存储游戏的相关信息。为了使结构更清晰，并在未来能够与其他存储项目区分开来，您可以为每个 ID 添加一个前缀。以下是存储结构的示例：

```go
var globalStore sdk.KVStore
// checkersStore := globalStore.getAtPrefix("checkers-")
// gamesStore := checkersStore.getAtPrefix("games-")
// storedGame := gamesStore.getAtPrefix(gameId)

// 每个 ID 添加一个前缀
storedGame := globalStore.getAtPrefix("checkers-games-" + gameId)
```

定义 StoredGame 的 ID。要返回单个对象，请在对象的值中包含 Index：
```
type StoredGame struct {
    ...
    Index string
}
```

定义所需的前缀：
```
package types

const (
    StoredGamesKey = collections.NewPrefix("StoredGame/value/")
)
```


```
package keeper

type Keeper struct {
    ...
    StoredGames collections.Map[string, checkers.StoredGame]
}

func NewKeeper(cdc codec.BinaryCodec, addressCodec address.Codec, storeService storetypes.KVStoreService, authority string) Keeper {
    k := Keeper{
        ...
        StoredGames: collections.NewMap(sb,
            checkers.StoredGamesKey, "storedGames", collections.StringKey,
            codec.CollValue[checkers.StoredGame](cdc)),
    }
    ...
}
// 通过id访问game对象
func (k Keeper) GetStoredGame(ctx sdk.Context, gameId string) (storedGame checkers.StoredGame, err error) {
    return k.StoredGames.Get(ctx, gameId)
}
```

如果您想保存游戏：
```
func (k Keeper) SetStoredGame(ctx sdk.Context, gameId string, storedGame checkers.StoredGame) error {
    return k.StoredGames.Set(ctx, gameId, storedGame) 
}
```

同样，如果您想删除已存储的游戏，请调用：
```
k.StoredGames.Remove(ctx, storedGame.Index)
```

KVStore 还允许您获取给定前缀上的迭代器。您可以列出所有存储的游戏，因为它们共享相同的前缀，您可以使用以下操作：

```
func (k Keeper) GetAllStoredGame(ctx context.Context) ([]checkers.StoredGame, error) {
    var storedGames []checkers.StoredGame
    if err := k.StoredGames.Walk(ctx, nil, func(index string, storedGame checkers.StoredGame) (bool, error) {
        storedGames = append(storedGames, storedGame)
        return false, nil
    }); err != nil {
        return nil, err
    }

    return storedGames, nil
}
```