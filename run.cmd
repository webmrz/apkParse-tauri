@echo off
echo 启动APK分析工具...
cd /d %~dp0
echo 当前目录: %CD%

echo 确认pnpm是否安装...
where pnpm
if %ERRORLEVEL% NEQ 0 (
  echo 未找到pnpm，正在尝试使用npm...
  where npm
  if %ERRORLEVEL% NEQ 0 (
    echo 未找到npm，请安装Node.js和pnpm
    pause
    exit /b 1
  )
)

echo 安装依赖...
call pnpm install || call npm install

echo 启动应用...
call pnpm tauri dev || call npm run tauri dev

if %ERRORLEVEL% NEQ 0 (
  echo 启动失败，错误代码: %ERRORLEVEL%
  echo 尝试直接运行Cargo...
  cd src-tauri
  cargo run --release
)

pause 