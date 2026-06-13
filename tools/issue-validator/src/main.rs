use anyhow::{
    bail,
    Context,
    Result
};
use clap::Parser;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{
    Path,
    PathBuf
};
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(
    name = "issue-validator",
    about = "Validate GitHub issue definitions from a YAML file"
)]
struct Cli {
    #[arg(value_name = "ISSUES_FILE")]
    input: PathBuf,

    #[arg(long, value_name = "OUTPUT_DIR")]
    out: Option<PathBuf>,
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

fn main() -> ExitCode {
    let cli = Cli::parse();

    println!("Starting validation for: {}", cli.input.display());

    let issues_file = match load_issues_file(&cli.input) {
        Ok(issues_file) => issues_file,
        Err(error) => {
            eprintln!("Validation failed.");
            eprintln!("Error: {error:#}");
            return ExitCode::from(1);
        }
    };

    let mut validation_errors = Vec::new();

    validation_errors.extend(validate_schema(&issues_file));
    validation_errors.extend(validate_duplicate_titles(&issues_file));

    if !validation_errors.is_empty() {
        eprintln!("Validation failed.");

        for error in &validation_errors {
            eprintln!("- {error}");
        }

        eprintln!(
            "Failure summary: {} validation error(s).",
            validation_errors.len()
        );

        return ExitCode::from(1);
    }

    let issue_count = issues_file.issues.as_ref().map_or(0, Vec::len);

    println!("Validation passed.");
    println!("Success summary: validated {issue_count} issue definition(s).");

    if let Some(output_dir) = &cli.out {
        println!("Starting generation into: {}", output_dir.display());

        if let Err(error) = generate_issue_files(&issues_file, output_dir) {
            eprintln!("Generation failed.");
            eprintln!("Error: {error:#}");
            return ExitCode::from(1);
        }

        println!("Generation completed successfully.");
        println!("Generated {issue_count} issue directorie(s).");
    }

    ExitCode::SUCCESS
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

fn validate_duplicate_titles(issues_file: &IssuesFile) -> Vec<String> {
    let mut errors = Vec::new();

    let Some(issues) = &issues_file.issues else {
        return errors;
    };

    if issues.is_empty() {
        return errors;
    }

    let mut title_positions: BTreeMap<String, Vec<usize>> = BTreeMap::new();

    for (index, issue) in issues.iter().enumerate() {
        let issue_number = index + 1;

        let Some(title) = &issue.title else {
            continue;
        };

        let normalized_title = title.trim();

        if normalized_title.is_empty() {
            continue;
        }

        title_positions
            .entry(normalized_title.to_string())
            .or_default()
            .push(issue_number);
    }

    for (title, positions) in title_positions {
        if positions.len() > 1 {
            let positions_text = positions
                .iter()
                .map(|position| position.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            errors.push(format!(
                "Duplicate title `{title}` found in issues: {positions_text}."
            ));
        }
    }

    errors
}

fn generate_issue_files(issues_file: &IssuesFile, output_dir: &Path) -> Result<()> {
    let Some(issues) = &issues_file.issues else {
        bail!("Cannot generate output because root field `issues` is missing.");
    };

    if output_dir.exists() {
        fs::remove_dir_all(output_dir).with_context(|| {
            format!(
                "Failed to clean output directory: {}",
                output_dir.display()
            )
        })?;
    }

    fs::create_dir_all(output_dir).with_context(|| {
        format!(
            "Failed to create output directory: {}",
            output_dir.display()
        )
    })?;

    let mut manifest_lines = Vec::new();

    for (index, issue) in issues.iter().enumerate() {
        let issue_number = index + 1;

        let title = issue
            .title
            .as_ref()
            .map(|title| title.trim())
            .context("Cannot generate issue because title is missing.")?;

        let body = issue
            .body
            .as_ref()
            .context("Cannot generate issue because body is missing.")?;

        let slug = slugify_title(title);
        let issue_dir_name = format!("{issue_number:03}-{slug}");
        let issue_dir = output_dir.join(issue_dir_name);

        fs::create_dir_all(&issue_dir).with_context(|| {
            format!(
                "Failed to create generated issue directory: {}",
                issue_dir.display()
            )
        })?;

        fs::write(issue_dir.join("title.txt"), format!("{title}\n")).with_context(|| {
            format!(
                "Failed to write title file for generated issue {}",
                issue_number
            )
        })?;

        fs::write(issue_dir.join("body.md"), body).with_context(|| {
            format!(
                "Failed to write body file for generated issue {}",
                issue_number
            )
        })?;

        if let Some(labels) = &issue.labels {
            write_lines_file(&issue_dir.join("labels.txt"), labels).with_context(|| {
                format!(
                    "Failed to write labels file for generated issue {}",
                    issue_number
                )
            })?;
        }

        if let Some(milestone) = &issue.milestone {
            let trimmed_milestone = milestone.trim();

            if !trimmed_milestone.is_empty() {
                fs::write(
                    issue_dir.join("milestone.txt"),
                    format!("{trimmed_milestone}\n"),
                )
                .with_context(|| {
                    format!(
                        "Failed to write milestone file for generated issue {}",
                        issue_number
                    )
                })?;
            }
        }

        if let Some(assignees) = &issue.assignees {
            write_lines_file(&issue_dir.join("assignees.txt"), assignees).with_context(|| {
                format!(
                    "Failed to write assignees file for generated issue {}",
                    issue_number
                )
            })?;
        }

        manifest_lines.push(issue_dir.display().to_string());
    }

    fs::write(
        output_dir.join("manifest.txt"),
        format!("{}\n", manifest_lines.join("\n")),
    )
    .with_context(|| {
        format!(
            "Failed to write manifest file in output directory: {}",
            output_dir.display()
        )
    })?;

    Ok(())
}

fn write_lines_file(path: &Path, values: &[String]) -> Result<()> {
    let cleaned_values = values
        .iter()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .collect::<Vec<&str>>();

    if cleaned_values.is_empty() {
        return Ok(());
    }

    fs::write(path, format!("{}\n", cleaned_values.join("\n")))?;

    Ok(())
}

fn slugify_title(title: &str) -> String {
    let mut slug = String::new();
    let mut previous_was_dash = false;

    for character in title.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            previous_was_dash = false;
        } else if !previous_was_dash {
            slug.push('-');
            previous_was_dash = true;
        }
    }

    let slug = slug.trim_matches('-').to_string();

    if slug.is_empty() {
        "issue".to_string()
    } else {
        slug
    }
}

fn is_missing_or_empty(value: &Option<String>) -> bool {
    match value {
        Some(text) => text.trim().is_empty(),
        None => true,
    }
}