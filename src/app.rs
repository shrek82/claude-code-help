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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initial_state() {
        let app = App::new();
        assert_eq!(app.current_tab, 0);
        assert!(matches!(app.input_mode, InputMode::Normal));
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
