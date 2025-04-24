# Tauri 开发指南：Rust 基础篇

## 1. Rust 基础语法

### 1.1 变量与类型

```rust
// 变量声明
let x = 5;              // 不可变变量
let mut y = 10;         // 可变变量
const PI: f64 = 3.14;   // 常量

// 基本类型
let integer: i32 = 42;      // 32位整数
let float: f64 = 3.14;      // 64位浮点数
let boolean: bool = true;   // 布尔值
let character: char = 'A';  // 字符
let string: &str = "Hello"; // 字符串切片
let string_obj: String = String::from("Hello"); // 字符串对象
```

### 1.2 控制流

```rust
// if-else
if x > 5 {
    println!("x 大于 5");
} else if x == 5 {
    println!("x 等于 5");
} else {
    println!("x 小于 5");
}

// match 表达式
match x {
    1 => println!("一"),
    2 => println!("二"),
    3..=5 => println!("三到五"),
    _ => println!("其他"),
}

// 循环
for i in 1..5 {
    println!("{}", i);
}

while x > 0 {
    x -= 1;
}

loop {
    if x == 0 {
        break;
    }
    x -= 1;
}
```

### 1.3 函数

```rust
// 基本函数
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 闭包
let add = |a: i32, b: i32| -> i32 { a + b };

// 泛型函数
fn print<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}
```

## 2. 所有权与借用

### 2.1 所有权规则

```rust
// 所有权转移
let s1 = String::from("hello");
let s2 = s1;  // s1 的所有权转移到 s2
// println!("{}", s1); // 错误！s1 不再有效

// 克隆
let s1 = String::from("hello");
let s2 = s1.clone();  // 深拷贝
println!("{}", s1);   // 正确，s1 仍然有效
```

### 2.2 借用

```rust
// 不可变借用
fn print_string(s: &String) {
    println!("{}", s);
}

let s = String::from("hello");
print_string(&s);  // 借用 s
println!("{}", s); // s 仍然有效

// 可变借用
fn modify_string(s: &mut String) {
    s.push_str(", world!");
}

let mut s = String::from("hello");
modify_string(&mut s);
println!("{}", s); // 输出: hello, world!
```

## 3. 结构体与枚举

### 3.1 结构体

```rust
// 定义结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 创建实例
let user = User {
    username: String::from("john"),
    email: String::from("john@example.com"),
    sign_in_count: 1,
    active: true,
};

// 方法实现
impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 0,
            active: true,
        }
    }

    fn is_active(&self) -> bool {
        self.active
    }
}
```

### 3.2 枚举

```rust
// 定义枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 使用枚举
let msg = Message::Write(String::from("hello"));

// 模式匹配
match msg {
    Message::Quit => println!("退出"),
    Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
    Message::Write(text) => println!("消息: {}", text),
    Message::ChangeColor(r, g, b) => println!("颜色: ({}, {}, {})", r, g, b),
}
```

## 4. 错误处理

### 4.1 Result 类型

```rust
// 定义可能失败的操作
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

// 处理 Result
match divide(10, 2) {
    Ok(result) => println!("结果: {}", result),
    Err(error) => println!("错误: {}", error),
}

// 使用 ? 运算符
fn process() -> Result<(), String> {
    let result = divide(10, 0)?;  // 如果出错，直接返回错误
    println!("结果: {}", result);
    Ok(())
}
```

### 4.2 Option 类型

```rust
// 定义可能为空的值
fn find_user(id: i32) -> Option<User> {
    if id == 1 {
        Some(User::new(String::from("john"), String::from("john@example.com")))
    } else {
        None
    }
}

// 处理 Option
match find_user(1) {
    Some(user) => println!("找到用户: {}", user.username),
    None => println!("未找到用户"),
}

// 使用 unwrap 和 expect
let user = find_user(1).unwrap();  // 如果为 None 则 panic
let user = find_user(1).expect("用户不存在");  // 自定义错误信息
```

## 5. 并发编程

### 5.1 线程

```rust
use std::thread;
use std::time::Duration;

// 创建线程
let handle = thread::spawn(|| {
    for i in 1..5 {
        println!("线程: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
});

// 等待线程完成
handle.join().unwrap();
```

### 5.2 通道

