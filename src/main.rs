mod hash_table;
mod reading;


use fifa_sorter::app::App;
use anyhow::Result;
use fifa_sorter::start_ui;
use fifa_sorter::io::handler::IoAsyncHandler;
use fifa_sorter::io::IoEvent;
use log::LevelFilter;
use std::{sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {

    let (sync_io_tx, mut sync_io_rx) = tokio::sync::mpsc::channel::<IoEvent>(100);

    let app = Arc::new(tokio::sync::Mutex::new(App::new(sync_io_tx.clone())));
    let app_ui = Arc::clone(&app);

    // Configue log
    tui_logger::init_logger(LevelFilter::Debug).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Debug);

    tokio::spawn(async move {
        let mut handler = IoAsyncHandler::new(app);
        while let Some(io_event) = sync_io_rx.recv().await {
            handler.handle_io_event(io_event).await;
        }
    });

    start_ui(&app_ui).await?;

    Ok(())
}