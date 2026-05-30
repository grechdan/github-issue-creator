# Issue Creation Automation - Architecture

## Goal

Automate creation of GitHub issues from YAML files while ensuring all issue definitions are validated before any issue is created.

## High-Level Architecture

User
  -> create-issues.sh
      -> Rust Validator
          -> issues.yml
      -> GitHub CLI (gh)
          -> GitHub Issues

## Components

### issues.yml
Source of truth containing issue definitions.

### Rust Validator
Responsibilities:
- Parse YAML
- Validate schema
- Validate required fields
- Detect duplicate titles
- Return non-zero exit code on failure

The validator never modifies GitHub.

### Bash Wrapper
Responsibilities:
- Call validator
- Stop on validation errors
- Create issues using gh
- Display progress

### GitHub CLI
Used for actual issue creation.

## Project Structure

project-root/
├── issues.yml
├── create-issues.sh
├── docs/
├── tools/
│   └── issue-validator/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
└── .github/

## Validation Rules

Required:
- title
- body

Optional:
- labels
- assignees
- milestone

Additional:
- unique titles
- valid YAML
- issues array exists

## Design Principles

- Validation before creation
- Fail fast
- Single source of truth
- Platform-independent validator
- Simple Bash orchestration
