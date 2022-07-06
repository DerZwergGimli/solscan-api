#[cfg(test)]
mod test_serde {
    use std::error::Error;
    use std::fs::File;
    use std::io::{BufReader, Read};

    use crate::structs::block::Block;
    use crate::structs::token::Token;
    use crate::structs::transaction::Transaction;

    fn load_file(path: &str) -> BufReader<File> {
        let file = File::open(path).unwrap();
        let mut reader_file = BufReader::new(file);
        reader_file
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
}