use crate::pb::sf::{
    stellar::r#type::v1::Block,
    substreams::stellar::r#type::v1::{Accounts, Payment, Payments},
};
use core::panic;
use std::io::Cursor;
use stellar_xdr::curr::{
    Limited, Limits, ReadXdr, TransactionEnvelope, TransactionMeta, TransactionResult, TransactionResultResult,
    TransactionV1Envelope,
};

#[substreams::handlers::map]
fn map_created_accounts(block: Block) -> Result<Accounts, substreams::errors::Error> {
    let mut accounts = Accounts::default();

    Ok(accounts)
}

#[substreams::handlers::map]
fn map_payments(block: Block) -> Result<Payments, substreams::errors::Error> {
    let mut payments = Payments::default();

    /*
        TODO:
            - loop over all the transactilns
            - decode the resultMetaXdr
            - match any Payment operation
            - add the payment to the payments object
    */

    block.transactions.iter().for_each(|transaction| {
        let trx_result = match decode_transaction_result(&transaction.result_xdr) {
            Ok(result) => result,
            Err(_) => panic!("Could not decode transaction result"),
        };

        let result = match trx_result.result {
            TransactionResultResult::TxSuccess(_) => true,
            _ => false,
        };

        if !result {
            return;
        }

        let trx = match decode_transaction_envelope(&transaction.envelope_xdr) {
            Ok(trx) => trx,
            Err(_) => panic!("Could not decode transaction envelope"),
        };

        let trx_v1 = match trx {
            TransactionEnvelope::Tx(v1) => v1,
            _ => panic!("Expected TransactionV1Envelope type"),
        };

        let trx = trx_v1.tx;
        let source = trx.source_account.to_string();
        trx.operations.iter().for_each(|operation| match &operation.body {
            stellar_xdr::curr::OperationBody::Payment(payment) => {
                // let amount = payment.amount;
                let amount = 10;
                // let asset = payment.asset.to_string();
                let asset = String::from("");
                let destination = payment.destination.to_string();
                payments.payments.push(Payment {
                    source: source.clone(),
                    amount,
                    asset,
                    destination,
                });
            }
            _ => {}
        });
    });

    Ok(payments)
}

fn decode_transaction_result(result_xdr: &Vec<u8>) -> Result<TransactionResult, stellar_xdr::curr::Error> {
    let buf = Cursor::new(result_xdr);
    let transaction_result = TransactionResult::read_xdr(&mut Limited::new(buf, Limits::none()));
    transaction_result
}

fn decode_transaction_envelope(envelope_xdr: &Vec<u8>) -> Result<TransactionEnvelope, stellar_xdr::curr::Error> {
    let buf = Cursor::new(envelope_xdr);
    let transaction_envelope = TransactionEnvelope::read_xdr(&mut Limited::new(buf, Limits::none()));
    transaction_envelope
}

fn decode_transaction_meta(result_meta_xdr: &String) -> Result<TransactionMeta, stellar_xdr::curr::Error> {
    let buf = Cursor::new(result_meta_xdr);
    let transaction_meta = TransactionMeta::read_xdr(&mut Limited::new(buf, Limits::none()));
    transaction_meta
}
