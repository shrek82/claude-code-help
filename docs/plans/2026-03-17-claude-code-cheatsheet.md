# Claude Code Cheatsheet 实现计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to implement this plan task-by-task.

**Goal:** 使用 Ratatui 实现终端版 Claude Code 快捷键速查表，支持 F 键搜索和三 Tab 导航。

**Architecture:** 单二进制应用，事件驱动架构。App 状态管理 + 组件化 UI 渲染 + 文件持久化。

**Tech Stack:** Rust, Ratatui 0.29, Crossterm 0.28, Serde/JSON 用于持久化。

---

## Task 1: 项目依赖配置

**Files:**
- Modify: `/Users/youxiao/projects/rust-oss/Cargo.toml`

**Step 1: 更新 Cargo.toml 添加依赖**

```toml
[package]
name = "rust-oss"
version = "0.1.0"
edition = "2021"

[dependencies]
ratatui = "0.29"
crossterm = "0.28"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
directories = "5.0"
```

**Step 2: 验证依赖可解析**

```bash
cargo check
```
Expected: 下载依赖并编译通过（仅有 main.rs 的 hello world）

**Step 3: 提交**

```bash
git add Cargo.toml
git commit -m "chore: add ratatui and crossterm dependencies"
```

---

## Task 2: 数据结构与内置数据

**Files:**
- Create: `/Users/youxiao/projects/rust-oss/src/data/mod.rs`
- Create: `/Users/youxiao/projects/rust-oss/src/data/builtin.rs`

**Step 1: 编写测试**

```rust
// src/data/mod.rs - 在文件底部
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_entries_not_empty() {
        let entries = BuiltinEntry::all();
        assert!(!entries.is_empty());
    }
}
```

**Step 2: 实现数据结构**

```rust
// src/data/mod.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Shortcuts,
    SlashCommands,
    Tips,
}

#[derive(Debug, Clone)]
pub struct BuiltinEntry {
    pub key: String,
    pub description: String,
    pub category: Category,
}

impl BuiltinEntry {
    pub fn all() -> Vec<Self> {
        vec![
            // 快捷键
            BuiltinEntry { key: "Ctrl+C".into(), description: "中断当前操作".into(), category: Category::Shortcuts },
            BuiltinEntry { key: "Ctrl+L".into(), description: "清屏".into(), category: Category::Shortcuts },
            BuiltinEntry { key: "Tab".into(), description: "自动补全".into(), category: Category::Shortcuts },
            BuiltinEntry { key: "↑/↓".into(), description: "浏览历史".into(), category: Category::Shortcuts },
            BuiltinEntry { key: "Esc".into(), description: "关闭弹窗/取消".into(), category: Category::Shortcuts },

            // 斜杠命令
            BuiltinEntry { key: "/help".into(), description: "获取帮助".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/clear".into(), description: "清除对话历史".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/editor".into(), description: "打开外部编辑器".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/git".into(), description: "查看 git 状态".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/test".into(), description: "运行测试".into(), category: Category::SlashCommands },
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntry {
    pub id: String,
    pub key: String,
    pub description: String,
    pub tags: Vec<String>,
}
```

**Step 3: 运行测试**

```bash
cargo test data::tests::test_builtin_entries_not_empty -- --nocapture
```
Expected: PASS

**Step 4: 提交**

```bash
git add src/data/
git commit -m "feat: define data structures and builtin entries"
```

---

## Task 3: 自定义条目持久化

**Files:**
- Create: `/Users/youxiao/projects/rust-oss/src/data/custom.rs`
- Modify: `/Users/youxiao/projects/rust-oss/src/data/mod.rs` (导出 custom 模块)

**Step 1: 编写测试**

```rust
// src/data/custom.rs 底部
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_load_empty_file() {
        let temp_path = "/tmp/test_empty.json";
        let result = CustomStore::load(temp_path);
        assert!(result.is_ok());
        assert!(result.unwrap().entries.is_empty());
    }

    #[test]
    fn test_save_and_load() {
        let temp_path = "/tmp/test_save.json";
        let mut store = CustomStore::default();
        store.entries.push(CustomEntry {
            id: "1".into(),
            key: "F1".into(),
            description: "Test".into(),
            tags: vec![],
        });
        store.save(temp_path).unwrap();

        let loaded = CustomStore::load(temp_path).unwrap();
        assert_eq!(loaded.entries.len(), 1);

        fs::remove_file(temp_path).ok();
    }
}
```

