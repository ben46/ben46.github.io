# LoadManager

```mermaid
graph LR
    subgraph LoadManager
        lm_logger["Logger"]
        lm_effort["HashMap<String, ShardEffort>"]
        lm_blocked_queries["HashSet<u64>"]
        lm_jailed_queries["RwLock<HashSet<QueryRef>>"]
        lm_kill_state["HashMap<String, RwLock<KillState>>"]
        lm_effort_gauge["Box<GaugeVec>"]
        lm_query_counters["HashMap<CacheStatus, Counter>"]
        lm_kill_rate_gauge["Box<GaugeVec>"]
    end

    subgraph Shard
        s_effort["ShardEffort"]
        s_effort_inner["ShardEffortInner"]
        s_kill_state["KillState"]
    end

    LoadManager --> Shard

    lm_effort --> s_effort
    lm_kill_state --> s_kill_state

    s_effort --> s_effort_inner
    s_effort_inner --> s_effort

    style LoadManager fill:#f9f,stroke:#333,stroke-width:4px
    style Shard fill:#ccf,stroke:#333,stroke-width:2px

```

# Storebuilder

## Architechture Graph

```mermaid
graph LR
    subgraph StoreBuilder
        sb_config["Config"]
        sb_node_id["NodeId"]
        sb_fork_base["ForkBase"]
        sb_metrics["MetricsRegistry"]
        sb_logger["Logger"]
    end

    subgraph NetworkStore
        ns_block_store["BlockStore"]
        ns_subgraph_store["SubgraphStore"]
    end

    subgraph Subscription
        sm["SubscriptionManager"]
        chul["ChainHeadUpdateListener"]
    end

    subgraph Database
        pp["PrimaryPool"]
    end

    subgraph Blockchain
        bm["BlockchainMap"]
    end

    StoreBuilder --> NetworkStore
    StoreBuilder --> Subscription
    StoreBuilder --> Database
    StoreBuilder --> Blockchain

    NetworkStore --> ns_block_store
    NetworkStore --> ns_subgraph_store

    Subscription --> sm
    Subscription --> chul

    Database --> pp

    Blockchain --> bm

    style StoreBuilder fill:#f9f,stroke:#333,stroke-width:4px
    style NetworkStore fill:#ccf,stroke:#333,stroke-width:2px
    style Subscription fill:#cfc,stroke:#333,stroke-width:2px
    style Database fill:#fcc,stroke:#333,stroke-width:2px
    style Blockchain fill:#ccf,stroke:#333,stroke-width:2px

```

## Sequence Diagram

```mermaid
sequenceDiagram
    participant Main
    participant StoreBuilder
    participant NetworkStore
    participant BlockStore
    participant SubscriptionManager
    participant ChainHeadUpdateListener
    participant PrimaryPool

    Main->>StoreBuilder: new(logger, node_id, config, fork_base, metrics_registry)
    StoreBuilder->>NetworkStore: new(config.chain_ids())
    StoreBuilder->>BlockStore: network_store.block_store()
    StoreBuilder->>SubscriptionManager: subscription_manager()
    StoreBuilder->>ChainHeadUpdateListener: chain_head_update_listener()
    StoreBuilder->>PrimaryPool: primary_pool()

    Main->>StoreBuilder: blockchain_map(env_vars, node_id, logger, block_store, logger_factory, metrics_registry, chain_head_update_listener)
    StoreBuilder->>NetworkStore: block_store.cleanup_ethereum_shallow_blocks(eth_network_names)

    Main->>StoreBuilder: graphql_runner(logger, network_store, subscription_manager, load_manager, graphql_metrics_registry)
    Main->>StoreBuilder: subgraph_instance_manager(logger_factory, env_vars, network_store.subgraph_store(), blockchain_map, sg_count, metrics_registry, link_resolver, ipfs_service, arweave_service, static_filters)
    Main->>StoreBuilder: subgraph_provider(logger_factory, link_resolver, subgraph_instance_manager, sg_count)
    Main->>StoreBuilder: subgraph_registrar(logger_factory, link_resolver, subgraph_provider, network_store.subgraph_store(), subscription_manager, blockchain_map, node_id, version_switching_mode, subgraph_settings)

    Main->>JsonRpcServer: serve(json_rpc_port, http_port, ws_port, subgraph_registrar, node_id, logger)
    Main->>GraphQLQueryServer: start(http_port, ws_port)
    Main->>GraphQLSubscriptionServer: serve(ws_port)
    Main->>IndexNodeServer: start(index_node_port)
    Main->>PrometheusMetricsServer: start(metrics_port)

```

# main

