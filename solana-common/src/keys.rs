use substreams_solana::{base58, pb::sf::solana::r#type::v1::ConfirmedTransaction};

/// transaction_program_and_account_keys returns an iterator of keys extracted from a transaction. It will
/// emit the account keys from the transaction message, the loaded writable addresses, the loaded readonly
/// addresses, and the program ids from the instructions.
pub(crate) fn transaction_program_and_account_keys(
    trx: &ConfirmedTransaction,
) -> impl Iterator<Item = String> + '_ {
    let meta = trx.meta.as_ref().unwrap();
    let message = trx.transaction.as_ref().unwrap().message.as_ref().unwrap();

    message
        .account_keys
        .iter()
        .chain(meta.loaded_writable_addresses.iter())
        .chain(meta.loaded_readonly_addresses.iter())
        .map(|acct| format!("account:{}", base58::encode(acct)))
        .chain(
            trx.walk_instructions()
                .map(|inst| format!("program:{}", inst.program_id())),
        )
}
