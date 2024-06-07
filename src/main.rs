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
use std::io::{BufRead, BufReader};

mod bit_folder;
mod bitwarden_json;
mod bitwarden_entry;

fn main() {
    let test_entry = "blahpassword\nusername:Billy\nExtra Text!!!!!";
    process_entry(test_entry);
    bit_folder::test_mod();
}

fn get_totp(input: &str) -> Option<String> {
    let reg: &Regex = regex_static::static_regex!("totp://");
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

fn process_entry(inp: &str) {
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
        None => String::from("No TOTP"),
        Some(x) => x,
    };
    println!("Username: {}", username);
    println!("Pass: {}", pass);
    println!("TOTP: {:?}", totp);
    println!("Remainder: {}", line_buf.fold(String::new(), |s, l| s + &l.expect("Got non-string in remainder print") + "\n"));
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
}


