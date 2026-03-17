pub mod layout;
pub mod tabs;
pub mod popup;

use ratatui::Frame;
use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let area = layout::create_layout(frame.area());
    tabs::render_tabs(frame, app, area);
    if app.input_mode == crate::app::InputMode::Searching {
        popup::render_search_popup(frame, app);
    }
}
