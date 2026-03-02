# Product Requirements Document: `xero-cli`

## A Fast, Intelligent Command-Line Interface for Xero Accounting

**Version:** 1.0.0-draft
**Author:** [Your Name]
**Date:** 2 March 2026
**Language:** Rust
**License:** MIT

---

## 1. Executive Summary

`xero-cli` is a Rust-based command-line tool for interacting with the Xero Accounting API. It provides a fast, single-binary CLI that handles OAuth2 authentication (PKCE + Client Credentials), intelligent rate limiting, local caching, and pipeline-friendly output formats. It is designed for developers, bookkeepers, accountants, and automation workflows that need programmatic access to Xero data without the overhead of building a full integration.

### Why This Exists

There is **no dedicated CLI tool** for Xero accounting operations today. The closest tools are:

- **xoauth** (by XeroAPI): Only handles OAuth token management — no data operations. Last updated 2020.
- **slickbench/xero-rs**: A Rust library (not a CLI) with very limited endpoint coverage.
- **pyxero**: A Python SDK, not a CLI, and community-maintained with stale OAuth1 patterns.
- **CData PowerShell Cmdlets**: Commercial, Windows-focused, SQL-like interface.

Developer pain points with Xero's API are well-documented: strict rate limits (60 calls/min, 5,000/day), 30-minute token expiry, painful pagination, no bank reconciliation API, limited aging report data, confusing scope requirements, and a new paid tiered pricing model launching March 2026. `xero-cli` addresses all of these at the CLI layer.

---

## 2. Goals & Non-Goals

### Goals

- Provide complete CLI coverage of the Xero Accounting API (and progressively: Assets, Files, Projects)
- Handle OAuth2 PKCE flow natively for interactive use and Client Credentials for automation/CI
- Implement intelligent rate limiting with automatic backoff and budget tracking
- Support multiple output formats: JSON, CSV, table, YAML — designed for piping to `jq`, `xsv`, `awk`, etc.
- Enable multi-organisation management from a single CLI session
- Provide local caching with `If-Modified-Since` support to minimize API calls
- Future-proof for Xero's upcoming granular scope changes (April 2026) and paid API tiers
- Ship as a single static binary for macOS, Linux, and Windows

### Non-Goals

- This is NOT a full accounting application — it is a data access and manipulation tool
- No GUI or TUI (text user interface) in v1 — pure CLI
- No attempt to replicate bank reconciliation (Xero deliberately blocks this via API)
- No Payroll API support in v1 (regional complexity — AU/NZ/UK have different APIs)
- Not an MCP server (Xero already provides one) — though output could feed one

---

## 3. Target Users

| Persona | Use Case |
|---|---|
| **Developer** | Build integrations, test API calls, automate data extraction |
| **Bookkeeper / Accountant** | Quick lookups, bulk invoice creation, report generation from terminal |
| **DevOps / SRE** | Automated financial data sync in CI/CD, scheduled exports, monitoring |
| **Small Business Owner** | Simple invoice/contact management without opening Xero UI |
| **Data Analyst** | Extract financial data to CSV/JSON for analysis in other tools |

---

## 4. Architecture

### 4.1 High-Level Design

```
┌─────────────────────────────────────────────────────┐
│                    xero-cli binary                    │
├──────────┬──────────┬───────────┬───────────────────┤
│  CLI     │  Auth    │  API      │  Output            │
│  Parser  │  Engine  │  Client   │  Formatter          │
│  (clap)  │  (OAuth2)│  (reqwest)│  (table/json/csv)  │
├──────────┴──────────┴───────────┴───────────────────┤
│                   Core Services                       │
├──────────┬──────────┬───────────┬───────────────────┤
│  Rate    │  Cache   │  Config   │  Scope              │
│  Limiter │  Layer   │  Manager  │  Manager            │
├──────────┴──────────┴───────────┴───────────────────┤
│                   Token Store                         │
│            (OS Keychain / encrypted file)             │
└─────────────────────────────────────────────────────┘
```

### 4.2 Crate Dependencies (Recommended)

| Crate | Purpose |
|---|---|
| `clap` (v4, derive) | CLI argument parsing with subcommands |
| `reqwest` | HTTP client (async, rustls) |
| `tokio` | Async runtime |
| `serde` / `serde_json` | Serialization/deserialization |
| `oauth2` | OAuth2 PKCE + Client Credentials flows |
| `keyring` | OS keychain integration for token storage |
| `chrono` | Date/time handling |
| `rust_decimal` | Financial decimal precision (never use floats for money) |
| `tabled` / `comfy-table` | Terminal table rendering |
| `csv` | CSV output |
| `dirs` | XDG-compliant config/cache directory resolution |
| `miette` or `color-eyre` | Rich error diagnostics |
| `indicatif` | Progress bars for long operations |
| `tracing` | Structured logging |
| `toml` | Config file parsing |

### 4.3 Project Structure

