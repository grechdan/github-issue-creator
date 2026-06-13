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

    let issues_file = load_issues_file(&cli.input)?;

    let schema_errors = validate_schema(&issues_file);

    if !schema_errors.is_empty() {
        eprintln!("Schema validation failed:");

        for error in &schema_errors {
            eprintln!("- {error}");
        }

        bail!(
            "Validation failed with {} schema error(s).",
            schema_errors.len()
        );
    }

    let issue_count = issues_file.issues.as_ref().map_or(0, Vec::len);

    println!("Validation passed.");
    println!("Validated {issue_count} issue definition(s).");

    Ok(())
}

fn load_issues_file(path: &PathBuf) -> Result<IssuesFile> {
    if !path.exists() {
        bail!("Input file does not exist: {}", path.display());
    }

    if !path.is_file() {
        bail!("Input path is not a file: {}", path.display());
    }

    let contents = fs::read_to_string(path)
        .with_context(|| format!("Failed to read issue file: {}", path.display()))?;

    let issues_file: IssuesFile = serde_yaml::from_str(&contents)
        .with_context(|| format!("Failed to parse YAML in file: {}", path.display()))?;

    Ok(issues_file)
}

fn validate_schema(issues_file: &IssuesFile) -> Vec<String> {
    let mut errors = Vec::new();

    let Some(issues) = &issues_file.issues else {
        errors.push("Root field `issues` is required.".to_string());
        return errors;
    };

    if issues.is_empty() {
        errors.push("Root field `issues` must contain at least one issue.".to_string());
        return errors;
    }

    for (index, issue) in issues.iter().enumerate() {
        let issue_number = index + 1;

        if is_missing_or_empty(&issue.title) {
            errors.push(format!("Issue {issue_number}: `title` is required."));
        }

        if is_missing_or_empty(&issue.body) {
            errors.push(format!("Issue {issue_number}: `body` is required."));
        }
    }

    errors
}

fn is_missing_or_empty(value: &Option<String>) -> bool {
    match value {
        Some(text) => text.trim().is_empty(),
        None => true,
    }
}