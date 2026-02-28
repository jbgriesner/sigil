use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A single secret entry stored in the vault.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Secret {
    pub id: Uuid,
    pub name: String,
    pub username: Option<String>,
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Secret {
    /// Create a new secret with a generated UUID and current timestamps.
    /// Optional fields default to `None` / empty — set them directly after construction.
    pub fn new(name: impl Into<String>, password: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            username: None,
            password: password.into(),
            url: None,
            notes: None,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Update `updated_at` to now. Call this before persisting a modified secret.
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}
