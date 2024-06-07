use uuid::Uuid;
use serde::Serialize;
use chrono::{DateTime, Utc};

mod history;

#[derive(Serialize)]
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
    login: u8 //TODO: Make login object per spec. 
}