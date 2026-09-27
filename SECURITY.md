# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| Latest  | ✅ Yes    |
| Older   | ❌ No     |

Security fixes are only applied to the latest release.

## Reporting a Vulnerability

**Do NOT open a public GitHub issue for security vulnerabilities.**

Instead, report it privately via [GitHub Security Advisory](https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer/security/advisories/new) or contact the maintainer via the GitHub profile.

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

A response within **48 hours** is the target, and the issue will be worked on promptly.

## Security Design Principles

- **Read-only by design.** The tool only uses read-only Microsoft Graph API permissions. No write operations are performed at any time.
- **Credentials via environment variables only.** No credentials are stored in code, configuration files tracked by git, or log output.
- **No data exfiltration.** All API responses are processed locally. No data is forwarded to external services.
- **Minimal permission scope.** The tool requests only the four permissions required for analysis. No broader scopes are used.
- **No persistent storage.** Analysis results are written only to files explicitly specified by the user via `--output`.
