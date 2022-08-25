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

    // // Timer
    // let start_total = std::time::Instant::now();

    // let mut jogadores = HashTable::new(22807);
    // let mut users : HashTable<u32, reading::User> = HashTable::new(28800001);
    // let mut tags : HashTable<String, Vec<u32>> = HashTable::new(438001);

    // reading::read_jogadores(&mut jogadores)?;
    // reading::read_rating(users, &mut jogadores)?;
    // reading::read_tags(&mut jogadores, &mut tags)?;


    // let ellapsed = start_total.elapsed();
    // println!("Total: {:?}", ellapsed);


    Ok(())
}