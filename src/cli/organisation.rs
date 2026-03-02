use crate::api::endpoints::organisation;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::output;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum OrganisationCommands {
    /// Get organisation details
    Get,
}

pub async fn execute(command: OrganisationCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;
    match command {
        OrganisationCommands::Get => {
            let org = organisation::get(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&org, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }
    Ok(())
}
