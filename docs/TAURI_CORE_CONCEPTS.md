# Tauri 开发指南：核心概念篇

## 1. Tauri 架构概述

### 1.1 核心组件

Tauri 应用由三个主要部分组成：
- **前端层**：使用 Web 技术（HTML, CSS, JavaScript）
- **后端层**：使用 Rust 编写的核心
- **通信层**：连接前端和后端的桥梁

### 1.2 项目结构

```
├── src/                    # 前端源代码
│   ├── components/        # Vue/React 组件
│   ├── assets/           # 静态资源
│   └── main.ts          # 入口文件
├── src-tauri/            # Tauri 后端
│   ├── src/             # Rust 源代码
│   ├── Cargo.toml       # Rust 依赖配置
│   └── tauri.conf.json  # Tauri 配置文件
└── package.json         # 前端依赖配置
```

## 2. 环境配置

### 2.1 安装依赖

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Tauri CLI
cargo install tauri-cli

# 创建新项目
cargo create-tauri-app
```

### 2.2 配置文件

```json
// tauri.conf.json
{
  "build": {
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build",
    "devPath": "http://localhost:1420",
    "distDir": "../dist"
  },
  "package": {
    "productName": "My App",
    "version": "0.1.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "all": false,
        "open": true
      },
      "fs": {
        "all": false,
        "readFile": true,
        "writeFile": true,
        "readDir": true
      }
    },
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.tauri.dev",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    },
    "security": {
      "csp": null
    },
    "windows": [
      {
        "fullscreen": false,
        "resizable": true,
        "title": "My App",
        "width": 800,
        "height": 600
      }
    ]
  }
}
```

## 3. 前后端通信

### 3.1 命令定义

```rust
// src-tauri/src/main.rs
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3.2 前端调用

```typescript
// 使用 @tauri-apps/api
import { invoke } from '@tauri-apps/api/tauri';

// 调用命令
const response = await invoke('greet', { name: 'World' });
console.log(response); // 输出: Hello, World!
```

### 3.3 事件系统

```rust
// 后端发送事件
#[tauri::command]
fn emit_event(window: tauri::Window) {
    window.emit("event-name", "event data").unwrap();
}

// 前端监听事件
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen('event-name', (event) => {
  console.log('收到事件:', event.payload);
});
```

## 4. 窗口管理

### 4.1 创建窗口

```rust
// 创建新窗口
tauri::Builder::default()
    .setup(|app| {
        let window = tauri::WindowBuilder::new(
            app,
            "external", /* 窗口标识符 */
            tauri::WindowUrl::App("index.html".into())
        )
        .title("新窗口")
        .inner_size(800.0, 600.0)
        .build()?;
        Ok(())
    })
```

### 4.2 窗口控制

```typescript
import { Window } from '@tauri-apps/api/window';

// 获取窗口实例
const window = Window.getCurrent();

// 窗口操作
await window.minimize();
await window.maximize();
await window.unmaximize();
await window.close();
```

### 4.3 窗口样式

```rust
// 设置窗口样式
tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
    .title("My App")
    .inner_size(800.0, 600.0)
    .min_inner_size(400.0, 200.0)
    .max_inner_size(1920.0, 1080.0)
    .resizable(true)
    .fullscreen(false)
    .decorations(true)
    .transparent(false)
    .always_on_top(false)
    .visible(true)
    .build()?;
```

## 5. 文件系统操作

### 5.1 文件读写

```rust
// 后端实现
#[tauri::command]
async fn read_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn write_file(path: String, contents: String) -> Result<(), String> {
    std::fs::write(path, contents)
        .map_err(|e| e.to_string())
}
```

### 5.2 文件选择器

```typescript
import { open, save } from '@tauri-apps/api/dialog';

// 打开文件选择器
const filePath = await open({
  multiple: false,
  filters: [{
    name: 'Text',
    extensions: ['txt']
  }]
});

// 保存文件选择器
const filePath = await save({
  filters: [{
    name: 'Text',
    extensions: ['txt']
  }]
});
```

### 5.3 文件监控

```rust
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

#[tauri::command]
fn watch_file(path: String, window: tauri::Window) {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
    watcher.watch(path, RecursiveMode::NonRecursive).unwrap();

    std::thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(event) => {
                    window.emit("file-changed", event).unwrap();
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
}
```