```
xero-cli/
├── Cargo.toml
├── src/
│   ├── main.rs                  # Entry point, CLI dispatch
│   ├── cli/
│   │   ├── mod.rs               # CLI definition (clap App)
│   │   ├── auth.rs              # Auth subcommands
│   │   ├── invoices.rs          # Invoice subcommands
│   │   ├── contacts.rs          # Contact subcommands
│   │   ├── accounts.rs          # Account subcommands
│   │   ├── payments.rs          # Payment subcommands
│   │   ├── bank_transactions.rs # Bank transaction subcommands
│   │   ├── reports.rs           # Report subcommands
│   │   ├── journals.rs          # Journal subcommands
│   │   ├── items.rs             # Item subcommands
│   │   ├── quotes.rs            # Quote subcommands
│   │   ├── purchase_orders.rs   # PO subcommands
│   │   ├── credit_notes.rs      # Credit note subcommands
│   │   ├── org.rs               # Organisation subcommands
│   │   ├── config.rs            # Config subcommands
│   │   └── common.rs            # Shared CLI args (output format, pagination, etc.)
│   ├── api/
│   │   ├── mod.rs               # API client (XeroClient)
│   │   ├── client.rs            # HTTP client with rate limiting, retry, auth
│   │   ├── endpoints/
│   │   │   ├── mod.rs
│   │   │   ├── invoices.rs
│   │   │   ├── contacts.rs
│   │   │   ├── accounts.rs
│   │   │   ├── payments.rs
│   │   │   ├── bank_transactions.rs
│   │   │   ├── bank_transfers.rs
│   │   │   ├── batch_payments.rs
│   │   │   ├── credit_notes.rs
│   │   │   ├── currencies.rs
│   │   │   ├── employees.rs
│   │   │   ├── expense_claims.rs
│   │   │   ├── items.rs
│   │   │   ├── journals.rs
│   │   │   ├── linked_transactions.rs
│   │   │   ├── manual_journals.rs
│   │   │   ├── organisation.rs
│   │   │   ├── overpayments.rs
│   │   │   ├── prepayments.rs
│   │   │   ├── purchase_orders.rs
│   │   │   ├── quotes.rs
│   │   │   ├── receipts.rs
│   │   │   ├── repeating_invoices.rs
│   │   │   ├── reports.rs
│   │   │   ├── tax_rates.rs
│   │   │   ├── tracking_categories.rs
│   │   │   ├── users.rs
│   │   │   ├── budgets.rs
│   │   │   ├── attachments.rs
│   │   │   ├── branding_themes.rs
│   │   │   ├── contact_groups.rs
│   │   │   └── payment_services.rs
│   │   ├── pagination.rs        # Auto-pagination logic
│   │   └── types.rs             # Shared API types/models
│   ├── auth/
│   │   ├── mod.rs
│   │   ├── pkce.rs              # PKCE flow (interactive)
│   │   ├── client_credentials.rs# Client credentials flow (M2M)
│   │   ├── token_store.rs       # Secure token storage (keychain + file fallback)
│   │   └── refresh.rs           # Automatic token refresh
│   ├── cache/
│   │   ├── mod.rs
│   │   ├── store.rs             # Local SQLite/file cache
│   │   └── strategy.rs          # Cache invalidation + If-Modified-Since
│   ├── rate_limit/
│   │   ├── mod.rs
│   │   ├── limiter.rs           # Token bucket / sliding window
│   │   ├── backoff.rs           # Exponential backoff with jitter
│   │   └── budget.rs            # Daily/minute budget tracker
│   ├── output/
│   │   ├── mod.rs
│   │   ├── json.rs              # JSON output (pretty + compact)
│   │   ├── csv.rs               # CSV output
│   │   ├── table.rs             # Terminal table output
│   │   └── yaml.rs              # YAML output
│   ├── config/
│   │   ├── mod.rs
│   │   ├── file.rs              # TOML config file management
│   │   └── profiles.rs          # Multi-org profile management
│   ├── models/
│   │   ├── mod.rs               # Re-exports
│   │   ├── invoice.rs
│   │   ├── contact.rs
│   │   ├── account.rs
│   │   ├── payment.rs
│   │   ├── bank_transaction.rs
│   │   ├── journal.rs
│   │   ├── report.rs
│   │   ├── item.rs
│   │   ├── quote.rs
│   │   ├── purchase_order.rs
│   │   ├── credit_note.rs
│   │   ├── organisation.rs
│   │   ├── employee.rs
│   │   ├── tax_rate.rs
│   │   ├── tracking_category.rs
│   │   ├── currency.rs
│   │   └── common.rs            # Shared types (Address, Phone, LineItem, etc.)
│   └── error.rs                 # Unified error types (miette diagnostics)
├── tests/
│   ├── integration/
│   │   ├── auth_test.rs
│   │   ├── invoices_test.rs
│   │   └── ...
│   └── fixtures/                # JSON response fixtures for testing
├── docs/
│   ├── AUTHENTICATION.md
│   ├── RATE_LIMITING.md
│   └── EXAMPLES.md
└── .github/
    └── workflows/
        └── ci.yml               # Build + test + release binaries
```

---

## 5. Authentication

### 5.1 PKCE Flow (Interactive / Default)

The primary auth method for interactive CLI use. Xero supports PKCE which is ideal for native/desktop apps with no client secret required.

```bash
# First-time setup — opens browser for consent
xero auth login

# With specific scopes
xero auth login --scopes "accounting.transactions,accounting.contacts.read"

# Check current auth status
xero auth status

# Force token refresh
xero auth refresh

# Logout / revoke
xero auth logout
```

**Implementation details:**
- Spin up a temporary local HTTP server on `localhost:<port>` to receive the callback
- Auto-open the browser to Xero's authorization URL
- Use PKCE code verifier/challenge (S256)
- Store tokens securely in OS keychain via `keyring` crate
- Fallback to encrypted file in `~/.config/xero-cli/tokens.enc` if keychain unavailable
- Access tokens expire after 30 minutes — auto-refresh using refresh token before expiry
- Display remaining token lifetime in `auth status`

