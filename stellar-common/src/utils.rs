use std::io::Cursor;
use stellar_xdr::curr::{
    Limited, Limits, ReadXdr, Transaction, TransactionEnvelope, TransactionMeta, TransactionResult,
    TransactionResultResult,
};

use crate::constants;

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

pub fn transaction_failed(status: i32) -> bool {
    status == 2
}

pub fn match_asset_code(asset: &stellar_xdr::curr::Asset) -> String {
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

pub fn match_change_trust_op_asset(change_trust_op_asset: &stellar_xdr::curr::ChangeTrustAsset) -> String {
    match change_trust_op_asset {
        stellar_xdr::curr::ChangeTrustAsset::Native => "XLM".to_string(),
        stellar_xdr::curr::ChangeTrustAsset::CreditAlphanum4(credit) => {
            format!("{}", credit.asset_code)
        }
        stellar_xdr::curr::ChangeTrustAsset::CreditAlphanum12(credit) => {
            format!("{}", credit.asset_code)
        }
        stellar_xdr::curr::ChangeTrustAsset::PoolShare(_) => {
            substreams::log::println("PoolShare asset type not supported yet");
            "".to_string()
        }
    }
}

pub fn fetch_asset_issuer(asset: &stellar_xdr::curr::Asset) -> String {
    match asset {
        stellar_xdr::curr::Asset::Native => constants::XLM_SOURCE_ACCOUNT.to_string(),
        stellar_xdr::curr::Asset::CreditAlphanum4(credit) => credit.issuer.0.to_string(),
        stellar_xdr::curr::Asset::CreditAlphanum12(credit) => credit.issuer.0.to_string(),
    }
}

pub fn fetch_change_trust_op_asset_issuer(change_trust_op_asset: &stellar_xdr::curr::ChangeTrustAsset) -> String {
    match change_trust_op_asset {
        stellar_xdr::curr::ChangeTrustAsset::Native => constants::XLM_SOURCE_ACCOUNT.to_string(),
        stellar_xdr::curr::ChangeTrustAsset::CreditAlphanum4(credit) => credit.issuer.0.to_string(),
        stellar_xdr::curr::ChangeTrustAsset::CreditAlphanum12(credit) => credit.issuer.0.to_string(),
        stellar_xdr::curr::ChangeTrustAsset::PoolShare(_) => {
            substreams::log::println("PoolShare asset type not supported yet");
            "".to_string()
        }
    }
}
