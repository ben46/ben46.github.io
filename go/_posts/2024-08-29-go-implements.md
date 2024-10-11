# "接口断言"或"编译时类型检查"

```go
package main

import (
    "fmt"
)

// 定义一个接口
type Writer interface {
    Write([]byte) (int, error)
}

// 定义一个结构体
type FileWriter struct{}

// 这是我们要确保 FileWriter 实现了 Writer 接口的地方
var _ Writer = (*FileWriter)(nil)

// 实现 Write 方法
func (fw FileWriter) Write(data []byte) (int, error) {
    fmt.Println("Writing data:", string(data))
    return len(data), nil
}

// 另一个结构体，故意不实现 Writer 接口
type DummyWriter struct{}

// 取消注释下面的行会导致编译错误，因为 DummyWriter 没有实现 Writer 接口
// var _ Writer = (*DummyWriter)(nil)

func main() {
    fw := FileWriter{}
    fw.Write([]byte("Hello, World!"))
}
```

