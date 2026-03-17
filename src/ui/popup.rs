use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph},
};
use crate::app::App;

pub fn render_search_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, frame.area());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 输入框
            Constraint::Min(0),    // 结果
        ])
        .split(area);

    // 渲染输入框
    let input = Paragraph::new(app.search_query.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("搜索 (/ 关闭)")
                .style(Style::default().fg(Color::Yellow)),
        )
        .style(Style::default().add_modifier(Modifier::REVERSED));

    frame.render_widget(input, chunks[0]);

    // 渲染结果
    if !app.search_query.is_empty() {
        let results = if app.search_results.is_empty() {
            "无匹配结果"
        } else {
            &format!("找到 {} 个匹配", app.search_results.len())
        };

        let result_text = Paragraph::new(results.to_string())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("搜索结果"),
            );

        frame.render_widget(result_text, chunks[1]);
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
