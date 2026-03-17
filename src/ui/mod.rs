pub mod layout;
pub mod tabs;
pub mod popup;
pub mod status_bar;

use ratatui::Frame;
use crate::app::{App, InputMode};

pub fn render(frame: &mut Frame, app: &mut App) {
    let areas = layout::create_layout(frame.area());
    tabs::render_tabs(frame, app, areas.main_area);
    status_bar::render_status_bar(frame, app, areas.status_bar_area);
    if matches!(app.input_mode, InputMode::Searching) {
        popup::render_search_popup(frame, app);
    }
}
