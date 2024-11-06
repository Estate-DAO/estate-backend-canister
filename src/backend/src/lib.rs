mod models;
pub use models::*;
use std::{cell::RefCell, collections::BTreeMap};

thread_local! {
    static STATE: RefCell<State> = RefCell::new(CanisterState::default());
}

#[ic_cdk_macros::init]
fn init() {
    init_hook();
}
