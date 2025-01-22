use crate::{
    pb::sf::{
        stellar::r#type::v1::Block,
        substreams::stellar::r#type::v1::{Payment, Payments},
    },
    utils,
};
use core::panic;

static XLM_SOURCE_ACCOUNT: &str = "GA6QUJYL4AYEIZC7W6OEPGU3QDRK33WJP5WMMZLDZBUE7M3VWDF7LTTR";

#[substreams::handlers::map]
fn map_payments(block: Block) -> Result<Payments, substreams::errors::Error> {
    let mut payments = Payments::default();

    block.transactions.iter().for_each(|transaction| {
        // TODO: update this with the latest change to the status enum of the transaction
        if transaction.status != "SUCCESS" {
            return;
        }

        let hash = base64::encode(transaction.hash.clone());

        let trx = match utils::decode_transaction(&transaction.result_xdr, &transaction.envelope_xdr) {
            Ok(trx) => trx,
            Err(_) => return,
        };

        trx.operations.iter().for_each(|operation| match &operation.body {
            stellar_xdr::curr::OperationBody::CreateAccount(create_account) => {
                substreams::log::println(format!("create account destination {}", create_account.destination));
            }
            stellar_xdr::curr::OperationBody::Payment(payment) => {
                let amount = payment.amount / 10000000; // todo: valid with them
                let asset = match_asset_code(&payment.asset);
                let destination = payment.destination.to_string();
                let source;
                if asset == "XLM" {
                    source = XLM_SOURCE_ACCOUNT.into();
                } else {
                    source = operation.source_account.as_ref().unwrap().to_string()
                }
                payments.payments.push(Payment {
                    source: source,
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
