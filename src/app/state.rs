
use std::time::Duration;
use log::info;
use tui_textarea::TextArea;

use crate::hash_table::HashTable;
use crate::inputs::key::Key;
use crate::reading;

#[derive(Debug, Clone, PartialEq)]
pub enum InputState {
    Normal,
    Focused
}

#[derive(Clone)]
pub enum AppPage<'a> {
    Home(InputState, TextArea<'a>),
    Loading,
    Error,
}

#[derive(Clone)]
pub enum AppState{
    Init,
    Initializing {
        progress: f32,
        message: String,
        start: std::time::Instant,
    },
    Initialized {
        jogadores: HashTable<u32, reading::JogadorComRating>,
        users: HashTable<u32, reading::User>,
        tags: HashTable<String, Vec<u32>>,
        duration: Duration,
        page: AppPage<'static>,
    },
}

impl AppState {
    pub fn initialized(jogadores: HashTable<u32, reading::JogadorComRating>, users: HashTable<u32, reading::User>, tags: HashTable<String, Vec<u32>>, timer: Duration) -> Self {
        Self::Initialized {
            jogadores,
            users,
            tags,
            duration: timer,
            page: AppPage::Home(InputState::Focused, TextArea::default())
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn initialized_in (&self) -> Duration {
        match self {
            Self::Initialized { duration, .. } => *duration,
            _ => Duration::from_secs(0),
        }
    }

    pub fn get_tables(&self) -> Option<(&HashTable<u32, reading::JogadorComRating>, &HashTable<u32, reading::User>, &HashTable<String, Vec<u32>>)> {
        match self {
            Self::Initialized
                {
                    jogadores,
                    users,
                    tags,
                    ..
                } => Some((jogadores, users, tags)),
            _ => None,
        }
    }

    pub fn is_capturing_input(&self) -> bool {
        match self {
            Self::Initialized { page, .. } => match page {
                AppPage::Home(a, ..) => matches!(a, InputState::Focused),
                _ => false,
            },
            _ => false,
        }
    }

    pub fn capture_input(&mut self, key: Key) {
        info!("Capture input {:?}", key);
        match self {
            Self::Initialized { page, .. } => match page {
                AppPage::Home(state, input) => {
                    match key {
                        Key::Esc => {
                            *state = InputState::Normal;
                        },
                        Key::Enter => {
                            *state = InputState::Normal;
                            let text = input.lines().join("");
                            println!("Text: {}", text);
                        },
                        c => {
                            if *state == InputState::Focused {
                                input.input(c);
                            }
                        }
                    }
                },
                _ => {}
            },
            _ => {}
        }
    }

    pub fn get_text_area(&self) -> Option<&TextArea> {
        match self {
            Self::Initialized { page, .. } => match page {
                AppPage::Home(_, input) => Some(input),
                _ => None,
            },
            _ => None,
        }
    }

}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}