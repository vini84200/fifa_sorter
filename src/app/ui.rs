use tui::{Frame, backend::Backend, layout::{Layout, Alignment, Direction, Constraint, Rect}, widgets::{Paragraph, Block, Borders, BorderType, Table, Row, Cell}, style::{Style, Color}, text::{Spans, Span}};

use crate::app::App;

use super::{state::AppState, actions::Actions};

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();

    check_size(&size);

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(size);

    // print!("{}", chunks.len());

    // Title block
    let title = draw_title();
    rect.render_widget(title, chunks[0]);

    // Body & Help
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(32)].as_ref())
        .split(chunks[1]);

    let body = draw_body(false, app.state());
    rect.render_widget(body, body_chunks[0]);

    let help = draw_help(app.actions());
    rect.render_widget(help, body_chunks[1]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Plop with TUI")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}



fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}

fn draw_body<'a>(loading: bool, state: &AppState) -> Paragraph<'a> {
    let loading_test = if loading { "Loading..." } else { "" };
    let tick_text = if let Some(tick) = state.count_tick() {
        format!("Tick: {}", tick)
    } else {
        "".to_string()
    };
    Paragraph::new(vec![
        Spans::from(loading_test),
        Spans::from(tick_text),
    ])
    .style(Style::default().fg(Color::White))
    .alignment(Alignment::Left)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain),
        )
}

fn draw_help(actions: &Actions) -> Table {
    let key_style = Style::default().fg(Color::LightCyan);
    let desc_style = Style::default().fg(Color::White);

    let mut rows = vec![];
    for action in actions.actions().iter() {
        let mut first = true;
        for key in action.keys() {
            let help = if first {
                first = false;
                action.to_string()
            } else {
                "".to_string()
            };
            let row = Row::new(vec![
                Cell::from(Span::styled(key.to_string(), key_style)),
                Cell::from(Span::styled(help, desc_style)),
            ]);

            rows.push(row);
        }
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Help"),
        )
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1)
}
