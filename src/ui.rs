use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Frame,
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
        .split(f.size());

    let datasets = vec![
        Dataset::default()
            .name("Entropy")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(&app.entropy_data),
    ];

    let x_labels = vec![
        Span::raw(format!("{:.1}", app.window_start)),
        Span::raw(format!("{:.1}", app.window_start + app.window_width / 2.0)),
        Span::raw(format!("{:.1}", app.window_start + app.window_width)),
    ];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "Shannon Entropy",
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("File Offset")
                .style(Style::default().fg(Color::Gray))
                .bounds([app.window_start, app.window_start + app.window_width])
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .title("Entropy")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 8.0])
                .labels(vec![
                    Span::raw("0.0"),
                    Span::raw("4.0"),
                    Span::raw("8.0"),
                ]),
        );

    f.render_widget(chart, chunks[0]);
    
    let instructions = Span::raw("Commands: [-/+] Zoom | [Arrows] Scroll | [q] Quit");
    f.render_widget(
        Block::default().borders(Borders::NONE).title(""), 
        chunks[1]
    );
    // Actually let's just put text in the bottom chunk
    use ratatui::widgets::Paragraph;
    let p = Paragraph::new(instructions).style(Style::default().fg(Color::White));
    f.render_widget(p, chunks[1]);
}
