use anyhow::{
    bail,
    Context,
    Result
};
use clap::Parser;
use serde::Deserialize;
use std::fs;
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

#[derive(Debug, Deserialize)]
struct IssuesFile {
    issues: Option<Vec<Issue>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Issue {
    title: Option<String>,
    body: Option<String>,
    labels: Option<Vec<String>>,
    assignees: Option<Vec<String>>,
    milestone: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("Starting validation for: {}", cli.input.display());

    if !cli.input.exists() {
        bail!("Input file does not exist: {}", cli.input.display());
    }

    if !cli.input.is_file() {
        bail!("Input path is not a file: {}", cli.input.display());
    }

    let contents = fs::read_to_string(&cli.input)
        .with_context(|| format!("Failed to read issue file: {}", cli.input.display()))?;

    let issues_file: IssuesFile = serde_yaml::from_str(&contents)
        .with_context(|| format!("Failed to parse YAML in file: {}", cli.input.display()))?;

    let issue_count = issues_file.issues.as_ref().map_or(0, Vec::len);

    println!("YAML loaded successfully.");
    println!("Found {issue_count} issue definition(s).");

    Ok(())
}