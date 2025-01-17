use crate::{
    pb::sf::{
        stellar::r#type::v1::Block,
        substreams::stellar::r#type::v1::{Account, Accounts},
    },
    utils,
};
use core::panic;

#[substreams::handlers::map]
fn map_created_accounts(block: Block) -> Result<Accounts, substreams::errors::Error> {
    let mut accounts = Accounts::default();

    block.transactions.iter().for_each(|transaction| {
        let hash = base64::encode(transaction.hash.clone());

        let decoded_transaction = match utils::decode_transaction(&transaction.result_xdr, &transaction.envelope_xdr) {
            Ok(trx) => trx,
            Err(_) => return,
        };

        decoded_transaction
            .operations
            .iter()
            .for_each(|operation| match &operation.body {
                stellar_xdr::curr::OperationBody::CreateAccount(create_account) => {
                    accounts.accounts.push(Account {
                        trx_hash: hash.clone(),
                        address: create_account.destination.to_string(),
                        balance: (create_account.starting_balance / 10000000) as u64,
                    });
                }
                _ => {}
            });
    });

    Ok(accounts)
}
