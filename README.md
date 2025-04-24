# APK分析工具

这是一个基于Tauri的APK分析工具，可以帮助用户分析Android APK文件的内容、权限、签名等信息。

## 功能特点

- 分析APK基本信息（包名、版本等）
- 检查APK签名信息和有效期
- 分析权限请求，特别标记危险权限
- 计算文件哈希值（MD5, SHA1, SHA256）
- 自动检测并下载必要工具（aapt2）

## 技术栈

- **前端**: Vue 3, TypeScript
- **后端**: Rust, Tauri
- **工具**: Android AAPT2

## 项目结构

```
tauri-app/
├── src/                  # 前端代码
│   ├── components/       # Vue组件
│   ├── stores/           # Pinia状态管理
│   ├── views/            # 页面视图
│   └── App.vue           # 主应用组件
├── src-tauri/            # Rust后端代码
│   ├── src/              # Rust源代码
│   │   ├── apk_parser.rs # APK解析核心功能
│   │   ├── commands.rs   # Tauri命令定义
│   │   ├── main.rs       # 应用入口
│   │   └── backend.rs    # 后端工具函数
│   └── Cargo.toml        # Rust依赖配置
├── docs/                 # 文档
└── public/               # 静态资源
```

## 安装和使用

### 环境要求

- Node.js 16+
- Rust 1.70+
- Tauri CLI

### 安装步骤

1. 克隆仓库
```bash
git clone <仓库URL>
cd tauri-app
```

2. 安装依赖
```bash
pnpm install
```

3. 开发模式运行
```bash
pnpm run tauri dev
```

4. 构建应用
```bash
pnpm run tauri build
```

## AAPT2工具

本应用使用Android AAPT2工具来提取APK信息。详细信息请参考[AAPT2使用指南](docs/aapt2_usage.md)。

## 贡献指南

欢迎贡献代码、报告问题或提供建议。请遵循以下步骤：


## 许可证

MIT
