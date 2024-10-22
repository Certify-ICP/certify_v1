mod types;

use std::{cell::RefCell, collections::HashSet, fmt::format};

use candid::{Nat, Principal};
use ic_cdk::{api, query, update};

use types::{
    CanisterError, CanisterMetaData, CanisterResult, CanisterState, CanisterStats, Certificate,
    CertificateId, CertificateInfo,
};

thread_local! {
    static STATE: RefCell<CanisterState> = RefCell::new(CanisterState::default());
}

/// Guard that verifies the account calling the contract is a custodian of this canister
fn is_custodian_guard() -> Result<(), String> {
    let user = api::caller();
    STATE.with_borrow(|state| {
        state
            .metadata
            .custodians
            .contains(&user)
            .then_some(())
            .ok_or("Not authorized as custodian".into())
    })
}

#[query(name = "get_canister_metadata")]
fn get_metadata() -> CanisterMetaData {
    STATE.with(|state| state.borrow().metadata.clone())
}

#[query(name = "get_my_principal")]
fn get_my_principal() -> Principal {
    api::caller()
}

#[query(name = "get_canister_stats")]
fn get_canister_stats() -> CanisterStats {
    STATE.with(|state| state.borrow().stats.clone())
}

#[query(name = "get_canister_logo")]
fn get_canister_logo() -> Option<String> {
    STATE.with(|state| state.borrow().metadata.logo.clone())
}

#[query(name = "get_canister_name")]
fn get_canister_name() -> String {
    STATE.with_borrow(|state| state.metadata.name.clone())
}

#[update(name = "set_canister_name", guard = "is_custodian_guard")]
fn set_canister_name(name: String) {
    STATE.with_borrow_mut(|state| {
        state.metadata.name = name;
    })
}

#[query(name = "get_canister_symbol")]
fn get_canister_symbol() -> Option<String> {
    STATE.with_borrow(|state| state.metadata.symbol.clone())
}

#[update(name = "set_canister_symbol", guard = "is_custodian_guard")]
fn set_canister_symbol(symbol: Option<String>) {
    STATE.with_borrow_mut(|state| {
        state.metadata.symbol = symbol;
    })
}

#[query(name = "get_canister_custodians")]
fn get_canister_custodians() -> HashSet<Principal> {
    STATE.with_borrow(|state| state.metadata.custodians.clone())
}

#[update(name = "set_canister_custodians", guard = "is_custodian_guard")]
fn set_canister_custodians(custodians: HashSet<Principal>) {
    STATE.with_borrow_mut(|state| {
        state.metadata.custodians = custodians;
    })
}

#[query(name = "get_canister_cycles")]
fn get_canister_cycles() -> Nat {
    STATE.with(|state| Nat::from(state.borrow().stats.cycles))
}

#[query(name = "total_unique_holders")]
fn get_total_unique_holders() -> Nat {
    STATE.with(|state| Nat::from(state.borrow().stats.total_unique_holders))
}

// ============= TOKEN HANDLERS ===============

#[query(name = "get_certificate")]
fn get_certificate(certificate_id: CertificateId) -> CanisterResult<Certificate> {
    STATE.with(|state| {
        if let Some(certificate) = state.borrow().certificates.get(&certificate_id) {
            Ok(certificate.clone())
        } else {
            Err(CanisterError::CertificateNotFound)
        }
    })
}

#[query(name = "get_certificate_info")]
fn get_certificate_info(certificate_id: CertificateId) -> CanisterResult<CertificateInfo> {
    STATE.with(|state| {
        let state = state.borrow();
        if let Some(certificate) = state.certificates.get(&certificate_id) {
            let issuer_name = state
                .issuers_principal_to_names
                .get(&certificate.issuer)
                .cloned()
                .unwrap();
            let holder_name = state
                .holders_principal_to_names
                .get(&certificate.holder)
                .cloned()
                .unwrap();

            Ok(CertificateInfo {
                id: certificate_id,
                title: certificate.title.clone(),
                description: certificate.description.clone(),
                issued_at: certificate.issued_at,
                issuer_name,
                issuer_address: certificate.issuer,
                holder_name,
                holder_address: certificate.holder,
            })
        } else {
            Err(CanisterError::CertificateNotFound)
        }
    })
}

