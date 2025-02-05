use substreams::Hex;

use crate::{
    constants,
    pb::sf::{
        stellar::r#type::v1::Block,
        substreams::stellar::r#type::v1::{Payment, Payments},
    },
    utils,
};
use core::panic;

#[substreams::handlers::map]
fn map_payments(block: Block) -> Result<Payments, substreams::errors::Error> {
    let mut payments = Payments::default();

    block.transactions.iter().for_each(|transaction| {
        if utils::transaction_failed(transaction.status) {
            return;
        }

        let hash = Hex(&transaction.hash).to_string();

        let trx =
            match utils::decode_transaction(&transaction.result_xdr, &transaction.envelope_xdr) {
                Ok(trx) => trx,
                Err(_) => return,
            };

        trx.operations
            .iter()
            .for_each(|operation| match &operation.body {
                stellar_xdr::curr::OperationBody::Payment(payment) => {
                    let amount = payment.amount as f64 / constants::XLM_DENOMINATOR;
                    let asset = utils::match_asset_code(&payment.asset);
                    let destination = payment.destination.to_string();
                    let source;
                    if asset == constants::XML_ASSET_CODE {
                        source = match operation.source_account.as_ref() {
                            Some(account) => account.to_string(),

                            None => {
                                let trx_source = trx.source_account.to_string();
                                if trx_source != "" {
                                    trx_source
                                } else {
                                    constants::XLM_SOURCE_ACCOUNT.into()
                                }
                            }
                        }
                    } else {
                        source = match operation.source_account.as_ref() {
                            Some(account) => account.to_string(),

                            None => {
                                let trx_source = trx.source_account.to_string();
                                if trx_source != "" {
                                    trx_source
                                } else {
                                    utils::fetch_asset_issuer(&payment.asset)
                                }
                            }
                        }
                    }
                    payments.payments.push(Payment {
                        source: source,
                        amount: amount,
                        asset,
                        destination,
                        trx_hash: hash.clone(),
                    });
                }
                stellar_xdr::curr::OperationBody::AccountMerge(muxed_account) => {
                    let destination_account = muxed_account.to_string();
                    let result_xdr = match utils::decode_transaction_result(&transaction.result_xdr)
                    {
                        Ok(result) => result,
                        Err(_) => return,
                    };
                    match utils::decode_account_merge_result(&result_xdr) {
                        Some(result) => {
                            let amount = result as f64 / constants::XLM_DENOMINATOR;
                            payments.payments.push(Payment {
                                source: trx.source_account.to_string(),
                                amount,
                                asset: constants::XML_ASSET_CODE.to_string(),
                                destination: destination_account,
                                trx_hash: hash.clone(),
                            });
                        }
                        None => return,
                    }
                }
                _ => {}
            });
    });

    Ok(payments)
}

#[substreams::handlers::map]
fn filtered_payments(
    query: String,
    payments: Payments,
) -> Result<Payments, substreams::errors::Error> {
    let query = substreams::expr_matcher(&query);

    let mut filtered_payments = Payments {
        payments: payments.payments,
    };

    filtered_payments.payments.retain(|payment| {
        query.matches_keys(&vec![
            format!("account:{}", &payment.source),
            format!("account:{}", &payment.destination),
        ])
    });

    Ok(filtered_payments)
}

/*
    TODO: MergeAccount operation
        - do we want to output also the merge account operation?
        -> https://stellar.expert/explorer/testnet/tx/0a0ecdb780d8eb06c45f433a3b45c6628e06dd6fd71a7c985363b4b9c7a4a413
        - Transfers the XLM balance of an account to another account and removes the source account from the ledger
        - https://developers.stellar.org/docs/learn/fundamentals/transactions/list-of-operations#account-merge
    Also, when a merge account operation is successful it will close the account and remove it from the ledger.

    This means we would need to listen on a that specific event also, so the client side, knows when to stop
    listening for that account and consider it closed.
*/
