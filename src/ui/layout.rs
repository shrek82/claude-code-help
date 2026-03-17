use ratatui::layout::{Constraint, Direction, Layout};

pub struct AppAreas {
    pub main_area: ratatui::layout::Rect,
    pub status_bar_area: ratatui::layout::Rect,
}

pub fn create_layout(area: ratatui::layout::Rect) -> AppAreas {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0), // 主内容区域
            Constraint::Length(3), // 底部状态栏
        ])
        .split(area);

    AppAreas {
        main_area: chunks[0],
        status_bar_area: chunks[1],
    }
}
