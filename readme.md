# SOLSCAN API WRAPPER

[![Open Source Love](https://badges.frapsoft.com/os/v1/open-source.svg?v=103)](https://github.com/ellerbrock/open-source-badges/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![buid-test](https://github.com/DerZwergGimli/solscan-api/actions/workflows/builttest.yml/badge.svg)

`-unoffical-`

## Solscan API

[Solscan-Public-API](https://public-api.solscan.io/docs/#/)

`Default Limit: 150 requests/ 30 seconds, 100k requests / day`

## Usage

Example: Fetching last 10 Blocks form Solana-Blockchain via SolscanAPI

```rust
use crate::solscan::SolscanAPI;

fn main() {
    let solscan_api = SolscanAPI::new();
    let result = solscan_api.get_block_last(Some(10)).await.unwrap();

    println("{:?}", result)
}
```

More examples can be found
under [/src/tests/test_endpoints](https://github.com/DerZwergGimli/solscan-api/tree/master/src/tests/test_endpoints) in
this repo.

## Implemented Endpoints

BaseURL: `https://public-api.solscan.io/`

### Block

| State* | Type  | Endpoint               |
|--------|-------|------------------------|
| ✅      | GET   | ``/block/last``        |
| ✅      | GET   | ``/block/transaction`` |
| ✅      | GET   | ``/block/{block}``     |

### Transaction

| State* | Type  | Endpoint                       |
|--------|-------|--------------------------------|
| ✅      | GET   | ``/transaction/last``          |
| ✅      | GET   | ``/transaction/{signature}``   |

### Account

| State* | Type  | Endpoint                        |
|--------|-------|---------------------------------|
| ✅      | GET   | ``/account/tokens``             |
| ✅      | GET   | ``/account/transactions``       |
| ✅      | GET   | ``/account/stakeAccounts``      |
| ✅      | GET   | ``/account/splTransfers``       |
| ✅      | GET   | ``/account/solTransfers``       |
| ✅      | GET   | ``/account/exportTransactions`` |
| ⛔      | GET   | ``/account/{account}``          |

### Token

| State* | Type  | Endpoint            |
|--------|-------|---------------------|
| ✅      | GET   | ``/token/holders``  |
| ✅      | GET   | ``/token/meta``     |
| ⛔      | GET   | ``/token/list``     |

### Market

| State* | Type  | Endpoint                          |
|--------|-------|-----------------------------------|
| ✅      | GET   | ``/market/token/{tokenAddress}``  |

### ChainInfo

| State* | Type  | Endpoint          |
|--------|-------|-------------------|
| ✅      | GET   | ``/chaininfo/``   |

### Tools

| State* | Type  | Endpoint      |
|--------|-------|---------------|
| ⛔      | GET   | ``/Tools/``   |

#### *State of implementation (yes/no)

## Dev Links

- [QuickType](https://app.quicktype.io/) a generator for Types from JSON

## Donate

## Donations

If you want to get me a ☕ so I won't 😴

- Solana-Wallet-Address: `BSW9zp3iJUcemTVWN4EThcaF6FxBQqP2wgnapSt1Z5mt`
- Solana-Wallet-Domain: `coffeeplease.sol`