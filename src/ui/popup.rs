use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, List, ListItem},
};
use crate::app::App;

pub fn render_search_popup(frame: &mut Frame, app: &mut App) {
    // 渲染结果列表
    let entries = app.get_all_entries_for_search();
    let mut matching_items: Vec<(usize, usize, String, String)> = Vec::new();

    for (section_idx, local_idx, key, desc) in entries.iter() {
        if key.contains(&app.search_query) || desc.contains(&app.search_query) {
            matching_items.push((*section_idx, *local_idx, key.clone(), desc.clone()));
        }
    }

    // 根据搜索结果数量动态计算弹窗高度
    // 输入框 3 行 + 上下边框 2 行 = 5 行基础高度
    // 结果列表：每个结果 1 行，最多显示 10 条
    let result_count = if app.search_query.is_empty() || matching_items.is_empty() {
        2  // 无结果时显示 2 行提示
    } else {
        matching_items.len().min(10)
    };
    let total_height_rows = 5 + result_count;

    // 计算最小高度百分比（基于屏幕高度）
    let screen_height = frame.area().height;
    let height_percent = ((total_height_rows as f32 / screen_height as f32) * 100.0).ceil() as u16;
    // 限制最小 20%，最大 80%
    let height_percent = height_percent.max(20).min(80);

    let area = centered_rect(50, height_percent, frame.area());

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

    // 渲染结果列表区域 - 始终显示，即使没有输入关键字
    let items: Vec<ListItem> = if app.search_query.is_empty() {
        // 未输入关键字时显示提示
        vec![
            ListItem::new(Line::from(Span::styled(
                "输入关键词搜索命令和快捷键",
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
            ))),
            ListItem::new(Line::from(Span::styled(
                "示例：复制、git、会话、粘贴",
                Style::default().fg(Color::DarkGray),
            ))),
        ]
    } else if matching_items.is_empty() {
        vec![
            ListItem::new(Line::from(Span::styled(
                "无匹配结果",
                Style::default().fg(Color::DarkGray),
            ))),
            ListItem::new(Line::from(Span::styled(
                "提示：尝试其他关键词，如 \"复制\"、\"git\"、\"会话\"",
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
            ))),
        ]
    } else {
        matching_items
            .iter()
            .enumerate()
            .map(|(idx, (section_idx, _local_idx, key, desc))| {
                let section_name = match section_idx {
                    0 => "CLI 参考",
                    1 => "内置命令",
                    _ => "快捷键",
                };

                // 构建高亮行：[分区] key - desc
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

                let mut item = ListItem::new(Line::from(spans));

                // 高亮当前选中的搜索结果
                if idx == app.search_selected_index {
                    item = item.style(Style::default().bg(Color::DarkGray).fg(Color::White).add_modifier(Modifier::BOLD));
                }

                item
            })
            .collect()
    };

    // 根据状态设置边框样式和标题
    let (border_style, title_text) = if app.search_query.is_empty() {
        (Style::default().fg(Color::DarkGray), " 输入关键词 ".to_string())
    } else if matching_items.is_empty() {
        (Style::default().fg(Color::Yellow), " 无匹配结果 ".to_string())
    } else {
        (Style::default().fg(Color::Green), format!(" 搜索结果 ({} 项) ", matching_items.len()))
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(title_text)
                .style(border_style),
        );

    frame.render_widget(list, chunks[1]);
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
