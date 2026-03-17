use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use crate::app::{App, InputMode};

pub fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let mode_text = match app.input_mode {
        InputMode::Normal => "正常模式",
        InputMode::Searching => "搜索模式",
    };

    let section_name = match app.current_section {
        0 => "快捷键",
        1 => "斜杠命令",
        2 => "CLI 参考",
        _ => "",
    };

    let shortcuts = if app.input_mode == InputMode::Searching {
        "Esc:关闭 Enter:确认"
    } else {
        "TAB:切换列 ↑/↓/j/k:导航 F:搜索 C:复制 Q/Esc:退出"
    };

    // 复制反馈消息（如果有）
    let feedback = match &app.copy_feedback {
        Some(msg) => format!(" ● {} ", msg),
        None => String::new(),
    };

    let content = Line::from(vec![
        Span::styled(
            format!(" {} ", mode_text),
            Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            format!("[{}] ", section_name),
            Style::default().fg(Color::Cyan),
        ),
        Span::raw(format!(
            "条目 {}/{} ",
            app.selected_index_in_section + 1,
            app.get_section_count(app.current_section)
        )),
        Span::styled(
            format!(" {} ", shortcuts),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(
            feedback,
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ),
    ]);

    let status_bar = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .style(Style::default().bg(Color::Black));

    frame.render_widget(status_bar, area);
}
