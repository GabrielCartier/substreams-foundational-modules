use crate::{
    index,
    pb::sf::tron::type::v1::{Block, Transaction, Transactions},
};

#[substreams::handlers::map]
fn map_transactions(block: Block) -> Result<Transactions, substreams::errors::Error> {
    // TODO: filter failed transactions
    let transactions: Vec<Transaction> = block.transactions;
    Ok(Transactions { transactions })
}

#[substreams::handlers::map]
fn filtered_transactions(
    query: String,
    transactions: Transactions,
) -> Result<Transactions, substreams::errors::Error> {
    let matcher = substreams::expr_matcher(&query);
    let transactions: Vec<Transaction> = transactions
        .transactions
        .into_iter()
        .filter(|transaction| {
            matcher.matches_keys(&index::transaction_keys(transaction))
        })
        .collect();

    Ok(Transactions { transactions })
} 