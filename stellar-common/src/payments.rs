use substreams::Hex;

use crate::{
    pb::sf::{
        stellar::r#type::v1::Block,
        substreams::stellar::r#type::v1::{Payment, Payments},
    },
    utils,
    constants,
};
use core::panic;

#[substreams::handlers::map]
fn map_payments(block: Block) -> Result<Payments, substreams::errors::Error> {
    let mut payments = Payments::default();

    block.transactions.iter().for_each(|transaction| {
        if is_failure(transaction.status){
            return;
        }

        let hash = Hex(&transaction.hash).to_string();

        let trx = match utils::decode_transaction(&transaction.result_xdr, &transaction.envelope_xdr) {
            Ok(trx) => trx,
            Err(_) => return,
        };

        trx.operations.iter().for_each(|operation| match &operation.body {
            stellar_xdr::curr::OperationBody::Payment(payment) => {
                let amount = payment.amount as f64 / constants::XLM_DENOMINATOR; // todo: valid with them
                let asset = match_asset_code(&payment.asset);
                let destination = payment.destination.to_string();
                let source;
                if asset == "XLM" {
                    source = constants::XLM_SOURCE_ACCOUNT.into();
                } else {
                    source = operation.source_account.as_ref().unwrap().to_string()
                }
                substreams::log::println(format!("payment.amount {}", payment.amount));
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
fn filtered_accounts(query: String, payments: Payments) -> Result<Payments, substreams::errors::Error> {
    let query = substreams::expr_matcher(&query);

    let mut filtered_payments = Payments {
        payments: payments.payments,
    };

    filtered_payments.payments.retain(|payment| query.matches_keys(&vec![format!("account:{}",&payment.source), format!("account:{}",&payment.destination)]));

    Ok(filtered_payments)
}

fn match_asset_code(asset: &stellar_xdr::curr::Asset) -> String {
    match asset {
        stellar_xdr::curr::Asset::Native => "XLM".to_string(),
        stellar_xdr::curr::Asset::CreditAlphanum4(credit) => {
            format!("{}", credit.asset_code)
        }
        stellar_xdr::curr::Asset::CreditAlphanum12(credit) => {
            format!("{}", credit.asset_code)
        }
    }
}

fn is_failure(status: i32) -> bool {
    status == 2
}