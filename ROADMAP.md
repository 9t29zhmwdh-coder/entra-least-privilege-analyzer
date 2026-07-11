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

## Dual-Licensing Readiness

Assessed 2026-07-11 as a Dual-Licensing candidate (Community MIT + Commercial/Enterprise tier): least-privilege and PIM auditing for Entra ID sits directly alongside Microsoft Secure Score and Defender for Identity, both already roadmap targets here, and is a well-established enterprise security spend category. Not ready yet; blocked on:

- [ ] No multi-tenant support yet: MSPs and consultancies auditing multiple customer tenants are a natural Commercial-tier audience
- [ ] No stable API for CI/CD pipeline integration yet (v1.0.0 item above): a Commercial tier's core value here is usually turnkey pipeline gating, not a one-off CLI run
- [ ] Microsoft Secure Score export and Defender for Identity integration are still only roadmap entries, not implemented
- [ ] No server or API component to gate a Commercial tier against: today this is a pure local CLI with no persistence layer

Once multi-tenant support and the Secure Score/Defender for Identity integrations (v1.0.0) land, revisit: candidate Enterprise-only features would be multi-tenant scanning, Secure Score/Defender export, and CI/CD pipeline gating, with the core scoring, overlap-detection and PIM-audit engine staying Community/MIT.
