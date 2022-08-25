
use self::app::App;
use std::io::stdout;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use app::AppReturn;
use inputs::InputEvent;
use inputs::key::Key;
use tui::Terminal;
use tui::backend::CrosstermBackend;


use crate::app::ui;
use crate::inputs::events::Events;

pub mod reading;
pub mod hash_table;
pub mod app;
pub mod inputs;
pub mod io;


pub async fn start_ui(app: &Arc<tokio::sync::Mutex<App>>) -> Result<()> {
    // Configure Crossterm backend for tui
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let tick_rate = Duration::from_millis(200);
    let mut events = Events::new(tick_rate);

    {
        let mut app = app.lock().await;
        app.dispatch(io::IoEvent::Initialize).await;
    }

    loop {
        let mut app = app.lock().await;
        // Render
        terminal.draw(|rect| ui::draw(rect, &app))?;

        let result = match events.next().await {
            
            InputEvent::Input(key) => {
                if app.is_capturing_input(){
                    app.capture_input(key)
                } else {
                    app.do_action(key).await
                }
            },

            InputEvent::Tick => app.update_on_tick().await,
        };

        if result == AppReturn::Exit {
            events.close();
            break;
        }
    }

    // Restore the terminal and close application
    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}