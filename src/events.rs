use crate::app::{App, InputMode};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub fn handle_event(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    if crossterm::event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => handle_normal_mode(app, key),
                InputMode::Searching => handle_searching_mode(app, key),
            }
        }
    }
    Ok(())
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('f') | KeyCode::Char('/') => app.toggle_search(),
        KeyCode::Right | KeyCode::Tab => app.next_tab(),
        KeyCode::Left => app.prev_tab(),
        KeyCode::Down => {
            let len = app.get_current_entries().len();
            if len > 0 {
                app.selected_index = (app.selected_index + 1) % len;
            }
        }
        KeyCode::Up => {
            let len = app.get_current_entries().len();
            if len > 0 {
                app.selected_index = (app.selected_index + len - 1) % len;
            }
        }
        KeyCode::Char('a') if app.current_tab == 2 => {
            // 添加自定义条目（仅在 Custom 页）
            app.add_custom_entry("NewKey".into(), "Description".into());
        }
        KeyCode::Char('s') if app.current_tab == 2 => {
            // 保存
            let _ = app.save_custom_entries();
        }
        _ => {}
    }
}

fn handle_searching_mode(app: &mut App, key: KeyEvent) {
    match key.code {
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
