英文原文: https://medium.com/@novablitz/storing-structs-is-costing-you-gas-774da988895e

假设做一个游戏

```solidity
//消耗120kgas

mapping(uint256 => address) owners;
mapping(uint256 => uint64) creationTime;
mapping(uint256 => uint256) dna;
mapping(uint256 => uint16) strength;
mapping(uint256 => uint16) race;
mapping(uint256 => uint16) class;
```
#### 写成结构体
```solidity
// 消耗75k gas

struct GameCharacter {
  address owner;
  uint64 creationTime;
  uint256 dna;
  uint16 strength;
  uint16 race;
  uint16 class;
}
mapping(uint256 => GameCharacter) characters;
```

#### 修改变量顺序, 修改变量类型大小

首先，我们将dna移动到最后。这是一个uint256，它已经是32字节，所以它不能打包任何东西。然后，我们将creationTime从uint64更改为uint48。这将使前五个字段全部打包为32字节。时间戳不需要超过uint48，Solidity（与大多数其他语言不同）允许uint是8的任意倍数，而不仅仅是8/16/32/64等。
```solidity
// 60k gas
struct GameCharacter {
  address owner; // 160个bit
  uint48 creationTime; //uint64 creationTime;
  uint16 strength;
  uint16 race;
  uint16 class; // 160+48+16+16+16 = 256
  uint256 dna;
}
mapping(uint256 => GameCharacter) characters;
```

#### 压缩成一个uint256
```solidity
//40k gas
mapping(uint256 => uint256) characters;
mapping(uint256 => uint256) dnaRecords;
function setCharacter(uint256 _id, address owner, uint256 creationTime, uint256 strength, uint256 race, uint256 class, uint256 dna) 
    external 
{
    uint256 character = uint256(owner);
    character |= creationTime<<160;
    character |= strength<<208;
    character |= race<<224;
    character |= class<<240;
    characters[_id] = character;
    dnaRecords[_id] = dna;
}
function getCharacter(uint256 _id) 
    external view
returns(address owner, uint256 creationTime, uint256 strength, uint256 race, uint256 class, uint256 dna) {
    uint256 character = characters[_id];
    dna = dnaRecords[_id];
    owner = address(character);
    creationTime = uint256(uint40(character>>160));
    strength = uint256(uint16(character>>208));
    race = uint256(uint16(character>>224));
    class = uint256(uint16(character>>240));
}
```
# 修改传参,节省gas
```solidity
struct GameCharacter {
  address owner;
  uint256 creationTime;
  uint256 strength;
  uint256 race;
  uint256 class;
  uint256 dna;
}
function getCharacterStruct(uint256 _id) 
    external view
returns(GameCharacter memory _character) {
    uint256 character = characters[_id];
    _character.dna = dnaRecords[_id];
    _character.owner = address(character);
    _character.creationTime = uint256(uint40(character>>160));
    _character.strength = uint256(uint16(character>>208));
    _character.race = uint256(uint16(character>>224));
    _character.class = uint256(uint16(character>>240));
}
```
所有字段都是uint256。这是最有效的数据类型。内存中的变量（甚至是结构）根本就没有打包，因此在内存中使用uint16不会获得任何好处，并且会因为solidity必须执行额外的操作才能将uint16转换为uint256进行计算而损失。



