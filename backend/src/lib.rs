mod types;

use std::{cell::RefCell, collections::HashSet};

use candid::{CandidType, Int, Nat, Principal};
use ic_cdk::{api, query, update};

use types::{
    CanisterError, CanisterMetaData, CanisterResult, CanisterState, CanisterStats, Certificate,
    CertificateData, CertificateId, CertificateMetaData, GenericValue,
};

thread_local! {
    static STATE: RefCell<CanisterState> = RefCell::new(CanisterState::default());
}

/// Guard that verifies the account calling the contract is a custodian of this canister
fn is_custodian_guard() -> Result<(), String> {
    let user = api::caller();
    // println!("Principal: {user}");
    STATE.with_borrow(|state| {
        state
            .metadata
            .custodians
            .contains(&user)
            .then_some(())
            .ok_or("Not authorized as custodian".into())
    })
}

#[query(name = "getCanisterMetadata")]
fn get_metadata() -> CanisterMetaData {
    STATE.with(|state| state.borrow().metadata.clone())
}

#[query(name = "getMyPrincipal")]
fn get_my_principal() -> Principal {
    api::caller()
}

#[query(name = "getCanisterStats")]
fn get_canister_stats() -> CanisterStats {
    STATE.with(|state| state.borrow().stats.clone())
}

#[query(name = "getCanisterLogo")]
fn get_canister_logo() -> Option<String> {
    STATE.with(|state| state.borrow().metadata.logo.clone())
}

#[update(name = "setCanisterLogo", guard = "is_custodian_guard")]
fn get_canister_name() -> Option<String> {
    STATE.with_borrow(|state| state.metadata.name.clone())
}

#[update(name = "setCanisterName", guard = "is_custodian_guard")]
fn set_canister_name(name: Option<String>) {
    STATE.with_borrow_mut(|state| {
        state.metadata.name = name;
    })
}

#[query(name = "getCanisterSymbol")]
fn get_canister_symbol() -> Option<String> {
    STATE.with_borrow(|state| state.metadata.name.clone())
}

#[update(name = "setCanisterSymbol", guard = "is_custodian_guard")]
fn set_canister_symbol(symbol: Option<String>) {
    STATE.with_borrow_mut(|state| {
        state.metadata.symbol = symbol;
    })
}

#[query(name = "getCanisterCustodians")]
fn get_canister_custodians() -> HashSet<Principal> {
    STATE.with_borrow(|state| state.metadata.custodians.clone())
}

#[update(name = "setCanisterCustodians", guard = "is_custodian_guard")]
fn set_canister_custodians(custodians: HashSet<Principal>) {
    STATE.with_borrow_mut(|state| {
        state.metadata.custodians = custodians;
    })
}

#[query(name = "getCanisterCycles")]
fn get_canister_cycles() -> Nat {
    STATE.with(|state| Nat::from(state.borrow().stats.cycles))
}

#[query(name = "totalUniqueHolders")]
fn get_total_unique_holders() -> Nat {
    STATE.with(|state| Nat::from(state.borrow().stats.total_unique_holders))
}

// ============= TOKEN HANDLERS ===============

#[query(name = "getTokenMetadata")]
fn get_token_metadata(token_id: CertificateId) -> CanisterResult<CertificateMetaData> {
    STATE.with(|state| {
        if let Some(token) = state.borrow().certificates.get(&token_id) {
            Ok(token.metadata.clone())
        } else {
            Err(CanisterError::TokenNotFound)
        }
    })
}

#[query(name = "balanceOf")]
fn get_user_token_count(user: Principal) -> CanisterResult<Nat> {
    STATE.with(|state| {
        if let Some(user) = state.borrow().owners.get(&user) {
            Ok(Nat::from(user.len()))
        } else {
            Err(CanisterError::OwnerNotFound)
        }
    })
}

#[query(name = "ownerOf")]
fn get_token_owner(token_id: u64) -> CanisterResult<Principal> {
    STATE.with(|state| {
        if let Some(nft) = state.borrow().certificates.get(&token_id) {
            Ok(nft.metadata.owner)
        } else {
            Err(CanisterError::TokenNotFound)
        }
    })
}

#[query(name = "ownerCertificateIds")]
fn get_tokens_by_owner(user: Principal) -> CanisterResult<HashSet<u64>> {
    STATE.with(|state| {
        if let Some(tokens) = state.borrow().owners.get(&user) {
            Ok(tokens.clone())
        } else {
            Err(CanisterError::OwnerNotFound)
        }
    })
}

#[query(name = "ownerTokenMetadata")]
fn get_tokens_metadata_by_owner(user: Principal) -> CanisterResult<Vec<CertificateMetaData>> {
    STATE.with(|state| {
        let state = state.borrow();
        if let Some(certficates) = state.owners.get(&user) {
            Ok(certficates
                .iter()
                .filter_map(|token_id| {
                    state
                        .certificates
                        .get(token_id)
                        .map(|nft| nft.metadata.clone())
                })
                .collect())
        } else {
            Err(CanisterError::OwnerNotFound)
        }
    })
}

#[query(name = "totalSupply")]
fn get_canister_supply() -> Nat {
    STATE.with_borrow(|state| Nat::from(state.stats.total_supply))
}

#[update(name = "mint")]
fn mint_token(
    user: Principal,
    username: String,
    certificate_id: Option<CertificateId>,
    certificate_data: CertificateData,
    properties: Option<Vec<(String, GenericValue)>>,
) -> CanisterResult<CertificateId> {
    STATE.with_borrow_mut(|state| {
        let certificate_id = certificate_id.unwrap_or(state.stats.total_supply + 1);
        if state.certificates.contains_key(&certificate_id) {
            return Err(CanisterError::CertificateAlreadyExists);
        }
        let token_metadata = CertificateMetaData::new(user, properties);

        let certificate = Certificate {
            metadata: token_metadata,
            data: certificate_data,
        };

        state.certificates.insert(certificate_id, certificate);

        if let Some(tokens) = state.owners.get_mut(&user) {
            tokens.insert(certificate_id);
        } else {
            state.owners.insert(user, HashSet::from([certificate_id]));
            state.stats.total_unique_holders += 1;
        }

        // Adds user's name that owns the token
        state.owner_names.entry(username).or_insert(user);

        state.stats.total_supply += 1;

        Ok(certificate_id)
    })
}

#[query(name = "getTokenData")]
fn get_token_data(token_id: CertificateId) -> CanisterResult<CertificateData> {
    STATE.with(|state| {
        if let Some(certificate) = state.borrow().certificates.get(&token_id) {
            Ok(certificate.data.clone())
        } else {
            Err(CanisterError::TokenNotFound)
        }
    })
}

#[query(name = "getUserByName")]
fn get_user_by_name(username: String) -> CanisterResult<Principal> {
    STATE.with_borrow(|state| {
        state
            .owner_names
            .get(&username)
            .ok_or(CanisterError::OwnerNotFound)
            .map(ToOwned::to_owned)
    })
}

#[update(name = "burnToken", guard = "is_custodian_guard")]
fn burn_token(token_id: CertificateId) -> CanisterResult<()> {
    STATE.with_borrow_mut(|state| {
        if let Some(certificate) = state.certificates.get_mut(&token_id) {
            certificate.metadata.is_burned = true;
            certificate.metadata.burned_at = Some(api::time());
            certificate.metadata.burned_by = Some(api::caller());
            certificate.metadata.owner = Principal::from_slice(&[]);
            state.stats.total_supply -= 1;
            Ok(())
        } else {
            Err(CanisterError::TokenNotFound)
        }
    })
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

ic_cdk::export_candid!();
