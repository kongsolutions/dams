use candid::CandidType;
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, CandidType, Deserialize)]
struct DIDDocument {
    id: String,
    verification_method: Vec<VerificationMethod>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct VerificationMethod {
    id: String,
    type_: String,
    public_key_base58: String,
}

type DIDDocumentRegistry = HashMap<String, DIDDocument>;

#[update]
fn store_did_document(did_document: DIDDocument) -> Result<(), String> {
    let mut registry = ic_cdk::storage::get_mut::<DIDDocumentRegistry>();
    if registry.contains_key(&did_document.id) {
        return Err("DID Document already exists".to_string());
    }
    registry.insert(did_document.id.clone(), did_document);
    Ok(())
}

#[query]
fn get_did_document(did: String) -> Result<DIDDocument, String> {
    let registry = ic_cdk::storage::get::<DIDDocumentRegistry>();
    registry
        .get(&did)
        .cloned()
        .ok_or("DID Document not found".to_string())
}