```rust
use std::sync::mpsc;
use std::thread;

// 创建通道
let (tx, rx) = mpsc::channel();

// 发送线程
thread::spawn(move || {
    tx.send("hello").unwrap();
});

// 接收消息
let received = rx.recv().unwrap();
println!("收到: {}", received);
```

### 5.3 互斥锁

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// 共享数据
let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("结果: {}", *counter.lock().unwrap());
```

## 6. 异步编程

### 6.1 async/await

```rust
use tokio::time::{sleep, Duration};

// 异步函数
async fn do_something() -> String {
    sleep(Duration::from_secs(1)).await;
    String::from("完成")
}

// 使用异步函数
#[tokio::main]
async fn main() {
    let result = do_something().await;
    println!("{}", result);
}
```

### 6.2 异步任务

```rust
use tokio::task;

// 创建异步任务
let handle = task::spawn(async {
    do_something().await
});

// 等待任务完成
let result = handle.await.unwrap();
println!("{}", result);
```

## 7. 常用标准库

### 7.1 文件操作

```rust
use std::fs;
use std::path::Path;

// 读取文件
let content = fs::read_to_string("file.txt")?;

// 写入文件
fs::write("output.txt", "Hello, world!")?;

// 检查文件是否存在
if Path::new("file.txt").exists() {
    println!("文件存在");
}
```

### 7.2 集合类型

```rust
// 向量
let mut vec = Vec::new();
vec.push(1);
vec.push(2);

// HashMap
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key", "value");

// HashSet
use std::collections::HashSet;
let mut set = HashSet::new();
set.insert(1);
set.insert(2);
```

### 7.3 时间处理

```rust
use std::time::{SystemTime, UNIX_EPOCH};

// 获取当前时间戳
let now = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();
```

## 8. 实用技巧

### 8.1 宏

```rust
// 定义宏
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

// 使用宏
say_hello!();
say_hello!("World");
```

### 8.2 特征（Trait）

```rust
// 定义特征
trait Greet {
    fn greet(&self) -> String;
}

// 为类型实现特征
impl Greet for User {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.username)
    }
}

// 使用特征
let user = User::new(String::from("John"), String::from("john@example.com"));
println!("{}", user.greet());
```

### 8.3 生命周期

```rust
// 生命周期注解
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体中的生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

## 9. 调试技巧

### 9.1 打印调试

```rust
// 使用 println! 宏
println!("变量 x = {}", x);

// 使用 dbg! 宏
let x = 5;
dbg!(x);  // 打印文件名、行号和值
```

### 9.2 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    #[should_panic]
    fn test_divide_by_zero() {
        divide(10, 0).unwrap();
    }
}
```

### 9.3 性能分析

```rust
// 使用 std::time 测量执行时间
use std::time::Instant;

let start = Instant::now();
// 执行代码
let duration = start.elapsed();
println!("耗时: {:?}", duration);
```

## 10. 常见错误与解决方案

### 10.1 编译错误

1. **所有权错误**
```rust
// 错误示例
let s = String::from("hello");
let s2 = s;
println!("{}", s);  // 错误！

// 解决方案
let s = String::from("hello");
let s2 = s.clone();  // 使用 clone
println!("{}", s);   // 正确
```

2. **生命周期错误**
```rust
// 错误示例
fn longest(x: &str, y: &str) -> &str {  // 错误！

// 解决方案
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  // 正确
```

### 10.2 运行时错误

1. **空指针错误**
```rust
// 错误示例
let x: Option<i32> = None;
println!("{}", x.unwrap());  // panic!

// 解决方案
match x {
    Some(value) => println!("{}", value),
    None => println!("值为空"),
}
```

2. **线程错误**
```rust
// 错误示例
let mut data = vec![1, 2, 3];
let handle = thread::spawn(|| {
    data.push(4);  // 错误！
});

// 解决方案
let data = Arc::new(Mutex::new(vec![1, 2, 3]));
let data_clone = Arc::clone(&data);
let handle = thread::spawn(move || {
    let mut data = data_clone.lock().unwrap();
    data.push(4);
});
```

## 11. 下一步学习

1. 学习 Tauri 框架的基本概念
2. 了解前端与 Rust 后端的通信机制
3. 掌握 Tauri 的 API 使用
4. 实践构建完整的桌面应用

这些基础知识将帮助你更好地理解和使用 Tauri 框架。在下一篇教程中，我们将深入探讨 Tauri 的核心概念和基本用法。 