use stellar_xdr::curr::AccountId;
use substreams::scalar::BigDecimal;

use crate::helpers::address;
use crate::pb::sf::substreams::stellar::r#type::v1::token_transfer::Event;
use crate::pb::sf::substreams::stellar::r#type::v1::{Address, Burn, Mint, Transfer};

pub fn create_transfer_event(from: AccountId, to: AccountId, amount: i64) -> Event {
    Event::Transfer(Transfer {
        from: Some(address::create_account_address(&from)),
        to: Some(address::create_account_address(&to)),
        amount: BigDecimal::from(amount).into(),
    })
}

pub fn create_mint_event(to: AccountId, amount: i64) -> Event {
    Event::Mint(Mint {
        to: Some(address::create_account_address(&to)),
        amount: BigDecimal::from(amount).into(),
    })
}

pub fn create_burn_event(from: AccountId, amount: i64) -> Event {
    Event::Burn(Burn {
        from: Some(address::create_account_address(&from)),
        amount: BigDecimal::from(amount).into(),
    })
}
