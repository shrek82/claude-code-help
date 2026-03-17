use crate::data::{BuiltinEntry, Category};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Searching,
}

pub struct App {
    pub current_section: usize, // 当前选中的列 (0-2)
    pub selected_index_in_section: usize, // 当前列内的选中索引
    pub scroll_offsets: [usize; 3], // 每个分区的滚动偏移
    pub input_mode: InputMode,
    pub search_query: String,
    pub should_quit: bool,
    pub copy_feedback: Option<String>, // 复制反馈消息
    // 搜索相关状态
    pub search_results: Vec<(usize, usize)>, // (section_idx, local_idx)
    pub search_selected_index: usize, // 搜索结果中的选中索引
    pub visible_height: usize, // 可见区域高度（由 UI 更新）
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_section: 0,
            selected_index_in_section: 0,
            scroll_offsets: [0, 0, 0],
            input_mode: InputMode::Normal,
            search_query: String::new(),
            should_quit: false,
            copy_feedback: None,
            search_results: Vec::new(),
            search_selected_index: 0,
            visible_height: 20, // 默认值，渲染时会被更新
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    // 获取指定列的条目数量
    pub fn get_section_count(&self, section_index: usize) -> usize {
        match section_index {
            0 => self.get_shortcuts_count(),
            1 => self.get_slash_commands_count(),
            _ => self.get_cli_commands_count(),
        }
    }

    // 获取指定分区的条目
    pub fn get_section_entries(&self) -> Vec<(usize, usize, String, String)> {
        let all_entries = BuiltinEntry::all();
        let mut result = Vec::new();
        let mut section_counts = [0, 0, 0];

        for entry in all_entries {
            let section_idx = match entry.category {
                Category::Shortcuts => 0,
                Category::SlashCommands => 1,
                Category::CliCommands => 2,
            };
            let local_idx = section_counts[section_idx];
            section_counts[section_idx] += 1;
            result.push((section_idx, local_idx, entry.key.clone(), entry.description.clone()));
        }

        result
    }

    // 获取指定分区的条目（仅返回该分区的）
    pub fn get_entries_for_section(&self, section_index: usize) -> Vec<(String, String)> {
        self.get_section_entries()
            .into_iter()
            .filter(|(section, _, _, _)| *section == section_index)
            .map(|(_, _local, key, desc)| (key, desc))
            .collect()
    }

    pub fn get_shortcuts_count(&self) -> usize {
        BuiltinEntry::all()
            .iter()
            .filter(|e| e.category == Category::Shortcuts)
            .count()
    }

    pub fn get_slash_commands_count(&self) -> usize {
        BuiltinEntry::all()
            .iter()
            .filter(|e| e.category == Category::SlashCommands)
            .count()
    }

    pub fn get_cli_commands_count(&self) -> usize {
        BuiltinEntry::all()
            .iter()
            .filter(|e| e.category == Category::CliCommands)
            .count()
    }

    // 获取所有条目用于搜索（返回 section_idx, local_idx, key, desc）
    pub fn get_all_entries_for_search(&self) -> Vec<(usize, usize, String, String)> {
        self.get_section_entries()
    }

    // 切换到下一列
    pub fn next_section(&mut self) {
        self.current_section = (self.current_section + 1) % 3;
        self.selected_index_in_section = 0;
    }

    // 切换到上一列
    pub fn prev_section(&mut self) {
        self.current_section = (self.current_section + 2) % 3;
        self.selected_index_in_section = 0;
    }

    // 在当前列内向下移动
    pub fn next_in_section(&mut self, visible_height: usize) {
        let count = self.get_section_count(self.current_section);
        if count > 0 {
            let old_index = self.selected_index_in_section;
            self.selected_index_in_section = (self.selected_index_in_section + 1) % count;

            // 检测是否发生了循环（从末尾到 0）
            let wrapped_around = old_index == count - 1 && self.selected_index_in_section == 0;

            if wrapped_around {
                // 循环到开头时，将滚动偏移重置为 0
                self.scroll_offsets[self.current_section] = 0;
            } else if self.selected_index_in_section >= self.scroll_offsets[self.current_section] + visible_height {
                // 确保选中项在可见区域内
                self.scroll_offsets[self.current_section] = self.selected_index_in_section - visible_height + 1;
            }
        }
    }

    // 在当前列内向上移动
    pub fn prev_in_section(&mut self, visible_height: usize) {
        let count = self.get_section_count(self.current_section);
        if count > 0 {
            let old_index = self.selected_index_in_section;
            self.selected_index_in_section = (self.selected_index_in_section + count - 1) % count;

            // 检测是否发生了循环（从 0 到末尾）
            let wrapped_around = old_index == 0 && self.selected_index_in_section == count - 1;

            if wrapped_around {
                // 循环到末尾时，将滚动偏移设置为使选中项在底部可见
                if count >= visible_height {
                    self.scroll_offsets[self.current_section] = count - visible_height;
                } else {
                    self.scroll_offsets[self.current_section] = 0;
                }
            } else if self.selected_index_in_section < self.scroll_offsets[self.current_section] {
                // 正常向上移动，确保选中项在可见区域内
                self.scroll_offsets[self.current_section] = self.selected_index_in_section;
            }
        }
    }

