use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Shortcuts,
    SlashCommands,
    CliCommands,
}

#[derive(Debug, Clone)]
pub struct BuiltinEntry {
    pub key: String,
    pub description: String,
    pub category: Category,
}

static ENTRIES: Lazy<Vec<BuiltinEntry>> = Lazy::new(|| {
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

            // 斜杠命令 - 会话管理
            BuiltinEntry { key: "/clear".into(), description: "清除对话历史 (别名：/reset, /new)".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/compact [instructions]".into(), description: "压缩对话，可选焦点说明".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/continue".into(), description: "恢复之前的会话".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/desktop".into(), description: "在桌面应用中继续会话 (别名：/app)".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/exit".into(), description: "退出 CLI (别名：/quit)".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/export [filename]".into(), description: "导出对话为纯文本".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/fork [name]".into(), description: "创建当前对话的分支".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/rename [name]".into(), description: "重命名当前会话".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/resume [session]".into(), description: "按 ID/名称恢复会话 (别名：/continue)".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/rewind".into(), description: "回退对话/代码到上一点 (别名：/checkpoint)".into(), category: Category::SlashCommands },

            // 斜杠命令 - 配置与设置
            BuiltinEntry { key: "/config".into(), description: "打开设置 (主题/模型/输出样式) (别名：/settings)".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/fast [on|off]".into(), description: "切换快速模式".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/model [model]".into(), description: "选择/切换 AI 模型".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/theme".into(), description: "更改颜色主题".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/vim".into(), description: "切换 Vim/ 正常编辑模式".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/terminal-setup".into(), description: "配置终端快捷键".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/keybindings".into(), description: "打开快捷键配置文件".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/statusline".into(), description: "配置状态行显示".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/sandbox".into(), description: "切换沙盒模式".into(), category: Category::SlashCommands },

            // 斜杠命令 - 账户与登录
            BuiltinEntry { key: "/login".into(), description: "登录 Anthropic 账户".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/logout".into(), description: "登出 Anthropic 账户".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/usage".into(), description: "显示计划使用限制和速率限制".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/cost".into(), description: "显示令牌使用统计".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/extra-usage".into(), description: "配置超额使用".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/upgrade".into(), description: "打开升级页面".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/privacy-settings".into(), description: "查看/更新隐私设置 (Pro/Max)".into(), category: Category::SlashCommands },

            // 斜杠命令 - 工具与集成
            BuiltinEntry { key: "/add-dir <path>".into(), description: "添加工作目录到当前会话".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/agents".into(), description: "管理 agent 配置".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/chrome".into(), description: "配置 Chrome 中的 Claude 设置".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/copy".into(), description: "复制最后响应到剪贴板".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/diff".into(), description: "打开交互式差异查看器".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/hooks".into(), description: "管理 hook 配置".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/ide".into(), description: "管理 IDE 集成状态".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/mcp".into(), description: "管理 MCP server 连接".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/mobile".into(), description: "显示二维码下载移动应用 (别名：/ios, /android)".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/permissions".into(), description: "查看/更新权限 (别名：/allowed-tools)".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/plugin".into(), description: "管理 plugins".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/remote-control".into(), description: "启用远程控制 (别名：/rc)".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/remote-env".into(), description: "配置远程环境".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/skills".into(), description: "列出可用 skills".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/tasks".into(), description: "列出行管理后台任务".into(), category: Category::SlashCommands },

            // 斜杠命令 - Git 与代码
            BuiltinEntry { key: "/git".into(), description: "查看 git 状态".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/init".into(), description: "使用 CLAUDE.md 初始化项目".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/install-github-app".into(), description: "设置 GitHub Actions 应用".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/pr-comments [PR]".into(), description: "获取 PR 评论".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/security-review".into(), description: "分析代码安全漏洞".into(), category: Category::SlashCommands },

            // 斜杠命令 - 信息与帮助
            BuiltinEntry { key: "/help".into(), description: "显示帮助和可用命令".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/context".into(), description: "上下文使用情况可视化".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/doctor".into(), description: "诊断安装和设置".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/feedback".into(), description: "提交反馈 (别名：/bug)".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/insights".into(), description: "生成会话分析报告".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/memory".into(), description: "编辑 CLAUDE.md 内存文件".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/passes".into(), description: "分享免费周体验".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/plan".into(), description: "进入 plan mode".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/release-notes".into(), description: "查看完整变更日志".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/reload-plugins".into(), description: "重新加载 plugins".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/install-slack-app".into(), description: "安装 Slack 应用".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/stats".into(), description: "可视化使用统计".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/status".into(), description: "显示状态信息".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/stickers".into(), description: "订购贴纸".into(), category: Category::SlashCommands },
            BuiltinEntry { key: "/btw <question>".into(), description: "提出快速侧问题".into(), category: Category::SlashCommands },

            // CLI 参考
            BuiltinEntry { key: "claude".into(), description: "启动交互式会话".into(), category: Category::CliCommands },
            BuiltinEntry { key: "claude \"query\"".into(), description: "使用初始提示启动会话".into(), category: Category::CliCommands },
            BuiltinEntry { key: "claude -p \"query\"".into(), description: "通过 SDK 查询，然后退出".into(), category: Category::CliCommands },
            BuiltinEntry { key: "cat file | claude -p".into(), description: "处理管道内容".into(), category: Category::CliCommands },
            BuiltinEntry { key: "claude -c".into(), description: "继续当前目录最近的对话".into(), category: Category::CliCommands },
            BuiltinEntry { key: "claude -c -p".into(), description: "通过 SDK 继续".into(), category: Category::CliCommands },
            BuiltinEntry { key: "claude -r \"session\"".into(), description: "按 ID 或名称恢复会话".into(), category: Category::CliCommands },
            BuiltinEntry { key: "claude update".into(), description: "更新到最新版本".into(), category: Category::CliCommands },
            BuiltinEntry { key: "claude auth login".into(), description: "登录 Anthropic 账户".into(), category: Category::CliCommands },
            BuiltinEntry { key: "claude auth logout".into(), description: "登出 Anthropic 账户".into(), category: Category::CliCommands },
            BuiltinEntry { key: "claude auth status".into(), description: "显示身份验证状态 (JSON)".into(), category: Category::CliCommands },
            BuiltinEntry { key: "claude agents".into(), description: "列出所有已配置的 subagents".into(), category: Category::CliCommands },
            BuiltinEntry { key: "claude mcp".into(), description: "配置 MCP 服务器".into(), category: Category::CliCommands },
            BuiltinEntry { key: "claude remote-control".into(), description: "启动 Remote Control 会话".into(), category: Category::CliCommands },
    ]
});

impl BuiltinEntry {
    pub fn all() -> &'static [BuiltinEntry] {
        &ENTRIES
    }
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
