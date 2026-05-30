use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "issue-validator",
    about = "Validate GitHub issue definitions from a YAML file"
)]
struct Cli {
    #[arg(value_name = "ISSUES_FILE")]
    input: PathBuf,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct IssuesFile {
    issues: Vec<Issue>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Issue {
    title: String,
    body: String,
    labels: Option<Vec<String>>,
    assignees: Option<Vec<String>>,
    milestone: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("Starting validation for: {}", cli.input.display());
    println!("Temporary success: YAML loading is not implemented yet.");

    Ok(())
}