```mermaid
flowchart TD
    start["Start"]
    env_vars["Load environment variables"]
    opt["Parse command-line options"]
    logger["Initialize logger"]
    version["Log version information"]
    poi_protection["Check POI protection"]
    config["Load configuration"]
    subgraph_settings["Load subgraph settings"]
    check_config["Check configuration"]
    node_id["Create node ID"]
    subgraph_["Obtain subgraph-related arguments"]
    ports["Obtain server ports"]
    fork_base["Obtain fork base URL"]
    elastic_config["Obtain Elasticsearch logging configuration"]
    prometheus["Set up Prometheus registry"]
    logger_factory["Create logger factory"]
    ipfs_clients["Create IPFS clients"]
    ipfs_service["Create IPFS service"]
    arweave_resolver["Create Arweave resolver"]
    arweave_service["Create Arweave service"]
    link_resolver["Create link resolver"]
    metrics_server["Create metrics server"]
    endpoint_metrics["Create endpoint metrics"]
    graphql_metrics_registry["Create GraphQL metrics registry"]
    contention_logger["Create contention logger"]
    expensive_queries["Read expensive queries"]
    store_builder["Create store builder"]
    launch_services["Launch services"]
    end_["END"]

    start --> env_vars
    env_vars --> opt
    opt --> logger
    logger --> version
    version --> poi_protection
    poi_protection --> config
    config --> subgraph_settings
    subgraph_settings --> check_config
    check_config --> node_id
    node_id --> subgraph_
    subgraph_ --> ports
    ports --> fork_base
    fork_base --> elastic_config
    elastic_config --> prometheus
    prometheus --> logger_factory
    logger_factory --> ipfs_clients
    ipfs_clients --> ipfs_service
    ipfs_service --> arweave_resolver
    arweave_resolver --> arweave_service
    arweave_service --> link_resolver
    link_resolver --> metrics_server
    metrics_server --> endpoint_metrics
    endpoint_metrics --> graphql_metrics_registry
    graphql_metrics_registry --> contention_logger
    contention_logger --> expensive_queries
    expensive_queries --> store_builder
    store_builder --> launch_services
    launch_services --> end_
```

# launch services

```mermaid
flowchart TD
    start["Start"]
    subscription_manager["Get subscription manager"]
    chain_head_update_listener["Get chain head update listener"]
    primary_pool["Get primary pool"]
    network_store["Get network store"]
    block_store["Get block store"]
    validator["Create validator"]
    network_adapters["Create network adapters"]
    blockchain_map["Create blockchain map"]
    cleanup_ethereum_shallow_blocks["Clean up Ethereum shallow blocks"]
    blockchain_map_arc["Create Arc<blockchain_map>"]
    shards["Get shards"]
    load_manager["Create load manager"]
    graphql_runner["Create GraphQL runner"]
    graphql_server["Create GraphQL server"]
    subscription_server["Create subscription server"]
    index_node_server["Create index node server"]
    block_ingestors["Start block ingestors"]
    job_runner["Create job runner"]
    register_store_jobs["Register store jobs"]
    static_filters["Get static filters"]
    subgraph_count["Create subgraph count metric"]
    subgraph_instance_manager["Create subgraph instance manager"]
    subgraph_provider["Create subgraph provider"]
    version_switching_mode["Get version switching mode"]
    subgraph_registrar["Create subgraph registrar"]
    spawn_subgraph_registrar["Spawn subgraph registrar"]
    json_rpc_server["Start JSON-RPC server"]
    add_cli_subgraph["Add CLI subgraph"]
    spawn_graphql_server["Spawn GraphQL server"]
    spawn_subscription_server["Spawn subscription server"]
    spawn_index_node_server["Spawn index node server"]
    spawn_metrics_server["Spawn metrics server"]

    start --> subscription_manager
    subscription_manager --> chain_head_update_listener
    chain_head_update_listener --> primary_pool
    primary_pool --> network_store
    network_store --> block_store
    block_store --> validator
    validator --> network_adapters
    network_adapters --> blockchain_map
    blockchain_map --> cleanup_ethereum_shallow_blocks
    cleanup_ethereum_shallow_blocks --> blockchain_map_arc
    blockchain_map_arc --> shards
    shards --> load_manager
    load_manager --> graphql_runner
    graphql_runner --> graphql_server
    graphql_runner --> subscription_server
    subscription_server --> index_node_server
    blockchain_map_arc --> block_ingestors
    block_ingestors --> job_runner
    job_runner --> register_store_jobs
    register_store_jobs --> static_filters
    static_filters --> subgraph_count
    subgraph_count --> subgraph_instance_manager
    subgraph_instance_manager --> subgraph_provider
    subgraph_provider --> version_switching_mode
    version_switching_mode --> subgraph_registrar
    subgraph_registrar --> spawn_subgraph_registrar
    spawn_subgraph_registrar --> json_rpc_server
    json_rpc_server --> add_cli_subgraph
    add_cli_subgraph --> spawn_graphql_server
    spawn_graphql_server --> spawn_subscription_server
    spawn_subscription_server --> spawn_index_node_server
    spawn_index_node_server --> spawn_metrics_server
```

# do_poll

