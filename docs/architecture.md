# Issue Creation Automation - Architecture

## Goal

Automate creation of GitHub issues from YAML files while ensuring all issue definitions are validated before any issue is created.

## Scope

This project creates GitHub issues from a version-controlled YAML backlog file.

The scope is intentionally narrow:

- local/manual issue creation first
- GitHub CLI as the only GitHub write mechanism
- Rust for parsing, validation, and generation
- Bash for orchestration only
- CI validation later, but CI must stay read-only

## High-Level Architecture

User
  -> create-issues.sh
      -> Rust Validator
          -> issues.yml
      -> GitHub CLI (gh)
          -> GitHub Issues

```text
issues.yml
   ↓
Rust validator/generator
   ↓
.generated/issues/
   ↓
Bash wrapper
   ↓
gh issue create
```

Bash must not parse YAML. It should only read generated plain-text files.

## Components

### `issues.yml`

Source of truth for issue definitions.

Responsibilities:

- store issue titles
- store issue bodies
- store labels
- store milestones
- store assignees

It should be human-editable and version-controlled.

### `tools/issue-validator`

Rust CLI tool.

Responsibilities:

- read `issues.yml`
- parse YAML using Rust libraries
- validate schema
- validate required fields
- detect duplicate issue titles
- return reliable exit codes
- generate Bash-friendly output under `.generated/issues/`

Non-responsibilities:

- must not call GitHub API
- must not call `gh`
- must not create GitHub issues

### `.generated/issues/`

Generated intermediate output.

Responsibilities:

- represent each validated issue as plain-text files
- avoid shell quoting problems
- preserve multiline body text
- allow Bash to stay simple

Recommended structure:

```text
.generated/issues/
  manifest.txt
  001-prepare-repository-structure/
    title.txt
    body.md
    labels.txt
    milestone.txt
    assignees.txt
```

File rules:

- `manifest.txt` lists generated issue directories in deterministic order.
- `title.txt` contains the GitHub issue title.
- `body.md` contains the GitHub issue body.
- `labels.txt` contains one label per line and is optional.
- `milestone.txt` contains the milestone name and is optional.
- `assignees.txt` contains one assignee per line and is optional.

The `.generated/` directory should be ignored by Git.

### `create-issues.sh`

Bash orchestration script.

Responsibilities:

- call Rust validator/generator
- stop if validation/generation fails
- loop through `.generated/issues/manifest.txt`
- build `gh issue create` commands
- support dry-run mode
- display progress

Non-responsibilities:

- must not parse YAML
- must not validate schema

### GitHub CLI

The only component that modifies GitHub.

Responsibilities:

- authenticate with GitHub
- create issues
- apply labels
- apply milestones
- assign users
- optionally query existing issues for duplicate-title skipping

## Project Structure

project-root/
├── issues.yml
├── create-issues.sh
├── README.md
├── .gitignore
├── docs/
├── tools/
│   └── issue-validator/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
└── .github/

## Data model

Example `issues.yml`:

```yaml
issues:
  - title: Prepare repository structure
    labels:
      - type:task
      - area:cli
      - area:docs
      - priority:high
    milestone: Phase 1 - Repository Preparation
    assignees:
      - justsharingaccs
    body: |
      ## Goal

      Create the initial repository structure.

      ## Context

      The project needs a clear folder layout before implementation starts.

      ## Tasks

      - [ ] Create tools directory.
      - [ ] Create docs directory.

      ## Done when

      - [ ] Repository contains the expected files and folders.
```

Required fields:

- `title`
- `body`

Optional fields:

- `labels`
- `milestone`
- `assignees`

## Validation Rules

Minimum validation rules:

- YAML file must exist.
- YAML must parse successfully.
- Root object must contain `issues`.
- `issues` must be a non-empty array.
- Every issue must have a non-empty `title`.
- Every issue must have a non-empty `body`.
- Duplicate titles inside the YAML file are not allowed.
- Optional fields may be omitted.
- All validation errors should be collected and printed together where practical.

Exit codes:

- `0` means validation/generation succeeded.
- `1` means validation/generation failed.

## Design Principles

- Validation before creation
- Fail fast
- Single source of truth
- Platform-independent validator
- Simple Bash orchestration
