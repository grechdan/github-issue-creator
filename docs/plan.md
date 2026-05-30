# Issue Creation Automation - Implementation Plan

## Purpose

Build a small local-first toolchain that creates GitHub issues from a version-controlled `issues.yml` file.

The important design decision is that Bash does **not** parse YAML. Rust owns YAML parsing, validation, and generation of Bash-friendly files. Bash only orchestrates the flow and calls GitHub CLI.

## Final workflow

```text
issues.yml
   ↓
tools/issue-validator
   ↓
validate schema and content
   ↓
generate .generated/issues/
   ↓
create-issues.sh
   ↓
gh issue create
```

## Required tools

Required:

- Rust/Cargo
- GitHub CLI: `gh`
- Bash or Git Bash-compatible shell

## Phase 1 - Repository preparation

Goal: create a clean repository layout before implementing logic.

Tasks:

- Create root files:
  - `issues.yml`
  - `create-issues.sh`
  - `README.md`
  - `.gitignore`
- Create folders:
  - `tools/issue-validator/`
  - `docs/`
  - `.github/workflows/`
- Add `.generated/` to `.gitignore`.
- Add a small example `issues.yml` with at least two issues.
- Document local setup in `docs/local-setup.md`.

Expected result:

```text
repo-root/
  issues.yml
  create-issues.sh
  README.md
  .gitignore
  docs/
    local-setup.md
  tools/
    issue-validator/
  .github/
    workflows/
```

## Phase 2 - Rust validator

Goal: build a Rust CLI that validates `issues.yml` and never touches GitHub.

Recommended dependencies:

- `clap` for CLI arguments
- `serde` with derive support
- `serde_yaml` for YAML parsing
- `anyhow` for error handling
- optionally `thiserror` if typed validation errors become useful

Initial command shape:

```bash
cargo run --manifest-path tools/issue-validator/Cargo.toml -- validate issues.yml
```

Validation rules:

- Root object must contain `issues`.
- `issues` must be a non-empty array.
- Each issue must have non-empty `title`.
- Each issue must have non-empty `body`.
- Optional fields may be absent:
  - `labels`
  - `assignees`
  - `milestone`
- Duplicate titles inside the YAML file must fail validation.
- Validation should collect multiple errors before returning failure.
- Success exits with code `0`.
- Validation failure exits with code `1`.

Suggested YAML shape:

```yaml
issues:
  - title: Prepare repository structure
    labels:
      - type:task
      - area:cli
      - priority:high
    milestone: Phase 1 - Repository Preparation
    body: |
      ## Goal

      Create the initial repository structure.

      ## Tasks

      - [ ] Create tools directory.
      - [ ] Create docs directory.
```

## Phase 3 - Rust generation output

Goal: extend the Rust CLI so Bash can consume generated plain-text files instead of parsing YAML.

Recommended command shape:

```bash
tools/issue-validator/target/release/issue-validator generate issues.yml --out .generated/issues
```

Generated output:

```text
.generated/issues/
  manifest.txt
  001-prepare-repository-structure/
    title.txt
    body.md
    labels.txt
    milestone.txt
    assignees.txt
  002-create-example-issues-yml-file/
    title.txt
    body.md
    labels.txt
```

Rules:

- Generation must only run after validation passes.
- The output directory may be deleted/recreated on each run.
- `manifest.txt` contains one issue directory path per line.
- `title.txt` contains the exact GitHub issue title.
- `body.md` contains the exact GitHub issue body.
- `labels.txt` contains one label per line and is omitted when there are no labels.
- `milestone.txt` contains one milestone name and is omitted when absent.
- `assignees.txt` contains one assignee per line and is omitted when there are no assignees.
- Generated directory names should be deterministic: index + slugified title.

This design avoids quoting, multiline body, comma escaping, and shell argument parsing problems.

## Phase 4 - Bash wrapper

Goal: create `create-issues.sh` as a thin orchestration script.

Responsibilities:

- Accept default input file `issues.yml`.
- Accept custom input file path.
- Support `--dry-run`.
- Call Rust validation/generation first.
- Stop immediately if validation/generation fails.
- Loop through generated issue directories from `.generated/issues/manifest.txt`.
- Call `gh issue create` for each issue.
- Stop on first GitHub CLI failure.

Non-responsibilities:

- Do not parse YAML.
- Do not validate issue schema.
- Do not require `yq`, `jq`, or Python.

Example wrapper flow:

```bash
#!/usr/bin/env bash
set -euo pipefail

ISSUES_FILE="issues.yml"
OUT_DIR=".generated/issues"
DRY_RUN="false"

# parse args here

./tools/issue-validator/target/release/issue-validator generate "$ISSUES_FILE" --out "$OUT_DIR"

while IFS= read -r issue_dir; do
  title="$(cat "$issue_dir/title.txt")"
  body_file="$issue_dir/body.md"

  args=(issue create --title "$title" --body-file "$body_file")

  if [[ -f "$issue_dir/labels.txt" ]]; then
    while IFS= read -r label; do
      [[ -n "$label" ]] && args+=(--label "$label")
    done < "$issue_dir/labels.txt"
  fi

  if [[ -f "$issue_dir/milestone.txt" ]]; then
    milestone="$(cat "$issue_dir/milestone.txt")"
    [[ -n "$milestone" ]] && args+=(--milestone "$milestone")
  fi

  if [[ -f "$issue_dir/assignees.txt" ]]; then
    while IFS= read -r assignee; do
      [[ -n "$assignee" ]] && args+=(--assignee "$assignee")
    done < "$issue_dir/assignees.txt"
  fi

  if [[ "$DRY_RUN" == "true" ]]; then
    printf 'Would create issue: %s
' "$title"
  else
    gh "${args[@]}"
  fi
done < "$OUT_DIR/manifest.txt"
```

## Phase 5 - Nice-to-have improvements

These should not block the first usable version.

Possible improvements:

- Dry-run mode.
- Skip issues that already exist in GitHub by title.
- CI validation workflow.
- Better error formatting.
- Shellcheck for Bash script.
- Milestone template generation design.

## Implementation order

Recommended order:

1. Prepare repository structure.
2. Create `issues.yml` example.
3. Document local prerequisites.
4. Scaffold Rust CLI.
5. Add Rust data model and YAML loading.
6. Implement schema validation.
7. Implement duplicate title detection.
8. Implement exit codes and summary output.
9. Add Rust generation output.
10. Create validation-first Bash wrapper.
11. Connect generated output to `gh issue create`.
12. Add dry-run mode.
13. Add CI validation.
14. Write/update README.
15. Add future design docs.

## Definition of done for the first usable version

The first usable version is done when:

- `issues.yml` can define several issues.
- Rust validates the file.
- Rust generates `.generated/issues/`.
- Bash reads generated files only.
- Bash creates issues through `gh issue create`.
- Validation failure prevents GitHub modification.