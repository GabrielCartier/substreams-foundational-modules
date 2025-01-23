use substreams::pb::sf::substreams::index::v1::Keys;
use crate::pb::sf::substreams::stellar::r#type::v1::Payments;

#[substreams::handlers::map]
fn index_payments(payments: Payments) -> Result<Keys, substreams::errors::Error> {
    let keys: Vec<String> = payments
        .payments
        .into_iter()
        .flat_map(|payment| {
          vec![
              format!("account:{}", payment.source),
              format!("account:{}", payment.destination),
          ]
        })
        .collect();

    Ok(Keys { keys })
}