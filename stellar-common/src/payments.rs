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

        let trx = match utils::decode_transaction(&transaction.result_xdr, &transaction.envelope_xdr) {
            Ok(trx) => trx,
            Err(_) => return,
        };

        trx.operations.iter().for_each(|operation| match &operation.body {
            stellar_xdr::curr::OperationBody::Payment(payment) => {
                let amount = payment.amount as f64 / constants::XLM_DENOMINATOR;
                let asset = utils::match_asset_code(&payment.asset);
                let destination = payment.destination.to_string();
                let source;
                if asset == constants::XML_ASSET_CODE {
                    source = constants::XLM_SOURCE_ACCOUNT.into();
                } else {
                    source = match operation.source_account.as_ref() {
                        Some(account) => account.to_string(),
                        // If there is no source account, we need to fetch the issuer of the asset
                        // it means we have a mint that occurred on chain.
                        None => utils::fetch_asset_issuer(&payment.asset),
                    }
                }
                // substreams::log::println(format!("payment.amount {}", payment.amount));
                payments.payments.push(Payment {
                    source: source,
                    amount: amount,
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

#[substreams::handlers::map]
fn filtered_payments(query: String, payments: Payments) -> Result<Payments, substreams::errors::Error> {
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
*/
