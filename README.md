# xero-cli

A fast, intelligent command-line interface for the Xero Accounting API.

## Installation

### Homebrew (macOS/Linux)

```bash
brew install osodevops/tap/xero-cli
```

### Cargo

```bash
cargo install --git https://github.com/osodevops/xero-cli
```

### GitHub Releases

Download pre-built binaries from the [Releases](https://github.com/osodevops/xero-cli/releases) page.

## Quick Start

### 1. Register a Xero App

Create an app at [developer.xero.com](https://developer.xero.com/app/manage) with:
- **App type:** Web app
- **Redirect URI:** `http://localhost:8080/callback`

### 2. Configure

```bash
xero config init
xero config set auth.client_id YOUR_CLIENT_ID
```

Or use environment variables:

```bash
export XERO_CLIENT_ID=your_client_id
```

### 3. Authenticate

```bash
# Interactive login (opens browser)
xero auth login

# Check status
xero auth status

# For CI/CD (client credentials)
xero auth setup-m2m --client-id ID --client-secret SECRET
```

### 4. Use

```bash
# List invoices
xero invoices list
xero invoices list --status AUTHORISED --output json

# Get a specific invoice
xero invoices get INV-ID

# Create an invoice
xero invoices create --contact "Acme Corp" --line-item "Consulting,10,150.00"

# List contacts
xero contacts list --search "Acme"

# View reports
xero reports profit-and-loss --from 2026-01-01 --to 2026-03-31
xero reports balance-sheet

# List accounts
xero accounts list --type REVENUE
```

## Output Formats

```bash
# Table (default for terminal)
xero invoices list

# JSON (default for pipes)
xero invoices list --output json

# CSV
xero invoices list --output csv

# YAML
xero invoices list --output yaml

# Compact JSON
xero invoices list --output json --compact
```

## Configuration

Config file: `~/.config/xero-cli/config.toml`

```toml
[default]
active_profile = "production"
output_format = "table"
page_size = 100

[auth]
client_id = "your_client_id"
callback_port = 8080
scopes = ["openid", "offline_access", "accounting.invoices.read", "accounting.contacts.read"]

[rate_limit]
calls_per_minute = 60
daily_limit = 5000
max_concurrent = 5

[profiles.production]
tenant_id = "your-tenant-id"
org_name = "My Company"
```

## Shell Completions

```bash
# Bash
xero completions bash > ~/.bash_completion.d/xero

# Zsh
xero completions zsh > ~/.zfunc/_xero

# Fish
xero completions fish > ~/.config/fish/completions/xero.fish
```

## Environment Variables

| Variable | Description |
|---|---|
| `XERO_CLIENT_ID` | OAuth2 client ID |
| `XERO_CLIENT_SECRET` | OAuth2 client secret (M2M only) |
| `XERO_ACCESS_TOKEN` | Direct access token (bypasses auth) |
| `XERO_TENANT_ID` | Xero tenant/organisation ID |
| `XERO_PROFILE` | Active config profile name |
| `XERO_OUTPUT` | Default output format |
| `XERO_CONFIG` | Config file path |

## License

MIT
