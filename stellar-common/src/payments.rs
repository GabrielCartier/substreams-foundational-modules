use crate::{
    pb::sf::{
        stellar::r#type::v1::Block,
        substreams::stellar::r#type::v1::{Payment, Payments},
    },
    utils,
};
use core::panic;
use stellar_xdr::curr::{TransactionEnvelope, TransactionResultResult};

#[substreams::handlers::map]
fn map_payments(block: Block) -> Result<Payments, substreams::errors::Error> {
    let mut payments = Payments::default();

    block.transactions.iter().for_each(|transaction| {
        let hash = base64::encode(transaction.hash.clone());

        // if hash != "3843945a3b14be22366ba5ba354e9b957d7415629bdb71c7e9e89059acffde04" {
        //     return;
        // }

        // substreams::log::println(format!("transaction hash {}", hash));

        let trx = match utils::decode_transaction(&transaction.result_xdr, &transaction.envelope_xdr) {
            Ok(trx) => trx,
            Err(_) => return,
        };

        let trx_meta = match utils::decode_transaction_meta(&transaction.result_meta_xdr) {
            Ok(trx_meta) => trx_meta,
            Err(_) => return,
        };

        let trx_meta_v3 = match trx_meta {
            stellar_xdr::curr::TransactionMeta::V3(v3) => v3,
            _ => return,
        };

        // Fetch all acounts where there was a change in the ledger
        let mut stellar_account_changes_before: Vec<StellarAccountChanges> = vec![];
        trx_meta_v3.tx_changes_before.iter().for_each(|change| match change {
            stellar_xdr::curr::LedgerEntryChange::Updated(ledger_entry)
            | stellar_xdr::curr::LedgerEntryChange::Created(ledger_entry)
            | stellar_xdr::curr::LedgerEntryChange::State(ledger_entry) => match &ledger_entry.data {
                stellar_xdr::curr::LedgerEntryData::Account(account) => {
                    stellar_account_changes_before.push(StellarAccountChanges {
                        account_id: account.account_id.to_string(),
                        balance: account.balance,
                    });
                }
                _ => return,
            },
            _ => return,
        });

        stellar_account_changes_before.iter().for_each(|change| {
            substreams::log::println(format!(
                "before account_id: {}, balance: {}",
                change.account_id, change.balance
            ));
        });

        let mut stellar_account_changes_after: Vec<StellarAccountChanges> = vec![];
        trx_meta_v3.tx_changes_after.iter().for_each(|change| match change {
            stellar_xdr::curr::LedgerEntryChange::Updated(ledger_entry)
            | stellar_xdr::curr::LedgerEntryChange::Created(ledger_entry)
            | stellar_xdr::curr::LedgerEntryChange::State(ledger_entry) => match &ledger_entry.data {
                stellar_xdr::curr::LedgerEntryData::Account(account) => {
                    stellar_account_changes_after.push(StellarAccountChanges {
                        account_id: account.account_id.to_string(),
                        balance: account.balance,
                    });
                }
                _ => return,
            },
            _ => return,
        });

        stellar_account_changes_after.iter().for_each(|change| {
            substreams::log::println(format!(
                "after account_id: {}, balance: {}",
                change.account_id, change.balance
            ));
        });

        substreams::log::println(format!("num of operations: {}", trx_meta_v3.operations.len()));
        let mut stellar_account_changes_operation: Vec<StellarAccountChanges> = vec![];
        trx_meta_v3.operations.iter().for_each(|change| {
            change.changes.iter().for_each(|ch| match ch {
                stellar_xdr::curr::LedgerEntryChange::Updated(ledger_entry)
                | stellar_xdr::curr::LedgerEntryChange::Created(ledger_entry)
                | stellar_xdr::curr::LedgerEntryChange::State(ledger_entry) => match &ledger_entry.data {
                    stellar_xdr::curr::LedgerEntryData::Account(account) => {
                        stellar_account_changes_operation.push(StellarAccountChanges {
                            account_id: account.account_id.to_string(),
                            balance: account.balance,
                        });
                    }
                    _ => return,
                },
                _ => return,
            })
        });

        stellar_account_changes_operation.iter().for_each(|change| {
            substreams::log::println(format!(
                "operation account_id: {}, balance: {}",
                change.account_id, change.balance
            ));
        });

        // The address of the source is found in the Trustline of the block, need to understand how to pick it correctly

        // https://stellar.expert/explorer/testnet/tx/3843945a3b14be22366ba5ba354e9b957d7415629bdb71c7e9e89059acffde04
        // here we have a mismatch between the source account and the source account in the transaction
        // investigate why
        substreams::log::println(format!("number of operations {}", trx.operations.len()));
        // to fetch the correct account for the source, then we need to check the balance before and after the transaction
        // and we have to match on the same source account with the same amount that was changed
        trx.operations.iter().for_each(|operation| match &operation.body {
            stellar_xdr::curr::OperationBody::CreateAccount(create_account) => {
                substreams::log::println(format!("create account destination {}", create_account.destination));
            }
            stellar_xdr::curr::OperationBody::Payment(payment) => {
                let amount = payment.amount / 10000000; // todo: valid with them
                let asset = match &payment.asset {
                    stellar_xdr::curr::Asset::Native => "XLM".to_string(),
                    stellar_xdr::curr::Asset::CreditAlphanum4(credit) => {
                        format!("{}", credit.asset_code)
                    }
                    stellar_xdr::curr::Asset::CreditAlphanum12(credit) => {
                        format!("{}", credit.asset_code)
                    }
                };
                let destination = payment.destination.to_string();
                payments.payments.push(Payment {
                    source: trx.source_account.to_string(), // the source should be: GC6ICL6XLKEYWWQAHTOGIDZC3VG3ZO7XPEUV3AV3O4EUZTLBMNJUB4AY
                    amount: amount as u64,
                    asset,
                    destination,
                    trx_hash: hash.clone(),
                });
            }
            _ => {}
        });
    });

    Ok(payments)
}

struct StellarAccountChanges {
    account_id: String,
    balance: i64,
}
