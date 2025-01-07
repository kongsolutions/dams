#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct DIDDocument {
    pub id: String,
    pub verification_method: Vec<VerificationMethod>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    pub type_: String,
    pub public_key_base58: String,
}
