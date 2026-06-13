#!/usr/bin/env bash
set -euo pipefail

ISSUES_FILE="${1:-issues.yml}"
VALIDATOR_MANIFEST="tools/issue-validator/Cargo.toml"
GENERATED_DIR=".generated/issues"
MANIFEST_FILE="${GENERATED_DIR}/manifest.txt"

echo "Issue creation started."
echo "Input file: ${ISSUES_FILE}"

if [[ ! -f "${ISSUES_FILE}" ]]; then
  echo "Error: issue file does not exist: ${ISSUES_FILE}" >&2
  exit 1
fi

if [[ ! -f "${VALIDATOR_MANIFEST}" ]]; then
  echo "Error: Rust validator project was not found at: ${VALIDATOR_MANIFEST}" >&2
  exit 1
fi

if ! command -v gh >/dev/null 2>&1; then
  echo "Error: GitHub CLI is not installed or not available in PATH." >&2
  exit 1
fi

echo "Checking GitHub CLI authentication..."
gh auth status >/dev/null

echo "Running Rust validator/generator..."

cargo run \
  --quiet \
  --manifest-path "${VALIDATOR_MANIFEST}" \
  -- "${ISSUES_FILE}" \
     --out "${GENERATED_DIR}"

echo "Validation and generation completed successfully."

if [[ ! -f "${MANIFEST_FILE}" ]]; then
  echo "Error: generated manifest file does not exist: ${MANIFEST_FILE}" >&2
  exit 1
fi

echo "Creating GitHub issues from generated output..."

created_count=0

while IFS= read -r issue_dir || [[ -n "${issue_dir}" ]]; do
  if [[ -z "${issue_dir}" ]]; then
    continue
  fi

  title_file="${issue_dir}/title.txt"
  body_file="${issue_dir}/body.md"
  labels_file="${issue_dir}/labels.txt"
  milestone_file="${issue_dir}/milestone.txt"
  assignees_file="${issue_dir}/assignees.txt"

  if [[ ! -f "${title_file}" ]]; then
    echo "Error: missing generated title file: ${title_file}" >&2
    exit 1
  fi

  if [[ ! -f "${body_file}" ]]; then
    echo "Error: missing generated body file: ${body_file}" >&2
    exit 1
  fi

  title="$(<"${title_file}")"

  if [[ -z "${title}" ]]; then
    echo "Error: generated title is empty in: ${title_file}" >&2
    exit 1
  fi

  echo "Creating issue: ${title}"

  gh_args=(
    issue create
    --title "${title}"
    --body-file "${body_file}"
  )

  if [[ -f "${labels_file}" ]]; then
    while IFS= read -r label || [[ -n "${label}" ]]; do
      if [[ -n "${label}" ]]; then
        gh_args+=(--label "${label}")
      fi
    done < "${labels_file}"
  fi

  if [[ -f "${milestone_file}" ]]; then
    milestone="$(<"${milestone_file}")"

    if [[ -n "${milestone}" ]]; then
      gh_args+=(--milestone "${milestone}")
    fi
  fi

  if [[ -f "${assignees_file}" ]]; then
    while IFS= read -r assignee || [[ -n "${assignee}" ]]; do
      if [[ -n "${assignee}" ]]; then
        gh_args+=(--assignee "${assignee}")
      fi
    done < "${assignees_file}"
  fi

  if ! created_url="$(gh "${gh_args[@]}")"; then
    echo "Error: failed to create GitHub issue: ${title}" >&2
    exit 1
  fi

  echo "Created: ${created_url}"

  created_count=$((created_count + 1))
done < "${MANIFEST_FILE}"

echo "Issue creation completed."
echo "Created ${created_count} GitHub issue(s)."