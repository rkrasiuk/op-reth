use std::{fs::File, io::BufReader};

use clap::{Parser, crate_version};
use reth::runner::CliContext;

/// Genesis command
#[derive(Debug, Parser)]
pub struct Command {
    /// The path to the genesis file
    #[arg(long, value_name = "GENESIS", verbatim_doc_comment, default_value="genesis.json")]
    path: String,
}

impl Command {
    /// Execute `node` command
    pub async fn execute(self, ctx: CliContext) -> eyre::Result<()> {
        tracing::info!(target: "op-reth::genesis", "loading genesis file {}", crate_version!());

        // Load the genesis file from the specified path
        let file = File::open(self.path)?;
        let reader = BufReader::new(file);

        // Deserialize the genesis file
        let genesis = serde_json::from_reader(reader)?;

        tracing::debug!(target: "op-reth::genesis", genesis = ?genesis, "genesis file loaded");

        Ok(())
    }
}