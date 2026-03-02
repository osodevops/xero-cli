# Changelog

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
