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
            // 快捷键 - 通用控制
            BuiltinEntry { key: "Ctrl+C".into(), description: "取消当前输入或生成".into(), category: Category::Shortcuts },
            BuiltinEntry { key: "Ctrl+D".into(), description: "退出 Claude Code 会话 (EOF)".into(), category: Category::Shortcuts },
            BuiltinEntry { key: "Ctrl+L".into(), description: "清除终端屏幕（保留对话历史）".into(), category: Category::Shortcuts },

            // 快捷键 - 代理控制
            BuiltinEntry { key: "Ctrl+F".into(), description: "终止所有后台代理 (3 秒内按两次确认)".into(), category: Category::Shortcuts },
            BuiltinEntry { key: "Ctrl+B".into(), description: "后台运行任务 (Tmux 用户按两次)".into(), category: Category::Shortcuts },

            // 快捷键 - 编辑器与任务
            BuiltinEntry { key: "Ctrl+G".into(), description: "在默认文本编辑器中编辑提示".into(), category: Category::Shortcuts },
            BuiltinEntry { key: "Ctrl+T".into(), description: "切换任务列表显示/隐藏".into(), category: Category::Shortcuts },

            // 快捷键 - 输出与历史
            BuiltinEntry { key: "Ctrl+O".into(), description: "切换详细输出（显示工具使用情况）".into(), category: Category::Shortcuts },
            BuiltinEntry { key: "Ctrl+R".into(), description: "反向搜索命令历史".into(), category: Category::Shortcuts },
            BuiltinEntry { key: "↑/↓".into(), description: "导航命令历史".into(), category: Category::Shortcuts },

            // 快捷键 - 粘贴
            BuiltinEntry { key: "Ctrl+V / Cmd+V / Alt+V".into(), description: "粘贴图像 (iTerm2: Cmd+V, Windows: Alt+V)".into(), category: Category::Shortcuts },

            // 快捷键 - 高级功能
            BuiltinEntry { key: "Esc, Esc".into(), description: "回退或总结（恢复到上一点或总结对话）".into(), category: Category::Shortcuts },
            BuiltinEntry { key: "Shift+Tab / Alt+M".into(), description: "切换权限模式（自动/Plan/ 正常）".into(), category: Category::Shortcuts },
            BuiltinEntry { key: "Option+P / Alt+P".into(), description: "切换模型（不清除提示）".into(), category: Category::Shortcuts },
            BuiltinEntry { key: "Option+T / Alt+T".into(), description: "切换扩展思考模式".into(), category: Category::Shortcuts },

            // 斜杠命令 - 基础命令
            BuiltinEntry { key: "/help".into(), description: "获取帮助".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/clear".into(), description: "清除对话历史".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/editor".into(), description: "打开外部编辑器".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/git".into(), description: "查看 git 状态".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/test".into(), description: "运行测试".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/terminal-setup".into(), description: "配置终端快捷键".into(), category: Category::SlashCommands },
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

pub mod custom;
pub use custom::CustomStore;
