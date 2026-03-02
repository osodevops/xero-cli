use crate::cache::CacheStore;
use crate::cli::GlobalArgs;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum CacheCommands {
    /// Clear the response cache.
    ///
    /// Removes cached API responses so that subsequent requests fetch fresh data
    /// from the Xero API. When called without options, all cached entries are
    /// removed. Use `--resource` to selectively clear only the entries for a
    /// single resource type while leaving the rest of the cache intact.
    #[command(after_long_help = r#"EXAMPLES:
  # Clear the entire cache
  xero cache clear

  # Clear only cached invoice data
  xero cache clear --resource Invoices

  # Clear only cached contact data
  xero cache clear --resource Contacts"#)]
    Clear {
        /// Restrict the clear to a single resource type (e.g. Invoices, Contacts, Accounts).
        ///
        /// When omitted, every cached entry is removed regardless of resource type.
        #[arg(long)]
        resource: Option<String>,
    },

    /// Show cache statistics.
    ///
    /// Displays a summary of the current cache contents, including the total
    /// number of cached entries, the combined size on disk, and a per-resource
    /// breakdown of entry counts. Useful for diagnosing stale-data issues or
    /// monitoring disk usage.
    #[command(after_long_help = r#"EXAMPLES:
  # Display cache statistics
  xero cache stats"#)]
    Stats,
}

pub async fn execute(command: CacheCommands, _global: &GlobalArgs) -> miette::Result<()> {
    let cache_dir = crate::config::default_cache_dir().map_err(|e| miette::miette!("{e}"))?;

    let store = CacheStore::new(&cache_dir).map_err(|e| miette::miette!("{e}"))?;

    match command {
        CacheCommands::Clear { resource } => {
            if let Some(res) = resource {
                store
                    .invalidate_by_resource(&res)
                    .map_err(|e| miette::miette!("{e}"))?;
                eprintln!("Cache cleared for resource: {res}");
            } else {
                store.clear().map_err(|e| miette::miette!("{e}"))?;
                eprintln!("Cache cleared.");
            }
        }

        CacheCommands::Stats => {
            let stats = store.stats().map_err(|e| miette::miette!("{e}"))?;
            println!("Cache Statistics:");
            println!("  Total entries: {}", stats.total_entries);
            println!(
                "  Total size: {:.2} KB",
                stats.total_size_bytes as f64 / 1024.0
            );
            if !stats.resource_counts.is_empty() {
                println!("  By resource:");
                for (resource, count) in &stats.resource_counts {
                    println!("    {resource}: {count} entries");
                }
            }
        }
    }

    Ok(())
}
