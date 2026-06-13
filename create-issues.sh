#!/usr/bin/env bash
set -euo pipefail

ISSUES_FILE="${1:-issues.yml}"
VALIDATOR_MANIFEST="tools/issue-validator/Cargo.toml"
GENERATED_DIR=".generated/issues"

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

echo "Running Rust validator..."

cargo run \
  --quiet \
  --manifest-path "${VALIDATOR_MANIFEST}" \
  -- "${ISSUES_FILE}"

echo "Validation completed successfully."

echo "Preparing generated output directory..."
rm -rf "${GENERATED_DIR}"
mkdir -p "${GENERATED_DIR}"

echo "Generation is not implemented yet."
echo "Expected future output directory: ${GENERATED_DIR}"

echo "Stopping before GitHub issue creation because generated output is not available yet."
echo "This will be completed after Rust generation is implemented."

exit 0