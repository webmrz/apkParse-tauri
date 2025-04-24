# Android Asset Packaging Tool (aapt2) 使用指南

## 简介

Android Asset Packaging Tool 2 (aapt2) 是 Android SDK 的一部分，用于编译和打包 Android 应用程序资源。在 APK 分析工具中，我们使用 aapt2 来提取应用的清单文件 (AndroidManifest.xml) 和其他元数据信息。

## 获取 aapt2

在我们的应用中，有几种方法可以获取 aapt2：

1. **自动下载**：应用程序会尝试通过 UI 界面自动下载 aapt2。
2. **手动下载**：您可以从以下位置手动下载 aapt2：
   - [Android SDK Build Tools](https://developer.android.com/studio/releases/build-tools)
   - [GitHub 镜像](https://github.com/google/android-emulator-container-scripts/raw/master/emu/templates/softwareupdate/aapt2-windows.exe)

## 安装位置

aapt2 应该被放置在以下位置之一：

- 当前目录 (`./aapt2.exe`)
- `tools` 子目录 (`tools/aapt2.exe`)
- 系统 PATH 中的任何位置

## 功能

aapt2 提供以下关键功能：

1. **查看 APK 清单**：提取和显示 AndroidManifest.xml 文件的内容。
   ```
   aapt2 dump xmltree --file AndroidManifest.xml example.apk
   ```

2. **查看 APK 元数据**：显示应用程序的包名、版本、活动等信息。
   ```
   aapt2 dump badging example.apk
   ```

3. **分析资源**：列出 APK 中的资源文件。
   ```
   aapt2 dump resources example.apk
   ```

## 故障排除

如果您在使用 aapt2 时遇到问题：

1. 确保文件具有可执行权限。
2. 对于 Windows 用户，确保您的系统有必要的 Visual C++ 运行时库。
3. 如果通过应用程序自动下载失败，请尝试手动下载并放置在正确位置。

## 相关资源

- [Android 官方文档 - AAPT2](https://developer.android.com/studio/command-line/aapt2)
- [Android 构建工具](https://developer.android.com/studio/releases/build-tools) 