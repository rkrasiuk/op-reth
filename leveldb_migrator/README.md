# `leveldb-migrator`

A CLI tool to assist in migrating the world trie within [goerli's bedrock leveldb](https://storage.googleapis.com/oplabs-goerli-data/goerli-bedrock.tar) into revm's MDBX database.

## Usage
1. Download the [goerli bedrock data directory](https://storage.googleapis.com/oplabs-goerli-data/goerli-bedrock.tar)
1. Untar the database: `tar -xvf ./goerli-bedrock.tar`
1. Run the binary to create the dump file:
```
cargo r --bin leveldb-migrator geth/chaindata
```
