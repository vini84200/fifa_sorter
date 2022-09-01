use std::sync::Arc;

use anyhow::Result;
use crate::structures::hash_table::HashTable;
use crate::structures::tst::Tst;

use log::{error, info};

use super::IoEvent;
use crate::{app::App, reading};

/// In the IO thread, we handle IO event without blocking the UI thread
pub struct IoAsyncHandler {
    app: Arc<tokio::sync::Mutex<App>>,
}

impl IoAsyncHandler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }

    /// We could be async here
    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
        };

        if let Err(err) = result {
            error!("Oops, something wrong happen: {:?}", err);
        }

        let mut app = self.app.lock().await;
        app.loaded();
    }

    /// We use dummy implementation here, just wait 1s
    async fn do_initialize(&mut self) -> Result<()> {
        info!("🚀 Initialize the application");
        {
            let mut app = self.app.lock().await;
            app.set_loading_message("Loading...".to_string());

            // Read csv files
            app.set_loading_message("Reading csv files...".to_string());
        }
        let start = std::time::Instant::now();
        let mut jogadores = HashTable::new(22807);
        //TODO: Size of Hash Table is not correct:: 28 800 001
        let mut users : HashTable<u32, reading::User> = HashTable::new(200001);
        let mut tags : HashTable<String, Vec<u32>> = HashTable::new(438001);

        let mut jogadores_tst: Tst<u32> = Tst::new();

        reading::read_jogadores(&mut jogadores, &mut jogadores_tst).await?;
        reading::read_rating(&mut users, &mut jogadores).await?;
        reading::read_tags(&mut jogadores, &mut tags).await?;


        {
            let mut app = self.app.lock().await;
            app.set_loading_message("Initializing...".to_string());
            let timer = start.elapsed();
            app.initialized(jogadores, jogadores_tst ,users, tags, timer);
            info!("👍 Application initialized in {:?}", timer);
        }


        Ok(())
    }

}