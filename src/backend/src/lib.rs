mod models;
pub use models::*;

use std::cell::RefCell;
use ic_cdk::{storage, post_upgrade, pre_upgrade};


thread_local! {
    static STATE: RefCell<CanisterState> = RefCell::new(CanisterState::default());
}

#[ic_cdk_macros::init]
fn init() {
    init_hook();
}

fn init_hook() {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| {
        storage::stable_save((&*state.borrow(),))
            .expect("Failed to save stable state");
    });
}

#[post_upgrade]
fn post_upgrade() {
    let state: Result<(CanisterState,), _> = storage::stable_restore();
    match state {
        Ok((restored_state,)) => {
            STATE.with(|state| {
                *state.borrow_mut() = restored_state;
            });
            init_hook();
        }
        Err(err) => {
            ic_cdk::trap(&format!("Failed to restore stable state: {}", err));
        }
    }
}

#[ic_cdk_macros::query]
fn get_state() -> CanisterState {
    STATE.with(|state| state.borrow().clone())
}

#[ic_cdk_macros::update]
fn add_booking(email: String, booking: Booking) -> Result<(), String> {
    STATE.with(|state| {
        state.borrow_mut().add_booking(&email, booking)
    })
}

#[ic_cdk_macros::query]
fn get_user_bookings(email: String) -> Option<Vec<Booking>> {
    STATE.with(|state| {
        state.borrow().get_user_bookings(&email).cloned()
    })
}

#[ic_cdk_macros::query]
fn get_booking(email: String, booking_id: String) -> Option<Booking> {
    STATE.with(|state| {
        state.borrow().get_booking(&email, &booking_id).cloned()
    })
}

#[ic_cdk_macros::update]
fn update_booking(email: String, booking_id: String, booking: Booking) -> Result<(), String> {
    STATE.with(|state| {
        state.borrow_mut().update_booking(&email, &booking_id, booking)
    })
}

#[ic_cdk_macros::update]
fn cancel_booking(email: String, booking_id: String) -> Result<(), String> {
    STATE.with(|state| {
        state.borrow_mut().cancel_booking(&email, &booking_id)
    })
}

#[ic_cdk_macros::query]
fn get_all_bookings() -> Vec<BookingSummary> {
    STATE.with(|state| {
        state.borrow().get_all_bookings()
    })
}

ic_cdk_macros::export_candid!();