```mermaid
flowchart TB
    A["Start"] --> B["do_poll(eth_adapter)"]
    B --> C["get head block hash and number from store"]
    C --> D["Fetch latest_block_header from RPC"]
    D --> node_1
    node_1["eth_getBlockByNumber(latest)"]
    node_1 --"one block with tx hash(no receipt)"--> E{{"latest block #lt;= head_block"}}

    E --"YES"--> K["return OK"]
    E --"NO"-->P

    P["Ingest latest block"]
    P --> Q["Fetch missing parent blocks"]
    Q --> R{{"Is missing block hash present?"}}
    R -- Yes --> S["Ingest missing block"]
    S --> Q

```

# ingest_block.eth_adapter

```mermaid

flowchart TB

  node_7["ingest_block(block_hash)"]
  node_1["eth_adapter.load_full_block(block)"]
  node_4["fetch_receipts_with_retry(hashes, block_hash)"]
  node_5{{"supports_block_receipts"}}
  node_8["fetch_block_receipts_with_retry"]
  node_3["transaction_receipt"]
  node_10["eth_getTransactionReceipt"]
  node_6["eth_getBlockReceipts(blockHash)"]
  node_9["eth_adapter.block_by_hash"]
  node_14["block_with_txs(block_hash)"]
  node_13["eth_getBlockByHash(hash)"]
  node_11["return Block#lt;Transcation#gt; without logs"]
  node_2["fetch_individual_receipts_with_retry(hashes,block_hash)"]
  node_12["fetch_transaction_receipt_with_retry(tx_hash,block_hash)"]
  node_15["return ethereum block"]
  node_16["mark as non-final"]
  node_17["chain_store.upsert_block"]
  node_18["chain_store.attempt_chain_head_update"]

  node_7 --> node_9

  subgraph block_by_hash
    node_9 --> node_14
    node_14 --> node_13
    node_13 --> node_11
  end

  subgraph load_full_block
    node_1 --"block.transactions"--> node_4
    node_4 --> node_5
    node_5 --"true(no iterate)"--> node_8
    node_5 --"false(need iterate)"--> node_2
    node_8 --> node_6
    node_2 --> node_12

    subgraph loop_txs
      node_12 --> node_3
      node_3 --> node_10

    end
  end
  node_10 --> node_15
  node_11 --> node_1
  node_6 --> node_15
  node_15 --> node_16
  node_16 --> node_17
  node_17 --> node_18

```

# block pooling stream
```mermaid
sequenceDiagram
    participant BlockStream as PollingBlockStream
    participant ChainStore as 区块链存储
    participant Adapter as 适配器
    participant Subgraph as 子图
    
    BlockStream->>ChainStore: 查询最新区块
    ChainStore-->>BlockStream: 返回最新区块
    BlockStream->>Adapter: 处理区块
    Adapter-->>BlockStream: 返回处理结果
    
    alt 子图指针落后于链头指针
        BlockStream->>Subgraph: 检查子图状态
        Subgraph-->>BlockStream: 子图指针状态
    
        alt 子图指针不在主链上
            BlockStream->>Subgraph: 获取父区块指针
            Subgraph-->>BlockStream: 返回父区块指针
            BlockStream->>Subgraph: 回退到父区块
            Subgraph-->>BlockStream: 更新子图状态
        else 子图指针在主链上
            BlockStream->>BlockStream: 继续处理
        end
    else 子图指针在主链上
        BlockStream->>BlockStream: 继续处理
    end

```

```mermaid
flowchart TD
    A[查询最新区块] --> B{{子图指针落后于链头指针?}}
    
    B --|是|--> C[检查子图状态]
    C --> D{{子图指针在主链上?}}
    
    D --|否|--> E[获取父区块指针]
    E --> F[回退到父区块]
    F --> G[更新子图状态]
    
    D --|是|--> H[继续处理]
    
    B --|否|--> H[继续处理]
```
# PollingBlockStream
Reconciliation: 

在数据流或数据库的上下文中，reconciliation 意味着确保不同数据源之间的信息一致。例如，合并来自不同来源的数据，以消除差异和不一致。

yield:

在这里，yield 表示从一个过程（区块流的对齐）中逐步产生输出（即区块），让系统处理这些区块。这个过程通常在异步编程中使用，以允许在计算中有序地发出结果，而不是一次性返回所有结果。

```mermaid
stateDiagram-v2
    [*] --> BeginReconciliation

    BeginReconciliation --> Reconciliation : start/restart
    Reconciliation --> YieldingBlocks : poll ready ok, NextBlocks Blocks
    YieldingBlocks --> YieldingBlocks : 消化每一个区块
    Reconciliation --> Idle : poll ready ok,NextBlocks Done, completed
    Reconciliation --> BeginReconciliation : poll ready ok,next blocks revert
    Reconciliation --> RetryAfterDelay : Poll Ready error
    YieldingBlocks --> BeginReconciliation : need reconciliation
    RetryAfterDelay --> BeginReconciliation : Poll Ready
    RetryAfterDelay --> RetryAfterDelay : poll pending
    Idle --> BeginReconciliation : chain head update
```


