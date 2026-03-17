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
