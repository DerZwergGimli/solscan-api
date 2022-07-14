# SOLSCAN API WRAPPER

[![Open Source Love](https://badges.frapsoft.com/os/v1/open-source.svg?v=103)](https://github.com/ellerbrock/open-source-badges/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![buid-test](https://https://github.com/DerZwergGimli/solscan-api/actions/workflows/buidtest.yml/badge.svg)

``unoffical``

# Solscan API

[Solscan-Public-API](https://public-api.solscan.io/docs/#/)

# Implemented Endpoints

### Block

- [x] ``/block/last``
- [x] ``/block/transaction``
- [x] ``/block/{block}``

### Transaction

- [x] ``/transaction/last``
- [x] ``/transaction/{signature}``

### Account

- [x] ``/account/tokens``
- [x] ``/account/transactions``
- [x] ``/account/stakeAccounts``
- [x] ``/account/splTransfers``
- [x] ``/account/solTransfers``
- [ ] ``/account/exportTransactions``
- [x] ``/account/{account}``

### Token

- [x] ``/token/holders``
- [x] ``/token/meta``
- [ ] ``/token/list``

### Market

- [x] ``/market/token/{tokenAddress}``

### ChainInfo

- [x] ``/chaininfo/``

### Tools

- [ ] ``/Tools/``

## Dev Links

- [QuickType](https://app.quicktype.io/) a generator for Types from JSON
