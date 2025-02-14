use crate::{
    constants::{self, XLM_ASSET_CODE},
    helpers,
    pb::sf::{
        stellar::r#type::v1::Block,
        substreams::stellar::r#type::v1::{
            address, token_transfer::Event, Address, EventMeta, TokenTransfer, TokenTransfers, Transfer,
        },
    },
    utils,
};
use core::panic;
use substreams::scalar::BigDecimal;
use substreams::Hex;

#[substreams::handlers::map]
fn map_token_transfers(block: Block) -> Result<TokenTransfers, substreams::errors::Error> {
    let mut token_transfers: TokenTransfers = TokenTransfers::default();

    block.transactions.iter().for_each(|transaction| {
        if utils::transaction_failed(transaction.status) {
            return;
        }

        let hash = Hex(&transaction.hash).to_string();

        let trx = match utils::decode_transaction(&transaction.result_xdr, &transaction.envelope_xdr) {
            Ok(trx) => trx,
            Err(_) => return,
        };

        // All operations that emit token transfers have been taken from https://github.com/stellar/go/issues/5580
        trx.operations.iter().for_each(|operation| match &operation.body {
            // Create account: https://stellar.expert/explorer/testnet/tx/8191ad23e96b7426fd6692cd0cf402e85067ea2ce40c97c6a6562bfffe71a0b2
            stellar_xdr::curr::OperationBody::CreateAccount(create_account_op) => {
                token_transfers.token_transfer_events.push(TokenTransfer {
                    meta: Some(helpers::event_meta::create_event_meta(
                        block.number,
                        block.created_at,
                        &hash,
                    )),
                    asset: Some(XLM_ASSET_CODE.to_string()),
                    event: Some(Event::Transfer(Transfer {
                        from: None, // TODO: what to do here?
                        to: Some(helpers::address::create_account_address(&create_account_op.destination)),
                        amount: BigDecimal::from(create_account_op.starting_balance).into(),
                    })),
                });
            }
            // Merge Account: https://stellar.expert/explorer/testnet/tx/2bab87af23a13fbe40c363dd326767bb85b00692a3428906f080f2a2a80423c6
            stellar_xdr::curr::OperationBody::AccountMerge(muxed_account) => {
                let result_xdr = match utils::decode_transaction_result(&transaction.result_xdr) {
                    Ok(result) => result,
                    Err(_) => return,
                };
                match utils::decode_account_merge_result(&result_xdr) {
                    Some(result) => {
                        token_transfers.token_transfer_events.push(TokenTransfer {
                            meta: Some(helpers::event_meta::create_event_meta(
                                block.number,
                                block.created_at,
                                &hash,
                            )),
                            asset: Some(XLM_ASSET_CODE.to_string()),
                            event: Some(Event::Transfer(Transfer {
                                from: Some(helpers::address::create_account_address(
                                    &trx.source_account.clone().account_id(),
                                )),
                                to: Some(helpers::address::create_account_address(
                                    &muxed_account.clone().account_id(),
                                )),
                                amount: BigDecimal::from(result).into(),
                            })),
                        });
                    }
                    None => return,
                }
            }
            stellar_xdr::curr::OperationBody::Payment(payment) => {
                let asset = utils::match_asset_code(&payment.asset);
                // TODO: add logic if the we are a Transfer, Mint or Burn
                token_transfers.token_transfer_events.push(TokenTransfer {
                    meta: Some(helpers::event_meta::create_event_meta(
                        block.number,
                        block.created_at,
                        &hash,
                    )),
                    asset: Some(asset),
                    event: Some(Event::Transfer(Transfer {
                        from: Some(helpers::address::create_account_address(
                            &trx.source_account.clone().account_id(),
                        )),
                        to: Some(helpers::address::create_account_address(
                            &payment.destination.clone().account_id(),
                        )),
                        amount: BigDecimal::from(payment.amount).into(),
                    })),
                });
            }
            _ => {}
        });
    });

    Ok(token_transfers)
}

// #[substreams::handlers::map]
// fn filtered_payments(query: String, payments: Payments) -> Result<Payments, substreams::errors::Error> {
//     let query = substreams::expr_matcher(&query);

//     let mut filtered_payments = Payments {
//         payments: payments.payments,
//     };

//     filtered_payments.payments.retain(|payment| {
//         query.matches_keys(&vec![
//             format!("account:{}", &payment.source),
//             format!("account:{}", &payment.destination),
//         ])
//     });

//     Ok(filtered_payments)
// }

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
