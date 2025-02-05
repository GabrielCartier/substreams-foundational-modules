use crate::pb::sf::substreams::solana::v1::Transactions;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

#[substreams::handlers::map]
fn transactions_by_programid_without_votes(
    query: String,
    block: Block,
) -> Result<Transactions, substreams::errors::Error> {
    _transactions_by_programid_without_votes(query, block)
}

/// _transactions_by_programid_without_votes is equal to [transactions_by_programid_without_votes] but exists only for unit testing purposes.
fn _transactions_by_programid_without_votes(
    query: String,
    block: Block,
) -> Result<Transactions, substreams::errors::Error> {
    let query = substreams::expr_matcher(&query);

    let mut transactions = Transactions {
        transactions: block.transactions,
    };

    transactions.transactions.retain(|trx| {
        trx.walk_instructions()
            .any(|view| query.matches_keys(&vec![format!("program:{}", view.program_id())]))
    });

    Ok(transactions)
}

#[substreams::handlers::map]
fn transactions_by_programid_and_account_without_votes(
    query: String,
    block: Block,
) -> Result<Transactions, substreams::errors::Error> {
    _transactions_by_programid_and_account_without_votes(query, block)
}

/// transactions_by_programid_and_account_without_votes is equal to [transactions_by_programid_and_account_without_votes] but exists only for unit testing purposes.
fn _transactions_by_programid_and_account_without_votes(
    query: String,
    block: Block,
) -> Result<Transactions, substreams::errors::Error> {
    let query = substreams::expr_matcher(&query);

    let mut transactions = Transactions {
        transactions: block.transactions,
    };

    transactions.transactions.retain(|trx| {
        trx.walk_instructions().any(|view| {
            query.matches_keys(
                &view
                    .accounts()
                    .iter()
                    .map(|acc| format!("account:{}", acc))
                    .chain(vec![format!("program:{}", view.program_id())])
                    .collect::<Vec<String>>(),
            )
        })
    });

    Ok(transactions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing;

    #[test]
    fn test_transactions_by_programid_without_votes() {
        // Given
        let block = testing::read_block("testdata/solana_mainnet_313000000.binpb.base64");

        // When
        let result = _transactions_by_programid_without_votes(
            "program:whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc".to_owned(),
            block,
        )
        .expect("Failed to execute function");

        // Expect
        result.transactions.into_iter().for_each(|transaction| {
            assert_eq!(
                transaction
                    .walk_instructions()
                    .any(|instruction| instruction.program_id().to_string()
                        == "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"),
                true
            )
        });
    }

    #[test]
    fn test_transactions_by_programid_and_account_without_votes() {
        // Given
        let block = testing::read_block("testdata/solana_mainnet_313000000.binpb.base64");

        // When
        let result = _transactions_by_programid_and_account_without_votes(
            "program:whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc && account:5qrvgpvr55Eo7c5bBcwopdiQ6TpvceiRm42yjHTbtDvc".to_owned(),
            block,
        )
        .expect("Failed to execute function");

        // Expect
        result.transactions.into_iter().for_each(|transaction| {
            let mut matched = true;

            // Check if the given program id is contained within the instructions.
            if !transaction.walk_instructions().any(|instruction| {
                instruction.program_id().to_string()
                    == "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"
            }) {
                matched = false
            }

            // For all the instructions of the given program id, check if the account is contained
            transaction
                .walk_instructions()
                .filter(|instruction| {
                    instruction.program_id().to_string()
                        == "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"
                })
                .for_each(|instruction| {
                    matched = instruction.accounts().iter().any(|account| {
                        account.to_string() == "5qrvgpvr55Eo7c5bBcwopdiQ6TpvceiRm42yjHTbtDvc"
                    })
                });

            assert_eq!(matched, true)
        });
    }
}
