use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
};

pub fn create_layout(area: Rect) -> Rect {
    // 直接使用整个区域
    area
}

pub fn create_status_bar(area: Rect) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);
    chunks[0]
}
