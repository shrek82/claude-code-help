use crate::data::{BuiltinEntry, Category, CustomStore, CustomEntry};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Searching,
}

pub struct App {
    pub input_mode: InputMode,
    pub search_query: String,
    pub search_results: Vec<usize>,
    pub selected_index: usize, // 全局索引
    pub custom_store: CustomStore,
    pub custom_path: String,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
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

        let all_entries = self.get_all_entries_with_section();
        for (global_idx, _, _) in all_entries.iter() {
            let key = &self.get_entry_key(*global_idx);
            let desc = &self.get_entry_desc(*global_idx);
            if key.contains(&self.search_query) || desc.contains(&self.search_query) {
                self.search_results.push(*global_idx);
            }
        }
        if !self.search_results.is_empty() && !self.search_results.contains(&self.selected_index) {
            self.selected_index = self.search_results[0];
        }
    }

    // 获取单个分区的条目
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

    // 获取所有条目的全局索引和所属分区
    pub fn get_all_entries_with_section(&self) -> Vec<(usize, usize, String)> {
        let mut result = Vec::new();
        let mut global_idx = 0;

        // 快捷键
        for _ in 0..self.get_shortcuts_count() {
            result.push((global_idx, 0, String::new()));
            global_idx += 1;
        }
        // 斜杠命令
        for _ in 0..self.get_commands_count() {
            result.push((global_idx, 1, String::new()));
            global_idx += 1;
        }
        // 自定义
        for _ in 0..self.custom_store.entries.len() {
            result.push((global_idx, 2, String::new()));
            global_idx += 1;
        }

        result
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

    pub fn get_total_entries_count(&self) -> usize {
        self.get_shortcuts_count() + self.get_commands_count() + self.custom_store.entries.len()
    }

    fn get_entry_key(&self, global_index: usize) -> String {
        let shortcuts_count = self.get_shortcuts_count();
        let commands_count = self.get_commands_count();

        if global_index < shortcuts_count {
            BuiltinEntry::all()
                .into_iter()
                .filter(|e| e.category == Category::Shortcuts)
                .nth(global_index)
                .map(|e| e.key)
                .unwrap_or_default()
        } else if global_index < shortcuts_count + commands_count {
            let idx = global_index - shortcuts_count;
            BuiltinEntry::all()
                .into_iter()
                .filter(|e| e.category == Category::SlashCommands)
                .nth(idx)
                .map(|e| e.key)
                .unwrap_or_default()
        } else {
            let idx = global_index - shortcuts_count - commands_count;
            self.custom_store.entries.get(idx).map(|e| e.key.clone()).unwrap_or_default()
        }
    }

    fn get_entry_desc(&self, global_index: usize) -> String {
        let shortcuts_count = self.get_shortcuts_count();
        let commands_count = self.get_commands_count();

        if global_index < shortcuts_count {
            BuiltinEntry::all()
                .into_iter()
                .filter(|e| e.category == Category::Shortcuts)
                .nth(global_index)
                .map(|e| e.description)
                .unwrap_or_default()
        } else if global_index < shortcuts_count + commands_count {
            let idx = global_index - shortcuts_count;
            BuiltinEntry::all()
                .into_iter()
                .filter(|e| e.category == Category::SlashCommands)
                .nth(idx)
                .map(|e| e.description)
                .unwrap_or_default()
        } else {
            let idx = global_index - shortcuts_count - commands_count;
            self.custom_store.entries.get(idx).map(|e| e.description.clone()).unwrap_or_default()
        }
    }

    // 全局导航：下一条
    pub fn next_item(&mut self) {
        let total = self.get_total_entries_count();
        if total > 0 {
            self.selected_index = (self.selected_index + 1) % total;
        }
    }

    // 全局导航：上一条
    pub fn prev_item(&mut self) {
        let total = self.get_total_entries_count();
        if total > 0 {
            self.selected_index = (self.selected_index + total - 1) % total;
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

    pub fn delete_selected_custom(&mut self) {
        let shortcuts_count = self.get_shortcuts_count();
        let commands_count = self.get_commands_count();
        let custom_start = shortcuts_count + commands_count;

        if self.selected_index >= custom_start {
            let custom_idx = self.selected_index - custom_start;
            if custom_idx < self.custom_store.entries.len() {
                self.custom_store.entries.remove(custom_idx);
                // 调整选中索引
                if self.selected_index >= custom_start + self.custom_store.entries.len() {
                    self.selected_index = self.selected_index.saturating_sub(1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initial_state() {
        let app = App::new();
        assert!(matches!(app.input_mode, InputMode::Normal));
        assert!(app.search_query.is_empty());
    }

    #[test]
    fn test_navigation() {
        let mut app = App::new();
        let initial = app.selected_index;
        app.next_item();
        assert!(app.selected_index > initial);
        app.prev_item();
        assert_eq!(app.selected_index, initial);
    }
}
