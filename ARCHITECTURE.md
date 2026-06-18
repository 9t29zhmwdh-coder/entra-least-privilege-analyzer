# Architecture

## Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     elpa-cli (binary: elpa)                      │
│    analyze | pim | export                                        │
└────────────────────┬────────────────────────────────────────────┘
                     │
          ┌──────────┴──────────┐
          │                     │
┌─────────▼──────────┐  ┌───────▼────────────────────────────┐
│    elpa-graph       │  │           elpa-core                 │
│                     │  │                                     │
│  GraphClient        │  │  analyzer::build_analysis_result   │
│    acquire_token()  │  │  analyzer::compute_privilege_scores│
│    get()            │  │  analyzer::detect_overprivileged   │
│    get_all_pages()  │  │  analyzer::find_role_overlap       │
│                     │  │  analyzer::analyze_pim             │
│  users::list_users  │  │                                     │
│  roles::list_*      │  │  report::to_json                   │
│  pim::list_*        │  │  report::to_markdown               │
│  pim::get_settings  │  │  report::to_sarif_stub             │
└─────────┬───────────┘  └───────────────────────────────────┘
          │
          │ HTTPS (read-only)
          │
┌─────────▼───────────────────────────────────────────────────┐
│                  Microsoft Graph API                          │
│                                                               │
│  /users                                                       │
│  /roleManagement/directory/roleAssignments                   │
│  /roleManagement/directory/roleEligibilitySchedules          │
│  /roleManagement/directory/roleAssignmentSchedules           │
│  /policies/roleManagementPolicyAssignments                   │
└──────────────────────────────────────────────────────────────┘
```

## Data Flow

1. CLI parses command and reads credentials from environment variables
2. `GraphClient::from_env()` constructs the client
3. `acquire_token()` performs OAuth2 client credentials flow against `login.microsoftonline.com`
4. Graph API endpoints are called with paginated reads via `get_all_pages()`
5. Raw Graph responses are mapped into `elpa-core` model types
6. `analyzer::build_analysis_result()` computes scores and gaps
7. Output is rendered as table (stdout), JSON, or Markdown

## Crate Responsibilities

| Crate | Responsibility |
|---|---|
| `elpa-core` | Domain types, analysis logic, report generation. No I/O. |
| `elpa-graph` | Microsoft Graph API client, OAuth2 token acquisition, endpoint calls. |
| `elpa-cli` | CLI parsing, orchestration, terminal rendering. |

## Security Boundary

All network traffic is outbound HTTPS to Microsoft endpoints only. No data is forwarded elsewhere. The `elpa-core` crate has no network dependencies.
