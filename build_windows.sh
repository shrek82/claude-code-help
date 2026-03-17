#!/bin/bash

# Claude Code Help - Windows 跨平台构建脚本
# 交叉编译 Windows 环境的二进制文件 (ch.exe)

set -e

BINARY_NAME="ch.exe"
TARGET="x86_64-pc-windows-gnu"
CARGO_PROFILE="${CARGO_PROFILE:-release}"

echo "========================================"
echo "  Claude Code Help - Windows 构建脚本"
echo "========================================"
echo ""

# 检测操作系统
OS="$(uname -s)"
ARCH="$(uname -m)"

echo "检测到平台信息:"
echo "  操作系统：$OS"
echo "  架构：$ARCH"
echo ""

# 检查是否安装了 Rust 目标
echo "检查 Rust 目标..."
if ! rustup target list --installed | grep -q "$TARGET"; then
    echo "正在添加 Windows 目标：$TARGET"
    rustup target add "$TARGET"
fi

# 设置 cargo 配置
if [ "$CARGO_PROFILE" = "release" ]; then
    echo "使用 release 模式构建..."
    CARGO_ARGS="--release"
else
    echo "使用 debug 模式构建..."
    CARGO_ARGS=""
fi

# 根据宿主机平台设置交叉编译环境变量
if [ "$OS" = "Darwin" ]; then
    echo ""
    echo "检测到 macOS，安装 mingw-w64..."

    if command -v brew &> /dev/null; then
        if ! brew list mingw-w64 &> /dev/null; then
            echo "正在通过 Homebrew 安装 mingw-w64..."
            brew install mingw-w64
        fi
    else
        echo "警告：未检测到 Homebrew，请手动安装 mingw-w64"
        echo "  brew install mingw-w64"
        exit 1
    fi

    # 设置交叉编译链接器
    export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="$(brew --prefix mingw-w64)/bin/x86_64-w64-mingw32-gcc"

elif [ "$OS" = "Linux" ]; then
    echo ""
    echo "检测到 Linux，检查 mingw-w64..."

    if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
        echo "mingw-w64 已安装"
    else
        echo "错误：未找到 mingw-w64"
        echo "请安装：sudo apt-get install mingw-w64 (Debian/Ubuntu)"
        echo "      或：sudo dnf install mingw64-gcc (Fedora)"
        exit 1
    fi

    # 设置交叉编译链接器
    export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
else
    echo "警告：未知平台，可能无法交叉编译"
fi

echo ""
echo "链接器：${CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER:-default}"
echo ""
echo "开始构建 Windows 二进制..."

# 构建项目
cargo build $CARGO_ARGS --target "$TARGET" --bin rust-oss

# 复制二进制文件
if [ "$CARGO_PROFILE" = "release" ]; then
    SOURCE_PATH="target/$TARGET/release/rust-oss.exe"
else
    SOURCE_PATH="target/$TARGET/debug/rust-oss.exe"
fi

if [ -f "$SOURCE_PATH" ]; then
    cp "$SOURCE_PATH" "$BINARY_NAME"
    echo ""
    echo "========================================"
    echo "  Windows 构建成功!"
    echo "========================================"
    echo "  二进制文件：./$BINARY_NAME"
    echo "  目标平台：Windows x86_64"
    echo ""

    # 显示文件信息
    if command -v file &> /dev/null; then
        file "$BINARY_NAME"
    fi
else
    echo "错误：找不到构建产物 $SOURCE_PATH"
    exit 1
fi
