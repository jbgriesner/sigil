use std::path::PathBuf;
use std::time::Instant;

use uuid::Uuid;

use valt_core::{Secret, VaultManager};

/// Fields of a secret being added or edited.
#[derive(Debug, Clone)]
pub struct SecretDraft {
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub tags: String, // comma-separated
    pub notes: String,
}

impl SecretDraft {
    pub fn empty() -> Self {
        Self {
            name: String::new(),
            username: String::new(),
            password: String::new(),
            url: String::new(),
            tags: String::new(),
            notes: String::new(),
        }
    }

    pub fn from_secret(s: &Secret) -> Self {
        Self {
            name: s.name.clone(),
            username: s.username.clone().unwrap_or_default(),
            password: s.password.clone(),
            url: s.url.clone().unwrap_or_default(),
            tags: s.tags.join(", "),
            notes: s.notes.clone().unwrap_or_default(),
        }
    }

    /// Returns an error message if the draft is not valid, or `None` if OK.
    pub fn validate(&self) -> Option<&'static str> {
        if self.name.trim().is_empty() {
            return Some("Name is required");
        }
        if self.password.is_empty() {
            return Some("Password is required");
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormMode {
    Add,
    Edit(Uuid),
}

#[derive(Debug)]
pub enum AppView {
    Locked {
        input: String,
        error: Option<String>,
    },
    List {
        search_query: String,
        selected_idx: usize,
    },
    Detail {
        secret_id: Uuid,
        show_password: bool,
    },
    Form {
        mode: FormMode,
        draft: SecretDraft,
        focused_field: usize,
        show_password: bool,
        error: Option<String>,
    },
    Help,
}

pub struct AppState {
    pub view: AppView,
    pub vault: Option<VaultManager>,
    pub vault_path: PathBuf,
    /// When set, the clipboard will be cleared at this instant.
    pub clipboard_clear_at: Option<Instant>,
    pub should_quit: bool,
    /// Transient status message shown in the list status bar.
    pub status: Option<String>,
}

impl AppState {
    pub fn new(vault_path: PathBuf) -> Self {
        Self {
            view: AppView::Locked {
                input: String::new(),
                error: None,
            },
            vault: None,
            vault_path,
            clipboard_clear_at: None,
            should_quit: false,
            status: None,
        }
    }

    pub fn vault_exists(&self) -> bool {
        self.vault_path.exists()
    }

    pub fn clipboard_secs_remaining(&self) -> Option<u32> {
        self.clipboard_clear_at.map(|deadline| {
            let now = Instant::now();
            if deadline > now {
                (deadline - now).as_secs() as u32
            } else {
                0
            }
        })
    }

    pub fn go_to_list(&mut self) {
        self.view = AppView::List {
            search_query: String::new(),
            selected_idx: 0,
        };
    }
}