**Step 2: 实现持久化逻辑**

```rust
// src/data/custom.rs
use super::CustomEntry;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CustomStore {
    pub entries: Vec<CustomEntry>,
}

impl CustomStore {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if !Path::new(path).exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path)?;
        let store: CustomStore = serde_json::from_str(&content)?;
        Ok(store)
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn add(&mut self, entry: CustomEntry) {
        self.entries.push(entry);
    }

    pub fn remove(&mut self, id: &str) {
        self.entries.retain(|e| e.id != id);
    }
}
```

**Step 3: 导出模块**

```rust
// src/data/mod.rs - 添加
pub mod custom;
pub use custom::CustomStore;
```

**Step 4: 运行测试**

```bash
cargo test data::custom::tests -- --nocapture
```
Expected: PASS

**Step 5: 提交**

```bash
git add src/data/
git commit -m "feat: implement custom entry persistence"
```

---

## Task 4: 应用状态管理

**Files:**
- Create: `/Users/youxiao/projects/rust-oss/src/app.rs`

**Step 1: 编写测试**

```rust
// app.rs 底部
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initial_state() {
        let app = App::new();
        assert_eq!(app.current_tab, 0);
        assert!(!app.searching);
        assert!(app.search_query.is_empty());
    }

    #[test]
    fn test_tab_navigation() {
        let mut app = App::new();
        app.next_tab();
        assert_eq!(app.current_tab, 1);
        app.next_tab();
        assert_eq!(app.current_tab, 2);
        app.next_tab();
        assert_eq!(app.current_tab, 0); // 循环
    }
}
```

**Step 2: 实现 App 状态**

```rust
// src/app.rs
use crate::data::{BuiltinEntry, Category, CustomStore, CustomEntry};

pub const NUM_TABS: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Searching,
}

pub struct App {
    pub current_tab: usize,
    pub input_mode: InputMode,
    pub search_query: String,
    pub search_results: Vec<usize>,
    pub selected_index: usize,
    pub custom_store: CustomStore,
    pub custom_path: String,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_tab: 0,
            input_mode: InputMode::Normal,
            search_query: String::new(),
            search_results: Vec::new(),
            selected_index: 0,
            custom_store: CustomStore::default(),
            custom_path: String::new(),
            should_quit: false,
        }
    }
}

impl App {
    pub fn new() -> Self {
        let mut app = Self::default();
        app.custom_path = app.get_custom_path();
        app.custom_store = CustomStore::load(&app.custom_path).unwrap_or_default();
        app
    }

    fn get_custom_path(&self) -> String {
        directories::BaseDirs::new()
            .map(|d| d.home_dir().join(".claude/cheatsheet/custom_entries.json").to_string_lossy().to_string())
            .unwrap_or_else(|| "custom_entries.json".to_string())
    }

    pub fn next_tab(&mut self) {
        self.current_tab = (self.current_tab + 1) % NUM_TABS;
        self.selected_index = 0;
    }

    pub fn prev_tab(&mut self) {
        self.current_tab = (self.current_tab + NUM_TABS - 1) % NUM_TABS;
        self.selected_index = 0;
    }

    pub fn toggle_search(&mut self) {
        self.input_mode = match self.input_mode {
            InputMode::Normal => {
                self.search_query.clear();
                self.search_results.clear();
                InputMode::Searching
            }
            InputMode::Searching => {
                self.search_results.clear();
                InputMode::Normal
            }
        };
    }

    pub fn update_search(&mut self) {
        self.search_results.clear();
        if self.search_query.is_empty() {
            return;
        }

        let entries = self.get_current_entries();
        for (i, (key, desc)) in entries.iter().enumerate() {
            if key.contains(&self.search_query) || desc.contains(&self.search_query) {
                self.search_results.push(i);
            }
        }
        self.selected_index = self.search_results.first().copied().unwrap_or(0);
    }

    pub fn get_current_entries(&self) -> Vec<(String, String)> {
        match self.current_tab {
            0 => BuiltinEntry::all()
                .into_iter()
                .filter(|e| e.category == Category::Shortcuts)
                .map(|e| (e.key, e.description))
                .collect(),
            1 => BuiltinEntry::all()
                .into_iter()
                .filter(|e| e.category == Category::SlashCommands)
                .map(|e| (e.key, e.description))
                .collect(),
            _ => self.custom_store
                .entries
                .iter()
                .map(|e| (e.key.clone(), e.description.clone()))
                .collect(),
        }
    }

    pub fn save_custom_entries(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.custom_store.save(&self.custom_path)?;
        Ok(())
    }

    pub fn add_custom_entry(&mut self, key: String, description: String) {
        let id = uuid::Uuid::new_v4().to_string();
        self.custom_store.entries.push(CustomEntry {
            id,
            key,
            description,
            tags: vec![],
        });
    }
}
```

