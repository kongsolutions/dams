use candid::CandidType;
use ic_cdk_macros::*;
use serde::Deserialize;
use std::cell::RefCell;

#[derive(CandidType, Deserialize, Clone)]
struct Entry {
    value: String,
}

thread_local! {
    static REGISTRY: RefCell<Vec<Entry>> = RefCell::new(Vec::new());
}

#[update]
fn add_entry(value: String) {
    REGISTRY.with(|registry| {
        registry.borrow_mut().push(Entry { value });
    });
}

#[query]
fn get_entries() -> Vec<String> {
    REGISTRY.with(|registry| {
        registry
            .borrow()
            .iter()
            .map(|entry| entry.value.clone())
            .collect()
    })
}
