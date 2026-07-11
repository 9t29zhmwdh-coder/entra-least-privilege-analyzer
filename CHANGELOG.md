# Changelog

## [0.2.5] - 2026-07-11

### Added

- Documented Dual-Licensing readiness assessment in ROADMAP.md.

## [0.2.4] - 2026-07-11

### Fixed

- Updated actions/checkout to its latest major version in CI, since GitHub is deprecating the Node.js 20 runtime and the previous version was being forced onto Node 24 and crashing during post-run cleanup.

## [0.2.3] - 2026-07-10

### Fixed

- Removed em-dashes from README.md/README.de.md/CHANGELOG.md, replaced with colons or plain hyphens
- Changed the language-switch link from a blockquote to plain text

## [0.2.2] - 2026-07-10

### Changed

- Moved the "New here? -> beginners guide" callout in README.md above the intro (previously only appeared near Requirements)

### Added

- Added the "New here?" beginner guide callout to README.de.md (was missing)

## [0.2.0] - 2026-07-03

### Added

- Bring-your-own-token mode: `ENTRA_ACCESS_TOKEN` (+ `ENTRA_TENANT_ID`) skips
  the client-credentials flow for callers that already hold a delegated
  Microsoft Graph token (e.g. admin portals). Token is used as-is, never
  refreshed: intended for one-shot runs. Existing flow unchanged.


## [0.1.0] - 2026-06-18

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
