use crate::pb::protocol;
use crate::pb::sf::tron::r#type::v1::ResponseCode;
use bs58;
use prost::Message;
use prost_types::Any;
use sha2::{Digest, Sha256};
use std::convert::TryFrom;

pub fn transaction_failed(status: i32) -> bool {
    status != ResponseCode::Success as i32
}

/// Macro to reduce repetition when extracting owner_address from contract parameters.
macro_rules! extract_owner {
    ($struct_type:ty, $parameter:expr, $field:ident) => {
        <$struct_type>::decode(&$parameter.value[..])
            .ok()
            .map(|c| c.$field)
    };
}

/// Extracts the 'from' (owner) address from a contract parameter, if available.
/// Returns None if the contract type does not have an owner address.
pub fn extract_from_address(contract_type: i32, parameter: &Any) -> Option<Vec<u8>> {
    use protocol::transaction::contract::ContractType;
    match ContractType::try_from(contract_type).ok() {
        Some(ContractType::TransferContract) => {
            extract_owner!(protocol::TransferContract, parameter, owner_address)
        }
        Some(ContractType::TransferAssetContract) => {
            extract_owner!(protocol::TransferAssetContract, parameter, owner_address)
        }
        Some(ContractType::VoteAssetContract) => {
            extract_owner!(protocol::VoteAssetContract, parameter, owner_address)
        }
        Some(ContractType::VoteWitnessContract) => {
            extract_owner!(protocol::VoteWitnessContract, parameter, owner_address)
        }
        Some(ContractType::WitnessCreateContract) => {
            extract_owner!(protocol::WitnessCreateContract, parameter, owner_address)
        }
        Some(ContractType::AssetIssueContract) => {
            extract_owner!(protocol::AssetIssueContract, parameter, owner_address)
        }
        Some(ContractType::WitnessUpdateContract) => {
            extract_owner!(protocol::WitnessUpdateContract, parameter, owner_address)
        }
        Some(ContractType::ParticipateAssetIssueContract) => extract_owner!(
            protocol::ParticipateAssetIssueContract,
            parameter,
            owner_address
        ),
        Some(ContractType::AccountUpdateContract) => {
            extract_owner!(protocol::AccountUpdateContract, parameter, owner_address)
        }
        Some(ContractType::FreezeBalanceContract) => {
            extract_owner!(protocol::FreezeBalanceContract, parameter, owner_address)
        }
        Some(ContractType::UnfreezeBalanceContract) => {
            extract_owner!(protocol::UnfreezeBalanceContract, parameter, owner_address)
        }
        Some(ContractType::WithdrawBalanceContract) => {
            extract_owner!(protocol::WithdrawBalanceContract, parameter, owner_address)
        }
        Some(ContractType::UnfreezeAssetContract) => {
            extract_owner!(protocol::UnfreezeAssetContract, parameter, owner_address)
        }
        Some(ContractType::UpdateAssetContract) => {
            extract_owner!(protocol::UpdateAssetContract, parameter, owner_address)
        }
        Some(ContractType::ProposalCreateContract) => {
            extract_owner!(protocol::ProposalCreateContract, parameter, owner_address)
        }
        Some(ContractType::ProposalApproveContract) => {
            extract_owner!(protocol::ProposalApproveContract, parameter, owner_address)
        }
        Some(ContractType::ProposalDeleteContract) => {
            extract_owner!(protocol::ProposalDeleteContract, parameter, owner_address)
        }
        Some(ContractType::SetAccountIdContract) => {
            extract_owner!(protocol::SetAccountIdContract, parameter, owner_address)
        }
        Some(ContractType::CreateSmartContract) => {
            extract_owner!(protocol::CreateSmartContract, parameter, owner_address)
        }
        Some(ContractType::TriggerSmartContract) => {
            extract_owner!(protocol::TriggerSmartContract, parameter, owner_address)
        }
        Some(ContractType::UpdateSettingContract) => {
            extract_owner!(protocol::UpdateSettingContract, parameter, owner_address)
        }
        Some(ContractType::ExchangeCreateContract) => {
            extract_owner!(protocol::ExchangeCreateContract, parameter, owner_address)
        }
        Some(ContractType::ExchangeInjectContract) => {
            extract_owner!(protocol::ExchangeInjectContract, parameter, owner_address)
        }
        Some(ContractType::ExchangeWithdrawContract) => {
            extract_owner!(protocol::ExchangeWithdrawContract, parameter, owner_address)
        }
        Some(ContractType::ExchangeTransactionContract) => extract_owner!(
            protocol::ExchangeTransactionContract,
            parameter,
            owner_address
        ),
        Some(ContractType::UpdateEnergyLimitContract) => extract_owner!(
            protocol::UpdateEnergyLimitContract,
            parameter,
            owner_address
        ),
        Some(ContractType::AccountPermissionUpdateContract) => extract_owner!(
            protocol::AccountPermissionUpdateContract,
            parameter,
            owner_address
        ),
        Some(ContractType::ClearAbiContract) => {
            extract_owner!(protocol::ClearAbiContract, parameter, owner_address)
        }
        Some(ContractType::UpdateBrokerageContract) => {
            extract_owner!(protocol::UpdateBrokerageContract, parameter, owner_address)
        }
        // For ShieldedTransferContract, the owner address is in the transparent_from_address field
        Some(ContractType::ShieldedTransferContract) => extract_owner!(
            protocol::ShieldedTransferContract,
            parameter,
            transparent_from_address
        ),
        Some(ContractType::MarketSellAssetContract) => {
            extract_owner!(protocol::MarketSellAssetContract, parameter, owner_address)
        }
        Some(ContractType::MarketCancelOrderContract) => extract_owner!(
            protocol::MarketCancelOrderContract,
            parameter,
            owner_address
        ),
        Some(ContractType::FreezeBalanceV2Contract) => {
            extract_owner!(protocol::FreezeBalanceV2Contract, parameter, owner_address)
        }
        Some(ContractType::UnfreezeBalanceV2Contract) => extract_owner!(
            protocol::UnfreezeBalanceV2Contract,
            parameter,
            owner_address
        ),
        Some(ContractType::WithdrawExpireUnfreezeContract) => extract_owner!(
            protocol::WithdrawExpireUnfreezeContract,
            parameter,
            owner_address
        ),
        Some(ContractType::DelegateResourceContract) => {
            extract_owner!(protocol::DelegateResourceContract, parameter, owner_address)
        }
        Some(ContractType::UnDelegateResourceContract) => extract_owner!(
            protocol::UnDelegateResourceContract,
            parameter,
            owner_address
        ),
        Some(ContractType::CancelAllUnfreezeV2Contract) => extract_owner!(
            protocol::CancelAllUnfreezeV2Contract,
            parameter,
            owner_address
        ),
        Some(ContractType::AccountCreateContract) => {
            extract_owner!(protocol::AccountCreateContract, parameter, owner_address)
        }
        // TODO: Validate this
        Some(ContractType::CustomContract) => None,
        Some(ContractType::GetContract) => None,
        None => None,
    }
}

