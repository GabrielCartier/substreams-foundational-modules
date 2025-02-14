use prost_types::Timestamp;

use crate::pb::sf::substreams::stellar::r#type::v1::EventMeta;

pub fn create_event_meta(ledger_sequence: u64, closed_at: Option<Timestamp>, trx_hash: &String) -> EventMeta {
    EventMeta {
        ledger_sequence: ledger_sequence,
        closed_at,
        trx_hash: trx_hash.to_string(),
    }
}
