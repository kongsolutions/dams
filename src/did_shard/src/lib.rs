use candid::CandidType;
use ic_cdk_macros::*;
use serde::Deserialize;
use std::cell::RefCell;

#[derive(CandidType, Deserialize, Clone)]
struct Shard {
    data: String,
}

thread_local! {
    static SHARDS: RefCell<Vec<Shard>> = RefCell::new(Vec::new());
}

#[update]
fn add_shard(data: String) {
    SHARDS.with(|shards| {
        shards.borrow_mut().push(Shard { data });
    });
}

#[query]
fn get_shards() -> Vec<String> {
    SHARDS.with(|shards| {
        shards
            .borrow()
            .iter()
            .map(|shard| shard.data.clone())
            .collect()
    })
}
