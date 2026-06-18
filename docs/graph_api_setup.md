# Microsoft Graph API Setup

## Step 1: Register an application in Entra ID

1. Open the [Azure Portal](https://portal.azure.com)
2. Navigate to **Entra ID > App registrations > New registration**
3. Set a name (e.g. `elpa-analyzer`)
4. Select **Accounts in this organizational directory only**
5. Leave the Redirect URI empty
6. Click **Register**

## Step 2: Add API permissions

1. Go to **API permissions > Add a permission > Microsoft Graph > Application permissions**
2. Add the following permissions:

| Permission | Type | Purpose |
|---|---|---|
| `Directory.Read.All` | Application | Users and group memberships |
| `RoleManagement.Read.All` | Application | Role definitions and assignments |
| `PrivilegedAccess.Read.AzureAD` | Application | PIM eligible and active assignments |
| `Policy.Read.All` | Application | Role management policies |

3. Click **Grant admin consent for [your tenant]**

## Step 3: Create a client secret

1. Go to **Certificates & secrets > New client secret**
2. Set an expiry (recommend 6 months for security tools)
3. Copy the **Value** immediately — it will not be shown again

## Step 4: Note your credentials

Find these values on the application overview page:

- **Tenant ID** — shown as "Directory (tenant) ID"
- **Client ID** — shown as "Application (client) ID"
- **Client Secret** — the value you copied in Step 3

## Step 5: Configure the tool

Create a `.env` file:

```env
ENTRA_TENANT_ID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
ENTRA_CLIENT_ID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
ENTRA_CLIENT_SECRET=your-secret-value
```

The `.env` file is excluded from git via `.gitignore`.
