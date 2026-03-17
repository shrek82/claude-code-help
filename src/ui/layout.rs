use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
};

pub fn create_layout(area: Rect) -> Rect {
    // 返回主内容区域（减去顶部状态栏）
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 状态栏
            Constraint::Min(0),    // 主内容
        ])
        .split(area);
    chunks[1]
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
