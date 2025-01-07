use serde::{Serialize, Deserialize};
use candid::{CandidType};
use ic_cdk_macros::*;
use std::collections::HashMap;

#[derive(Clone, Debug, CandidType, Deserialize)]
struct User {
    email: Option<String>,
    google_sub: Option<String>,
    did: String,
}

type UserRegistry = HashMap<String, User>; // email or google_sub -> User

#[init]
fn init() {
    ic_cdk::storage::get_mut::<UserRegistry>().clear();
}

#[update]
async fn register_user(
    email: Option<String>,
    google_sub: Option<String>,
    did_document: DIDDocument,
) -> Result<String, String> {
    let mut registry = ic_cdk::storage::get_mut::<UserRegistry>();

    if let Some(email) = &email {
        if registry.contains_key(email) {
            return Err("Email is already registered".to_string());
        }
    }

    if let Some(google_sub) = &google_sub {
        if registry.contains_key(google_sub) {
            return Err("Google account is already registered".to_string());
        }
    }

    let did = did_document.id.clone();

    // Store the DID Document in the Shard Registry
    let response: Result<String, String> = ic_cdk::call(
        "shard_registry_canister_id",
        "register_did",
        (did.clone(), did_document),
    )
    .await
    .map_err(|e| format!("Failed to register DID document: {:?}", e))?;

    if response.is_ok() {
        // Store the user in the User Registry
        let key = email.clone().unwrap_or_else(|| google_sub.clone().unwrap());
        registry.insert(
            key,
            User {
                email,
                google_sub,
                did: did.clone(),
            },
        );

        Ok("User registered successfully".to_string())
    } else {
        Err("Failed to store DID Document".to_string())
    }
}
