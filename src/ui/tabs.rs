use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};
use crate::app::App;

const SECTION_COLORS: [Color; 3] = [
    Color::Rgb(0, 135, 255),   // 蓝色 - 快捷键
    Color::Rgb(0, 200, 100),   // 绿色 - 斜杠命令
    Color::Rgb(150, 100, 255), // 紫色 - 自定义
];

pub fn render_tabs(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    // 垂直分为三个区域
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 3), // 快捷键
            Constraint::Ratio(1, 3), // 斜杠命令
            Constraint::Min(4),      // 自定义
        ])
        .split(area);

    // 渲染三个分区
    render_section(frame, app, chunks[0], 0, "快捷键", SECTION_COLORS[0]);
    render_section(frame, app, chunks[1], 1, "斜杠命令", SECTION_COLORS[1]);
    render_section(frame, app, chunks[2], 2, "自定义", SECTION_COLORS[2]);
}

fn render_section(
    frame: &mut Frame,
    app: &App,
    area: ratatui::layout::Rect,
    section_index: usize,
    title: &str,
    color: Color,
) {
    let entries = app.get_section_entries(section_index);

    // 计算全局索引偏移
    let offset = match section_index {
        0 => 0,
        1 => app.get_shortcuts_count(),
        _ => app.get_shortcuts_count() + app.get_commands_count(),
    };

    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(i, (key, desc))| {
            let global_index = offset + i;
            let style = if global_index == app.selected_index {
                Style::default()
                    .fg(color)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            let content = if global_index == app.selected_index {
                format!("► {} │ {}", key, desc)
            } else {
                format!("  {} │ {}", key, desc)
            };

            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .title_style(Style::default().fg(color).add_modifier(Modifier::BOLD))
                .border_style(Style::default().fg(color))
        );

    frame.render_widget(list, area);
}