### 5.2 Client Credentials (M2M / Automation)

For Custom Connections (single-org, machine-to-machine). No browser interaction needed.

```bash
# Configure client credentials
xero auth setup-m2m --client-id <ID> --client-secret <SECRET>

# Or via environment variables
XERO_CLIENT_ID=xxx XERO_CLIENT_SECRET=yyy xero invoices list
```

**Implementation details:**
- No refresh tokens needed — request new access token using client_id + client_secret
- No xero-tenant-id header required (single org)
- Ideal for CI/CD pipelines and cron jobs
- Client secret stored in OS keychain

### 5.3 Scope Management

Xero is transitioning from broad to granular scopes (April 2026, broad scopes deprecated September 2027). The CLI must be future-proof.

```bash
# View current scopes
xero auth scopes

# Add scopes (triggers re-auth)
xero auth scopes add accounting.transactions.read

# Use scope presets
xero auth scopes preset read-only
xero auth scopes preset full-access
xero auth scopes preset bookkeeper
```

**Scope presets (built-in):**

| Preset | Scopes |
|---|---|
| `read-only` | `openid`, `offline_access`, `accounting.transactions.read`, `accounting.contacts.read`, `accounting.settings.read`, `accounting.reports.read`, `accounting.journals.read`, `accounting.attachments.read` |
| `bookkeeper` | read-only + `accounting.transactions`, `accounting.contacts`, `accounting.attachments` |
| `full-access` | All available accounting scopes |
| `reports-only` | `openid`, `offline_access`, `accounting.reports.read`, `accounting.settings.read` |

---

## 6. Command Structure

### 6.1 Global Flags

```
xero [GLOBAL FLAGS] <COMMAND> [COMMAND FLAGS]

Global Flags:
  --profile <name>       Use a specific org profile (default: "default")
  --output <format>      Output format: json, csv, table, yaml (default: table)
  --compact              Compact JSON output (no pretty-printing)
  --no-cache             Bypass local cache for this request
  --no-color             Disable colored output
  --verbose              Enable verbose logging
  --quiet                Suppress all non-data output
  --dry-run              Show the API request that would be made without executing
  --page-size <n>        Results per page: 1-1000 (default: 100)
  --all-pages            Auto-paginate and return all results
  --modified-since <dt>  Only return records modified after this ISO8601 datetime
  --config <path>        Path to config file (default: ~/.config/xero-cli/config.toml)
```

### 6.2 Resource Commands

Each API resource follows a consistent CRUD pattern:

```
xero <resource> list [FLAGS]           # GET all (with filters)
xero <resource> get <ID>               # GET by ID
xero <resource> create [FLAGS|--file]  # PUT (create new)
xero <resource> update <ID> [FLAGS]    # POST (update existing)
xero <resource> delete <ID>            # DELETE (where supported)
xero <resource> history <ID>           # GET history/notes
xero <resource> attachments <ID>       # List/manage attachments
```

### 6.3 Full Command Reference

#### Auth
```bash
xero auth login                    # Interactive PKCE login
xero auth login --port 9090        # Custom callback port
xero auth setup-m2m                # Configure client credentials
xero auth status                   # Show auth status, token expiry, org info
xero auth refresh                  # Force token refresh
xero auth logout                   # Clear stored tokens
xero auth scopes                   # List current scopes
xero auth scopes add <scope>       # Add scope (triggers re-auth)
xero auth scopes preset <name>     # Apply scope preset
```

#### Organisations
```bash
xero org list                      # List connected organisations (tenants)
xero org switch <name|id>          # Switch active organisation
xero org info                      # Show current organisation details
xero org actions                   # Show available actions
```

#### Config
```bash
xero config init                   # Interactive config setup
xero config show                   # Show current config
xero config set <key> <value>      # Set config value
xero config profiles list          # List org profiles
xero config profiles add <name>    # Add new profile
xero config profiles remove <name> # Remove profile
```

#### Invoices
```bash
xero invoices list                              # List all invoices
xero invoices list --status AUTHORISED           # Filter by status
xero invoices list --contact <name|id>           # Filter by contact
xero invoices list --from 2025-01-01 --to 2025-12-31  # Date range
xero invoices list --where "AmountDue > 0"       # Custom where clause
xero invoices list --order "DueDate DESC"        # Custom ordering
xero invoices get <InvoiceID>                    # Get single invoice
xero invoices create --file invoice.json         # Create from file
xero invoices create --contact "Acme" \
  --line-item "Consulting,2,150.00" \
  --due-date 2026-04-01                          # Create inline
xero invoices update <ID> --status VOIDED        # Update invoice
xero invoices email <ID>                         # Email invoice via Xero
xero invoices online-url <ID>                    # Get online invoice URL
xero invoices history <ID>                       # Get history
xero invoices attachments list <ID>              # List attachments
xero invoices attachments upload <ID> <file>     # Upload attachment
xero invoices attachments download <ID> <name>   # Download attachment
```

#### Contacts
```bash
xero contacts list
xero contacts list --search "Acme"               # Search by name
xero contacts list --where "IsCustomer=true"
xero contacts get <ContactID>
xero contacts create --name "New Co" --email "a@b.com" --tax-number "GB123456789"
xero contacts update <ID> --name "Updated Name"
xero contacts history <ID>
xero contacts attachments list <ID>
```

