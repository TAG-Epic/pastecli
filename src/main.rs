use std::io::{self, BufRead};
use reqwest;
use serde_json::Value;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

const API_BASE: &str = "https://paste.nextcord.dev";
const DEFAULT_LANGUAGE: &str = "python";

#[tokio::main]
async fn main() {
    let language_or_file = std::env::args().nth(1);

    if let Some(potential_file) = language_or_file {
        let file_path = Path::new(&potential_file);
        if !file_path.is_file() {
            let text = read_piped();
            let url = upload_paste(text, potential_file).await;
            println!("{}", url);
            return;
        } else {
            let mut file = File::open(file_path).expect("Could not open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Could not read from file");

            let file_extension = potential_file.split(".").nth(1).unwrap_or(DEFAULT_LANGUAGE);
            let file_type = match file_extension {
                "py" => "python",
                "rs" => "rust",
                "js" => "javascript",
                _ => file_extension
            };

            let url = upload_paste(contents, file_type).await;
            println!("{}", url);
            return;
        }
    } else {
        let text = read_piped();
        let url = upload_paste(text, DEFAULT_LANGUAGE).await;
        println!("{}", url);
    }
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

async fn upload_paste(text: impl AsRef<str>, language: impl AsRef<str>) -> String {
    let client = reqwest::Client::builder().build().unwrap();
    let body = text.as_ref().as_bytes().to_vec();

    // Send the request
    let resp = client.post(API_BASE.to_owned() + "/api/new")
        .body(body)
        .send()
        .await
        .expect("Could not send request");
    let resp_body = resp.text().await.expect("Could not get response body");
    let data: Value = serde_json::from_str(&resp_body).expect("Could not decode JSON body");

    format!("{}?id={}&language={}", API_BASE, data["key"].as_str().expect("Could not get paste key"), language.as_ref())
}
