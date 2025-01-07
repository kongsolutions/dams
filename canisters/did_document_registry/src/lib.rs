use candid::Principal;
use candid::CandidType;
use ic_cdk::api::call::call;
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ic_cdk::storage::{stable_restore, stable_save};

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
struct ShardInfo {
    canister_id: String,
    document_count: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
struct DIDDocument {
    id: String,
    verification_method: Vec<VerificationMethod>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
struct VerificationMethod {
    id: String,
    type_: String,
    public_key_base58: String,
}

type ShardRegistry = Vec<ShardInfo>;
type DIDToShardMap = HashMap<String, String>;

#[init]
fn init() {
    stable_save((Vec::<ShardInfo>::new(), HashMap::<String, String>::new())).unwrap();
}

#[update]
async fn register_did(did: String, did_document: DIDDocument) -> Result<String, String> {
    let (mut registry, mut did_to_shard): (ShardRegistry, DIDToShardMap) =
        stable_restore().unwrap_or((Vec::new(), HashMap::new()));

    let shard = match registry
        .iter_mut()
        .find(|shard| shard.document_count < 800_000)
    {
        Some(shard) => shard,
        None => {
            let new_shard_id = create_new_shard().await?;
            registry.push(ShardInfo {
                canister_id: new_shard_id.clone(),
                document_count: 0,
            });
            registry.last_mut().unwrap()
        }
    };

    let shard_id = shard.canister_id.clone();
    let principal = Principal::from_text(shard_id.clone())
        .map_err(|e| format!("Invalid shard_id format: {:?}", e))?;

    let store_response: Result<(), _> = call(principal, "store_did_document", (did_document,)).await;
    store_response
        .map_err(|e| format!("Failed to store DID document: {:?}", e))?;

    shard.document_count += 1;
    did_to_shard.insert(did.clone(), shard_id);

    stable_save((registry, did_to_shard)).unwrap();

    Ok("DID Document stored successfully".to_string())
}

#[query]
async fn get_did_document(did: String) -> Result<DIDDocument, String> {
    let (_registry, did_to_shard): (ShardRegistry, DIDToShardMap) =
        stable_restore().unwrap_or_default();

    let shard_id = did_to_shard
        .get(&did)
        .ok_or_else(|| "DID not found".to_string())?;

    let principal = Principal::from_text(shard_id.clone())
        .map_err(|e| format!("Invalid shard_id format: {:?}", e))?;

    let result: Result<(DIDDocument,), _> = call(principal, "get_did_document", (did,)).await;
    match result {
        Ok((doc,)) => Ok(doc),
        Err(e) => Err(format!("Failed to fetch DID document: {:?}", e)),
    }
}

#[derive(CandidType, Deserialize, Serialize)]
struct CreateCanisterArgs {
    settings: Option<CanisterSettings>,
}

#[derive(CandidType, Deserialize, Serialize)]
struct CanisterSettings {
    controllers: Option<Vec<Principal>>,
    compute_allocation: Option<u64>,
    memory_allocation: Option<u64>,
    freezing_threshold: Option<u64>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
struct CreateCanisterResult {
    canister_id: Principal,
}

async fn create_new_shard() -> Result<String, String> {
    let management_canister = Principal::management_canister();

    let args = CreateCanisterArgs { settings: None };

    // Посочваме, че очакваме (CreateCanisterResult,) като резултат
    let response: Result<(CreateCanisterResult,), _> = call(
        management_canister,
        "create_canister",
        (args,),
    )
    .await;

    match response {
        Ok((result,)) => Ok(result.canister_id.to_string()),
        Err(err) => Err(format!("Failed to create shard: {:?}", err)),
    }
}
