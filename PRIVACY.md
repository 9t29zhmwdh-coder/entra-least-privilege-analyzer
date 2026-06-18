# Privacy Statement

## Data Handling

Entra Least-Privilege Analyzer is a **fully local, read-only tool**. The following principles apply:

- **No data leaves your machine.** API responses from Microsoft Graph are processed in memory and never forwarded to any external service.
- **No telemetry.** The tool does not collect, transmit, or store usage data of any kind.
- **No persistent storage by default.** Analysis results are only written to disk when the user explicitly uses `--output`.
- **Credentials are not logged.** Client secrets and tokens are never written to stdout, stderr, or log files.

## Data Accessed via Graph API

The tool reads the following data from your Entra ID tenant:

| Data type | Purpose |
|---|---|
| User display names and UPNs | Identifying principals in findings |
| Role assignments | Core analysis input |
| PIM eligibility schedules | PIM gap detection |
| Role management policies | PIM settings audit |

This data is processed in memory for the duration of the analysis and discarded afterwards unless written to an output file by the user.

## Compliance Considerations

When exporting reports containing user principal names or role data, treat the output as sensitive internal data in accordance with your organization's data classification policy.
