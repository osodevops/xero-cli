#![allow(deprecated)]
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn help_flag_works() {
    Command::cargo_bin("xero")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "A fast CLI for the Xero Accounting API",
        ))
        .stdout(predicate::str::contains("GETTING STARTED"))
        .stdout(predicate::str::contains("ENVIRONMENT VARIABLES"))
        .stdout(predicate::str::contains("EXAMPLES"));
}

#[test]
fn version_flag_works() {
    Command::cargo_bin("xero")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.4.0"));
}

#[test]
fn no_args_shows_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn auth_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["auth", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("login"))
        .stdout(predicate::str::contains("status"))
        .stdout(predicate::str::contains("logout"));
}

#[test]
fn invoices_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["invoices", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"));
}

#[test]
fn contacts_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["contacts", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"));
}

#[test]
fn accounts_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["accounts", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("archive"));
}

#[test]
fn reports_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["reports", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("profit-and-loss"))
        .stdout(predicate::str::contains("balance-sheet"))
        .stdout(predicate::str::contains("trial-balance"));
}

#[test]
fn config_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["config", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("show"))
        .stdout(predicate::str::contains("set"));
}

#[test]
fn completions_bash() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete"));
}

#[test]
fn completions_zsh() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("compdef"));
}

#[test]
fn completions_fish() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete"));
}

#[test]
fn config_init_creates_file() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    Command::cargo_bin("xero")
        .unwrap()
        .args(["config", "init", "--config", config_path.to_str().unwrap()])
        .assert()
        .success();

    assert!(config_path.exists());
}

#[test]
fn config_show_default() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("nonexistent.toml");

    Command::cargo_bin("xero")
        .unwrap()
        .args(["config", "show", "--config", config_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("output_format"));
}

#[test]
fn payments_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["payments", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("delete"))
        .stdout(predicate::str::contains("history"));
}

#[test]
fn items_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["items", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn bank_transactions_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["bank-transactions", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn bank_transfers_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["bank-transfers", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"));
}

#[test]
fn credit_notes_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["credit-notes", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("allocate"));
}

#[test]
fn purchase_orders_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["purchase-orders", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("history"));
}

#[test]
fn quotes_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["quotes", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"));
}

#[test]
fn currencies_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["currencies", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"));
}

#[test]
fn employees_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["employees", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn users_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["users", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn budgets_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["budgets", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn branding_themes_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["branding-themes", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn repeating_invoices_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["repeating-invoices", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn organisation_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["organisation", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("get"));
}

#[test]
fn payment_services_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["payment-services", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"));
}

#[test]
fn cache_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["cache", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("clear"))
        .stdout(predicate::str::contains("stats"));
}

#[test]
fn tax_rates_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["tax-rates", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"));
}

#[test]
fn contact_groups_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["contact-groups", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn manual_journals_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["manual-journals", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"));
}

#[test]
fn linked_transactions_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["linked-transactions", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn receipts_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["receipts", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("history"));
}

#[test]
fn batch_payments_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["batch-payments", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn expense_claims_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["expense-claims", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("history"));
}

#[test]
fn overpayments_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["overpayments", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("allocate"))
        .stdout(predicate::str::contains("history"));
}

#[test]
fn prepayments_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["prepayments", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("allocate"))
        .stdout(predicate::str::contains("history"));
}

#[test]
fn tracking_categories_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["tracking-categories", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("add-option"))
        .stdout(predicate::str::contains("update-option"))
        .stdout(predicate::str::contains("remove-option"));
}

#[test]
fn journals_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["journals", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn invalid_output_format() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["--output", "xml", "invoices", "list"])
        .assert()
        .failure();
}

#[test]
fn short_help_is_concise() {
    // -h should NOT include EXAMPLES section (only --help shows after_long_help)
    Command::cargo_bin("xero")
        .unwrap()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "A fast CLI for the Xero Accounting API",
        ))
        .stdout(predicate::str::contains("EXAMPLES").not());
}

#[test]
fn invoices_list_long_help() {
    Command::cargo_bin("xero")
        .unwrap()
        .args(["invoices", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("EXAMPLES"))
        .stdout(predicate::str::contains("DRAFT"))
        .stdout(predicate::str::contains("AUTHORISED"));
}

#[test]
fn man_page_generation() {
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().to_str().unwrap();

    Command::cargo_bin("xero")
        .unwrap()
        .args(["completions", "man", "--output-dir", out])
        .assert()
        .success();

    // Verify key man pages were created
    assert!(dir.path().join("xero.1").exists());
    assert!(dir.path().join("xero-invoices.1").exists());
    assert!(dir.path().join("xero-invoices-list.1").exists());
    assert!(dir.path().join("xero-contacts.1").exists());
    assert!(dir.path().join("xero-auth.1").exists());
    assert!(dir.path().join("xero-auth-login.1").exists());
    assert!(dir.path().join("xero-reports.1").exists());
    assert!(dir.path().join("xero-completions.1").exists());
    assert!(dir.path().join("xero-completions-man.1").exists());

    // Count total man pages — should be 100+ (xero + 34 commands + all subcommands)
    let count = std::fs::read_dir(dir.path())
        .unwrap()
        .filter(|e| {
            e.as_ref()
                .map(|e| e.path().extension().map(|ext| ext == "1").unwrap_or(false))
                .unwrap_or(false)
        })
        .count();
    assert!(count >= 100, "Expected 100+ man pages, got {count}");
}