#### Accounts
```bash
xero accounts list
xero accounts list --type REVENUE                # Filter by type
xero accounts list --class ASSET                 # Filter by class
xero accounts get <AccountID>
xero accounts create --name "Travel" --code 400 --type EXPENSE
xero accounts archive <AccountID>
xero accounts attachments list <ID>
```

#### Payments
```bash
xero payments list
xero payments list --invoice <InvoiceID>
xero payments get <PaymentID>
xero payments create --invoice <InvoiceID> --account <AccountID> --amount 500.00 --date 2026-03-01
xero payments delete <PaymentID>
xero payments history <ID>
```

#### Bank Transactions
```bash
xero bank-transactions list
xero bank-transactions list --account <AccountID>
xero bank-transactions list --from 2025-01-01
xero bank-transactions get <ID>
xero bank-transactions create --file transaction.json
xero bank-transactions delete <ID>
xero bank-transactions history <ID>
```

#### Bank Transfers
```bash
xero bank-transfers list
xero bank-transfers get <ID>
xero bank-transfers create --from-account <ID> --to-account <ID> --amount 1000.00
```

#### Credit Notes
```bash
xero credit-notes list
xero credit-notes get <ID>
xero credit-notes create --file credit-note.json
xero credit-notes allocate <ID> --invoice <InvoiceID> --amount 50.00
xero credit-notes history <ID>
```

#### Items (Inventory)
```bash
xero items list
xero items list --where "Name.Contains(\"Widget\")"
xero items get <ItemID>
xero items create --code "WDG-001" --name "Widget" --sale-price 29.99
xero items update <ID> --sale-price 34.99
xero items delete <ID>
xero items history <ID>
```

#### Journals
```bash
xero journals list                             # Auto-paginate (100/page limit)
xero journals list --from 2025-01-01
xero journals get <JournalID>
```

#### Manual Journals
```bash
xero manual-journals list
xero manual-journals get <ID>
xero manual-journals create --file journal.json
xero manual-journals update <ID> --file journal.json
```

#### Purchase Orders
```bash
xero purchase-orders list
xero purchase-orders list --status AUTHORISED
xero purchase-orders get <ID>
xero purchase-orders create --contact "Supplier Co" --file po-lines.json
xero purchase-orders history <ID>
```

#### Quotes
```bash
xero quotes list
xero quotes list --status DRAFT
xero quotes get <ID>
xero quotes create --file quote.json
xero quotes update <ID> --expiry-date 2026-06-01
```

#### Reports
```bash
xero reports profit-and-loss                    # P&L current period
xero reports profit-and-loss --from 2025-01-01 --to 2025-12-31
xero reports balance-sheet                      # Balance sheet
xero reports balance-sheet --date 2025-12-31
xero reports trial-balance
xero reports bank-summary
xero reports budget-summary
xero reports executive-summary
xero reports aged-payables --contact <ContactID>
xero reports aged-receivables --contact <ContactID>
xero reports aged-payables --all-contacts        # Aggregate (workaround for API limitation)
xero reports aged-receivables --all-contacts      # Aggregate (workaround for API limitation)
```

> **Note on Aged Reports:** The Xero API only returns totals per contact without age breakdown via the summary endpoint. Our `--all-contacts` flag iterates individual contacts and aggregates the per-contact aged data into a proper aging report with 30/60/90/120+ day buckets. This addresses a major developer pain point.

#### Tax Rates
```bash
xero tax-rates list
xero tax-rates create --name "Reduced VAT" --rate 5.0 --components ...
xero tax-rates update <TaxType> --status ACTIVE
```

#### Tracking Categories
```bash
xero tracking list
xero tracking get <ID>
xero tracking create --name "Department"
xero tracking update <ID> --name "Division"
xero tracking options add <CategoryID> --name "Engineering"
xero tracking options remove <CategoryID> <OptionID>
```

#### Currencies
```bash
xero currencies list
```

#### Users
```bash
xero users list
xero users get <UserID>
```

#### Budgets
```bash
xero budgets list
xero budgets get <BudgetID>
```

#### Expense Claims
```bash
xero expense-claims list
xero expense-claims get <ID>
xero expense-claims create --file claim.json
xero expense-claims update <ID> --status AUTHORISED
xero expense-claims history <ID>
```

#### Receipts
```bash
xero receipts list
xero receipts get <ID>
xero receipts create --file receipt.json
xero receipts history <ID>
xero receipts attachments upload <ID> receipt.pdf
```

#### Repeating Invoices
```bash
xero repeating-invoices list
xero repeating-invoices get <ID>
xero repeating-invoices create --file template.json
xero repeating-invoices history <ID>
```

#### Branding Themes
```bash
xero branding-themes list
xero branding-themes get <ID>
xero branding-themes payment-services <ID>         # List payment services
xero branding-themes payment-services add <ID> <ServiceID>  # Add payment service
```

#### Batch Payments
```bash
xero batch-payments list
xero batch-payments get <ID>
xero batch-payments create --file batch.json
xero batch-payments delete <ID>
```

#### Overpayments
```bash
xero overpayments list
xero overpayments get <ID>
xero overpayments allocate <ID> --invoice <InvoiceID> --amount 100.00
xero overpayments history <ID>
```

#### Prepayments
```bash
xero prepayments list
xero prepayments get <ID>
xero prepayments allocate <ID> --invoice <InvoiceID> --amount 100.00
xero prepayments history <ID>
```

