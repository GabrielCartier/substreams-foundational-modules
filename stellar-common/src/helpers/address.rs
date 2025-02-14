use stellar_xdr::curr::AccountId;
use substreams::Hex;

use crate::pb::sf::substreams::stellar::r#type::v1::{
    address, token_transfer::Event, Address, EventMeta, TokenTransfer, TokenTransfers, Transfer,
};

pub fn create_account_address(addr: &AccountId) -> Address {
    Address {
        address_type: Some(address::AddressType::AccountAddress(addr.to_string())),
    }
}
