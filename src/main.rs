use std::io::{self, BufRead};
use reqwest;
use serde_json::{Value};

const API_BASE: &str = "https://paste.nextcord.dev";

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let mut text = String::new();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        text += &(line + "\n");
    }
    let client = reqwest::Client::builder().build().unwrap();
    let raw_text = text.as_bytes().to_vec();

    let resp = client.post(API_BASE.to_owned() + "/api/new")
        .body(raw_text)
        .send()
        .await
        .unwrap();
    let resp_text = resp.text().await.unwrap();
    let data: Value = serde_json::from_str(&resp_text).unwrap();

    let language = std::env::args().nth(1).unwrap_or("python".to_string());
    
    let url = format!("{}?id={}&language={}", API_BASE, data["key"].as_str().unwrap(), language);
    
    println!("{}", url);
}
