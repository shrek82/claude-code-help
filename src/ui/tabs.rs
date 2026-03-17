use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Tabs},
};
use crate::app::App;

const TAB_NAMES: [&str; 3] = ["快捷键", "斜杠命令", "自定义"];
const TAB_COLORS: [Color; 3] = [
    Color::Rgb(0, 135, 255),   // 蓝色
    Color::Rgb(0, 200, 100),   // 绿色
    Color::Rgb(150, 100, 255), // 紫色
];

pub fn render_tabs(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tab 标题
            Constraint::Min(0),    // 内容
        ])
        .split(area);

    // 渲染 Tab 标题
    let titles: Vec<Line> = TAB_NAMES
        .iter()
        .enumerate()
        .map(|(i, &t)| {
            let color = if i == app.current_tab {
                TAB_COLORS[i]
            } else {
                Color::Gray
            };
            Line::from(Span::styled(
                t,
                Style::default().fg(color),
            ))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Claude Code Cheatsheet"))
        .select(app.current_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(TAB_COLORS[app.current_tab])
        );

    frame.render_widget(tabs, chunks[0]);

    // 渲染内容
    render_content(frame, app, chunks[1]);
}

fn render_content(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let entries = app.get_current_entries();
    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(i, (key, desc))| {
            let style = if i == app.selected_index {
                Style::default()
                    .fg(TAB_COLORS[app.current_tab])
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let content = if i == app.selected_index {
                format!("► {} │ {}", key, desc)
            } else {
                format!("  {} │ {}", key, desc)
            };

            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(TAB_NAMES[app.current_tab]));

    frame.render_widget(list, area);
}
