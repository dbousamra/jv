use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Paragraph,
    },
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let container_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100), Constraint::Min(1)])
        .split(frame.size());

    let container = container_layout[0];
    let footer = container_layout[1];

    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Percentage(50)].as_ref())
        .split(container);

    let sidebar = outer_layout[0];
    let content = outer_layout[1];

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(content);

    let query_editor = inner_layout[0];
    let query_results = inner_layout[1];

    frame.render_widget(Block::default(), container);

    let line = Line::from(vec![
        Span::styled("Q", Style::default().fg(Color::LightBlue)),
        Span::raw(" "),
        Span::styled("Quit", Style::default().fg(Color::Green)),
        Span::raw(" "),
        Span::styled("F1", Style::default().fg(Color::LightBlue)),
        Span::raw(" "),
        Span::styled("Help", Style::default().fg(Color::Green)),
    ]);

    frame.render_widget(Paragraph::new(line).block(Block::default()), footer);

    frame.render_widget(
        Paragraph::new("Hello World!").block(
            Block::default()
                .title("Schema")
                .title_style(Style::default().fg(Color::LightGreen))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        ),
        sidebar,
    );
    frame.render_widget(
        Paragraph::new("Hello World!").block(
            Block::default()
                .title("Query editor")
                .title_style(Style::default().fg(Color::LightGreen))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        ),
        query_editor,
    );

    frame.render_widget(
        Paragraph::new("Hello World!").block(
            Block::default()
                .title("Query results")
                .title_style(Style::default().fg(Color::LightGreen))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        ),
        query_results,
    );
}
