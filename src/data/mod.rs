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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_entries_not_empty() {
        let entries = BuiltinEntry::all();
        assert!(!entries.is_empty());
    }
}
