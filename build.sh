#!/bin/bash

# Claude Code Help - 跨平台构建脚本
# 输出二进制文件名为 ch (claude help 简称)

set -e

BINARY_NAME="ch"
CARGO_PROFILE="${CARGO_PROFILE:-release}"

echo "========================================"
echo "  Claude Code Help - 构建脚本"
echo "========================================"
echo ""

# 检测操作系统
OS="$(uname -s)"
ARCH="$(uname -m)"

echo "检测到平台信息:"
echo "  操作系统：$OS"
echo "  架构：$ARCH"
echo ""

# 设置 cargo 配置
if [ "$CARGO_PROFILE" = "release" ]; then
    echo "使用 release 模式构建..."
    CARGO_ARGS="--release"
else
    echo "使用 debug 模式构建..."
    CARGO_ARGS=""
fi

echo ""
echo "开始构建..."

# 构建项目
cargo build $CARGO_ARGS --bin rust-oss

# 复制二进制文件
if [ "$CARGO_PROFILE" = "release" ]; then
    SOURCE_PATH="target/release/rust-oss"
else
    SOURCE_PATH="target/debug/rust-oss"
fi

# 根据平台使用不同的复制命令
if [ "$OS" = "Darwin" ] || [ "$OS" = "Linux" ]; then
    if [ -f "$SOURCE_PATH" ]; then
        cp "$SOURCE_PATH" "$BINARY_NAME"
        echo ""
        echo "========================================"
        echo "  构建成功!"
        echo "========================================"
        echo "  二进制文件：./$BINARY_NAME"
        echo "  运行方式：./$BINARY_NAME"
        echo ""

        # 尝试赋予执行权限
        chmod +x "$BINARY_NAME" 2>/dev/null || true
    else
        echo "错误：找不到构建产物 $SOURCE_PATH"
        exit 1
    fi
else
    echo "警告：未知平台，尝试直接构建..."
    cargo build $CARGO_ARGS
fi
