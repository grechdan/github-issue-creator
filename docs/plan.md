# Issue Creation Automation - Implementation Plan

## Phase 1 - Repository Preparation

### Task 1
Create folders:

- issue-validator
- docs

### Task 2
Create issues.yml example file.

### Task 3
Install tools:

- Rust
- gh
- yq

Success Criteria:
- gh auth login works
- cargo --version works
- yq --version works

---

## Phase 2 - Rust Validator

### Task 4
Create Rust CLI project.

Commands:

cargo new issue-validator

### Task 5
Add dependencies:

- serde
- serde_yaml
- clap
- anyhow

### Task 6
Implement YAML loading.

Validator must:
- read file path
- deserialize YAML
- print friendly errors

### Task 7
Implement schema validation.

Check:
- title exists
- body exists

### Task 8
Implement duplicate detection.

Example:

Issue A
Issue A

Result:
Validation failed.

### Task 9
Implement exit codes.

Success:
exit 0

Failure:
exit 1

---

## Phase 3 - Bash Integration

### Task 10
Create create-issues.sh.

Flow:

1. Call validator
2. Stop on error
3. Create issues

### Task 11
Loop through YAML entries.

Use yq to extract:
- title
- body
- labels
- milestone
- assignees

### Task 12
Call:

gh issue create

for each validated issue.

---

## Phase 4 - Nice-to-Have Improvements

### Task 13
Dry-run mode.

Example:

./create-issues.sh --dry-run

### Task 14
Skip duplicates already existing in GitHub.

### Task 15
Support Azure DevOps backend later.

### Task 16
Generate issues from milestone templates.

---

## Definition of Done

- One YAML file contains all issues.
- Validator checks entire file.
- No issue is created if validation fails.
- Bash creates all issues after successful validation.
- Process can run locally and in CI pipelines.
