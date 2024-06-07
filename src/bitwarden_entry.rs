use uuid::Uuid;
use serde::Serialize;
use chrono::{DateTime, Utc};

mod history;
mod login;

#[derive(Serialize, Debug)]
pub struct BitwardenEntry {
    name: String,
    id: Uuid,
    passwordHistory: Vec<history::PasswordHistoryEntry>,
    revisionDate: DateTime<Utc>,
    creationDate: DateTime<Utc>,
    deletedDate: Option<DateTime<Utc>>,
    organizationId: Option<Uuid>,
    folderId: Uuid,
    #[serde(rename = "type")]
    entry_type: u8,
    reprompt: u8,
    notes: String,
    collectionIds: Option<Vec<Uuid>>,
    login: login::Login 
}

impl BitwardenEntry {
    pub fn from_pass(name: String, username: String, password: String, totp: String, folderId: Uuid) -> Self {
        Self { name, folderId, login: login::Login::from_pass(username, password, totp), .. Default::default() }
    }
}

impl Default for BitwardenEntry{
    fn default() -> Self{
        Self { 
            name: String::from("DEFAULT"),
            id: Uuid::new_v4(),
            passwordHistory: Vec::new(),
            revisionDate: Utc::now(),
            creationDate: Utc::now(),
            deletedDate: None,
            organizationId: None,
            folderId: Uuid::nil(),
            entry_type: 1,
            reprompt: 0,
            notes: String::new(),
            collectionIds: None,
            login: Default::default(), 
        }
    }
}