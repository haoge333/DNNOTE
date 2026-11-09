@echo off
chcp 65001 >nul
echo ==========================================
echo 龙之谷小助手 - Tauri 一键编译脚本
echo ==========================================
echo.

REM 检查 cargo 是否已安装
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [错误] 未找到 cargo，请先安装 Rust：
    echo         https://rustup.rs/
    echo.
    pause
    exit /b 1
)

echo [1/3] 安装 Tauri CLI...
cargo install tauri-cli --version "^1.0" --quiet
if %errorlevel% neq 0 (
    echo [警告] Tauri CLI 安装可能已存在或失败，继续尝试编译...
)

echo [2/3] 编译项目...
cd /d "%~dp0"
cargo tauri build
if %errorlevel% neq 0 (
    echo [错误] 编译失败，请检查网络连接和 Rust 环境
    pause
    exit /b 1
)

echo [3/3] 编译完成！
echo.
echo 生成的 exe 文件位于：
echo   %~dp0src-tauri\target\release\dn-dragon-nest.exe
echo.
echo 安装包位于：
echo   %~dp0src-tauri\target\release\bundle\nsis\
echo.
pause
