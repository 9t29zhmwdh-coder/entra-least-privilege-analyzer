# Changelog

## [0.1.0] — 2026-06-18

### Added

- Privilege scoring engine with weighted role scores
- Over-privileged account detection with configurable threshold
- Role overlap analysis for accounts holding multiple high-impact roles
- PIM gap detection for permanent high-privilege assignments
- PIM settings audit covering MFA, justification, and activation duration
- JSON export via `elpa export --format json`
- Markdown export via `elpa export --format md`
- SARIF stub for future GitHub Advanced Security integration
- CI pipeline on Ubuntu and Windows
