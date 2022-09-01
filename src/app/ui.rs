use log::warn;
use tui::{Frame, backend::Backend, layout::{Layout, Alignment, Direction, Constraint, Rect}, widgets::{Paragraph, Block, Borders, BorderType, Table, Row, Cell}, style::{Style, Color}, text::{Spans, Span}};
use tui_logger::TuiLoggerWidget;
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
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(12)
            ].as_ref())
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

    if app.is_loading {
        let body = draw_loading_body(app.is_loading(), app.state(), app.loading_message());
        rect.render_widget(body, body_chunks[0]);
    } else {
        draw_center(&body_chunks, app, rect);
    }


    let help = draw_help(app.actions());
    rect.render_widget(help, body_chunks[1]);

    let logs = draw_logs();
    rect.render_widget(logs, chunks[2]);
}

fn draw_center<B>(body_chunks: &Vec<Rect>, app: &App, rect: &mut Frame<B>) where B: Backend, {
    // let textarea = app.state().get_text_area().unwrap();

    /* let some_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(16), Constraint::Length(2)].as_ref())
        .split(body_chunks[0]);
    let input = textarea.widget();
    rect.render_widget(input, some_chunks[0]);*/
    let body = draw_body(app.state());
    rect.render_widget(body, body_chunks[0]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("⚽ Fifa Search")
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

fn draw_loading_body<'a>(loading: bool, state: &AppState, loading_message: &'a str) -> Paragraph<'a> {
    let initalized_text = if state.is_initialized() {
        format!("Initialized in {:.3} s", state.initialized_in().as_secs_f64())
    } else {
        "Not Initialized !".to_owned()
    };
    let loading_text = if loading { loading_message } else { "" };
    // let sleep_text = if let Some(sleeps) = state.count_sleep() {
    //     format!("Sleep count: {}", sleeps)
    // } else {
    //     String::default()
    // };
    // let tick_text = if let Some(tick) = state.count_tick() {
    //     format!("Tick: {}", tick)
    // } else {
    //     "".to_string()
    // };
    Paragraph::new(vec![
        Spans::from(Span::raw(initalized_text)),
        Spans::from(Span::raw(loading_text)),
        // Spans::from(Span::raw(sleep_text)),
        // Spans::from(Span::raw(tick_text)),
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

fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Logs")
                .border_style(Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
}

fn draw_body(state: &AppState) -> Table {
    if let Some((jogadores, tst, _, t)) = state.get_tables() {
        let mut rows = vec![];
        for (j,id) in tst.find_from_prefix("Jon".to_string()) {
            if let Some(jogador) = jogadores.get(&id) {
                let row = Row::new(vec![
                    Cell::from(jogador.name.to_string()),
                    Cell::from(jogador.player_positions.to_string()),
                    Cell::from(format!("{:.1}", jogador.rating)),
                ]);
                rows.push(row);
            }
        }
        Table::new(rows)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .title("Jogadores"),
            )
            .widths(&[Constraint::Min(30), Constraint::Min(20), Constraint::Min(10)])
    } else {
        // warn!("No tables");
        Table::new(vec![])
            .style(Style::default().bg(Color::Red))
    }
}