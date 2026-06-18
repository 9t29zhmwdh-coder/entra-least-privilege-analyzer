# Security Policy

## Supported Versions

| Version | Supported |
|---|---|
| 0.1.x | Yes |

## Reporting a Vulnerability

To report a security vulnerability, please open a GitHub issue with the label `security`.

Do not include tenant IDs, credentials, or personal data in the report.

I will acknowledge receipt within 72 hours and aim to provide a fix or mitigation within 14 days for confirmed vulnerabilities.

## Security Design Principles

- **Read-only by design.** The tool only uses read-only Microsoft Graph API permissions. No write operations are performed at any time.
- **Credentials via environment variables only.** No credentials are stored in code, configuration files tracked by git, or log output.
- **No data exfiltration.** All API responses are processed locally. No data is forwarded to external services.
- **Minimal permission scope.** The tool requests only the four permissions required for analysis. No broader scopes are used.
- **No persistent storage.** Analysis results are written only to files explicitly specified by the user via `--output`.
