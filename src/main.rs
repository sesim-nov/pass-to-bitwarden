/*
 *   pass-to-bitwarden: Export unix password manager data to bitwarden. 
 *   Copyright (C) 2024 "Sesim"

 *   This program is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 3 of the License, or
 *   (at your option) any later version.

 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details.

 *   You should have received a copy of the GNU General Public License
 *   along with this program.  If not, see <https://www.gnu.org/licenses/>.
 * */

use regex::Regex;
use regex_static;
use serde::Serialize;
use std::io::{BufRead, BufReader, Read};
use std::fs::{File};
use uuid::Uuid;
use walkdir::WalkDir;

mod bit_folder;
mod bitwarden_json;
mod bitwarden_entry;

fn main() {
    let fsroot = std::env::args().nth(1).expect("Expected arg for root of pass dir.");
    let mut dir = WalkDir::new(fsroot).into_iter();
    let folder_name = dir
        .next()
        .unwrap()
        .unwrap()
        .into_path()
        .to_str()
        .unwrap()
        .split("/")
        .last()
        .unwrap()
        .to_string();
    //println!("Folder name: {}", folder_name);
    let folder = bit_folder::BitwardenFolder::new(&folder_name);
    let folder_id = folder.get_id();
    let folders = vec![folder];

    let mut items = Vec::new();
    for item in dir{
        let item = item.unwrap();
        if item.file_type().is_file() {
            let entry_name = String::from(item.file_name().to_str().unwrap());
            let mut buf = String::new();
            //println!("{:?}", entry_name);
            let mut file = File::open(item.path()).expect("Error opening file");
            let fstring = file.read_to_string(&mut buf);
            let new_entry = process_entry(&entry_name, folder_id, &buf);
            items.push(new_entry);
        }
    }

    let json = bitwarden_json::BitwardenJson::with_entries(folders, items);
    let pretty_text = serde_json::to_string_pretty(&json).unwrap();
    println!("{}", pretty_text)
}

fn get_totp(input: &str) -> Option<String> {
    let reg: &Regex = regex_static::static_regex!("(totp|otpauth)://");
    let needle = reg.find(input)?.start();
    let modified_string: &str = &input[needle..];
    Some(String::from(modified_string.split_whitespace().next()?))
}

fn get_username(input: &str) -> Option<String> {
    let reg = regex_static::static_regex!("[Uu]sername:");
    let needle = reg.find(input)?.end();
    let modified_string = &input[needle..];
    Some(String::from(modified_string.split('\n').next()?))
}

fn process_entry(name: &str, folder_id: Uuid, inp: &str) -> bitwarden_entry::BitwardenEntry {
    let buf = BufReader::new(inp.as_bytes());
    let mut line_buf = buf.lines();
    let pass = line_buf
        .next()
        .expect("There should be at least one line...")
        .expect("There should also be a string to open...");
    let username = match get_username(inp){
        None => String::from("No-username"),
        Some(x) => {
            let _ = line_buf.next();
            x
        },
    };
    let totp = match get_totp(inp){
        None => String::from(""),
        Some(x) => x,
    };
    let notes: String = line_buf.map(|c| c.unwrap()).collect::<Vec<String>>().join("\n");
    bitwarden_entry::BitwardenEntry::from_pass(
        String::from(name), 
        username, 
        pass, 
        totp, 
        folder_id,
        notes
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totp_works() {
        let hay = "this is an OTP token totp://blah hooray";
        let stack = "This is not a totp token";
        assert_eq!(get_totp(hay), Some("totp://blah".to_string()));
        assert_eq!(get_totp(stack), None);
    }

    #[test]
    fn username_works(){
        let hay = "username: Bob Dylan";
        let hay2 = "Username: Bob Dylan";
        let stack = "usernam: lol";
        assert_eq!(get_username(hay), Some(String::from(" Bob Dylan")));
        assert_eq!(get_username(hay2), Some(String::from(" Bob Dylan")));
        assert_eq!(get_username(stack), None);
    }

    #[test]
    fn conversion_workflow_works() {
        let folder = bit_folder::BitwardenFolder::new("Test Folder 001");
        let folder_id = folder.get_id();
        let test_name = "TestName";
        let test_entry = String::from("testpass\nUsername:Test Username\ntotp://test-totp\nextra_notes");
        let test_converted: bitwarden_entry::BitwardenEntry = process_entry(test_name, folder_id, &test_entry);
        assert_eq!(test_converted.login.username, "Test Username");
    }

    struct SimulatedEntry{
        pub name: String,
        pub entry: String,
    }

    #[test]
    fn full_bitwarden_json_works() {
        //Arrange
        let entry_1 = SimulatedEntry{
            name: String::from("Test 1"),
            entry: String::from("testpass01#@$\nUsername:Test Uaer\n"),
        };
        let entry_2 = SimulatedEntry{
            name: String::from("Test 2"),
            entry: String::from("testpw02*(&$#$%#$%&^**%@##$!#@$@$^%%&\nUsername:Derp\ntotp://derpyderp"),
        };
        let entries = vec![entry_1, entry_2];
        let folder = bit_folder::BitwardenFolder::new("folder_name");
        let folder_id = folder.get_id();

        //Act
        let converted_entries: Vec<bitwarden_entry::BitwardenEntry> = entries.into_iter()
            .map(|x| {
                process_entry(&x.name, folder_id, &x.entry)
            }).collect();
        let json = bitwarden_json::BitwardenJson::with_entries(vec![folder], converted_entries);
    }
}


