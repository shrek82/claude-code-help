use crate::data::{BuiltinEntry, Category, CustomStore, CustomEntry};

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
    pub custom_store: CustomStore,
    pub custom_path: String,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_section: 0,
            selected_index_in_section: 0,
            input_mode: InputMode::Normal,
            search_query: String::new(),
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

    // 获取指定列的条目数量
    pub fn get_section_count(&self, section_index: usize) -> usize {
        match section_index {
            0 => self.get_shortcuts_count(),
            1 => self.get_commands_count(),
            _ => self.custom_store.entries.len(),
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
            _ => self.custom_store
                .entries
                .iter()
                .map(|e| (e.key.clone(), e.description.clone()))
                .collect(),
        }
    }

    pub fn get_shortcuts_count(&self) -> usize {
        BuiltinEntry::all()
            .into_iter()
            .filter(|e| e.category == Category::Shortcuts)
            .count()
    }

    pub fn get_commands_count(&self) -> usize {
        BuiltinEntry::all()
            .into_iter()
            .filter(|e| e.category == Category::SlashCommands)
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
        // 搜索时高亮第一个匹配项所在的列和位置
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

        // 自定义
        for (i, entry) in self.custom_store.entries.iter().enumerate() {
            result.push((2, i, entry.key.clone(), entry.description.clone()));
        }

        result
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

    pub fn delete_selected_custom(&mut self) {
        if self.current_section == 2 && !self.custom_store.entries.is_empty() {
            if self.selected_index_in_section < self.custom_store.entries.len() {
                self.custom_store.entries.remove(self.selected_index_in_section);
                // 调整索引
                if self.selected_index_in_section >= self.custom_store.entries.len() {
                    self.selected_index_in_section = self.selected_index_in_section.saturating_sub(1);
                }
            }
        }
    }

    // 获取当前选中的条目
    pub fn get_selected_entry(&self) -> Option<(String, String)> {
        let entries = self.get_section_entries(self.current_section);
        entries.get(self.selected_index_in_section).cloned()
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

        // 测试循环
        for _ in 0..shortcuts_count {
            app.next_in_section();
        }
        assert_eq!(app.selected_index_in_section, 0);
    }
}