#### Payment Services
```bash
xero payment-services list
```

#### Attachments (Generic)
```bash
xero attachments list <endpoint> <ID>            # e.g. xero attachments list invoices <ID>
xero attachments upload <endpoint> <ID> <file>
xero attachments download <endpoint> <ID> <filename> --out ./local.pdf
```

#### Rate Limit Monitoring
```bash
xero rate-limit status                           # Show current rate limit budget
xero rate-limit history                          # Show recent API call history
```

---

## 7. Rate Limiting & API Efficiency

This is a core differentiator. Xero's rate limits are notoriously strict and a top developer complaint.

### 7.1 Rate Limits to Handle

| Limit | Value | Scope |
|---|---|---|
| Minute limit | 60 calls/minute | Per org, per app |
| Daily limit | 5,000 calls/day | Per org, per app |
| Concurrent limit | 5 simultaneous calls | Per org, per app |
| App-wide minute limit | 10,000 calls/minute | Across all tenants |

### 7.2 Implementation

```rust
// Conceptual rate limiter design
struct RateLimiter {
    minute_window: SlidingWindow,    // 60 calls per 60s
    daily_counter: DailyCounter,     // 5,000 per 24h
    concurrent: Semaphore,           // max 5 in-flight
}
```

**Behaviours:**
- **Pre-flight check**: Before each request, check if budget allows it. If not, wait with a progress indicator.
- **Response header tracking**: Read `x-rate-limit-remaining`, `x-rate-limit-limit`, `Retry-After` headers from every response.
- **Automatic 429 retry**: On `429 Too Many Requests`, apply exponential backoff with jitter (initial 1s, max 60s, up to 5 retries).
- **Budget display**: `xero rate-limit status` shows remaining calls for minute and daily windows.
- **Pause-and-resume**: For `--all-pages` operations that may hit limits, automatically pause and resume rather than failing.
- **Dry-run mode**: `--dry-run` shows the request without consuming rate limit budget.

### 7.3 API Call Optimization

- **pageSize=1000**: Always use maximum page size when `--all-pages` is used (Xero now supports up to 1,000 per page for major endpoints).
- **If-Modified-Since**: When cache exists, send `If-Modified-Since` header to get only changed records.
- **Selective fields**: Where supported, request only needed fields to reduce payload.
- **Batch operations**: Combine creates/updates where the API supports batch PUT/POST.
- **Webhook awareness**: In future versions, support webhook subscriptions to reduce polling.

---

## 8. Caching

### 8.1 Cache Location

```
~/.cache/xero-cli/
├── <org-id>/
│   ├── invoices.cache        # Cached response data
│   ├── contacts.cache
│   ├── accounts.cache
│   └── ...
├── cache.db                  # SQLite metadata (ETags, timestamps, sizes)
└── cache.lock                # Lock file for concurrent access
```

### 8.2 Cache Strategy

- **Default TTL**: 5 minutes for list operations, 15 minutes for get-by-id
- **Configurable**: `cache.ttl_seconds` in config file
- **If-Modified-Since**: Use `DateTimeUTC` from last successful response as `If-Modified-Since` header
- **Cache bypass**: `--no-cache` flag or `cache.enabled = false` in config
- **Cache clear**: `xero cache clear` or `xero cache clear invoices`
- **Cache stats**: `xero cache stats` shows cache hit rate and size
- **Write-through**: Mutations (create/update/delete) invalidate relevant caches immediately

### 8.3 Offline Mode

```bash
# Use cached data even if stale (no API calls)
xero invoices list --offline

# Export all data for offline use
xero cache warm                    # Fetch and cache all common endpoints
xero cache warm --endpoints invoices,contacts,accounts
```

---

## 9. Output Formatting

### 9.1 Formats

```bash
# Table (default for interactive use, auto-detected via TTY)
xero invoices list
┌──────────────┬─────────────────┬──────────┬───────────┬────────────┐
│ Invoice #    │ Contact         │ Status   │ Amount    │ Due Date   │
├──────────────┼─────────────────┼──────────┼───────────┼────────────┤
│ INV-0042     │ Acme Corp       │ SENT     │ £1,250.00 │ 2026-04-01 │
│ INV-0041     │ Widget Co       │ PAID     │   £480.50 │ 2026-03-15 │
└──────────────┴─────────────────┴──────────┴───────────┴────────────┘

# JSON (default when piped)
xero invoices list --output json | jq '.[] | .InvoiceNumber'

# CSV
xero invoices list --output csv > invoices.csv

# YAML
xero invoices list --output yaml

# Compact JSON (single line per record, for streaming)
xero invoices list --output json --compact

# Raw API response (no transformation)
xero invoices list --raw
```

### 9.2 Column Selection

```bash
# Select specific columns for table/CSV output
xero invoices list --columns InvoiceNumber,Contact.Name,Total,Status

# Exclude columns
xero invoices list --exclude LineItems,Payments
```

### 9.3 Pipeline Integration

```bash
# Pipe to jq
xero invoices list -o json | jq '[.[] | select(.AmountDue > 0)]'

# Export overdue invoices to CSV
xero invoices list --status OVERDUE -o csv > overdue.csv

# Count contacts
xero contacts list --all-pages -o json | jq length

# Create invoice from template
cat invoice-template.json | xero invoices create --file -

# Combine with other tools
xero reports profit-and-loss -o csv | xsv table
```

