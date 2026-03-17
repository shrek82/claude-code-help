use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, List, ListItem},
};
use crate::app::App;

pub fn render_search_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(50, 40, frame.area());

    // 清除弹窗区域背景
    frame.render_widget(Clear, area);

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

                    // 构建高亮行：[分区] key - desc
                    // 高亮匹配的文字部分
                    let mut spans = vec![
                        Span::styled(format!("[{}] ", section_name), Style::default().fg(Color::DarkGray)),
                    ];

                    // 高亮 key 中的匹配
                    let key_spans = highlight_text(key, &app.search_query, Color::Cyan);
                    spans.extend(key_spans);

                    spans.push(Span::raw(" - "));

                    // 高亮 desc 中的匹配
                    let desc_spans = highlight_text(desc, &app.search_query, Color::Green);
                    spans.extend(desc_spans);

                    ListItem::new(Line::from(spans))
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

/// 高亮文本中匹配的部分
fn highlight_text<'a>(text: &'a str, query: &'a str, highlight_color: Color) -> Vec<Span<'a>> {
    let mut spans = Vec::new();
    let query_lower = query.to_lowercase();
    let text_lower = text.to_lowercase();

    let mut start = 0;
    let mut matches: Vec<(usize, usize)> = Vec::new();

    // 查找所有匹配位置
    let mut search_start = 0;
    while let Some(pos) = text_lower[search_start..].find(&query_lower) {
        let abs_pos = search_start + pos;
        matches.push((abs_pos, abs_pos + query.len()));
        search_start = abs_pos + query.len();
    }

    if matches.is_empty() {
        return vec![Span::raw(text)];
    }

    // 构建 spans
    for (i, (match_start, match_end)) in matches.iter().enumerate() {
        // 添加匹配前的普通文本
        if *match_start > start {
            spans.push(Span::raw(&text[start..*match_start]));
        }
        // 添加高亮的匹配文本
        spans.push(Span::styled(
            &text[*match_start..*match_end],
            Style::default().fg(highlight_color).add_modifier(Modifier::BOLD),
        ));
        start = *match_end;

        // 如果是最后一个匹配，添加剩余文本
        if i == matches.len() - 1 && *match_end < text.len() {
            spans.push(Span::raw(&text[*match_end..]));
        }
    }

    spans
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
