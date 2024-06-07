use uuid::Uuid;
use serde::Serialize;

pub fn test_mod() {
    println!("Test mod");
}

#[derive(Debug, Serialize)]
pub struct BitwardenFolder {
    name: String,
    id: Uuid,
}

impl BitwardenFolder {
    pub fn new(folder_name: &str) -> Self {
        let name = String::from(folder_name);
        let folder_uuid = Uuid::new_v4();
        Self { name, id: folder_uuid }
    }

    pub fn get_id(&self) -> Uuid {
        self.id.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_instance() {
        let name = "Test Name";
        let folder = BitwardenFolder::new(name);
        assert_eq!(folder.name, name);
        println!("{:?}", serde_json::to_string(&folder).unwrap_or(String::from("LOL SERDE FUCKED IT")));
    }
}