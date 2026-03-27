# Changelog

## [0.5.0] - 2026-03-27

### Changed
- Migrate OAuth scopes from deprecated broad scopes to Xero's new granular scopes (required for apps created after March 2, 2026)
- Replace `accounting.transactions.read` with `accounting.invoices.read`, `accounting.payments.read`, `accounting.banktransactions.read`
- Replace `accounting.reports.read` with `accounting.reports.profitandloss.read`, `accounting.reports.balancesheet.read`
- Update all four scope presets (read-only, bookkeeper, full-access, reports-only) with granular scopes
- Update help text and doc comments with correct granular scope names

### Fixed
- Fix `unauthorized_client` error when authenticating with Xero apps created after March 2, 2026
- Fix `invalid_client` error during token exchange by adding `XERO_CLIENT_SECRET` support for Web app (confidential client) PKCE flow
- Fix invalid scope name `accounting.reports.balancesheets.read` (plural) to `accounting.reports.balancesheet.read` (singular)

## [0.4.1] - 2026-03-03

### Fixed

- Fix release pipeline: auto-tag uses PAT, correct Homebrew formula URLs, Scoop support


## [0.4.0] - 2026-03-02

### Added
- Rich `--help` text for all 34 commands and every subcommand with detailed descriptions, valid parameter values, and concrete usage examples
- Man page generation via `xero completions man --output-dir <dir>` producing 164 individual man pages
- Getting Started guide, Environment Variables reference, and Examples section in top-level `--help`
- OAuth scope requirements documented for every command group

### Fixed
- Shell completions now cover all 34 commands (previously only 7 were included)

### Changed
- Moved `Cli` struct to library crate for reuse by completions and man page generation
- Completions use the real derive-based command tree instead of a manually maintained duplicate

## [0.3.0] - 2026-03-02

### Added
- SQLite response cache with TTL-based expiry and ETag support
- Cache management commands (`cache clear`, `cache stats`)
- `--no-cache` global flag to bypass cache
- Overpayments (list, get, allocate, history)
- Prepayments (list, get, allocate, history)
- Tracking categories with nested option management (list, get, create, update, add-option, update-option, remove-option)
- Journals with offset-based pagination (list, get)
- Currencies (list)
- Employees (list, get)
- Users (list, get)
- Budgets (list, get)
- Branding themes (list, get)
- Repeating invoices (list, get)
- Organisation details (get)
- Payment services (list)
- Tax rates (list, create, update)
- Contact groups (list, get, create, update, delete)
- Manual journals (list, get, create, update)
- Linked transactions (list, get, create, update, delete)
- Receipts (list, get, create, history)
- Batch payments (list, get, create, delete)
- Expense claims (list, get, create, update, history)

## [0.2.0] - 2026-03-02

### Added
- Payments (list, get, create, delete, history)
- Items (list, get, create, update, delete, history)
- Bank transactions (list, get, create, delete, history)
- Bank transfers (list, get, create)
- Credit notes (list, get, create, allocate, history)
- Purchase orders (list, get, create, history)
- Quotes (list, get, create, update)
- Shared history and allocation infrastructure

## [0.1.0] - 2026-03-02

### Added
- OAuth2 authentication (PKCE + Client Credentials)
- Invoice management (list, get, create, update)
- Contact management (list, get, create, update)
- Account management (list, get, create, archive)
- Financial reports (P&L, balance sheet, trial balance, bank summary, budget summary, executive summary, aged receivables/payables)
- Multi-format output (table, JSON, CSV, YAML)
- Intelligent rate limiting with exponential backoff
- Daily API budget tracking
- Auto-pagination support
- Multi-org profile management
- Shell completions (bash, zsh, fish)
- Secure token storage via OS keychain
- Configuration via TOML file
