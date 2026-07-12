# Enterprise Features

This document lists features planned for the Enterprise Edition of this
project, licensed separately under
[LICENSE.COMMERCIAL](LICENSE.COMMERCIAL). See [COMMERCIAL.md](COMMERCIAL.md)
for the licensing model.

## Status

No Enterprise features have shipped yet. This list is a forward-looking plan,
not a changelog of existing functionality: everything currently in this
repository is part of the Community Edition and remains MIT-licensed. See the
repository's own [ROADMAP.md](ROADMAP.md), "Dual-Licensing Readiness"
section, for the prerequisites that need to land first.

## Planned

- Multi-tenant scanning: auditing multiple customer Entra ID tenants from a
  single install, for MSPs and consultancies.
- Microsoft Secure Score export and Defender for Identity integration:
  correlating least-privilege findings with an organization's existing
  security posture tooling.
- CI/CD pipeline gating: a stable API for automated pipeline integration,
  instead of a one-off CLI run.

## Not planned

The core scoring, overlap-detection, and PIM-audit engine stay in the
Community Edition permanently. Dual-licensing governs only new, enterprise-
shaped capabilities such as the ones listed above, not the tool's standalone
usefulness for a single tenant.
