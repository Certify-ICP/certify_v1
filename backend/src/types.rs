use std::collections::{HashMap, HashSet};

use candid::{CandidType, Int, Nat, Principal};
use ic_cdk::api;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Default)]
pub struct CanisterState {
    pub metadata: CanisterMetaData,
    pub stats: CanisterStats,

    pub certificates: HashMap<CertificateId, Certificate>,

    pub issuers_principal_to_certificates: HashMap<Principal, HashSet<CertificateId>>,
    pub issuers_principal_to_names: HashMap<Principal, String>,
    pub issuers_names_to_principal: HashMap<String, Principal>,

    pub holders_principal_to_certificates: HashMap<Principal, HashSet<CertificateId>>,
    pub holders_principal_to_names: HashMap<Principal, String>,
    pub holders_names_to_principal: HashMap<String, Principal>,
}

/// Metadata for ICP NFT standard.
/// 
#[derive(CandidType, Deserialize, Clone)]
pub struct CanisterMetaData {
    pub name: String,
    pub symbol: Option<String>,
    pub logo: Option<String>,
    pub created_at: u64,
    pub upgraded_at: u64,
    /// Admin accounts associated with the canister
    pub custodians: HashSet<Principal>,
}

impl Default for CanisterMetaData {
    fn default() -> Self {
        CanisterMetaData {
            name: "certify_v1".to_string(),
            symbol: None,
            logo: None,
            created_at: api::time(),
            upgraded_at: api::time(),
            custodians: HashSet::default(),
        }
    }
}

pub type CertificateId = u64;

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct Certificate {
    pub title: String,
    pub description: String,
    pub blob: Vec<u8>,
    pub holder: Principal,
    pub issuer: Principal,
    pub issued_at: u64,
}

// Helper type for fetching information for displaying the certificate.
//
#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct CertificateInfo {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub issuer_name: String,
    pub issuer_address: Principal,
    pub holder_name: String,
    pub holder_address: Principal,
    pub issued_at: u64,
}

impl Certificate {
    pub fn new(
        title: String,
        description: String,
        holder: Principal,
        issuer: Principal,
        blob: Vec<u8>,
    ) -> Self {
        Self {
            title,
            description,
            blob,
            holder,
            issuer,
            issued_at: api::time(),
        }
    }
}

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct CanisterStats {
    pub total_supply: u64,
    pub total_transactions: u64,
    pub total_unique_holders: u64,
    pub cycles: u64,
}

// =================== Errors ============================

pub type CanisterResult<T> = Result<T, CanisterError>;

#[derive(CandidType, Serialize)]
pub enum CanisterError {
    AttemptedSelfTransfer,
    CertificateNotFound,
    TransactionNotFound,
    NotAuthorizedAsCustodian,
    NotAuthorizedAsHolder,
    NotAuthorizedAsIssuer,
    NotAuthorizedAsOperator,
    CertificateAlreadyExists,
    HolderNotFound,
    Other(String),
}

// =================== Utilities =========================

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub enum GenericValue {
    BoolContent(bool),
    TextContent(String),
    BlobContent(Vec<u8>),
    Principal(Principal),
    Nat8Content(u8),
    Nat16Content(u16),
    Nat32Content(u32),
    Nat64Content(u64),
    NatContent(Nat),
    Int8Content(i8),
    Int16Content(i16),
    Int32Content(i32),
    Int64Content(i64),
    IntContent(Int),
    FloatContent(f64),
    NestedContent(Vec<(String, GenericValue)>),
}