#[query(name = "get_certificate_holder_principal")]
fn get_certificate_holder_principal(certificate_id: u64) -> CanisterResult<Principal> {
    STATE.with(|state| {
        if let Some(certificate) = state.borrow().certificates.get(&certificate_id) {
            Ok(certificate.holder)
        } else {
            Err(CanisterError::CertificateNotFound)
        }
    })
}

#[query(name = "get_certificate_holder_name")]
fn get_certificate_holder_name(certificate_id: u64) -> CanisterResult<String> {
    STATE.with(|state| {
        if let Some(certificate) = state.borrow().certificates.get(&certificate_id) {
            let holder_name = state
                .borrow()
                .holders_principal_to_names
                .get(&certificate.holder)
                .unwrap()
                .clone();

            Ok(holder_name)
        } else {
            Err(CanisterError::CertificateNotFound)
        }
    })
}

#[query(name = "get_certificate_issuer_principal")]
fn get_certificate_issuer_principal(certificate_id: u64) -> CanisterResult<Principal> {
    STATE.with(|state| {
        if let Some(certificate) = state.borrow().certificates.get(&certificate_id) {
            Ok(certificate.issuer)
        } else {
            Err(CanisterError::CertificateNotFound)
        }
    })
}

#[query(name = "get_certificate_issuer_name")]
fn get_certificate_issuer_name(certificate_id: u64) -> CanisterResult<String> {
    STATE.with(|state| {
        if let Some(certificate) = state.borrow().certificates.get(&certificate_id) {
            let issuer_name = state
                .borrow()
                .issuers_principal_to_names
                .get(&certificate.issuer)
                .unwrap()
                .clone();

            Ok(issuer_name)
        } else {
            Err(CanisterError::CertificateNotFound)
        }
    })
}

#[query(name = "get_holder_certificate_count")]
fn get_holder_certificate_count(user: Principal) -> CanisterResult<Nat> {
    STATE.with(|state| {
        if let Some(holder_certificates) =
            state.borrow().holders_principal_to_certificates.get(&user)
        {
            Ok(Nat::from(holder_certificates.len()))
        } else {
            Err(CanisterError::HolderNotFound)
        }
    })
}

#[query(name = "get_issuer_certificate_count")]
fn get_issuer_certificate_count(user: Principal) -> CanisterResult<Nat> {
    STATE.with(|state| {
        if let Some(certificates) = state.borrow().issuers_principal_to_certificates.get(&user) {
            Ok(Nat::from(certificates.len()))
        } else {
            Err(CanisterError::HolderNotFound)
        }
    })
}

#[query(name = "get_certificates_by_holder")]
fn get_certificates_by_holder(user: Principal) -> CanisterResult<Vec<Certificate>> {
    STATE.with(|state| {
        let state = state.borrow();
        if let Some(certficates) = state.holders_principal_to_certificates.get(&user) {
            Ok(certficates
                .iter()
                .filter_map(|certificate_id| {
                    state
                        .certificates
                        .get(certificate_id)
                        .map(|certificate| certificate.clone())
                })
                .collect())
        } else {
            Err(CanisterError::HolderNotFound)
        }
    })
}

#[query(name = "get_certificates_info_by_holder_name")]
fn get_certificates_info_by_holder_name(
    holder_name: String,
) -> CanisterResult<Vec<CertificateInfo>> {
    STATE.with(|state| {
        let state = state.borrow();

        let holder_principal = state
            .holders_names_to_principal
            .get(&holder_name)
            .ok_or(CanisterError::HolderNotFound)?;

        if let Some(certficate_ids) = state
            .holders_principal_to_certificates
            .get(&holder_principal)
        {
            let mut certificates_display_info = Vec::new();

            for certificate_id in certficate_ids.iter() {
                certificates_display_info.push(get_certificate_info(*certificate_id)?);
            }

            Ok(certificates_display_info)
        } else {
            Err(CanisterError::HolderNotFound)
        }
    })
}

