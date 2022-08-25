
use std::time::Duration;
use crate::hash_table::HashTable;
use crate::reading;

#[derive(Debug, Clone, Default)]
pub enum AppPage {
    #[default]
    Home,
    Loading,
    Error,
}

#[derive(Clone)]
pub enum AppState {
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
        page: AppPage
    },
}

impl AppState {
    pub fn initialized(jogadores: HashTable<u32, reading::JogadorComRating>, users: HashTable<u32, reading::User>, tags: HashTable<String, Vec<u32>>, timer: Duration) -> Self {
        Self::Initialized {
            jogadores,
            users,
            tags,
            duration: timer,
            page: AppPage::Home
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

    pub fn getTables(&self) -> Option<(&HashTable<u32, reading::JogadorComRating>, &HashTable<u32, reading::User>, &HashTable<String, Vec<u32>>)> {
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

}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}