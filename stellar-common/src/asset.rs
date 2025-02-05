use crate::{
    pb::sf::{
        stellar::r#type::v1::Block,
        substreams::stellar::r#type::v1::{Asset, Assets},
    },
    utils,
};
use substreams::Hex;

#[substreams::handlers::map]
fn map_issue_asset(block: Block) -> Result<Assets, substreams::errors::Error> {
    let mut assets = Assets::default();

    block.transactions.iter().for_each(|transaction| {
        if utils::transaction_failed(transaction.status) {
            return;
        }
        let hash = Hex(&transaction.hash).to_string();
        let decoded_transaction =
            match utils::decode_transaction(&transaction.result_xdr, &transaction.envelope_xdr) {
                Ok(trx) => trx,
                Err(_) => return,
            };

        decoded_transaction
            .operations
            .iter()
            .for_each(|operation| match &operation.body {
                // We listen on the ChangeTrust operation. This will tell us if a new asset was created
                // or if an asset was updated (the trustline was updated).
                // The sink running this piece of code, should ignore any asset that have already been created.
                stellar_xdr::curr::OperationBody::ChangeTrust(change_trust_op) => {
                    let asset_code = utils::match_change_trust_op_asset(&change_trust_op.line);
                    assets.assets.push(Asset {
                        trx_hash: hash.clone(),
                        code: asset_code,
                        issuer: utils::fetch_change_trust_op_asset_issuer(&change_trust_op.line),
                    });
                }
                _ => {}
            });
    });

    Ok(assets)
}
