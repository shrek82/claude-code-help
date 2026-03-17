use crate::app::{App, InputMode};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub fn handle_event(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    if crossterm::event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => handle_normal_mode(app, key.code),
                InputMode::Searching => handle_searching_mode(app, key.code),
            }
        }
    }
    Ok(())
}

fn handle_normal_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        KeyCode::Char('f') | KeyCode::Char('/') => app.toggle_search(),
        KeyCode::Down | KeyCode::Char('j') => app.next_item(),
        KeyCode::Up | KeyCode::Char('k') => app.prev_item(),
        KeyCode::Char('a') => {
            // 添加自定义条目
            app.add_custom_entry("NewKey".into(), "Description".into());
        }
        KeyCode::Char('s') => {
            // 保存
            let _ = app.save_custom_entries();
        }
        KeyCode::Char('d') => {
            // 删除选中的自定义条目
            app.delete_selected_custom();
        }
        _ => {}
    }
}

fn handle_searching_mode(app: &mut App, key: KeyCode) {
    match key {
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
