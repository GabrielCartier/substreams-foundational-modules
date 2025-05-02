use std::collections::HashSet;

use substreams::pb::sf::substreams::index::v1::Keys;

use crate::pb::sf::tron::type::v1::Transaction;
use crate::pb::sf::tron::type::v1::Transactions;
use crate::pb::protocol;

#[substreams::handlers::map]
fn index_transactions(transactions: Transactions) -> Result<Keys, substreams::errors::Error> {
    let mut keys = HashSet::new();

    for transaction in transactions.transactions {
        for key in transaction_keys(&transaction) {
            keys.insert(key);
        }
    }

    Ok(Keys {
        keys: keys.into_iter().collect(),
    })
}

pub fn transaction_keys(transaction: &Transaction) -> Vec<String> {
    let mut keys = Vec::new();

    if let Some(ext) = &transaction.transaction_extention {
        if let Some(trx) = &ext.transaction {
            if let Some(raw) = &trx.raw_data {
                for contract in &raw.contract {
                    let type_name = protocol::transaction::contract::ContractType::from_i32(contract.r#type)
                        .map(|t| t.as_str_name())
                        .unwrap_or("Unknown");
                    keys.push(format!("contract_type:{}", type_name));
                }
            }
        }
    }

    keys
}