**Step 3: 添加 uuid 依赖**

```toml
# Cargo.toml
[dependencies]
uuid = { version = "1.0", features = ["v4"] }
```

**Step 4: 运行测试**

```bash
cargo test app::tests -- --nocapture
```
Expected: PASS

**Step 5: 提交**

```bash
git add src/app.rs Cargo.toml
git commit -m "feat: implement app state management"
```

---

## Task 5: 事件处理模块

**Files:**
- Create: `/Users/youxiao/projects/rust-oss/src/events.rs`

**Step 1: 实现事件处理**

```rust
// src/events.rs
use crate::app::{App, InputMode};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub fn handle_event(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    if crossterm::event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => handle_normal_mode(app, key),
                InputMode::Searching => handle_searching_mode(app, key),
            }
        }
    }
    Ok(())
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('f') | KeyCode::Char('/') => app.toggle_search(),
        KeyCode::Right | KeyCode::Tab => app.next_tab(),
        KeyCode::Left => app.prev_tab(),
        KeyCode::Down => {
            let len = app.get_current_entries().len();
            if len > 0 {
                app.selected_index = (app.selected_index + 1) % len;
            }
        }
        KeyCode::Up => {
            let len = app.get_current_entries().len();
            if len > 0 {
                app.selected_index = (app.selected_index + len - 1) % len;
            }
        }
        KeyCode::Char('a') if app.current_tab == 2 => {
            // 添加自定义条目（仅在 Custom 页）
            app.add_custom_entry("NewKey".into(), "Description".into());
        }
        KeyCode::Char('s') if app.current_tab == 2 => {
            // 保存
            let _ = app.save_custom_entries();
        }
        _ => {}
    }
}

fn handle_searching_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Enter => app.toggle_search(),
        KeyCode::Backspace => {
            app.search_query.pop();
            app.update_search();
        }
        KeyCode::Char(c) => {
            app.search_query.push(c);
            app.update_search();
        }
        KeyCode::Down => {
            if !app.search_results.is_empty() {
                let current_pos = app.search_results.iter().position(|&i| i == app.selected_index);
                if let Some(pos) = current_pos {
                    let next_pos = (pos + 1) % app.search_results.len();
                    app.selected_index = app.search_results[next_pos];
                }
            }
        }
        KeyCode::Up => {
            if !app.search_results.is_empty() {
                let current_pos = app.search_results.iter().position(|&i| i == app.selected_index);
                if let Some(pos) = current_pos {
                    let prev_pos = (pos + app.search_results.len() - 1) % app.search_results.len();
                    app.selected_index = app.search_results[prev_pos];
                }
            }
        }
        _ => {}
    }
}
```

**Step 2: 运行编译检查**

```bash
cargo check
```
Expected: PASS

**Step 3: 提交**

```bash
git add src/events.rs
git commit -m "feat: implement input event handling"
```

---

## Task 6: UI 布局与 Tab 渲染

**Files:**
- Create: `/Users/youxiao/projects/rust-oss/src/ui/mod.rs`
- Create: `/Users/youxiao/projects/rust-oss/src/ui/layout.rs`
- Create: `/Users/youxiao/projects/rust-oss/src/ui/tabs.rs`

**Step 1: 创建 UI 模块**

```rust
// src/ui/mod.rs
pub mod layout;
pub mod tabs;
pub mod popup;

use ratatui::Frame;
use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let area = layout::create_layout(frame.area());
    tabs::render_tabs(frame, app, area);
    if app.input_mode == crate::app::InputMode::Searching {
        popup::render_search_popup(frame, app);
    }
}
```

**Step 2: 实现布局**

```rust
// src/ui/layout.rs
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn create_layout(area: Rect) -> Rect {
    // 返回主内容区域（减去顶部状态栏）
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 状态栏
            Constraint::Min(0),    // 主内容
        ])
        .split(area);
    chunks[1]
}

pub fn create_status_bar(area: Rect) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);
    chunks[0]
}
```

