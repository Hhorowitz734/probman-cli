// src/commands/get.rs

use serde::Deserialize;
use uuid::Uuid;
use crate::config::Config;
use std::fs::File;
use std::io::{self, Write};


#[derive(Debug, Deserialize)]
pub struct Problem {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub difficulty: String,
    pub input_type: String,
    pub output_type: String
}

fn slugify(title: &str) -> String {
    title.to_lowercase().replace(' ', "-")
}

pub fn generate_stub(problem: &Problem)
    -> io::Result<()> {

    let filename = format!("{}.py", slugify(&problem.title));
    let mut file = File::create(&filename)?;

    // Comment each line of the description
    for line in problem.description.lines() {
        writeln!(file, "# {}", line)?;
    }

    // Add a blank line after the comment block
    writeln!(file)?;

    // Write the solution stub
    writeln!(file, "def solution():")?;
    writeln!(file, "    pass  # TODO: implement your code")?;
    writeln!(file, "\n\nsolution()")?;

    println!("Created {}", filename);

    Ok(())
}

pub async fn handle(problem: String, config: &Config) 
    -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching problem: {}", problem);

    let problem_url = 
        format!("{}/problem/{}", config.api_base_url, problem);
    
    let result = reqwest::get(&problem_url).await?;

    let problem: Problem = result.json().await?;
    
    println!("Fetched: {}\n{}", problem.title, problem.description);

    generate_stub(&problem);

    Ok(())
}
