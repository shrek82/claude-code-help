pub mod layout;
pub mod tabs;
pub mod popup;

use ratatui::Frame;
use crate::app::{App, InputMode};

pub fn render(frame: &mut Frame, app: &App) {
    let area = layout::create_layout(frame.area());
    tabs::render_tabs(frame, app, area);
    if matches!(app.input_mode, InputMode::Searching) {
        popup::render_search_popup(frame, app);
    }
}
