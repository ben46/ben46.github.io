1. Rust 中的借用(Borrowing)是如何实现的？
2. 解引用运算符在 Rust 中的作用是什么？
3. 可变引用在 Rust 中有什么限制？
4. 什么是悬垂引用(Dangling References)？Rust 如何避免悬垂引用？
5. Rust 中的借用规则是什么？

----

## 1. Rust 中的借用(Borrowing)是如何实现的？

在 Rust 中的借用（Borrowing）类似 Objective-C 中的弱引用（Weak Reference）

通过引用，可以访问变量的值但不拥有该值。借用可以分为不可变引用和可变引用两种类型，分别用 `&` 和 `&mut` 来表示。不可变引用允许读取值但不允许修改，而可变引用则允许修改值。通过借用的方式，可以在不转移所有权的情况下对变量进行操作，从而避免程序变得复杂。

### number

```rust
let x = 5; // 创建一个整数值 x

// 借用 x 的引用给 y
let y = &x;

// 使用断言检查 x 和 *y 的值是否相等
assert_eq!(5, x);
assert_eq!(5, *y);
```

### String

```rust
fn main() {
    let s1 = String::from("hello"); // 创建一个 String 类型的变量 s1
    let len = calculate_length(&s1); // 将 s1 的引用作为参数传递给 calculate_length 函数
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len() // 返回字符串 s 的长度
}
```



## 2. 解引用运算符在 Rust 中的作用是什么？

解引用运算符 * 可以将一个引用转换为指向的值。

## 3. 可变引用在 Rust 中有什么限制？

同一作用域内，特定数据只能有**一个可变引用**

同一作用域内，特定数据只能有**多个不可变引用**

可变引用与不可变引用不能同时存在

即多个指针同时访问同一数据，其中至少有一个指针用于写入数据，并且没有同步数据访问的机制。

## 4. 什么是悬垂引用(Dangling References)？Rust 如何避免悬垂引用？

指针指向某一个内存地址, 这个指针指向的内存地址里面的value已经被释放. 

编译器已经保证了这个.

