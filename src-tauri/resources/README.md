# APK解析工具 - AAPT2

此目录应包含APK解析所需的Android Asset Packaging Tool 2 (aapt2) 可执行文件。

## 手动下载说明

要获取aapt2.exe：

1. 从Android SDK Manager或Android Studio下载Windows版本的Android SDK Build Tools。
2. 导航到Android SDK安装目录中的build-tools/{版本}目录。
3. 将aapt2.exe复制到此目录。

## 从Android Studio获取

如果您已安装Android Studio：

1. 找到您的Android SDK安装位置（在Android Studio中，转到设置 -> 外观与行为 -> 系统设置 -> Android SDK）。
2. 导航到`[SDK位置]/build-tools/[最新版本]/`。
3. 将`aapt2.exe`复制到此目录。

## 通过命令行获取

如果您已安装Android命令行工具：

```
sdkmanager "build-tools;30.0.3"
```

然后在以下位置找到可执行文件：
```
%ANDROID_HOME%\build-tools\30.0.3\aapt2.exe
```

## 占位符文件

如果未找到实际的可执行文件，应用程序将创建占位符文件，但完整功能需要真正的aapt2工具。 