# Roadmap

## v0.1.0 (current)

- Privilege scoring engine
- Over-privileged account detection
- Role overlap analysis
- PIM gap detection (permanent assignments, weak settings)
- PIM settings audit (MFA, justification, duration)
- JSON and Markdown export
- CI on Ubuntu and Windows

## v0.2.0

- Conditional Access gap analysis (identify roles not covered by CA policies)
- SARIF export for GitHub Advanced Security
- Group-based role assignment detection
- HTML report template

## v0.3.0

- Privileged Identity Management activation history analysis
- Service principal / managed identity privilege audit
- Comparison mode (diff between two analysis snapshots)

## v1.0.0

- Stable API for integration into CI/CD pipelines
- Full benchmark against Microsoft Cloud Security Benchmark (MCSB) identity controls
- Microsoft Defender for Identity integration (correlate privilege gaps with threat detections)
- Microsoft Secure Score export (map findings to Secure Score improvement actions)
- Entra External ID support
