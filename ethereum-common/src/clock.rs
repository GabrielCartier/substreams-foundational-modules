use crate::pb::sf::substreams::v1::Clock;
use substreams::{errors::Error, Hex};
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
pub fn map_clock(blk: Block) -> Result<Clock, Error> {
    Ok(Clock {
        timestamp: Some(blk.header.unwrap().timestamp.unwrap()),
        id: Hex::encode(&blk.hash),
        number: blk.number,
    })
}
