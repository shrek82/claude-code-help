# Claude Code Cheatsheet 设计文档

## 项目目标

使用 Ratatui 实现一个终端版的 Claude Code 快捷键速查表，支持 F 键搜索和自定义条目管理。

## 功能需求

### 核心功能
1. **三 Tab 分页展示**
   - 快捷键 (Shortcuts) - 蓝色主题
   - 斜杠命令 (Slash Commands) - 绿色主题
   - 自定义条目 (Custom) - 紫色主题

2. **F 键即时搜索**
   - 弹出搜索框
   - 输入时实时过滤和高亮匹配项
   - Esc 关闭，Enter 确认跳转

3. **自定义条目管理**
   - A: 添加新条目
   - E: 编辑选中条目
   - D: 删除选中条目
   - S: 保存到 ~/.claude/cheatsheet/custom_entries.json

4. **导航控制**
   - ← → / Tab: 切换分类
   - ↑ ↓: 导航列表项
   - Q: 退出应用

### 界面要求
- 紧凑布局，最大化信息密度
- 分类色彩标识，快速识别
- 选中项高亮 + 边框
- 顶部状态栏显示快捷键提示

## 技术架构

### 依赖
```toml
[dependencies]
ratatui = "0.29"
crossterm = "0.28"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

### 模块结构
```
src/
├── main.rs          # 入口与事件循环
├── app.rs           # 应用状态管理
├── events.rs        # 输入事件处理
├── ui/
│   ├── mod.rs
│   ├── layout.rs    # 布局管理
│   ├── tabs.rs      # Tab 渲染
│   └── popup.rs     # 搜索弹窗
└── data/
    ├── mod.rs
    ├── builtin.rs   # 内置数据
    └── custom.rs    # 自定义持久化
```

### 数据结构
```rust
enum Category { Shortcuts, SlashCommands, Tips }

struct BuiltinEntry {
    key: String,
    description: String,
    category: Category,
}

struct CustomEntry {
    id: String,
    key: String,
    description: String,
    tags: Vec<String>,
}
```

## 配色方案
- Shortcuts: 蓝色 (0, 135, 255)
- SlashCommands: 绿色 (0, 200, 100)
- Custom: 紫色 (150, 100, 255)
- Selected: 反向 + 黄色边框
- SearchHit: 橙色高亮