#[query(name = "get_certificates_info_by_issuer_name")]
fn get_certificates_info_by_issuer_name(
    issuer_name: String,
) -> CanisterResult<Vec<CertificateInfo>> {
    STATE.with(|state| {
        let state = state.borrow();

        let issuer_principal = state
            .issuers_names_to_principal
            .get(&issuer_name)
            .ok_or(CanisterError::HolderNotFound)?;

        if let Some(certficate_ids) = state
            .issuers_principal_to_certificates
            .get(&issuer_principal)
        {
            let mut certificates_display_info = Vec::new();

            for certificate_id in certficate_ids.iter() {
                certificates_display_info.push(get_certificate_info(*certificate_id)?);
            }

            Ok(certificates_display_info)
        } else {
            Err(CanisterError::HolderNotFound)
        }
    })
}

#[query(name = "get_certificates_by_issuer")]
fn get_certificates_by_issuer(user: Principal) -> CanisterResult<Vec<Certificate>> {
    STATE.with(|state| {
        let state = state.borrow();
        if let Some(certficates) = state.issuers_principal_to_certificates.get(&user) {
            Ok(certficates
                .iter()
                .filter_map(|certificate_id| {
                    state
                        .certificates
                        .get(certificate_id)
                        .map(|certificate| certificate.clone())
                })
                .collect())
        } else {
            Err(CanisterError::HolderNotFound)
        }
    })
}

#[update(name = "mint_certificate")]
fn mint_certificate(
    title: String,
    description: String,
    holder_principal: Principal,
    holder_name: String,
    issuer_principal: Principal,
    issuer_name: String,
    blob: Vec<u8>,
) -> CanisterResult<CertificateId> {
    STATE.with_borrow_mut(|state| {
        let certificate_id = state.stats.total_supply + 1;

        if state.certificates.contains_key(&certificate_id) {
            return Err(CanisterError::CertificateAlreadyExists);
        }
        // if api::caller() != issuer {
        //     return Err(CanisterError::NotAuthorizedAsIssuer);
        // }

        let certificate =
            Certificate::new(title, description, holder_principal, issuer_principal, blob);

        state.certificates.insert(certificate_id, certificate);

        // Add the certificate under the holder
        if let Some(certificates) = state
            .holders_principal_to_certificates
            .get_mut(&holder_principal)
        {
            certificates.insert(certificate_id);
        } else {
            state
                .holders_principal_to_certificates
                .insert(holder_principal, HashSet::from([certificate_id]));
            state.stats.total_unique_holders += 1;
        }

        // Add the certificate under the issuer
        if let Some(certificates) = state
            .issuers_principal_to_certificates
            .get_mut(&holder_principal)
        {
            certificates.insert(certificate_id);
        } else {
            state
                .issuers_principal_to_certificates
                .insert(issuer_principal, HashSet::from([certificate_id]));
        }

        // Adds name of the user that owns the token
        if let None = state.holders_names_to_principal.get(&holder_name) {
            state
                .holders_names_to_principal
                .insert(holder_name.clone(), holder_principal);

            state
                .holders_principal_to_names
                .insert(holder_principal, holder_name);
        }
        if let None = state.issuers_names_to_principal.get(&issuer_name) {
            state
                .issuers_names_to_principal
                .insert(issuer_name.clone(), issuer_principal);

            state
                .issuers_principal_to_names
                .insert(issuer_principal, issuer_name);
        }

        state.stats.total_supply += 1;

        Ok(certificate_id)
    })
}

#[query(name = "greet")]
fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

ic_cdk::export_candid!();
