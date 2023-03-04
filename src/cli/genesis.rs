use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use clap::{crate_version, Parser};
use eyre::Result;
use reth::runner::CliContext;
use reth_primitives::{Address, GenesisAccount};
use serde::{Deserialize, Serialize};

/// Genesis command
#[derive(Debug, Parser)]
pub struct Command {
    /// The path to the genesis file
    #[arg(long, value_name = "GENESIS", verbatim_doc_comment, default_value = "genesis.json")]
    path: String,
}

impl Command {
    /// Execute the command
    pub async fn execute(self, _ctx: CliContext) -> eyre::Result<()> {
        tracing::info!(target: "op-reth::genesis", "loading genesis file {}", crate_version!());

        let genesis = Genesis::from_file(self.path)?;
        println!("Genesis: {:#?}", genesis);

        tracing::debug!(target: "op-reth::genesis", genesis = ?genesis, "genesis file loaded");

        Ok(())
    }
}

/// Optimism Object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Optimism {
    #[serde(rename = "eip1559Elasticity")]
    pub eip1559_elasticity: u64,
    #[serde(rename = "eip1559Denominator")]
    pub eip1559_denominator: u64,
}

/// The genesis inner config object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisConfig {
    #[serde(rename = "ChainName")]
    pub chain_name: String,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    #[serde(rename = "homesteadBlock")]
    pub homestead_block: u64,
    #[serde(rename = "eip150Block")]
    pub eip150_block: u64,
    #[serde(rename = "eip150Hash")]
    pub eip150_hash: String,
    #[serde(rename = "eip155Block")]
    pub eip155_block: u64,
    #[serde(rename = "eip158Block")]
    pub eip158_block: u64,
    #[serde(rename = "byzantiumBlock")]
    pub byzantium_block: u64,
    #[serde(rename = "constantinopleBlock")]
    pub constantinople_block: u64,
    #[serde(rename = "petersburgBlock")]
    pub petersburg_block: u64,
    #[serde(rename = "istanbulBlock")]
    pub istanbul_block: u64,
    #[serde(rename = "muirGlacierBlock")]
    pub muir_glacier_block: u64,
    #[serde(rename = "berlinBlock")]
    pub berlin_block: u64,
    #[serde(rename = "londonBlock")]
    pub london_block: u64,
    #[serde(rename = "arrowGlacierBlock")]
    pub arrow_glacier_block: u64,
    #[serde(rename = "grayGlacierBlock")]
    pub gray_glacier_block: u64,
    #[serde(rename = "mergeNetsplitBlock")]
    pub merge_netsplit_block: u64,
    #[serde(rename = "bedrockBlock")]
    pub bedrock_block: u64,
    #[serde(rename = "terminalTotalDifficulty")]
    pub terminal_total_difficulty: u64,
    #[serde(rename = "terminalTotalDifficultyPassed")]
    pub terminal_total_difficulty_passed: bool,
    pub optimism: Optimism,
}

/// The genesis file object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genesis {
    pub config: GenesisConfig,
    pub difficulty: String,
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
    pub extradata: String,
    pub alloc: HashMap<Address, GenesisAccount>,
}

impl Genesis {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }
}