## 6. 系统托盘

### 6.1 创建托盘

```rust
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_item(hide);

    let system_tray = SystemTray::new()
        .with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 6.2 托盘图标

```rust
// 设置托盘图标
SystemTray::new()
    .with_icon(tauri::Icon::Raw(include_bytes!("../icons/icon.png").to_vec()))
    .with_menu(tray_menu)
```

## 7. 插件系统

### 7.1 使用官方插件

```rust
// 添加插件到 Cargo.toml
[dependencies]
tauri-plugin-store = "0.5"

// 在 main.rs 中注册插件
tauri::Builder::default()
    .plugin(tauri_plugin_store::Builder::default().build())
```

### 7.2 创建自定义插件

```rust
// 定义插件
use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;

#[tauri::command]
fn custom_command() -> String {
    "Hello from plugin!".into()
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("custom")
        .invoke_handler(tauri::generate_handler![custom_command])
        .build()
}

// 使用插件
tauri::Builder::default()
    .plugin(init())
```

## 8. 安全配置

### 8.1 CSP 配置

```json
{
  "tauri": {
    "security": {
      "csp": "default-src 'self'; img-src 'self' data: https:; style-src 'self' 'unsafe-inline'"
    }
  }
}
```

### 8.2 权限控制

```json
{
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {
        "all": false,
        "readFile": true,
        "writeFile": true
      },
      "shell": {
        "all": false,
        "open": true
      }
    }
  }
}
```

## 9. 调试与测试

### 9.1 开发工具

```bash
# 开发模式
cargo tauri dev

# 构建
cargo tauri build

# 检查更新
cargo tauri update
```

### 9.2 日志系统

```rust
// 配置日志
use log::{info, error, warn};

fn main() {
    env_logger::init();
    info!("应用启动");
    error!("发生错误");
    warn!("警告信息");
}
```

### 9.3 性能分析

```rust
// 使用性能分析工具
#[cfg(profile = "release")]
fn benchmark() {
    let start = std::time::Instant::now();
    // 执行代码
    let duration = start.elapsed();
    println!("耗时: {:?}", duration);
}
```

## 10. 打包与分发

### 10.1 应用打包

```bash
# 构建应用
cargo tauri build

# 构建特定平台
cargo tauri build --target x86_64-pc-windows-msvc
cargo tauri build --target aarch64-apple-darwin
cargo tauri build --target x86_64-unknown-linux-gnu
```

### 10.2 更新配置

```json
{
  "tauri": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://myapp.com/update.json"
      ],
      "dialog": true,
      "pubkey": "YOUR_PUBLIC_KEY"
    }
  }
}
```

## 11. 最佳实践

### 11.1 错误处理

```rust
// 统一的错误处理
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Custom(String),
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::Io(error)
    }
}

#[tauri::command]
async fn handle_file(path: String) -> Result<String, AppError> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}
```

### 11.2 状态管理

```rust
// 全局状态
struct AppState {
    counter: std::sync::Mutex<i32>,
}

#[tauri::command]
fn increment_counter(state: tauri::State<AppState>) -> i32 {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    *counter
}
```

### 11.3 性能优化

1. 使用异步操作处理耗时任务
2. 实现缓存机制
3. 优化资源加载
4. 使用适当的并发策略

## 12. 常见问题与解决方案

### 12.1 跨平台兼容性

1. 路径处理
```rust
use std::path::PathBuf;

fn get_app_path() -> PathBuf {
    tauri::api::path::app_dir().unwrap()
}
```

2. 文件系统权限
```rust
#[tauri::command]
async fn check_permissions(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}
```

### 12.2 内存管理

1. 资源释放
```rust
struct Resource {
    data: Vec<u8>,
}

impl Drop for Resource {
    fn drop(&mut self) {
        // 清理资源
        self.data.clear();
    }
}
```

2. 大文件处理
```rust
async fn process_large_file(path: &str) -> Result<(), std::io::Error> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut buffer = vec![0; 1024];
    
    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        // 处理数据块
    }
    Ok(())
}
```

## 13. 下一步学习

1. 深入学习前端框架集成
2. 探索更多 Tauri API
3. 实现复杂业务逻辑
4. 优化应用性能

在下一篇教程中，我们将通过实际项目案例，展示如何构建一个完整的 Tauri 应用。 