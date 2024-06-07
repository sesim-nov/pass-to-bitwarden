use crate::bit_folder;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct BitwardenJson {
    folders: Vec<bit_folder::BitwardenFolder>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_serde() {
        let folders = vec![
            bit_folder::BitwardenFolder::new("Jerry"),
            bit_folder::BitwardenFolder::new("Tom"),
        ];
        let article = BitwardenJson{folders};
        println!("{:?}", serde_json::to_string_pretty(&article));
    }
}