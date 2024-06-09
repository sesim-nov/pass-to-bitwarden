use crate::{bit_folder::BitwardenFolder, 
    bitwarden_entry::BitwardenEntry};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct BitwardenJson {
    folders: Vec<BitwardenFolder>,
    items: Vec<BitwardenEntry>,
}

impl BitwardenJson {
    pub fn with_entries(folders: Vec<BitwardenFolder>, items: Vec<BitwardenEntry>) -> Self {
        Self { folders, items }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_serde() {
        let jerry = BitwardenFolder::new("Jerry");
        let jerry_id = jerry.get_id();
        let folders = vec![
            jerry,
            BitwardenFolder::new("Tom"),
        ];
        let items = vec![BitwardenEntry::from_pass(
            String::from("Test pw"),
            String::from("bob"),
            String::from("abc123"),
            String::from(""),
            jerry_id,
            String::from(""),
        )];
        let article = BitwardenJson{folders, items};
        println!("{:?}", serde_json::to_string_pretty(&article));
    }
}