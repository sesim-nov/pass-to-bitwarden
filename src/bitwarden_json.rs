use crate::{bit_folder, bitwarden_entry};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct BitwardenJson {
    folders: Vec<bit_folder::BitwardenFolder>,
    items: Vec<bitwarden_entry::BitwardenEntry>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_serde() {
        let jerry = bit_folder::BitwardenFolder::new("Jerry");
        let jerry_id = jerry.get_id();
        let folders = vec![
            jerry,
            bit_folder::BitwardenFolder::new("Tom"),
        ];
        let items = vec![bitwarden_entry::BitwardenEntry::from_pass(
            String::from("Test pw"),
            String::from("bob"),
            String::from("abc123"),
            String::from(""),
            jerry_id)];
        let article = BitwardenJson{folders, items};
        println!("{:?}", serde_json::to_string_pretty(&article));
    }
}