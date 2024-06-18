# 内联汇编小汇总
## compound
`CErc20.sol`保证转账成功函数
```solidity
function doTransferIn(address from, uint amount)virtual override internal returns(uint){
  //省略...
  token.transferFrom(from, address(this), amount);
  bool success;
  assembly{
    switch returndatasize() // Pushes the size of the return data buffer onto the stack将返回值缓冲区的大小压入堆栈
      case 0 {
        success := not(0)//非运算
      }
      case 32 {
        returndatacopy(0, 0, 32)//Copies data from the return data buffer to memory将数据从返回值缓冲区复制到内存
        success := mload(0)// 从内存读取
      }
      default {
        revert(0, 0)
      }      
  }
  require(success, "TOKEN_TRANSFER_IN_FAILED");
  //省略...
}
```
`returndatacopy`函数源码
```go
func opReturnDataCopy(pc *uint64, interpreter *EVMInterpreter, scope *ScopeContext) ([]byte, error) {
	var (
		memOffset  = scope.Stack.pop() // 出栈, 内存下标
		dataOffset = scope.Stack.pop() // 出栈, 数据
		length     = scope.Stack.pop() // 出栈, 数据长度
	)

	offset64, overflow := dataOffset.Uint64WithOverflow()
	if overflow { // 检查数据是否溢出2^64
		return nil, ErrReturnDataOutOfBounds
	}
	// we can reuse dataOffset now (aliasing it for clarity)
	var end = dataOffset // 复制一份
	end.Add(&dataOffset, &length) // 计算end位置
	end64, overflow := end.Uint64WithOverflow()
	if overflow || uint64(len(interpreter.returnData)) < end64 {
    // 检查溢出
		return nil, ErrReturnDataOutOfBounds
	}
  // 从interpreter的bytes数组中取切片
  // 把切片放入内存中
	scope.Memory.Set(memOffset.Uint64(), length.Uint64(), interpreter.returnData[offset64:end64])
	return nil, nil
}
```

## uniswapv2
`UniswapV2Factory.sol`
```solidity
function createPair(address tokenA, address tokenB) external returns(address pair){
	//省略...
	bytes memory bytecode = type(UniswapV2Pair).creationCode;
	bytes32 salt = keccak256(abi.encodePacked(token0, token1))
	assembly{
		// add就是相加
		// mload, 从内存读取
		// create2 根据salt生成固定地址
		pair := create2(0, // 创建合约时往合约中打的 ETH 数量
			add(bytecode, 32), //memory_start（代码在内存中的起始位置，一般固定为 add(bytecode, 0x20) ）
			mload(bytecode), //memory_length（代码长度，一般固定为 mload(bytecode) ）
			salt)
	}
	//省略...
}
```

evm opcode example
```
byte(vm.PUSH1), 0x00, // salt + 32
byte(vm.PUSH1), byte(len(initCode)), // size 
byte(vm.PUSH1), byte(32 - len(initCode)), // offset
byte(vm.PUSH1), 0x00, // endowment
byte(vm.CREATE2),
```

`create2`源码
```go
func opCreate2(pc *uint64, interpreter *EVMInterpreter, scope *ScopeContext) ([]byte, error) {
	if interpreter.readOnly {
		return nil, ErrWriteProtection
	}
	var (
		endowment    = scope.Stack.pop()//endowment（创建合约时往合约中打的 ETH 数量）
		offset, size = scope.Stack.pop(), scope.Stack.pop()
		salt         = scope.Stack.pop()
		// 根据下标和长度, 从内存中取出input,也就是代码
		input        = scope.Memory.GetCopy(int64(offset.Uint64()), int64(size.Uint64()))
		gas          = scope.Contract.Gas
	)
	// Apply EIP150
	gas -= gas / 64
	scope.Contract.UseGas(gas)
	// reuse size int for stackvalue
	stackvalue := size
	//TODO: use uint256.Int instead of converting with toBig()
	bigEndowment := big0
	if !endowment.IsZero() {
		bigEndowment = endowment.ToBig()
	}
	res, addr, returnGas, suberr := interpreter.evm.Create2(scope.Contract, input, gas,
		bigEndowment, &salt)
	// Push item on the stack based on the returned error.
	if suberr != nil {
		stackvalue.Clear()
	} else {
		stackvalue.SetBytes(addr.Bytes())
	}
	scope.Stack.push(&stackvalue)
	scope.Contract.Gas += returnGas

	if suberr == ErrExecutionReverted {
		interpreter.returnData = res // set REVERT data to return data buffer
		return res, nil
	}
	interpreter.returnData = nil // clear dirty return data buffer
	return nil, nil
}

func (evm *EVM) Create2(caller ContractRef, code []byte, gas uint64, endowment *big.Int, salt *uint256.Int) (ret []byte, contractAddr common.Address, leftOverGas uint64, err error) {
	codeAndHash := &codeAndHash{code: code}
	contractAddr = crypto.CreateAddress2(caller.Address(), salt.Bytes32(), codeAndHash.Hash().Bytes())
	return evm.create(caller, codeAndHash, gas, endowment, contractAddr, CREATE2)
}

func CreateAddress2(b common.Address, salt [32]byte, inithash []byte) common.Address {
	// 计算地址
	return common.BytesToAddress(Keccak256([]byte{0xff}, b.Bytes(), salt[:], inithash)[12:])
}
```




