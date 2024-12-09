最常用的方式是这样的：

### 项目结构

```
my_workspace/
├── Cargo.toml          # 工作区的配置文件
├── core_lib/           # 核心库（lib crate）
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs      # 核心库的入口
│       ├── module1.rs  # 模块1
│       └── module2.rs  # 模块2
├── app/                # 应用程序（binary crate）
│   ├── Cargo.toml
│   └── src/
│       └── main.rs     # 应用程序入口
└── utils_lib/          # 工具库（lib crate）
    ├── Cargo.toml
    └── src/
        └── lib.rs      # 工具库的入口
```

### 组织方式

1. **工作区（workspace）**:
   - 顶层 `Cargo.toml` 定义工作区，管理多个 crate。
   - 每个子目录（如 `core_lib`、`app`、`utils_lib`）是一个独立的 package。

2. **核心库（core_lib）**:
   - 提供项目的核心逻辑，组织为多个模块（`module1.rs`、`module2.rs`）。
   - 其他 crate（如 `app`）通过依赖 `core_lib` 使用其功能。

3. **工具库（utils_lib）**:
   - 提供通用的工具函数或辅助功能，供核心库和应用程序复用。

4. **应用程序（app）**:
   - 是一个二进制 crate，依赖 `core_lib` 和 `utils_lib`，实现具体的业务逻辑。

### 示例代码

#### 顶层 `Cargo.toml`（工作区配置）
```toml
[workspace]
members = [
    "core_lib",
    "app",
    "utils_lib"
]
```

#### `core_lib/src/lib.rs`（核心库）
```rust
pub mod module1;
pub mod module2;

pub fn core_function() {
    println!("Core library function");
}
```

#### `core_lib/src/module1.rs`（模块1）
```rust
pub fn module1_function() {
    println!("Module 1 function");
}
```

#### `utils_lib/src/lib.rs`（工具库）
```rust
pub fn util_function() {
    println!("Utility function");
}
```

#### `app/src/main.rs`（应用程序）
```rust
use core_lib::core_function;
use core_lib::module1::module1_function;
use utils_lib::util_function;

fn main() {
    core_function();
    module1_function();
    util_function();
}
```

---

### 总结

- **工作区** 管理多个 package。
- **核心库（lib crate）** 提供主要逻辑，组织为模块。
- **工具库（lib crate）** 提供通用功能。
- **应用程序（binary crate）** 依赖核心库和工具库，完成具体功能。

这种方式是开发大型项目时最常用的组织方式，清晰、模块化、易扩展。
