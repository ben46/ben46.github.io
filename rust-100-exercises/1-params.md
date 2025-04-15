### 返回值
返回字符串切片`&static str`

```rust
// `fn` <function_name> ( <input params> ) -> <return_type> { <body> }
fn greeting() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to __!"
}
```

### 入参

字符串入参`&str`

```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```
