use log::{debug, warn, error, info};

use crate::{inputs::key::Key, io::IoEvent};
use crate::hash_table::HashTable;
use crate::reading;
use std::time::Duration;

use self::{state::AppState, actions::Actions};

use crate::app::actions::Action;

pub mod state;
pub mod ui;
pub mod actions;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

/// The main application, containing the state
pub struct App {
    /// State
    state: AppState,
    actions: Actions,

    is_loading: bool,
    loading_message: String,
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
}

impl App {
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>) -> Self { // for now it could be replaced with impl Default 
        let actions = vec![Action::Quit].into();
        let is_loading = false;
        let state = AppState::default();
        Self {
            state,
            actions,
            is_loading,
            io_tx,
            loading_message: String::new(),
        }
    }

    pub async fn dispatch(&mut self, action: IoEvent) {
        // `is_loading` will be set to false again after the async action has finished in io/handler.rs
        self.is_loading = true;
        if let Err(e) = self.io_tx.send(action).await {
            self.is_loading = false;
            error!("Error from dispatch {}", e);
        };
    }

    /// Handle a user action
    pub async fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            debug!("Run action [{:?}]", action);
            match action {
                Action::Quit => AppReturn::Exit,
            }
        } else {
            warn!("No action accociated to {}", key);
            AppReturn::Continue
        }
    }

    pub async fn update_on_tick(&mut self) -> AppReturn {
        AppReturn::Continue
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }


    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn initialized(&mut self, jogadores: HashTable<u32, reading::JogadorComRating>, users: HashTable<u32, reading::User>, tags: HashTable<String, Vec<u32>>, timer: Duration) {
        // Update contextual actions
        self.state = AppState::initialized(jogadores, users, tags, timer);
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }

    pub fn set_loading_message(&mut self, message: String) {
        info!("...: {}", message);
        self.loading_message = message;
    }

    pub fn loading_message(&self) -> &str {
        &self.loading_message
    }

    pub fn is_capturing_input(&self) -> bool {
        self.state.is_capturing_input()
    }

    pub fn capture_input(&mut self, key: Key) -> AppReturn{
        self.state.capture_input(key);
        AppReturn::Continue
    }

}