**Step 3: 实现 Tab 渲染**

```rust
// src/ui/tabs.rs
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
};
use crate::app::App;

const TAB_NAMES: [&str; 3] = ["快捷键", "斜杠命令", "自定义"];
const TAB_COLORS: [Color; 3] = [
    Color::Rgb(0, 135, 255),   // 蓝色
    Color::Rgb(0, 200, 100),   // 绿色
    Color::Rgb(150, 100, 255), // 紫色
];

pub fn render_tabs(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tab 标题
            Constraint::Min(0),    // 内容
        ])
        .split(area);

    // 渲染 Tab 标题
    let titles: Vec<Line> = TAB_NAMES
        .iter()
        .enumerate()
        .map(|(i, &t)| {
            let color = if i == app.current_tab {
                TAB_COLORS[i]
            } else {
                Color::Gray
            };
            Line::from(Span::styled(
                *t,
                Style::default().fg(color),
            ))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Claude Code Cheatsheet"))
        .select(app.current_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(TAB_COLORS[app.current_tab])
        );

    frame.render_widget(tabs, chunks[0]);

    // 渲染内容
    render_content(frame, app, chunks[1]);
}

fn render_content(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let entries = app.get_current_entries();
    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(i, (key, desc))| {
            let style = if i == app.selected_index {
                Style::default()
                    .fg(TAB_COLORS[app.current_tab])
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let content = if i == app.selected_index {
                format!("► {} │ {}", key, desc)
            } else {
                format!("  {} │ {}", key, desc)
            };

            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(TAB_NAMES[app.current_tab]));

    frame.render_widget(list, area);
}
```

**Step 4: 运行编译检查**

```bash
cargo check
```
Expected: PASS

**Step 5: 提交**

```bash
git add src/ui/
git commit -m "feat: implement UI layout and tab rendering"
```

---

## Task 7: 搜索弹窗

**Files:**
- Create: `/Users/youxiao/projects/rust-oss/src/ui/popup.rs`

**Step 1: 实现搜索弹窗**

```rust
// src/ui/popup.rs
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph},
};
use crate::app::App;

pub fn render_search_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, frame.area());

    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" 搜索 (Esc 关闭)")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    frame.render_widget(block, area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    let search_input = Paragraph::new(app.search_query.as_str())
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("输入搜索关键词"));

    frame.render_widget(search_input, inner[0]);

    if !app.search_results.is_empty() {
        let results_text = format!("找到 {} 个匹配项", app.search_results.len());
        let results = Paragraph::new(results_text)
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Center);
        frame.render_widget(results, inner[1]);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
```

**Step 2: 运行编译检查**

```bash
cargo check
```
Expected: PASS

**Step 3: 提交**

```bash
git add src/ui/popup.rs
git commit -m "feat: implement search popup"
```

---

## Task 8: 主程序入口

**Files:**
- Modify: `/Users/youxiao/projects/rust-oss/src/main.rs`

**Step 1: 编写完整的主程序**

```rust
// src/main.rs
mod app;
mod data;
mod events;
mod ui;

use app::{App, InputMode};
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    while !app.should_quit {
        terminal.draw(|f| ui::render(f, app))?;
        events::handle_event(app)?;
    }
    Ok(())
}
```

**Step 2: 编译并测试**

```bash
cargo build --release
```
Expected: 编译成功

**Step 3: 运行应用**

```bash
cargo run
```
Expected: 启动应用，可以看到三个 Tab 和内置数据

**Step 4: 提交**

```bash
git add src/main.rs
git commit -m "feat: complete main entry point"
```

---

## Task 9: 验证与收尾

**Step 1: 测试所有功能**

```bash
# 启动应用
cargo run

# 手动测试:
# 1. ← → 切换 Tab
# 2. ↑ ↓ 导航列表
# 3. F 打开搜索，输入关键词
# 4. Esc 关闭搜索
# 5. Q 退出
```

**Step 2: 运行所有测试**

```bash
cargo test
```
Expected: 所有测试通过

**Step 3: 最终提交**

```bash
git add -A
git commit -m "feat: complete Claude Code cheatsheet application"
```

---

## 完成标准

- ✅ 三个 Tab 正常切换
- ✅ F 键搜索功能正常
- ✅ 上下键导航正常
- ✅ Q 键退出正常
- ✅ 所有测试通过
- ✅ 代码编译无警告
