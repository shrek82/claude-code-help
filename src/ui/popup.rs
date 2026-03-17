use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, List, ListItem},
};
use crate::app::App;

pub fn render_search_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(50, 40, frame.area());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 输入框
            Constraint::Min(0),    // 结果列表
        ])
        .split(area);

    // 渲染输入框 - 使用圆角边框
    let input = Paragraph::new(app.search_query.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(" 搜索 (Esc 关闭) ")
                .style(Style::default().fg(Color::Yellow)),
        );

    frame.render_widget(input, chunks[0]);

    // 渲染结果列表
    let entries = app.get_all_entries_for_search();
    let mut matching_items: Vec<(usize, usize, String, String)> = Vec::new();

    for (section_idx, local_idx, key, desc) in entries.iter() {
        if key.contains(&app.search_query) || desc.contains(&app.search_query) {
            matching_items.push((*section_idx, *local_idx, key.clone(), desc.clone()));
        }
    }

    if !app.search_query.is_empty() {
        let items: Vec<ListItem> = if matching_items.is_empty() {
            vec![ListItem::new("无匹配结果")]
        } else {
            matching_items
                .iter()
                .map(|(section_idx, _local_idx, key, desc)| {
                    let section_name = match section_idx {
                        0 => "快捷键",
                        1 => "斜杠命令",
                        _ => "CLI",
                    };

                    // 高亮匹配的词
                    let match_style = if app.search_query.is_empty() {
                        Style::default()
                    } else {
                        // 检查是 key 还是 desc 匹配
                        if key.to_lowercase().contains(&app.search_query.to_lowercase()) {
                            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        }
                    };

                    let content = format!("[{}] {} - {}", section_name, key, desc);
                    ListItem::new(Line::from(Span::styled(content, match_style)))
                })
                .collect()
        };

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title(format!(" 搜索结果 ({} 项) ", matching_items.len()))
                    .style(Style::default().fg(Color::Green)),
            );

        frame.render_widget(list, chunks[1]);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
