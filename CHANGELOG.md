# Changelog

## [1.0.9] - 2026-08-02

### Changed

- `thiserror` 1.0.69 to 2.0.18, merged since 1.0.8 and carried by this version.

---

## [1.0.8] - 2026-07-31

### Fixed

- CI checked Linux and Windows but not macOS, while the release workflow builds and publishes a macOS binary. That artefact went out without ever having been compile-checked, so a fault appearing only on macOS would have surfaced in somebody's download rather than in a pull request. The `check` matrix covers all three platforms the release targets.

---

## [1.0.7] - 2026-07-31

### Fixed

- The supported-versions table in `SECURITY.md` still listed `0.1.x`, a release line that no longer exists. Somebody reporting a vulnerability reads that table first, and it told them the current release was out of scope. It lists `1.0.x`.

---

## [1.0.6] - 2026-07-31

### Changed

- Both READMEs now open with how over-privilege actually accumulates, one exception at a time until nobody can name the standing admins, rather than with the tool's category. The four commands follow directly, and a short paragraph explains why the removing is deliberately left to a human: an automated de-privileging pass across a live tenant locks out people who were supposed to keep working.

---

## [1.0.5] - 2026-07-29

### Security

- The release workflow no longer grants `contents: write` for its whole run. The permission moves to the one job that publishes the release, and everything else runs with `contents: read`. OpenSSF Scorecard scores the Token-Permissions check 0 out of 10 whenever any workflow holds a top-level write permission, regardless of how little of the run needs it, so this single line was what held the check at zero.

---

## [1.0.4] - 2026-07-29

### Changed

Dependency and workflow updates merged since 1.0.3:

- chore(ci): bump the actions group across 1 directory with 2 updates
- chore(deps): bump the cargo group across 1 directory with 7 updates

---

## [1.0.3] - 2026-07-29

### Changed

- CodeQL moved from GitHub's default setup to an advanced setup with a committed `.github/workflows/codeql.yml`. The default setup decides on its own when to run and skips pull requests that touch no code of a given language, so a dependency pull request changing only `Cargo.lock` reported `skipping` on the required `Analyze` checks and could never be merged. The workflow runs on every pull request regardless of what changed and uses the `security-extended` query suite, which the default setup does not allow choosing. Required checks are unchanged.
- The Cargo group in `.github/dependabot.yml` is limited to `minor` and `patch` updates. Without that limit a major bump lands inside a grouped pull request that reads as routine, which is how a breaking change slips in unreviewed.

---

## [1.0.2] - 2026-07-28

### Added

- `.github/dependabot.yml`, covering GitHub Actions and Cargo with grouped weekly updates. The file was missing, and without it there are no version updates at all: repository security alerts only fire for disclosed vulnerabilities. Follows `engineering-standards` v0.10.0.

### Fixed

- 6 action references were pinned to a mutable tag or branch rather than a commit SHA, `dtolnay/rust-toolchain@stable` among them. A branch HEAD can be moved to point at different code at any time without the workflow file changing, which is exactly what `standards/ci-cd.md` section 2 exists to prevent. All are now pinned to SHAs with the version in the comment. Pinned at their current versions, not upgraded: a major bump belongs in its own reviewed PR, and Dependabot will now propose one.
- `actions/checkout` pins were inconsistent across workflows. All now use v7.0.1 with the full version in the comment, per `standards/ci-cd.md` section 2.

## [1.0.1] - 2026-07-20

### Changed

- OpenSSF Scorecard workflow and badge.
- `copilot-instructions.md` for consistent AI-assisted contributions.
- Unified the EN/DE language-switch link format.
- Split the README's security/CI badges onto their own line, separate from the platform/tech/AI badges (they were rendering as a single merged line).

## [1.0.0] - 2026-07-17

First stable release: a real release pipeline now builds and attaches
`elpa` binaries for Linux, macOS, and Windows to every GitHub Release,
the prerequisite for a 1.0 release per this portfolio's own SemVer
discipline.

### Added
- Release workflow (`release.yml`) that cross-compiles `elpa` for Linux/macOS/Windows on every `v*` tag push and attaches the binaries to a GitHub Release. Previously there was no prebuilt binary; users had to build from source.

## [0.3.1] - 2026-07-17

### Changed
- CI: added an explicit `permissions: contents: read` block to the workflow(s) that were missing one (CodeQL `actions/missing-workflow-permissions`), narrowing the default GITHUB_TOKEN scope.

## [0.3.0] - 2026-07-15

### Added

- Test coverage for `elpa-graph` (client token acquisition/caching, pagination, error handling, and all three Graph-consuming modules `roles`/`pim`/`users`), which previously had zero tests, using `wiremock` against hand-built fixture JSON matching the documented Graph response shapes. No live Entra ID tenant is required to run these.
- Test coverage for `elpa-core::report` and `AnalysisSummary::from_gaps`, previously untested.
- `GraphClient::new()`/`GraphClient::from_token()` constructors alongside the existing `from_env()`, and `with_graph_base()`/`with_token_url()` to override the Graph API base URL and OAuth token endpoint (useful for national cloud variants, and what makes the new tests possible without a real tenant).
- CLI smoke tests (`elpa-cli/tests/demo.rs`) covering the offline `demo` subcommand, `--version`, and the clean-failure path when no Entra ID credentials are configured.

### Fixed

- `elpa --version` was hardcoded to `"0.1.0"` in the `clap` command definition regardless of the actual crate version; now reads `CARGO_PKG_VERSION` like the rest of the workspace.

## [0.2.8] - 2026-07-13

### Fixed

- README.de.md section order now matches README.md (Voraussetzungen/Requirements moved from the end to before Quick Start).
- Added the missing "Beispielausgabe" (Sample Output) section to README.de.md, which README.md has.

## [0.2.7] - 2026-07-12

### Fixed

- Removed em-dashes/en-dashes across source comments, docs, and scripts (Swiss German orthography rule).

## [0.2.6] - 2026-07-12

### Added

- Dual-Licensing skeleton: LICENSE.COMMERCIAL, COMMERCIAL.md, and ENTERPRISE_FEATURES.md, documenting the licensing model for a future Enterprise Edition ahead of any actual feature split. The existing MIT LICENSE and all currently released code are unchanged; nothing in this repository is restricted by this addition.

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
