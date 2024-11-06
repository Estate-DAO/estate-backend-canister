mod models;
pub use models::*;

use std::cell::RefCell;

thread_local! {
    static STATE: RefCell<CanisterState> = RefCell::new(CanisterState::default());
}

#[ic_cdk_macros::init]
fn init() {
    init_hook();
}
