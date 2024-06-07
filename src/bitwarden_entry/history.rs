use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct PasswordHistoryEntry {
    lastUsedDate: DateTime<Utc>,
    password: String,
}

impl PasswordHistoryEntry {
    pub fn new(password: &str, lastUsedDate: DateTime<Utc>) -> Self {
        Self{password: String::from(password), lastUsedDate}
    }
}