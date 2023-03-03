# op-reth

This repository will contain an executable for running OP Reth client.

Optimism modifications are exposed in [reth crates](https://github.com/paradigmxyz/reth/tree/optimism) via an `optimism` feature flag.


## Feature Support

- [ ] Read [optimism-goerli genesis file](https://github.com/testinprod-io/erigon/blob/pcw109550/state-import/state-import/genesis.json)
- [ ] Import block headers and transactions to your new client.
     - RLP encoded block: [export_0_4061224](https://drive.google.com/file/d/1z1pGEhy8acPi_U-6Sz0oo_-zJSzU8zb-/view?usp=sharing)
- [ ] Import transaction receipts to your new client
     - RLP encoded receipt: [export_receipt_0_4061223](https://drive.google.com/file/d/1QJpv-SNv6I3j9z4FfHzZ3fHlCuFMn8b0/view?usp=sharing)
     - There is no receipt for block 4061224 because no transaction at block 4061224.
- [ ] Import world state trie at block 4061224: [alloc_everything_4061224_final.json](https://drive.google.com/file/d/1k9yopW6F8SyHAR-8JT2hfxptQGT-DqKe/view?usp=sharing) to your new client. I wrote [custom import functionality](https://github.com/testinprod-io/erigon/blob/pcw109550/state-import/turbo/app/import.go) for op-erigon to achieve this.

You may ask that is it okay not to have world state trie for prebedrock block. You may simply relay the requests to l2geth node. Daisy chain will handle these prebedrock jobs.