    // 快速向下翻页（PageDown）
    pub fn page_down(&mut self, visible_height: usize) {
        let count = self.get_section_count(self.current_section);
        if count > 0 {
            let scroll_offset = self.scroll_offsets[self.current_section];
            self.selected_index_in_section = (self.selected_index_in_section + visible_height).min(count - 1);
            // 确保选中项在可见区域内
            if self.selected_index_in_section >= scroll_offset + visible_height {
                self.scroll_offsets[self.current_section] = self.selected_index_in_section - visible_height + 1;
            }
        }
    }

    // 快速向上翻页（PageUp）
    pub fn page_up(&mut self, visible_height: usize) {
        let count = self.get_section_count(self.current_section);
        if count > 0 {
            self.selected_index_in_section = self.selected_index_in_section.saturating_sub(visible_height);
            // 确保选中项在可见区域内
            if self.selected_index_in_section < self.scroll_offsets[self.current_section] {
                self.scroll_offsets[self.current_section] = self.selected_index_in_section;
            }
        }
    }

    // 复制选中的命令到剪贴板
    pub fn copy_selection(&mut self) {
        let entries = self.get_entries_for_section(self.current_section);
        if let Some((key, desc)) = entries.get(self.selected_index_in_section) {
            let text = format!("{} - {}", key, desc);
            match arboard::Clipboard::new().and_then(|mut c| c.set_text(&text)) {
                Ok(_) => {
                    self.copy_feedback = Some(format!("已复制：{}", key));
                }
                Err(_) => {
                    self.copy_feedback = Some("复制失败".into());
                }
            }
        }
    }

    pub fn toggle_search(&mut self) {
        self.input_mode = match self.input_mode {
            InputMode::Normal => {
                self.search_query.clear();
                self.search_results.clear();
                self.search_selected_index = 0;
                InputMode::Searching
            }
            InputMode::Searching => InputMode::Normal,
        };
    }

    pub fn update_search(&mut self) {
        self.search_results.clear();
        self.search_selected_index = 0;

        let all_entries = self.get_all_entries_for_search();
        for (section_idx, local_idx, key, desc) in all_entries.iter() {
            if key.contains(&self.search_query) || desc.contains(&self.search_query) {
                self.search_results.push((*section_idx, *local_idx));
            }
        }

        // 更新选中位置到第一个搜索结果
        if let Some((section_idx, local_idx)) = self.search_results.first() {
            self.current_section = *section_idx;
            self.selected_index_in_section = *local_idx;
        }
    }

    // 在搜索结果中向下导航
    pub fn next_search_result(&mut self) {
        if self.search_results.is_empty() {
            return;
        }
        self.search_selected_index = (self.search_selected_index + 1) % self.search_results.len();
        let (section_idx, local_idx) = self.search_results[self.search_selected_index];
        self.current_section = section_idx;
        self.selected_index_in_section = local_idx;
        // 更新滚动偏移，确保选中项可见
        self.scroll_offsets[section_idx] = local_idx;
    }

    // 在搜索结果中向上导航
    pub fn prev_search_result(&mut self) {
        if self.search_results.is_empty() {
            return;
        }
        self.search_selected_index = (self.search_selected_index + self.search_results.len() - 1) % self.search_results.len();
        let (section_idx, local_idx) = self.search_results[self.search_selected_index];
        self.current_section = section_idx;
        self.selected_index_in_section = local_idx;
        // 更新滚动偏移，确保选中项可见
        self.scroll_offsets[section_idx] = local_idx;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initial_state() {
        let app = App::new();
        assert_eq!(app.current_section, 0);
        assert_eq!(app.selected_index_in_section, 0);
        assert!(matches!(app.input_mode, InputMode::Normal));
    }

    #[test]
    fn test_section_navigation() {
        let mut app = App::new();
        app.next_section();
        assert_eq!(app.current_section, 1);
        app.next_section();
        assert_eq!(app.current_section, 2);
        app.next_section();
        assert_eq!(app.current_section, 0);

        app.prev_section();
        assert_eq!(app.current_section, 2);
    }

    #[test]
    fn test_in_section_navigation() {
        let mut app = App::new();
        let shortcuts_count = app.get_shortcuts_count();
        app.next_in_section(20);
        assert_eq!(app.selected_index_in_section, 1);
        app.prev_in_section(20);
        assert_eq!(app.selected_index_in_section, 0);

        for _ in 0..shortcuts_count {
            app.next_in_section(20);
        }
        assert_eq!(app.selected_index_in_section, 0);
    }
}
