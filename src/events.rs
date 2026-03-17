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
        KeyCode::Tab => app.next_section(),
        KeyCode::BackTab => app.prev_section(),
        KeyCode::Char('l') => app.next_section(), // vim 风格：l 右
        KeyCode::Char('h') => app.prev_section(), // vim 风格：h 左
        KeyCode::Down | KeyCode::Char('j') => app.next_in_section(),
        KeyCode::Up | KeyCode::Char('k') => app.prev_in_section(),
        KeyCode::Char('f') | KeyCode::Char('/') => app.toggle_search(),
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
        _ => {}
    }
}
