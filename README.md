<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>Entra Least-Privilege Analyzer</h1>
</div>

> 🇩🇪 [Deutsche Version](README.de.md)

**Read-only Rust CLI for analyzing Entra ID privilege configurations, detecting over-privileged accounts, role overlap and PIM gaps.**

Entra Least-Privilege Analyzer connects to Microsoft Graph API using application credentials and produces a structured privilege report. Entirely read-only, no data leaves your machine.

Built for Zero Trust environments. Aligned with the [Microsoft Cloud Security Benchmark (MCSB)](https://learn.microsoft.com/en-us/security/benchmark/azure/overview) identity controls and Microsoft Secure Score recommendations.

[![CI](https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer/actions) ![Microsoft | Entra ID](https://img.shields.io/badge/Microsoft-Entra_ID-0078d4?logo=microsoftazure&logoColor=white) ![Platform](https://img.shields.io/badge/Platform-Windows_%7C_Ubuntu-lightgrey) ![Rust](https://img.shields.io/badge/Rust-CE422B?logo=rust&logoColor=white) ![AI | Claude Code](https://img.shields.io/badge/AI-Claude_Code-black?logo=anthropic&logoColor=white) ![AI | Copilot](https://img.shields.io/badge/AI-Copilot-black?logo=github&logoColor=white) [![Release](https://img.shields.io/github/v/release/9t29zhmwdh-coder/entra-least-privilege-analyzer?color=3F8E7E)](https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer/releases) [![License](https://img.shields.io/github/license/9t29zhmwdh-coder/entra-least-privilege-analyzer?color=lightgrey)](LICENSE)

---

## Features

| Capability | Description |
|---|---|
| Privilege scoring | Assigns weighted scores to accounts based on held roles |
| Over-privileged account detection | Flags accounts exceeding configurable score thresholds |
| Role overlap analysis | Identifies accounts holding redundant or conflicting roles |
| PIM gap detection | Detects permanent high-privilege assignments, weak PIM settings |
| PIM settings audit | Checks MFA requirement, justification, max activation duration |
| JSON / Markdown export | Structured output for ticketing, audits, and documentation |
| SARIF stub | Prepared for GitHub Advanced Security integration (v0.2) |

---

## Required Graph API Permissions

Register an application in Entra ID with the following **application permissions** (not delegated):

| Permission | Purpose |
|---|---|
| `Directory.Read.All` | Read users and group memberships |
| `RoleManagement.Read.All` | Read role definitions and assignments |
| `PrivilegedAccess.Read.AzureAD` | Read PIM eligible and active assignments |
| `Policy.Read.All` | Read role management policies and PIM settings |

All permissions are **read-only**. No write permissions are required or used.

---

## App Registration Setup

1. Open [Azure Portal](https://portal.azure.com) and navigate to **Entra ID > App registrations > New registration**
2. Name the application (e.g. `elpa-analyzer`) and register
3. Go to **API permissions** and add the four permissions listed above
4. Grant admin consent for your tenant
5. Go to **Certificates & secrets > New client secret** and copy the value
6. Note your **Tenant ID**, **Client ID**, and **Client Secret**

---

## Quick Start

```bash
git clone https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer
cd entra-least-privilege-analyzer

# Copy and fill in your credentials
cp .env.example .env

cargo build --release

# Full analysis
./target/release/elpa analyze

# PIM-only analysis
./target/release/elpa pim

# Export as Markdown
./target/release/elpa export --format md --output report.md

# Export as JSON
./target/release/elpa export --format json --output report.json
```

---

## Configuration

Create a `.env` file in the project root:

```env
ENTRA_TENANT_ID=your-tenant-id
ENTRA_CLIENT_ID=your-client-id
ENTRA_CLIENT_SECRET=your-client-secret
```

The `.env` file is listed in `.gitignore`. Credentials are never committed.

### Alternative: Bring your own token

If you already hold a valid Microsoft Graph access token (e.g. issued to an
admin portal via delegated auth), you can skip the client-credentials flow:

```env
ENTRA_TENANT_ID=your-tenant-id
ENTRA_ACCESS_TOKEN=eyJ0eXAi...
```

The token is used as-is and never refreshed — suitable for one-shot runs
where the caller manages token lifetime. `ENTRA_CLIENT_ID`/`ENTRA_CLIENT_SECRET`
are not required in this mode.


---

## Findings Severity

| Level | Meaning | Examples |
|---|---|---|
| Critical | Immediate remediation required | Permanent Global Admin without PIM |
| High | Remediate in next sprint | PIM without MFA, over-privileged account |
| Medium | Remediate within 30 days | Role overlap, long PIM activation window |
| Low | Best practice improvement | Missing justification requirement |

---

## Sample Output

```
=== Entra Least-Privilege Analyzer ===

Users: 142  Assignments: 38  Findings: 5

Findings:
+----------+------------------------------------------------------+-----------+
| Severity | Title                                                | Affected  |
+----------+------------------------------------------------------+-----------+
| CRITICAL | Permanent high-privilege roles without PIM           | 2         |
| HIGH     | Over-privileged account: admin@contoso.com           | 1         |
| HIGH     | PIM activation for 'Global Admin' lacks MFA          | 1         |
| MEDIUM   | Role overlap detected for ops@contoso.com            | 1         |
| LOW      | PIM activation missing justification requirement     | 3         |
+----------+------------------------------------------------------+-----------+
```

See `reports/sample_report.md` for a full annotated example.

---

## Requirements

- Rust 1.78+
- Entra ID tenant with an app registration
- Network access to `login.microsoftonline.com` and `graph.microsoft.com`

---

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Active · v0.2.0 · **License:** MIT
