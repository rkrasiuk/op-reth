use std::path::PathBuf;

use eyre::{eyre, Result};
use leveldb::database::iterator::Iterable;
use leveldb::db::Database;
use leveldb::options::Options;
use leveldb::options::ReadOptions;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

// https://github.com/ethereum/go-ethereum/blob/master/core/rawdb/schema.go

/// Byte prefix for header keys
/// HEADER_PREFIX ++ number (uint64 big endian) + hash -> header
#[allow(dead_code)]
static HEADER_PREFIX: &[u8] = b"h";

/// Byte prefix for block body keys
/// BODY_PREFIX ++ number (uint64 big endian) + hash -> body
#[allow(dead_code)]
static BODY_PREFIX: &[u8] = b"b";

/// Byte prefix for transaction lookup keys
/// TRANSACTION_PREFIX ++ hash -> transaction / receipt lookup metadata
#[allow(dead_code)]
static TX_LOOKUP_PREFIX: &[u8] = b"l";

/// Account trie prefix
/// ACCOUNT_TRIE_PREFIX ++ hexPath -> trie node
/// TODO: Do we want the trie node, or the trie node value? If we want the value, b"a"
#[allow(dead_code)]
static ACCOUNT_TRIE_PREFIX: &[u8] = b"A";

/// Storage trie prefix
/// STORAGE_TRIE_PREFIX ++ accountHash ++ hexPath -> trie node
/// TODO: Do we want the trie node, or the trie node value? If we want the value, b"o"
#[allow(dead_code)]
static STORAGE_TRIE_PREFIX: &[u8] = b"O";

fn main() -> Result<()> {
    // Setup tracing
    setup_tracing()?;

    // Designate the path to the leveldb database
    let args: Vec<String> = std::env::args().collect();
    let db_path_buf = PathBuf::from(args.get(1).unwrap());
    let db_path = db_path_buf.as_path();

    info!("Opening database at path {:?}", db_path_buf);

    // Open the database
    let mut options = Options::new();
    options.create_if_missing = false;
    let db = Database::open(db_path, &options)?;

    info!("Opened leveldb database! Iterating...");

    // Walk the DB and look for keys with the prefixes we want. If we find an entry with a desired prefix,
    // we need to deserialize the value based on the prefix and convert it to a newly defined rust type.
    // We can then re-serialize each type into MDBX compatible data for insertion into the new database.
    for (key, value) in db.iter(&ReadOptions::new()) {
        // TODO
        tracing::debug!("key: {:?} | value: {:?}", key, value);
    }

    info!("Finished iterating! Writing dump file...");

    // TODO: Open reth's MDBX database
    // TODO: Insert serialized reth data into MDBX database

    Ok(())
}

/// Sets up the tracing subscriber with default options.
fn setup_tracing() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_| eyre!("Failed to set up tracing subscriber"))
}
