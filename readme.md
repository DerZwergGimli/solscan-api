# SOLSCAN API WRAPPER

[![Open Source Love](https://badges.frapsoft.com/os/v1/open-source.svg?v=103)](https://github.com/ellerbrock/open-source-badges/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![buid-test](https://github.com/DerZwergGimli/solscan-api/actions/workflows/builttest.yml/badge.svg)

`-unoffical-`

## Solscan API

[Solscan-Public-API](https://public-api.solscan.io/docs/#/)

`Default Limit: 150 requests/ 30 seconds, 100k requests / day`

## Help this Project

If you find any issues please create an [Issue](https://github.com/DerZwergGimli/solscan-api/issues) and provide the
following data:

- Endpoint that is not working properly.
- All Parameters that you have used for making that request.
- JSON Data of the response.

## Usage

Example: Fetching last 10 Blocks form Solana-Blockchain via SolscanAPI

```rust
use solscan_api::enums::solscan_errors;

#[tokio::main]
async fn main() -> Result<(), solscan_errors::SolscanError> {
    let solscan_api = solscan_api::solscan::SolscanAPI::new();
    let result = solscan_api.get_block_last(Some(10)).await.unwrap();

    println!("{:?}", result);
    Ok(())
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

If you want to get me a ☕ so I won't 😴

- Solana-Wallet-Address: `BSW9zp3iJUcemTVWN4EThcaF6FxBQqP2wgnapSt1Z5mt`
- Solana-Wallet-Domain: `coffeeplease.sol`