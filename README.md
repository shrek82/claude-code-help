# Claude Code Help

终端版的 Claude Code 快捷键速查表，基于 Rust + Ratatui 构建。

## 功能特性

- **三栏布局**：快捷键、斜杠命令、CLI 参考，一屏掌握所有常用命令
- **即时搜索**：按 `F` 或 `/` 快速搜索，高亮匹配结果
- **一键复制**：按 `C` 或 `Enter` 复制选中命令到剪贴板
- **底部状态栏**：实时显示模式、位置和快捷提示
- **翻页导航**：支持 PageUp/PageDown 快速翻页
- **跨平台支持**：提供 macOS、Linux、Windows 构建脚本

## 快速开始

### 从 GitHub 下载

访问 [Releases](https://github.com/shrek82/claude-code-help/releases) 下载对应平台的二进制文件。

### 本地构建

```bash
# macOS / Linux
./build.sh

# Windows 交叉编译
./build_windows.sh
```

构建产物为 `ch` (Unix) 或 `ch.exe` (Windows)。

## 使用指南

### 导航操作

| 按键 | 功能 |
|------|------|
| `Tab` / `→` / `l` | 切换到下一列 |
| `Shift+Tab` / `←` / `h` | 切换到上一列 |
| `↑` / `k` | 向上移动 |
| `↓` / `j` | 向下移动 |
| `PageUp` | 向上翻页 |
| `PageDown` | 向下翻页 |

### 功能操作

| 按键 | 功能 |
|------|------|
| `C` / `Enter` | 复制选中命令到剪贴板 |
| `F` / `/` | 打开/关闭搜索框 |
| `Q` / `Esc` | 退出应用 |

### 搜索模式

| 按键 | 功能 |
|------|------|
| `Esc` | 退出搜索（不复制） |
| `Enter` | 复制选中命令并退出搜索 |
| `Backspace` | 删除搜索关键词 |

## 项目结构

```
src/
├── main.rs          # 程序入口
├── app.rs           # 应用状态管理
├── events.rs        # 输入事件处理
├── data/
│   └── mod.rs       # 内置条目数据
└── ui/
    ├── mod.rs       # UI 模块入口
    ├── layout.rs    # 布局管理
    ├── tabs.rs      # Tab 渲染
    ├── popup.rs     # 搜索弹窗
    └── status_bar.rs # 底部状态栏
```

## 技术栈

- **Rust** - 系统编程语言
- **Ratatui** - 终端 UI 框架
- **Crossterm** - 跨平台终端控制
- **Arboard** - 跨平台剪贴板支持

## 构建脚本说明

| 脚本 | 目标平台 | 输出文件 |
|------|---------|---------|
| `build.sh` | 当前平台 (macOS/Linux) | `ch` |
| `build_windows.sh` | Windows x86_64 | `ch.exe` |

`build.sh` 会自动检测当前机器架构（Intel/ARM）并构建对应版本。

## 许可证

MIT License
