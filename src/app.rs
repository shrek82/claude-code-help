use crate::data::{BuiltinEntry, Category};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Searching,
}

pub struct App {
    pub current_section: usize, // 当前选中的列 (0-2)
    pub selected_index_in_section: usize, // 当前列内的选中索引
    pub input_mode: InputMode,
    pub search_query: String,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_section: 0,
            selected_index_in_section: 0,
            input_mode: InputMode::Normal,
            search_query: String::new(),
            should_quit: false,
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
    pub fn get_section_entries(&self, section_index: usize) -> Vec<(String, String)> {
        match section_index {
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
            _ => BuiltinEntry::all()
                .into_iter()
                .filter(|e| e.category == Category::CliCommands)
                .map(|e| (e.key, e.description))
                .collect(),
        }
    }

    pub fn get_shortcuts_count(&self) -> usize {
        BuiltinEntry::all()
            .into_iter()
            .filter(|e| e.category == Category::Shortcuts)
            .count()
    }

    pub fn get_slash_commands_count(&self) -> usize {
        BuiltinEntry::all()
            .into_iter()
            .filter(|e| e.category == Category::SlashCommands)
            .count()
    }

    pub fn get_cli_commands_count(&self) -> usize {
        BuiltinEntry::all()
            .into_iter()
            .filter(|e| e.category == Category::CliCommands)
            .count()
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
    pub fn next_in_section(&mut self) {
        let count = self.get_section_count(self.current_section);
        if count > 0 {
            self.selected_index_in_section = (self.selected_index_in_section + 1) % count;
        }
    }

    // 在当前列内向上移动
    pub fn prev_in_section(&mut self) {
        let count = self.get_section_count(self.current_section);
        if count > 0 {
            self.selected_index_in_section = (self.selected_index_in_section + count - 1) % count;
        }
    }

    pub fn toggle_search(&mut self) {
        self.input_mode = match self.input_mode {
            InputMode::Normal => {
                self.search_query.clear();
                InputMode::Searching
            }
            InputMode::Searching => InputMode::Normal,
        };
    }

    pub fn update_search(&mut self) {
        let all_entries = self.get_all_entries_for_search();
        for (section_idx, local_idx, key, desc) in all_entries.iter() {
            if key.contains(&self.search_query) || desc.contains(&self.search_query) {
                self.current_section = *section_idx;
                self.selected_index_in_section = *local_idx;
                return;
            }
        }
    }

    pub fn get_all_entries_for_search(&self) -> Vec<(usize, usize, String, String)> {
        let mut result = Vec::new();

        // 快捷键
        let shortcuts = self.get_section_entries(0);
        for (i, (key, desc)) in shortcuts.iter().enumerate() {
            result.push((0, i, key.clone(), desc.clone()));
        }

        // 斜杠命令
        let commands = self.get_section_entries(1);
        for (i, (key, desc)) in commands.iter().enumerate() {
            result.push((1, i, key.clone(), desc.clone()));
        }

        // CLI 参考
        let cli = self.get_section_entries(2);
        for (i, (key, desc)) in cli.iter().enumerate() {
            result.push((2, i, key.clone(), desc.clone()));
        }

        result
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
        app.next_in_section();
        assert_eq!(app.selected_index_in_section, 1);
        app.prev_in_section();
        assert_eq!(app.selected_index_in_section, 0);

        for _ in 0..shortcuts_count {
            app.next_in_section();
        }
        assert_eq!(app.selected_index_in_section, 0);
    }
}
