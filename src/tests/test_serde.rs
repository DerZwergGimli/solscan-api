#![allow(unused_variables)]

#[cfg(test)]
mod test_serde {
    use std::fs::File;
    use std::io::BufReader;

    use crate::structs::block::Block;
    use crate::structs::spl_transfer::SplTransfer;
    use crate::structs::token::Token;
    use crate::structs::transaction::Transaction;

    fn load_file(path: &str) -> BufReader<File> {
        let file = File::open(path).unwrap();
        BufReader::new(file)
    }


    #[test]
    fn test_serde_blocks() {
        let reader_file = load_file("./src/tests/sample_data/block_last_20.json");
        let blocks: Vec<Block> = serde_json::from_reader(reader_file).unwrap();
    }


    #[test]
    fn test_serde_account_tokens() {
        let reader_file = load_file("./src/tests/sample_data/account_tokens.json");
        let tokens: Vec<Token> = serde_json::from_reader(reader_file).unwrap();
    }

    #[test]
    fn test_serde_account_transactions() {
        let reader_file = load_file("./src/tests/sample_data/account_transactions.json");
        let tokens: Vec<Transaction> = serde_json::from_reader(reader_file).unwrap();
    }

    #[test]
    fn test_serde_account_spl_transfer() {
        let reader_file = load_file("./src/tests/sample_data/spl_transfer_50.json");
        let tokens: SplTransfer = serde_json::from_reader(reader_file).unwrap();
    }
}