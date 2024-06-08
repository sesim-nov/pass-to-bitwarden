use serde::Serialize;
use uuid::Uuid;


#[derive(Serialize, Debug)]
pub struct Login {
    pub(crate) uris: Vec<Uri>,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) totp: String, 
    pub(crate) collectionIds: Option<Vec<Uuid>>,
}

impl Login{
    pub fn from_pass(username: String, password: String, totp: String) -> Self {
        Self { username, password, totp, .. Default::default() }
    }
}

impl Default for Login {
    fn default() -> Self {
        Self { 
            uris: Vec::new(), 
            username: String::new(), 
            password: String::new(), 
            totp: String::new(), 
            collectionIds: None }
    }
}

#[derive(Serialize, Debug)]
pub struct Uri {
    #[serde(rename="match")]
    match_thing: Option<String>,
    uri: String,
}