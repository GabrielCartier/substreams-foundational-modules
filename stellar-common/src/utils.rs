use std::io::Cursor;
use stellar_xdr::curr::{
    Limited, Limits, ReadXdr, Transaction, TransactionEnvelope, TransactionMeta, TransactionResult,
    TransactionResultResult,
};

pub fn decode_transaction(
    result_xdr: &Vec<u8>,
    envelope_xdr: &Vec<u8>,
) -> Result<Transaction, substreams::errors::Error> {
    let trx_result = match decode_transaction_result(result_xdr) {
        Ok(result) => result,
        Err(_) => panic!("Could not decode transaction result"),
    };

    match trx_result.result {
        TransactionResultResult::TxSuccess(_) => {}
        _ => return Err(substreams::errors::Error::msg("Transaction failed")),
    }

    let trx = match decode_transaction_envelope(envelope_xdr) {
        Ok(trx) => trx,
        Err(_) => panic!("Could not decode transaction envelope"),
    };

    let trx_v1 = match trx {
        TransactionEnvelope::Tx(v1) => v1,
        _ => panic!("Expected TransactionV1Envelope type"),
    };

    return Ok(trx_v1.tx);
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

pub fn _decode_transaction_meta(result_meta_xdr: &Vec<u8>) -> Result<TransactionMeta, stellar_xdr::curr::Error> {
    let buf = Cursor::new(result_meta_xdr);
    let transaction_meta = TransactionMeta::read_xdr(&mut Limited::new(buf, Limits::none()));
    transaction_meta
}
