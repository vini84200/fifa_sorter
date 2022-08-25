use log::{debug, warn, error};

use crate::{inputs::key::Key, io::IoEvent};

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
                Action::Sleep => {
                    if let Some(duration) = self.state.duration().cloned() {
                        self.dispatch(IoEvent::Sleep(duration)).await;
                    };
                    AppReturn::Continue
                }
            }
        } else {
            warn!("No action accociated to {}", key);
            AppReturn::Continue
        }
    }

    /// We could update the app or dispatch event on tick
    pub async fn update_on_tick(&mut self) -> AppReturn {
        // here we just increment a counter
        self.state.incr_tick();
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

    pub fn initialized(&mut self) {
        // Update contextual actions
        self.actions = vec![Action::Quit, Action::Sleep].into();
        self.state = AppState::initialized()
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }

    pub fn sleeped(&mut self) {
        self.state.incr_sleep();
    }

}