use log::{debug, warn};

use crate::inputs::key::Key;

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
}

impl App {
    pub fn new() -> Self { // for now it could be replaced with impl Default 
        let actions = vec![Action::Quit].into();
        let state = AppState::initialized();
        Self { actions, state }
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }
    /// Handle a user action
    pub fn do_action(&mut self, key: Key) -> AppReturn {
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

    /// We could update the app or dispatch event on tick
    pub fn update_on_tick(&mut self) -> AppReturn {
        // here we just increment a counter
        self.state.incr_tick();
        AppReturn::Continue
    }
    pub fn actions(&self) -> &Actions {
        &self.actions
    }

}