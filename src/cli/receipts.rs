use std::path::Path;

use clap::Parser;
use eyre::Result;
use reth::runner::CliContext;
use reth_primitives::{rpc::H256, U256};
use rlp::Decodable;
use serde::{Deserialize, Serialize};

/// Receipts command
#[derive(Debug, Parser)]
pub struct Command {
    /// The path to the receipts export
    #[arg(long, value_name = "RECEIPTS", verbatim_doc_comment, default_value = "data/")]
    path: String,
}

impl Command {
    /// Execute the command
    pub async fn execute(self, _ctx: CliContext) -> eyre::Result<()> {
        tracing::info!(target: "reth::cli", "loading receipts file \"{}\"", self.path);
        let receipts = Receipt::from_file(self.path)?;
        tracing::info!(target: "reth::cli", "got {} receipts", receipts.len());
        Ok(())
    }
}

/// ## Receipt
///
/// This is a receipt types based on the [HackReceipt](https://github.com/testinprod-io/erigon/blob/pcw109550/state-import/core/types/receipt.go#L81)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    /// The receipt type
    #[serde(rename = "type")]
    pub ty: u8,
    /// The post state root
    #[serde(rename = "root")]
    pub post_state: Vec<u8>,
    /// The tx receipt status
    pub status: u64,
    /// The cumulative gas used
    #[serde(rename = "cumulativeGasUsed")]
    pub cumulative_gas_used: u64,
    /// The bloom filter
    #[serde(rename = "logsBloom")]
    pub bloom: Vec<u8>,
    /// Receipt logs
    pub logs: Vec<u8>,
    /// The transaction hash
    #[serde(rename = "transactionHash")]
    pub tx_hash: H256,
    /// The contract address
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    /// The gas used
    #[serde(rename = "gasUsed")]
    pub gas_used: u64,
    /// The block hash
    #[serde(rename = "blockHash")]
    pub block_hash: H256,
    /// The block number
    #[serde(rename = "blockNumber")]
    pub block_number: U256,
    /// The transaction index
    #[serde(rename = "transactionIndex")]
    pub transaction_index: u64,
    /// The L1 gas price
    #[serde(rename = "l1GasPrice")]
    pub l1_gas_price: U256,
    /// The L1 gas used
    #[serde(rename = "l1GasUsed")]
    pub l1_gas_used: U256,
    /// The L1 fee
    #[serde(rename = "l1Fee")]
    pub l1_fee: U256,
    /// The L1 fee scalar
    #[serde(rename = "l1FeeScalar")]
    pub l1_fee_scalar: String,
}

impl rlp::Decodable for Receipt {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let ty = rlp.val_at(0)?;
        let post_state = rlp.val_at(1)?;
        let status = rlp.val_at(2)?;
        let cumulative_gas_used = rlp.val_at(3)?;
        let bloom = rlp.val_at(4)?;
        let logs = rlp.at(5)?.as_raw();
        let tx_hash = rlp.val_at(6)?;
        let contract_address = rlp.val_at(7)?;
        let gas_used = rlp.val_at(8)?;
        let block_hash = rlp.val_at(9)?;
        let block_number = rlp.val_at(10)?;
        let transaction_index = rlp.val_at(11)?;
        let l1_gas_price = rlp.val_at(12)?;
        let l1_gas_used = rlp.val_at(13)?;
        let l1_fee = rlp.val_at(14)?;
        let l1_fee_scalar = rlp.val_at(15)?;

        let r = Receipt {
            ty,
            post_state,
            status,
            cumulative_gas_used,
            bloom,
            logs: logs.to_vec(),
            tx_hash,
            contract_address,
            gas_used,
            block_hash,
            block_number,
            transaction_index,
            l1_gas_price,
            l1_gas_used,
            l1_fee,
            l1_fee_scalar,
        };
        Ok(r)
    }
}

impl Receipt {
    fn decode_receipt_vec(rlp: &rlp::Rlp) -> Result<Vec<Receipt>, rlp::DecoderError> {
        let mut receipts = Vec::new();
        for (_, item) in rlp.iter().enumerate() {
            if item.is_empty() {
                continue
            }
            let r = if let Ok(r) = Receipt::decode(&item) {
                r
            } else {
                let mut inner_vec = Receipt::decode_receipt_vec(&item)?;
                receipts.append(&mut inner_vec);
                continue
            };
            receipts.push(r);
        }
        Ok(receipts)
    }

    /// Decodes receipts from an rlp-encoded list of receipts file
    pub fn from_file(path: impl AsRef<Path>) -> Result<Vec<Receipt>> {
        let data = std::fs::read(&path)?;
        let rlp_data = rlp::Rlp::new(&data[1..]);
        if rlp_data.is_empty() {
            tracing::warn!(target: "reth::cli", "rlp data is empty!");
        }
        if rlp_data.is_null() {
            tracing::warn!(target: "reth::cli", "rlp data is null!");
        }
        if rlp_data.is_list() {
            tracing::debug!(target: "reth::cli", "decoding rlp data as list");
        }
        let receipts = Receipt::decode_receipt_vec(&rlp_data).map_err(|e| eyre::eyre!(e))?;
        Ok(receipts)
    }
}
