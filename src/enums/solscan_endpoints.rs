#[derive(Debug)]
pub enum SolscanEndpoints {
    //Block
    BlockLast,
    BlockTransactions,
    Block,
    //Transaction
    TransactionLast,
    Transaction,
    //Account
    AccountTokens,
    AccountTransaction,
    AccountStakeAccounts,
    AccountSPLTransfer,
    AccountSolTransfers,
    AccountExportTransactions,
    Account,
    //Token
    TokenHolders,
    TokenMeta,
    TokenList,
    //Market
    MarketToken,
    //ChainInformation
    ChainInfo,
    //Tools
    ToolsInspect,
}

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
            SolscanEndpoints::AccountTokens => "/account/tokens?account=",
            //Account
            SolscanEndpoints::AccountTransaction => "/account/transactions",
            SolscanEndpoints::AccountStakeAccounts => "/account/stakeAccounts",
            SolscanEndpoints::AccountSPLTransfer => "/account/splTransfers",
            SolscanEndpoints::AccountSolTransfers => "/account/solTransfers",
            SolscanEndpoints::AccountExportTransactions => "/account/exportTransactions",
            SolscanEndpoints::Account => "/account/",
            //Token
            SolscanEndpoints::TokenHolders => "/token/holders",
            SolscanEndpoints::TokenMeta => "/token/meta",
            SolscanEndpoints::TokenList => "/token/list",
            //Market
            SolscanEndpoints::MarketToken => "/market/token",
            //ChainInformation
            SolscanEndpoints::ChainInfo => "/chaininfo/",
            //Tools
            SolscanEndpoints::ToolsInspect => "Tools",
        }
    }
}