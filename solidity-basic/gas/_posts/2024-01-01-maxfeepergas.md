# eip1559里面, 如果 maxFeePerGas 高于 basefee，那么用户实际支付的费用将由 basefee 决定. 这种说法对吗?

EIP-1559 是一个以太坊的更新，它在2021年8月的伦敦升级中被部署。它旨在改善以太坊的交易费用市场，引入了一个可灵活变化的区块大小机制，并且烧毁了交易费用的一部分以减少总供应。

在 EIP-1559 中，每个交易都有两个主要的费用参数：

1. **Base Fee（基础费）**: 这是每个区块中所有交易所必须支付的最低费用，它是算法确定的，并随网络拥堵程度动态调整。如果区块满载，则基础费会增加；如果区块未满，则基础费会下降。

2. **Max Fee Per Gas（最大费用/燃料）**: 这是用户愿意为每单位 gas 支付的最大费用。它包括了基础费和一个小费（tip）给矿工。

实际交易费用的确定方式如下：

- **Base Fee（基础费）** 是必须支付的，且该费用会被烧毁，不会支付给矿工。
- **Priority Fee（优先费，也被称为小费或矿工小费）**: 用户可以设置一个小费给矿工，以增加他们交易被优先处理的可能性。小费的大小由用户自行决定。
- **Max Fee Per Gas（最大费用/燃料）**: 这是用户愿意为每单位 gas 支付的最大总费用，包括基础费和优先费。

当用户进行交易时：

- 如果**Max Fee Per Gas 高于 Base Fee**加上设置的小费，实际支付的费用将等于**Base Fee 加上实际给矿工的小费**。剩余的（即Max Fee Per Gas - Base Fee - Priority Fee）将退还给用户。
- 如果**Max Fee Per Gas 低于 Base Fee**，那么交易可能不会被包括在区块中，因为没有足够的费用支付给网络。

所以，说用户实际支付的费用由 Base Fee 决定是不完全准确的。实际支付的费用是基础费加上任何支付给矿工的优先费。用户支付的总费用不会超过他们设置的 Max Fee Per Gas，但有可能低于这个数值，取决于他们设置的优先费和当前的基础费。

总结: maxFeePerGas的目的是, 如果过了几个区块basefee 特别高, 那么用户的交易讲不会被包括在区块中, 避免了用户支付过高的 gas.
