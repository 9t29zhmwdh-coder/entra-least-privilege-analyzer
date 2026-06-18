# Contributing

Contributions are welcome. Please follow these guidelines:

## Before submitting a PR

- `cargo clippy --workspace -- -D warnings` must pass with no warnings
- `cargo test --workspace` must pass
- No credentials, tenant IDs, or user data in any committed file
- New Graph API endpoints must be read-only

## Commit style

Use the prefix format: `[feat]`, `[fix]`, `[docs]`, `[refactor]`, `[test]`

Example: `[feat] Add service principal privilege analysis`

## Adding a new analysis capability

1. Add the relevant model types to `elpa-core/src/models.rs`
2. Implement the analysis logic in `elpa-core/src/analyzer.rs` with unit tests
3. Add the Graph API call in the appropriate `elpa-graph/src/*.rs` file
4. Wire the new capability into `elpa-cli/src/main.rs`
5. Update `ROADMAP.md` and `CHANGELOG.md`
