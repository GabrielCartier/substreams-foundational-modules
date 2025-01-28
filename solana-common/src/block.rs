use substreams_solana::b58;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

static VOTE_INSTRUCTION: [u8; 32] = b58!("Vote111111111111111111111111111111111111111");

#[substreams::handlers::map]
fn blocks_without_votes(mut block: Block) -> Result<Block, substreams::errors::Error> {
    return _blocks_without_votes(block);
}

pub fn _blocks_without_votes(mut block: Block) -> Result<Block, substreams::errors::Error> {
    block.transactions.retain(|trx| {
        let meta = match trx.meta.as_ref() {
            Some(meta) => meta,
            None => return false,
        };
        if meta.err.is_some() {
            return false;
        }

        let transaction = match trx.transaction.as_ref() {
            Some(transaction) => transaction,
            None => return false,
        };
        let message = transaction.message.as_ref().expect("Message is missing");

        // Retain only transactions that do **not** contain a vote instruction
        !message.account_keys.iter().any(|v| v == &VOTE_INSTRUCTION)
    });

    Ok(block)
}

mod tests {
    use std::fs;

    use anyhow::Error;
    use prost::Message;
    use substreams_solana::pb::sf::solana::r#type::v1::Block;
    use base64::decode;

    use super::{_blocks_without_votes, blocks_without_votes, VOTE_INSTRUCTION};

    #[test]
    fn test_block_without_votes() {
        // Given
        let block = parse_block().expect("Failed to parse block");

        // When
        let result = _blocks_without_votes(block).expect("Failed to execute function");

        // Expect
        result.transactions().for_each(|t| {
            assert_eq!(t.transaction.clone().unwrap().message.unwrap().account_keys.iter().all(|acct| acct != &VOTE_INSTRUCTION), true)
        });
    }

    fn parse_block() -> Result<Block, Error> {
        let encoded = fs::read_to_string("./src/test_block_313000000")?;

        // Decode Base64 into raw bytes
        let raw_bytes = decode(&encoded)?;

        return Ok(Block::decode(&*raw_bytes).expect("Not able to decode Block"));
    }
}
