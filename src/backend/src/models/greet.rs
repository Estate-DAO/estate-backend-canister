use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(
    CandidType, Deserialize, Default, Serialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord,
)]
pub struct GreetParams(pub String);

#[derive(
    CandidType, Deserialize, Default, Serialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord,
)]
pub struct GreetResponse(pub String);
