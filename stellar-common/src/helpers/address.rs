use stellar_xdr::curr::{AccountId, PoolId};
use substreams::Hex;

use crate::pb::sf::substreams::stellar::r#type::v1::{
    address, token_transfer::Event, Address, EventMeta, TokenTransfer, TokenTransfers, Transfer,
};

pub fn create_account_address(addr: &AccountId) -> Address {
    Address {
        address_type: Some(address::AddressType::AccountAddress(addr.to_string())),
    }
}

pub fn create_account_address_from_pool_id(pool_id: &PoolId) -> Address {
    Address {
        address_type: Some(address::AddressType::AccountAddress(pool_id.0.to_string())),
    }
}
