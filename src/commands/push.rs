// src/commands/push.rs

use std::fs;
use std::error::Error;
use std::path::Path;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;
use tokio::time::{sleep, Duration};

use crate::config::Config;

#[derive(Serialize)]
struct NewSubmissionRequest {
    problem_id: Uuid,
    code: String,
}

#[derive(Deserialize)]
struct SubmissionResponse {
    id: Uuid,
}

pub async fn handle(problem_id: String, file: String, config: &Config) 
    -> Result<(), Box<dyn Error>> 
{
    println!("Pushing solution: {}", file);

    let problem_id = Uuid::parse_str(&problem_id)?;
    let code = fs::read_to_string(Path::new(&file))?;

    let submission = NewSubmissionRequest { problem_id, code };
    let client = Client::new();
    let url = format!("{}/submissions", config.api_base_url);

    let resp = client.post(&url).json(&submission).send().await?;

    if !resp.status().is_success() {
        eprintln!("Failed to submit. Status: {}", resp.status());
        let text = resp.text().await?;
        eprintln!("Error: {}", text);
        return Ok(());
    }

    let SubmissionResponse { id } = resp.json().await?;
    println!("Submission pushed. ID: {}", id);
    
    sleep(Duration::from_secs(1)).await;

    // Start polling
    let poll_url = format!("{}/submissions/{}", config.api_base_url, id);
    loop {
        let poll_resp = client.get(&poll_url).send().await?;

        if !poll_resp.status().is_success() {
            eprintln!("Polling failed. Status: {}", poll_resp.status());
            break;
        }

        let json: Value = poll_resp.json().await?;

        let verdict = json["verdict"]
            .as_str()
            .unwrap_or("Pending")
            .to_string();

        let detail = json["verdict_detail"]
            .as_str()
            .unwrap_or("")
            .trim()
            .to_string();

        println!("\nFinal Verdict: {}\n", verdict);

        if !detail.is_empty() {
            println!("Details:\n{}\n", detail);
        }

        if verdict != "Pending" && verdict != "Running" {
            break;
        }

        sleep(Duration::from_secs(2)).await;
    }

    Ok(())
}
