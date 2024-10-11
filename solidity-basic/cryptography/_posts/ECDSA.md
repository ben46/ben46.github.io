ECDSA（椭圆曲线数字签名算法）有多种实现算法，主要基于不同的椭圆曲线和密码学库。以下是一些常见的实现算法：

1. **secp256k1**：`ecrecover`. 这是比特币和以太坊等区块链系统中广泛使用的椭圆曲线，提供高效的签名和验证过程。
https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/cryptography/ECDSA.sol

性能好,区块链常用

2. **secp256r1**（也称为 P-256）：这是由 NIST（美国国家标准与技术研究院）推荐的椭圆曲线，广泛用于各种安全协议，如 TLS 和数字签名。
https://github.com/Vectorized/solady/blob/main/src/utils/P256.sol

不常用