/// Converts Tron address bytes to a Base58Check-encoded string (with checksum).
/// The address bytes must already include the Tron prefix (0x41).
pub fn tron_address_to_base58(address: &[u8]) -> String {
    let hash1 = Sha256::digest(address);
    let hash2 = Sha256::digest(&hash1);
    let checksum = &hash2[0..4];
    let mut payload = address.to_vec();
    payload.extend_from_slice(checksum);
    bs58::encode(payload).into_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pb::protocol;
    use crate::pb::protocol::transaction::contract::ContractType;
    use prost_types::Any;

    // Helper macro to automate test generation for contract types with owner_address
    macro_rules! test_owner_extract {
        ($name:ident, $contract_type:expr, $struct_type:ty, $field:ident) => {
            #[test]
            fn $name() {
                let owner = vec![1, 2, 3, 4];
                let mut contract = <$struct_type>::default();
                contract.$field = owner.clone();
                let any = Any {
                    type_url: format!("type.googleapis.com/{}", stringify!($struct_type)),
                    value: contract.encode_to_vec(),
                };
                let result = extract_from_address($contract_type as i32, &any);
                assert_eq!(result, Some(owner));
            }
        };
    }

    test_owner_extract!(
        extract_transfer_contract,
        ContractType::TransferContract,
        protocol::TransferContract,
        owner_address
    );
    test_owner_extract!(
        extract_transfer_asset_contract,
        ContractType::TransferAssetContract,
        protocol::TransferAssetContract,
        owner_address
    );
    test_owner_extract!(
        extract_vote_asset_contract,
        ContractType::VoteAssetContract,
        protocol::VoteAssetContract,
        owner_address
    );
    test_owner_extract!(
        extract_vote_witness_contract,
        ContractType::VoteWitnessContract,
        protocol::VoteWitnessContract,
        owner_address
    );
    test_owner_extract!(
        extract_witness_create_contract,
        ContractType::WitnessCreateContract,
        protocol::WitnessCreateContract,
        owner_address
    );
    test_owner_extract!(
        extract_asset_issue_contract,
        ContractType::AssetIssueContract,
        protocol::AssetIssueContract,
        owner_address
    );
    test_owner_extract!(
        extract_witness_update_contract,
        ContractType::WitnessUpdateContract,
        protocol::WitnessUpdateContract,
        owner_address
    );
    test_owner_extract!(
        extract_participate_asset_issue_contract,
        ContractType::ParticipateAssetIssueContract,
        protocol::ParticipateAssetIssueContract,
        owner_address
    );
    test_owner_extract!(
        extract_account_update_contract,
        ContractType::AccountUpdateContract,
        protocol::AccountUpdateContract,
        owner_address
    );
    test_owner_extract!(
        extract_freeze_balance_contract,
        ContractType::FreezeBalanceContract,
        protocol::FreezeBalanceContract,
        owner_address
    );
    test_owner_extract!(
        extract_unfreeze_balance_contract,
        ContractType::UnfreezeBalanceContract,
        protocol::UnfreezeBalanceContract,
        owner_address
    );
    test_owner_extract!(
        extract_withdraw_balance_contract,
        ContractType::WithdrawBalanceContract,
        protocol::WithdrawBalanceContract,
        owner_address
    );
    test_owner_extract!(
        extract_unfreeze_asset_contract,
        ContractType::UnfreezeAssetContract,
        protocol::UnfreezeAssetContract,
        owner_address
    );
    test_owner_extract!(
        extract_update_asset_contract,
        ContractType::UpdateAssetContract,
        protocol::UpdateAssetContract,
        owner_address
    );
    test_owner_extract!(
        extract_proposal_create_contract,
        ContractType::ProposalCreateContract,
        protocol::ProposalCreateContract,
        owner_address
    );
    test_owner_extract!(
        extract_proposal_approve_contract,
        ContractType::ProposalApproveContract,
        protocol::ProposalApproveContract,
        owner_address
    );
    test_owner_extract!(
        extract_proposal_delete_contract,
        ContractType::ProposalDeleteContract,
        protocol::ProposalDeleteContract,
        owner_address
    );
    test_owner_extract!(
        extract_set_account_id_contract,
        ContractType::SetAccountIdContract,
        protocol::SetAccountIdContract,
        owner_address
    );
    test_owner_extract!(
        extract_create_smart_contract,
        ContractType::CreateSmartContract,
        protocol::CreateSmartContract,
        owner_address
    );
    test_owner_extract!(
        extract_trigger_smart_contract,
        ContractType::TriggerSmartContract,
        protocol::TriggerSmartContract,
        owner_address
    );
    test_owner_extract!(
        extract_update_setting_contract,
        ContractType::UpdateSettingContract,
        protocol::UpdateSettingContract,
        owner_address
    );
    test_owner_extract!(
        extract_exchange_create_contract,
        ContractType::ExchangeCreateContract,
        protocol::ExchangeCreateContract,
        owner_address
    );
    test_owner_extract!(
        extract_exchange_inject_contract,
        ContractType::ExchangeInjectContract,
        protocol::ExchangeInjectContract,
        owner_address
    );
    test_owner_extract!(
        extract_exchange_withdraw_contract,
        ContractType::ExchangeWithdrawContract,
        protocol::ExchangeWithdrawContract,
        owner_address
    );
    test_owner_extract!(
        extract_exchange_transaction_contract,
        ContractType::ExchangeTransactionContract,
        protocol::ExchangeTransactionContract,
        owner_address
    );
    test_owner_extract!(
        extract_update_energy_limit_contract,
        ContractType::UpdateEnergyLimitContract,
        protocol::UpdateEnergyLimitContract,
        owner_address
    );
    test_owner_extract!(
        extract_account_permission_update_contract,
        ContractType::AccountPermissionUpdateContract,
        protocol::AccountPermissionUpdateContract,
        owner_address
    );
    test_owner_extract!(
        extract_clear_abi_contract,
        ContractType::ClearAbiContract,
        protocol::ClearAbiContract,
        owner_address
    );
    test_owner_extract!(
        extract_update_brokerage_contract,
        ContractType::UpdateBrokerageContract,
        protocol::UpdateBrokerageContract,
        owner_address
    );
    test_owner_extract!(
        extract_market_sell_asset_contract,
        ContractType::MarketSellAssetContract,
        protocol::MarketSellAssetContract,
        owner_address
    );
    test_owner_extract!(
        extract_market_cancel_order_contract,
        ContractType::MarketCancelOrderContract,
        protocol::MarketCancelOrderContract,
        owner_address
    );
    test_owner_extract!(
        extract_freeze_balance_v2_contract,
        ContractType::FreezeBalanceV2Contract,
        protocol::FreezeBalanceV2Contract,
        owner_address
    );
    test_owner_extract!(
        extract_unfreeze_balance_v2_contract,
        ContractType::UnfreezeBalanceV2Contract,
        protocol::UnfreezeBalanceV2Contract,
        owner_address
    );
    test_owner_extract!(
        extract_withdraw_expire_unfreeze_contract,
        ContractType::WithdrawExpireUnfreezeContract,
        protocol::WithdrawExpireUnfreezeContract,
        owner_address
    );
    test_owner_extract!(
        extract_delegate_resource_contract,
        ContractType::DelegateResourceContract,
        protocol::DelegateResourceContract,
        owner_address
    );
    test_owner_extract!(
        extract_undelegate_resource_contract,
        ContractType::UnDelegateResourceContract,
        protocol::UnDelegateResourceContract,
        owner_address
    );
    test_owner_extract!(
        extract_cancel_all_unfreeze_v2_contract,
        ContractType::CancelAllUnfreezeV2Contract,
        protocol::CancelAllUnfreezeV2Contract,
        owner_address
    );
    test_owner_extract!(
        extract_account_create_contract,
        ContractType::AccountCreateContract,
        protocol::AccountCreateContract,
        owner_address
    );

    #[test]
    fn extract_shielded_transfer_contract() {
        let owner = vec![7, 7, 7, 7];
        let mut contract = protocol::ShieldedTransferContract::default();
        contract.transparent_from_address = owner.clone();
        let any = Any {
            type_url: "type.googleapis.com/protocol.ShieldedTransferContract".to_string(),
            value: contract.encode_to_vec(),
        };
        let result = extract_from_address(ContractType::ShieldedTransferContract as i32, &any);
        assert_eq!(result, Some(owner));
    }

    #[test]
    fn extract_custom_contract_none() {
        let any = Any {
            type_url: "type.googleapis.com/protocol.CustomContract".to_string(),
            value: vec![],
        };
        let result = extract_from_address(ContractType::CustomContract as i32, &any);
        assert_eq!(result, None);
    }

    #[test]
    fn extract_get_contract_none() {
        let any = Any {
            type_url: "type.googleapis.com/protocol.GetContract".to_string(),
            value: vec![],
        };
        let result = extract_from_address(ContractType::GetContract as i32, &any);
        assert_eq!(result, None);
    }

    #[test]
    fn test_tron_address_to_base58() {
        // Example: QVeYR7tyCAZ4qm6OjPuRnxjT105C (base64) => THxNDMy3y9NP7Bfat9CTKmZ4PFfj1v4gWa
        let base64 = "QVeYR7tyCAZ4qm6OjPuRnxjT105C";
        let expected = "THxNDMy3y9NP7Bfat9CTKmZ4PFfj1v4gWa";
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(base64)
            .unwrap();
        let addr = tron_address_to_base58(&bytes);
        assert_eq!(addr, expected);
    }
}