---

## 10. Configuration

### 10.1 Config File

Location: `~/.config/xero-cli/config.toml`

```toml
[default]
output = "table"
page_size = 100
color = true

[auth]
method = "pkce"                    # pkce | client_credentials
port = 8080                        # Callback port for PKCE
auto_refresh = true                # Auto-refresh tokens before expiry

[cache]
enabled = true
ttl_seconds = 300                  # Default cache TTL
max_size_mb = 100                  # Max cache size
directory = "~/.cache/xero-cli"

[rate_limit]
warn_threshold = 10                # Warn when < 10 calls remaining in minute window
daily_warn_threshold = 500         # Warn when < 500 calls remaining in daily window
auto_wait = true                   # Auto-wait on rate limit instead of failing

[profiles.default]
tenant_id = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
org_name = "My Company"
scopes = ["offline_access", "accounting.transactions", "accounting.contacts", "accounting.settings", "accounting.reports.read"]

[profiles.client_a]
tenant_id = "yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy"
org_name = "Client A"
scopes = ["offline_access", "accounting.transactions.read", "accounting.contacts.read"]
```

### 10.2 Environment Variables

All config can be overridden via environment variables:

```bash
XERO_CLIENT_ID           # OAuth2 client ID
XERO_CLIENT_SECRET       # OAuth2 client secret (for M2M)
XERO_TENANT_ID           # Default tenant ID
XERO_ACCESS_TOKEN        # Direct access token (skip auth flow)
XERO_OUTPUT_FORMAT       # Default output format
XERO_PROFILE             # Active profile name
XERO_NO_CACHE            # Disable caching (1/true)
XERO_CONFIG_PATH         # Custom config file path
```

---

## 11. Error Handling

### 11.1 Rich Diagnostics

Using `miette` for actionable error messages:

```
Error: × Xero API returned 401 Unauthorized
  ╰─▶ Your access token has expired.

  help: Run `xero auth refresh` to obtain a new token, or enable
        auto-refresh with `xero config set auth.auto_refresh true`
```

```
Error: × Rate limit exceeded (429 Too Many Requests)
  ├─▶ 60/60 minute calls used
  ╰─▶ Retry-After: 23 seconds

  help: The CLI will automatically retry in 23 seconds.
        Use `xero rate-limit status` to monitor your usage.
        Consider using --modified-since to reduce API calls.
```

```
Error: × Missing required scope: accounting.settings
  ├─▶ The 'accounts' endpoint requires the 'accounting.settings' scope
  ╰─▶ Current scopes: accounting.transactions, accounting.contacts

  help: Run `xero auth scopes add accounting.settings` to add
        the required scope. This will require re-authentication.
```

### 11.2 Error Codes

Map all Xero HTTP status codes to actionable CLI errors:

| Code | Error | CLI Behaviour |
|---|---|---|
| 400 | Bad Request | Show validation errors from response body |
| 401 | Unauthorized | Auto-refresh token, retry once. If still failing, prompt re-auth |
| 403 | Forbidden | Show missing scope/permission with help text |
| 404 | Not Found | Show which resource was not found |
| 429 | Rate Limited | Auto-wait using Retry-After header, then retry |
| 500 | Server Error | Retry with backoff (up to 3 times), then fail with Xero status page link |
| 503 | Rate Limit (legacy) | Same as 429 handling |

---

## 12. Multi-Organisation Support

### 12.1 Profile Management

```bash
# List all connected organisations
xero org list
┌─────────┬────────────────────┬──────────────┐
│ Profile │ Organisation       │ Status       │
├─────────┼────────────────────┼──────────────┤
│ default │ My Company Ltd     │ ● Connected  │
│ clienta │ Client A Holdings  │ ● Connected  │
│ clientb │ Client B Services  │ ○ Expired    │
└─────────┴────────────────────┴──────────────┘

# Switch context
xero org switch clienta

# Run command against specific profile without switching
xero --profile clienta invoices list

# Run command against ALL profiles
xero --all-profiles invoices list --status OVERDUE
```

### 12.2 Cross-Org Operations

```bash
# Export overdue invoices across all connected orgs
xero --all-profiles invoices list --status OVERDUE -o csv > all-overdue.csv

# Show P&L for specific orgs
xero --profile "default,clienta" reports profit-and-loss
```

---

## 13. Smart Aggregation Commands

These address specific Xero API limitations that frustrate developers.

### 13.1 Aged Debtor/Creditor Report (Full)

The Xero API only returns totals per contact without aging buckets. This command aggregates per-contact data.

```bash
# Full aged receivables with 30/60/90/120+ buckets
xero smart aged-receivables
┌─────────────────┬──────────┬──────────┬──────────┬──────────┬──────────┐
│ Contact         │ Current  │ 30 Days  │ 60 Days  │ 90 Days  │ 120+ Days│
├─────────────────┼──────────┼──────────┼──────────┼──────────┼──────────┤
│ Acme Corp       │ £500.00  │ £250.00  │    £0.00 │  £100.00 │    £0.00 │
│ Widget Co       │   £0.00  │   £0.00  │ £800.00  │    £0.00 │  £150.00 │
└─────────────────┴──────────┴──────────┴──────────┴──────────┴──────────┘

# Same for payables
xero smart aged-payables

# Export
xero smart aged-receivables -o csv > aged-ar.csv
```

### 13.2 Account Transactions Extract

