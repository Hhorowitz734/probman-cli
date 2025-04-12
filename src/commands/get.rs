// src/commands/get.rs

use serde::Deserialize;
use uuid::Uuid;
use crate::config::Config;

#[derive(Debug, Deserialize)]
pub struct Problem {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub difficulty: String,
    pub input_type: String,
    pub output_type: String
}

pub async fn handle(problem: String, config: &Config) 
    -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching problem: {}", problem);

    let problem_url = 
        format!("{}/problem/{}", config.api_base_url, problem);
    
    let result = reqwest::get(&problem_url).await?;

    let problem: Problem = result.json().await?;
    
    println!("Fetched: {}\n{}", problem.title, problem.description);
    Ok(())
}
