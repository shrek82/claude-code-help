use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use crate::app::App;

const SECTION_TITLES: [&str; 3] = ["快捷键", "斜杠命令", "CLI 参考"];

const SECTION_COLORS: [Color; 3] = [
    Color::Rgb(0, 135, 255),   // 蓝色 - 快捷键
    Color::Rgb(0, 200, 100),   // 绿色 - 斜杠命令
    Color::Rgb(255, 140, 0),   // 橙色 - CLI 参考
];

pub fn render_tabs(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    // 垂直分为：标题 + 三列内容
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 顶部标题
            Constraint::Min(0),    // 三列内容
        ])
        .split(area);

    // 渲染顶部标题
    render_header(frame, chunks[0]);

    // 水平分为三列
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(chunks[1]);

    // 渲染三个分区
    for (i, title) in SECTION_TITLES.iter().enumerate() {
        render_section(
            frame,
            app,
            columns[i],
            i,
            title,
            SECTION_COLORS[i],
            app.current_section == i,
        );
    }
}

fn render_header(frame: &mut Frame, area: ratatui::layout::Rect) {
    let date = chrono::Local::now().format("%Y年%m月%d日").to_string();
    let title = format!(" Claude Code 功能速查   {} ", date);

    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            title,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .alignment(ratatui::layout::Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    );

    frame.render_widget(header, area);
}

fn render_section(
    frame: &mut Frame,
    app: &App,
    area: ratatui::layout::Rect,
    section_index: usize,
    title: &str,
    color: Color,
    is_active: bool,
) {
    let entries = app.get_section_entries(section_index);

    // 计算全局索引偏移（用于搜索高亮）
    let _offset = match section_index {
        0 => 0,
        1 => app.get_shortcuts_count(),
        _ => app.get_shortcuts_count() + app.get_cli_commands_count(),
    };

    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(i, (key, desc))| {
            let local_index = i;
            let is_selected = is_active && local_index == app.selected_index_in_section;

            let style = if is_selected {
                Style::default()
                    .fg(color)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            let marker = if is_selected { "► " } else { "  " };
            let content = format!("{}{} │ {}", marker, key, desc);

            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let border_style = if is_active {
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(color)
    };

    let title_style = if is_active {
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(color)
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} [{}] ", title, if is_active { "●" } else { "○" }))
                .title_style(title_style)
                .border_style(border_style),
        );

    frame.render_widget(list, area);
}
