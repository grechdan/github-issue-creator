# Local Setup

This project is designed to run locally before CI is added. Install the tools
below before running issue creation manually.

YAML parsing, validation, and generation are handled by the Rust
validator/generator. No separate YAML tooling such as `yq`, `jq`, or Python is
required for issue creation.

## Required Tools

### Rust and Cargo

Rust and Cargo are required for the issue validator/generator. Cargo is
installed with the standard Rust toolchain.

Verify the installation:

```sh
cargo --version
```

### GitHub CLI

GitHub CLI is required to create issues in GitHub.

Verify the installation:

```sh
gh --version
```

Verify authentication:

```sh
gh auth status
```

If authentication is not configured, run:

```sh
gh auth login
```

`gh auth login` must be completed before issue creation can work.

## Setup Checklist

- [ ] Rust and Cargo are installed.
- [ ] `cargo --version` prints a Cargo version.
- [ ] `gh --version` prints a GitHub CLI version.
- [ ] `gh auth status` confirms GitHub CLI is authenticated.

## Validator exit codes

The Rust validator uses stable exit codes so it can be called safely from Bash and CI.

| Exit code | Meaning |
|---:|---|
| 0 | Validation passed |
| 1 | Validation failed, file could not be read, or YAML could not be parsed |

Validation success messages are printed to stdout. Validation errors are printed to stderr.