//! # Solscan-API-Endpoints
//! This module represents the Solscan-API-Endpoints


#[derive(Debug)]
/// Creating an enum with the name SolscanEndpoints.
pub enum SolscanEndpoints {
    //Block
    /// GET: /block/last
    BlockLast,
    /// GET: /block/transactions
    BlockTransactions,
    /// GET: /block/{block}
    Block,

    //Transaction
    /// GET: /transaction/last
    TransactionLast,
    /// GET: /transaction/{signature}
    Transaction,

    //Account
    /// GET: /account/tokens
    AccountTokens,
    /// GET: /account/transaction
    AccountTransaction,
    /// GET: /account/stakeAccounts
    AccountStakeAccounts,
    /// GET: /account/splTransfers
    AccountSPLTransfers,
    /// GET: /account/solTransfers
    AccountSolTransfers,
    /// GET: /account/ExportTransactions
    AccountExportTransactions,
    /// GET: /account/{account}
    Account,

    //Token
    /// GET: /token/holders
    TokenHolders,
    /// GET: /token/meta
    TokenMeta,
    /// GET: /token/list
    TokenList,

    //Market
    /// GET: /market/token
    MarketToken,

    //ChainInfo
    /// GET: /chaininfo/
    ChainInfo,

    //Tools
    /// GET: /tools/inspect
    ToolsInspect,

}

/// Implementation of Solscan-API-Endpoints Enums as string-text
impl SolscanEndpoints {
    pub(crate) fn value(&self) -> &str {
        match *self {
            //Block
            SolscanEndpoints::BlockLast => "/block/last",
            SolscanEndpoints::BlockTransactions => "/block/transactions",
            SolscanEndpoints::Block => "/block",
            //Transaction
            SolscanEndpoints::TransactionLast => "/transaction/last",
            SolscanEndpoints::Transaction => "/transaction",
            SolscanEndpoints::AccountTokens => "/account/tokens",
            //Account
            SolscanEndpoints::AccountTransaction => "/account/transactions",
            SolscanEndpoints::AccountStakeAccounts => "/account/stakeAccounts",
            SolscanEndpoints::AccountSPLTransfers => "/account/splTransfers",
            SolscanEndpoints::AccountSolTransfers => "/account/solTransfers",
            SolscanEndpoints::AccountExportTransactions => "/account/exportTransactions",
            SolscanEndpoints::Account => "/account",
            //Token
            SolscanEndpoints::TokenHolders => "/token/holders",
            SolscanEndpoints::TokenMeta => "/token/meta",
            SolscanEndpoints::TokenList => "/token/list",
            //Market
            SolscanEndpoints::MarketToken => "/market/token",
            //ChainInformation
            SolscanEndpoints::ChainInfo => "/chaininfo",
            //Tools
            SolscanEndpoints::ToolsInspect => "Tools",
        }
    }
}