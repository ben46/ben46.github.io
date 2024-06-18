# 通过go-ethereum源码来看staticcall到底啥意思

一般用于调用view/pure函数

[staticcall eip-214](https://eips.ethereum.org/EIPS/eip-214)

可用于调用另一个合约（或它自己），同时不允许在调用（及其子调用，如果存在）期间对状态进行任何修改。

## graph

```mermaid
graph TD
  A[检查调用深度是否超过最大限制] --> B{{是否超过}} 
  B -->|是| C[返回错误及剩余gas]
  B -->|否| D[获取数据库快照]
  D --> E[给调用者的余额+0]
  E --> F{{是否是预编译合约}}
  F -->|是| G[运行预编译合约]
  F -->|否| H[初始化新的合约]
  H --> I[设置合约代码]
  I --> J[执行合约代码]
  J --> K[获取返回值及剩余gas]
  K --> L{{是否报错}}
  L -->|是| M[恢复数据库到快照状态]
  L -->|否| N[返回返回值及剩余gas]
```
## source code
```go
// StaticCall executes the contract associated with the addr with the given input
// as parameters while disallowing any modifications to the state during the call.
// Opcodes that attempt to perform such modifications will result in exceptions
// instead of performing the modifications.
func (evm *EVM) StaticCall(caller ContractRef, addr common.Address, input []byte, gas uint64) (ret []byte, leftOverGas uint64, err error) {
	// Fail if we're trying to execute above the call depth limit
	if evm.depth > int(params.CallCreateDepth) { // 不能超过最大深度1024
		return nil, gas, ErrDepth
	}
	// We take a snapshot here. This is a bit counter-intuitive, and could probably be skipped.
	// However, even a staticcall is considered a 'touch'. On mainnet, static calls were introduced
	// after all empty accounts were deleted, so this is not required. However, if we omit this,
	// then certain tests start failing; stRevertTest/RevertPrecompiledTouchExactOOG.json.
	// We could change this, but for now it's left for legacy reasons
	var snapshot = evm.StateDB.Snapshot() // 拿到数据库快照

	// We do an AddBalance of zero here, just in order to trigger a touch.
	// This doesn't matter on Mainnet, where all empties are gone at the time of Byzantium,
	// but is the correct thing to do and matters on other networks, in tests, and potential
	// future scenarios
	evm.StateDB.AddBalance(addr, big0) // 在这里给调用者的余额+0，就是为了保持接触，在主网上无所谓，在测试网上必须要做

	// Invoke tracer hooks that signal entering/exiting a call frame
	if evm.Config.Debug {
		evm.Config.Tracer.CaptureEnter(STATICCALL, caller.Address(), addr, input, gas, nil)
		defer func(startGas uint64) {
			evm.Config.Tracer.CaptureExit(ret, startGas-gas, err)
		}(gas)
	}
  
	if p, isPrecompile := evm.precompile(addr); isPrecompile {
    //是否是预编译
		ret, gas, err = RunPrecompiledContract(p, input, gas)
	} else {
		// At this point, we use a copy of address. If we don't, the go compiler will
		// leak the 'contract' to the outer scope, and make allocation for 'contract'
		// even if the actual execution ends on RunPrecompiled above.
    // 我们复制地址，否则go编译器会把address的暴露出来（我猜测可能是内存中会暴露内存地址，给坏人可乘之机）
		addrCopy := addr
		// Initialise a new contract and set the code that is to be used by the EVM.
		// The contract is a scoped environment for this execution context only.
    // 初始化一个新的合约，并能被evm使用的设置code
		contract := NewContract(caller, AccountRef(addrCopy), new(big.Int), gas)
		contract.SetCallCode(&addrCopy, evm.StateDB.GetCodeHash(addrCopy), evm.StateDB.GetCode(addrCopy))
		// When an error was returned by the EVM or when setting the creation code
		// above we revert to the snapshot and consume any gas remaining. Additionally
		// when we're in Homestead this also counts for code storage gas errors.
	        // evm报错，或者当设置creation code在
	        // 另外当我们在， 这也会当作代码储存燃料费错误
		ret, err = evm.interpreter.Run(contract, input, true)
		gas = contract.Gas
	}
	if err != nil {
	        //如果报错， 将数据库恢复到快照的样子
		evm.StateDB.RevertToSnapshot(snapshot)
		if err != ErrExecutionReverted {
			gas = 0
		}
	}
	return ret, gas, err
}
```
