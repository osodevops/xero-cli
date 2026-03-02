use crate::cache::CacheStore;
use crate::cli::GlobalArgs;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum CacheCommands {
    /// Clear the response cache
    Clear {
        /// Only clear entries for a specific resource type (e.g. Invoices)
        #[arg(long)]
        resource: Option<String>,
    },
    /// Show cache statistics
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
