#![feature(format_args_capture)]

use std::io::{self, BufRead};
use serde_json::Value;
use std::path::Path;
use std::fs::read_to_string;
use ureq;

const API_BASE: &str = "https://paste.nextcord.dev";
const DEFAULT_LANGUAGE: &str = "python";

fn main() {
    let language_or_file = std::env::args().nth(1);

    let language_or_file = match language_or_file {
        Some(language_or_file) => language_or_file,
        None => {
            let url = upload_paste(read_piped(), DEFAULT_LANGUAGE);
            println!("{url}");
            return;
        }
    };

    let file_path = Path::new(&language_or_file);

    if !file_path.is_file() {
        let url = upload_paste(read_piped(), &language_or_file);
        println!("{url}");
        return;
    }

    let contents = read_to_string(file_path).expect("Failed to open file");

    let file_extension = file_path.extension().and_then(|s| s.to_str()).unwrap_or(DEFAULT_LANGUAGE);
    let file_type = match file_extension {
        "py" => "python",
        "rs" => "rust",
        "js" => "javascript",
        _ => file_extension
    };
    let url = upload_paste(contents, &file_type);
    println!("{url}");
}

fn read_piped() -> String {
    let stdin = io::stdin();
    let mut text = String::new();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        text += &(line + "\n");
    }
    text
}

fn upload_paste(text: impl AsRef<str>, language: impl AsRef<str>) -> String {
    let resp = ureq::get(&(API_BASE.to_owned() + "/api/new"))
        .send_string(text.as_ref())
        .unwrap();
    let resp_body = resp.into_string().expect("Could not get response body");
    let data: Value = serde_json::from_str(&resp_body).expect("Could not decode JSON body");

    format!("{}?id={}&language={}", API_BASE, data["key"].as_str().expect("Could not get paste key"), language.as_ref())
}