Getting all transactions for an account requires combining multiple endpoints. This command does it automatically.

```bash
# All transactions for an account within a date range
xero smart account-transactions --account "Sales" --from 2025-01-01 --to 2025-12-31
xero smart account-transactions --code 200 --from 2025-01-01

# Includes: Invoices, Bank Transactions, Credit Notes, Manual Journals, Payments
```

### 13.3 Cash Flow Summary

```bash
# Aggregate cash movement from bank transactions
xero smart cash-flow --from 2025-01-01 --to 2025-12-31
xero smart cash-flow --account "Business Account" --from 2025-01-01
```

### 13.4 Outstanding Balance

```bash
# Quick view of all outstanding invoices/bills
xero smart outstanding
xero smart outstanding --type receivable     # Only AR
xero smart outstanding --type payable        # Only AP
```

---

## 14. Testing Strategy

### 14.1 Unit Tests

- All models: deserialization from Xero JSON fixtures
- Rate limiter: window tracking, backoff calculations
- Cache: TTL, invalidation, If-Modified-Since logic
- Output formatters: table, CSV, JSON rendering
- Config: TOML parsing, env var overrides, profile resolution

### 14.2 Integration Tests

- Mock HTTP server (using `wiremock` crate) simulating Xero API responses
- Full auth flow tests against mock OIDC provider
- Pagination tests with multi-page responses
- Rate limit 429 response handling
- Token refresh flow

### 14.3 E2E Tests

- Against Xero Demo Company (free, read-only test data)
- Smoke tests for all major commands
- CI pipeline using GitHub Actions

### 14.4 Fixtures

Store real Xero API response samples in `tests/fixtures/` for deterministic testing:
```
tests/fixtures/
├── invoices_list.json
├── invoices_get.json
├── contacts_list.json
├── reports_profit_and_loss.json
├── error_401.json
├── error_429.json
└── ...
```

---

## 15. Build & Distribution

### 15.1 Binary Targets

```yaml
# GitHub Actions matrix
targets:
  - x86_64-unknown-linux-musl      # Linux (static)
  - aarch64-unknown-linux-musl      # Linux ARM64 (static)
  - x86_64-apple-darwin             # macOS Intel
  - aarch64-apple-darwin            # macOS Apple Silicon
  - x86_64-pc-windows-msvc         # Windows
```

### 15.2 Distribution

- **GitHub Releases**: Pre-built binaries for all targets
- **Homebrew**: `brew install xero-cli` (via custom tap)
- **Cargo**: `cargo install xero-cli`
- **Docker**: `ghcr.io/<org>/xero-cli:latest` (for CI/CD)
- **AUR**: Arch Linux package

### 15.3 CI Pipeline

```yaml
on:
  push:
    tags: ['v*']
jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
          - os: ubuntu-latest
            target: aarch64-unknown-linux-musl
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release --target ${{ matrix.target }}
      - run: cargo test --release
```

---

## 16. Versioning & Release Cadence

- **Semantic Versioning** (semver)
- **v0.1.0**: Auth (PKCE + Client Credentials) + Invoices + Contacts + Accounts + Reports + Output formatting
- **v0.2.0**: Payments, Bank Transactions, Credit Notes, Items, POs, Quotes
- **v0.3.0**: All remaining Accounting API endpoints + cache layer
- **v0.4.0**: Smart aggregation commands (aged reports, account transactions)
- **v0.5.0**: Multi-org support + cross-org operations
- **v1.0.0**: Stable API, full Accounting API coverage, comprehensive tests
- **v1.1.0+**: Assets API, Files API, Projects API

---

## 17. Configuration Reference

### 17.1 Xero API Base URLs

| API | Base URL |
|---|---|
| Accounting | `https://api.xero.com/api.xro/2.0/` |
| Assets | `https://api.xero.com/assets.xro/1.0/` |
| Files | `https://api.xero.com/files.xro/1.0/` |
| Projects | `https://api.xero.com/projects.xro/2.0/` |
| Identity | `https://api.xero.com/connections` |
| OAuth2 | `https://identity.xero.com/connect/token` |
| Authorize | `https://login.xero.com/identity/connect/authorize` |

### 17.2 Xero API Rate Limits

| Limit | Value |
|---|---|
| Per-minute (per org) | 60 calls |
| Daily (per org) | 5,000 calls |
| Concurrent (per org) | 5 simultaneous |
| App-wide per minute | 10,000 calls |
| Page size max | 1,000 (for supported endpoints) |
| Token expiry | 30 minutes |
| High-volume threshold | 100,000 documents per GET |

### 17.3 OpenAPI Spec

The Xero OpenAPI specifications are maintained at `https://github.com/XeroAPI/Xero-OpenAPI` and should be used as the source of truth for model definitions. Consider code-generating the model types from the YAML specs:

- `xero_accounting.yaml`
- `xero-identity.yaml`
- `xero_assets.yaml`
- `xero_files.yaml`
- `xero-finance.yaml`
- `xero_bankfeeds.yaml`
- `xero-payroll-au.yaml`
- `xero-payroll-uk.yaml`
- `xero-payroll-nz.yaml`
- `xero-projects.yaml`

---

## 18. Known Xero API Limitations (Design Around These)

These are deliberate Xero API limitations that the CLI should handle gracefully:

