use crate::{
    constants::{self, XLM_ASSET_CODE},
    helpers,
    pb::sf::{
        stellar::r#type::v1::Block,
        substreams::stellar::r#type::v1::{
            address, token_transfer::Event, Address, Clawback, EventMeta, TokenTransfer, TokenTransfers, Transfer,
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
            stellar_xdr::curr::OperationBody::CreateClaimableBalance(create_claim_balance) => {
                let asset = utils::match_asset_code(&create_claim_balance.asset);
                // let to = create_claim_balance.claimants.first().unwrap();
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
                        to: None, // TODO: what to do here?
                        amount: BigDecimal::from(create_claim_balance.amount).into(),
                    })),
                });
            }
            stellar_xdr::curr::OperationBody::ClaimClaimableBalance(claim_claim_balance) => {
                // TODO: does the transfer only occur when the claimable balance is claimed?
            }
            stellar_xdr::curr::OperationBody::Clawback(clawback) => {
                let asset = utils::match_asset_code(&clawback.asset);
                // let to = create_claim_balance.claimants.first().unwrap();
                token_transfers.token_transfer_events.push(TokenTransfer {
                    meta: Some(helpers::event_meta::create_event_meta(
                        block.number,
                        block.created_at,
                        &hash,
                    )),
                    asset: Some(asset),
                    event: Some(Event::Clawback(Clawback {
                        from: Some(helpers::address::create_account_address(
                            &trx.source_account.clone().account_id(),
                        )),
                        amount: BigDecimal::from(clawback.amount).into(),
                    })),
                });
            }
            stellar_xdr::curr::OperationBody::ClawbackClaimableBalance(clawback_claimable_balance) => {
                // TODO: same logic here, does the clawback only occur when the claimable balance is claimed?
            }
            stellar_xdr::curr::OperationBody::AllowTrust(allow_trust) => {
                // TODO: how can I fetch the amount here?
            }
            stellar_xdr::curr::OperationBody::SetTrustLineFlags(set_trust_line_flags) => {
                // TODO: how can I fetch the amount here?
            }
            stellar_xdr::curr::OperationBody::LiquidityPoolDeposit(liquidity_pool_deposit) => {
                // TODO: here to create the 2 transfers, I need to know the assets that are present in the pool?
                //  How can I fetch the asset of the pool at runtime?
                // TODO: if the from account happens to be the issuer of one of the assets being
                //  moved in the LP, then a Mint event will be emitted instead of a Transfer

                token_transfers.token_transfer_events.push(TokenTransfer {
                    meta: Some(helpers::event_meta::create_event_meta(
                        block.number,
                        block.created_at,
                        &hash,
                    )),
                    asset: None, // TODO: need to fetch the asset from the pool
                    event: Some(Event::Transfer(Transfer {
                        from: Some(helpers::address::create_account_address(
                            &trx.source_account.clone().account_id(),
                        )),
                        to: Some(helpers::address::create_account_address_from_pool_id(
                            &liquidity_pool_deposit.liquidity_pool_id,
                        )),
                        amount: BigDecimal::from(liquidity_pool_deposit.max_amount_a).into(), // TODO: also what value should I use here? The min or the max?
                    })),
                });
                token_transfers.token_transfer_events.push(TokenTransfer {
                    meta: Some(helpers::event_meta::create_event_meta(
                        block.number,
                        block.created_at,
                        &hash,
                    )),
                    asset: None, // TODO: need to fetch the asset from the pool
                    event: Some(Event::Transfer(Transfer {
                        from: Some(helpers::address::create_account_address(
                            &trx.source_account.clone().account_id(),
                        )),
                        to: Some(helpers::address::create_account_address_from_pool_id(
                            &liquidity_pool_deposit.liquidity_pool_id,
                        )),
                        amount: BigDecimal::from(liquidity_pool_deposit.max_amount_b).into(), // TODO: also what value should I use here? The min or the max?
                    })),
                });
            }
            stellar_xdr::curr::OperationBody::LiquidityPoolWithdraw(liquidity_pool_withdraw) => {
                // TODO: here to create the 2 transfers, I need to know the assets that are present in the pool?
                //  How can I fetch the asset of the pool at runtime?
                // TODO: If the to account happens to be the issuer of one of the assets being
                //  moved in the LP, then a Burn event will be emitted instead of a Transfer

                token_transfers.token_transfer_events.push(TokenTransfer {
                    meta: Some(helpers::event_meta::create_event_meta(
                        block.number,
                        block.created_at,
                        &hash,
                    )),
                    asset: None, // TODO: need to fetch the asset from the pool
                    event: Some(Event::Transfer(Transfer {
                        from: Some(helpers::address::create_account_address(
                            &trx.source_account.clone().account_id(),
                        )),
                        to: Some(helpers::address::create_account_address_from_pool_id(
                            &liquidity_pool_withdraw.liquidity_pool_id,
                        )),
                        amount: BigDecimal::from(liquidity_pool_withdraw.min_amount_a).into(),
                    })),
                });
                token_transfers.token_transfer_events.push(TokenTransfer {
                    meta: Some(helpers::event_meta::create_event_meta(
                        block.number,
                        block.created_at,
                        &hash,
                    )),
                    asset: None, // TODO: need to fetch the asset from the pool
                    event: Some(Event::Transfer(Transfer {
                        from: Some(helpers::address::create_account_address(
                            &trx.source_account.clone().account_id(),
                        )),
                        to: Some(helpers::address::create_account_address_from_pool_id(
                            &liquidity_pool_withdraw.liquidity_pool_id,
                        )),
                        amount: BigDecimal::from(liquidity_pool_withdraw.min_amount_b).into(),
                    })),
                });
            }
            stellar_xdr::curr::OperationBody::ManageBuyOffer(manage_buy_offer) => {
                // TODO: If the from account happens to be the issuer of one of the assets being
                //  traded, then a Mint event will be emitted instead of a Transfer
                // TODO: If the to account happens to be the issuer of one of the assets being
                //  traded, then a Burn event will be emitted instead of a Transfer

                // Also how we handle the from and to accounts?
                // How do we handle the price?
            }
            stellar_xdr::curr::OperationBody::ManageSellOffer(manage_sell_offer) => {
                // TODO: If the from account happens to be the issuer of one of the assets being
                //  traded, then a Mint event will be emitted instead of a Transfer
                // TODO: If the to account happens to be the issuer of one of the assets being
                //  traded, then a Burn event will be emitted instead of a Transfer

                // Also how we handle the from and to accounts?
                // How do we handle the price?
            }
            stellar_xdr::curr::OperationBody::CreatePassiveSellOffer(create_passive_sell) => {
                // TODO: If the from account happens to be the issuer of one of the assets being
                //  traded, then a Mint event will be emitted instead of a Transfer
                // TODO: If the to account happens to be the issuer of one of the assets being
                //  traded, then a Burn event will be emitted instead of a Transfer

                // Also how we handle the from and to accounts?
            }
            stellar_xdr::curr::OperationBody::PathPaymentStrictSend(path_payment_strict_send) => {
                let send_asset = utils::match_asset_code(&path_payment_strict_send.send_asset);
                let dest_asset = utils::match_asset_code(&path_payment_strict_send.dest_asset);
                if send_asset == dest_asset {
                    // This is considered as a normal transfer
                    token_transfers.token_transfer_events.push(TokenTransfer {
                        meta: Some(helpers::event_meta::create_event_meta(
                            block.number,
                            block.created_at,
                            &hash,
                        )),
                        asset: Some(send_asset),
                        event: Some(Event::Transfer(Transfer {
                            from: Some(helpers::address::create_account_address(
                                &trx.source_account.clone().account_id(),
                            )),
                            to: Some(helpers::address::create_account_address(
                                &path_payment_strict_send.clone().destination.account_id(),
                            )),
                            amount: BigDecimal::from(path_payment_strict_send.send_amount).into(),
                        })),
                    });
                }
                // TODO: If the from account happens to be the issuer of one of the assets being
                //  traded, then a Mint event will be emitted instead of a Transfer
                // TODO: If the to account happens to be the issuer of one of the assets being
                //  traded, then a Burn event will be emitted instead of a Transfer

                // Also how we handle the from and to accounts?
            }
            stellar_xdr::curr::OperationBody::PathPaymentStrictReceive(path_payment_strict_receive) => {
                let send_asset = utils::match_asset_code(&path_payment_strict_receive.send_asset);
                let dest_asset = utils::match_asset_code(&path_payment_strict_receive.dest_asset);
                if send_asset == dest_asset {
                    // This is considered as a normal transfer
                    token_transfers.token_transfer_events.push(TokenTransfer {
                        meta: Some(helpers::event_meta::create_event_meta(
                            block.number,
                            block.created_at,
                            &hash,
                        )),
                        asset: Some(send_asset),
                        event: Some(Event::Transfer(Transfer {
                            from: Some(helpers::address::create_account_address(
                                &trx.source_account.clone().account_id(),
                            )),
                            to: Some(helpers::address::create_account_address(
                                &path_payment_strict_receive.clone().destination.account_id(),
                            )),
                            amount: BigDecimal::from(path_payment_strict_receive.dest_amount).into(),
                        })),
                    });
                }
                // TODO: If the from account happens to be the issuer of one of the assets being
                //  traded, then a Mint event will be emitted instead of a Transfer
                // TODO: If the to account happens to be the issuer of one of the assets being
                //  traded, then a Burn event will be emitted instead of a Transfer

                // Also how we handle the from and to accounts?
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