| Limitation | Impact | CLI Mitigation |
|---|---|---|
| No bank reconciliation API | Cannot automate matching bank lines to transactions | Document clearly; focus on bank transaction CRUD instead |
| Aged reports lack aging buckets | Summary endpoint returns totals only | `xero smart aged-receivables` aggregates per-contact data |
| Cannot send statements via API | No programmatic statement delivery | Note in help text; suggest Xero UI |
| Journals limited to 100/page | Cannot use pageSize param | Auto-paginate using offset, track progress |
| No detailed account transaction endpoint | Must combine invoices + bank txns + journals | `xero smart account-transactions` command |
| Token expires every 30 min | Frequent re-auth needed | Auto-refresh with `refresh_token`; pre-emptive refresh at 25 min |
| Webhook payload minimal | Only contains resource ID, not data | Not relevant for CLI v1 |
| New paid API tiers (March 2026) | Higher costs for transaction access | Rate limit budgeting; cache aggressively; minimize redundant calls |
| Granular scopes (April 2026) | Broad scopes deprecated Sept 2027 | Ship with granular scope support from day 1 |

---

## 19. Security Considerations

- **Token storage**: OS keychain (macOS Keychain, Linux Secret Service, Windows Credential Manager) via `keyring` crate. Fallback to AES-256 encrypted file.
- **Client secrets**: Never logged, never displayed in `--verbose` output, masked in config display.
- **HTTPS only**: All Xero API communication over TLS (rustls, no OpenSSL dependency).
- **No telemetry**: Zero data collection. The CLI communicates only with Xero APIs.
- **Audit log**: Optional local audit log of all API calls made (timestamp, endpoint, method, status).

---

## 20. Open Questions & Future Considerations

- [ ] Should we generate Rust types from the Xero OpenAPI YAML specs, or hand-write models?
  - **Recommendation:** Generate from OpenAPI specs using a custom build script, with manual overrides for ergonomic improvements (e.g., `rust_decimal` for monetary fields).
- [ ] Should the cache use SQLite, sled, or flat JSON files?
  - **Recommendation:** SQLite via `rusqlite` — proven, supports concurrent access, good for metadata queries.
- [ ] Should we support shell completions (bash, zsh, fish)?
  - **Recommendation:** Yes, via `clap_complete`. Ship from v0.1.0.
- [ ] Should there be a `watch` mode for monitoring changes?
  - **Recommendation:** v1.1.0+ feature. `xero invoices watch --status OVERDUE` polls with `If-Modified-Since`.
- [ ] MCP Server mode?
  - **Recommendation:** Consider as a separate binary (`xero-mcp`) sharing the same core library.

---

## Appendix A: Example Workflows

### A.1 Daily Bookkeeping

```bash
# Morning check: outstanding invoices
xero invoices list --status OVERDUE --all-pages -o table

# Quick P&L
xero reports profit-and-loss --from 2026-01-01

# Create an invoice
xero invoices create \
  --contact "Acme Corp" \
  --line-item "March Consulting,1,5000.00" \
  --line-item "Expenses,1,250.00" \
  --due-date 2026-04-01 \
  --reference "MAR-2026"
```

### A.2 CI/CD Data Sync

```bash
#!/bin/bash
# Nightly export of all financial data
export XERO_CLIENT_ID="..."
export XERO_CLIENT_SECRET="..."

DATE=$(date -d 'yesterday' +%Y-%m-%d)

xero invoices list --modified-since "$DATE" -o json > exports/invoices.json
xero contacts list --modified-since "$DATE" -o json > exports/contacts.json
xero payments list --modified-since "$DATE" -o json > exports/payments.json
xero bank-transactions list --modified-since "$DATE" -o json > exports/bank-txns.json
```

### A.3 Multi-Client Accounting Firm

```bash
# Check overdue invoices across all clients
xero --all-profiles invoices list --status OVERDUE -o csv > all-overdue.csv

# Generate P&L for specific client
xero --profile "client-abc" reports profit-and-loss -o csv > client-abc-pl.csv
```

---

## Appendix B: Competitive Comparison

| Feature | xero-cli (ours) | xoauth | slickbench/xero-rs | pyxero | CData PowerShell |
|---|---|---|---|---|---|
| Language | Rust | Go | Rust | Python | .NET |
| CLI Interface | ✅ Full | ✅ Auth only | ❌ Library | ❌ Library | ✅ Cmdlets |
| Data Operations | ✅ Full CRUD | ❌ | ✅ Partial | ✅ Full | ✅ Full |
| OAuth2 PKCE | ✅ | ✅ | ✅ | ✅ | ✅ |
| Client Credentials | ✅ | ❌ | ✅ | ❌ | ✅ |
| Rate Limiting | ✅ Smart | ❌ | ❌ | ❌ | ✅ Basic |
| Local Cache | ✅ | ❌ | ❌ | ❌ | ❌ |
| Multi-org | ✅ | ✅ | ❌ | ❌ | ❌ |
| Output Formats | JSON/CSV/Table/YAML | JSON | JSON | Dict | DataTable |
| Single Binary | ✅ | ✅ | N/A | ❌ (Python) | ❌ (.NET) |
| Smart Aggregation | ✅ | ❌ | ❌ | ❌ | ❌ |
| Granular Scopes | ✅ | ❌ | ❌ | ❌ | ❌ |
| Free & Open Source | ✅ MIT | ✅ MIT | ✅ MIT | ✅ MIT | ❌ Commercial |
| Active (2025+) | ✅ | ❌ (2020) | ✅ (2025) | ⚠️ Minimal | ✅ |
