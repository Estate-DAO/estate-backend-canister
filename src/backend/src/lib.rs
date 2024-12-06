mod models;
use candid::CandidType;
pub use models::*;

use ic_cdk::{post_upgrade, pre_upgrade, print, storage};
use std::cell::RefCell;

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

        if state.users.is_empty() {
            // Insert a default user if the map is empty
            state
                .users
                .insert("a@b.com".to_string(), UserInfoAndBookings::default());
        }
    });
}

////////////////////////////
// upgrade API
////////////////////////////

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| {
        storage::stable_save((&*state.borrow(),)).expect("Failed to save stable state");
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

////////////////////////////

////////////////////////////
// CREATE / UPDATE
////////////////////////////

#[ic_cdk_macros::update]
// PRINCIPAL should be admin principal
// is_controller
fn add_booking(email: String, booking: Booking) -> Result<String, String> {
    STATE.with(|state| state.borrow_mut().add_booking_and_user(&email, booking))
}

// #[ic_cdk_macros::update]
// fn update_book_room_response(booking_id: BookingId, book_room_response: BookRoomResponse) -> Result<(), String> {
//     STATE.with(|state| {
//         state.borrow_mut().update_book_room_response(booking_id, book_room_response)
//     })
// }

#[ic_cdk_macros::update]
fn update_payment_details(
    booking_id: BookingId,
    payment_details: PaymentDetails,
) -> Result<Booking, String> {
    STATE.with(|state| {
        state
            .borrow_mut()
            .update_payment_details(booking_id, payment_details)
    })
}

// #[ic_cdk_macros::update]
// fn update_booking(email: String, booking_id: String, booking: Booking) -> Result<(), String> {
//     STATE.with(|state| {
//         state.borrow_mut().update_booking(&email, &booking_id, booking)
//     })
// }

// #[ic_cdk_macros::update]
// fn cancel_booking(email: String, booking_id: String) -> Result<(), String> {
//     STATE.with(|state| {
//         state.borrow_mut().cancel_booking(&email, &booking_id)
//     })
// }

////////////////////////////
// READ
////////////////////////////

#[ic_cdk_macros::query]
fn get_user_bookings(email: String) -> Option<Vec<Booking>> {
    STATE.with(|state| {
        state
            .borrow()
            .get_user_bookings(&email)
            .map(|bookings| bookings.values().cloned().collect())
    })
}

#[ic_cdk_macros::update]
fn update_book_room_response(
    booking_id: BookingId,
    book_room_response: BookRoomResponse,
) -> Result<(), String> {
    STATE.with(|state| {
        state
            .borrow_mut()
            .update_book_room_response(booking_id, book_room_response)
    })
}

// #[ic_cdk_macros::query]
// fn get_booking(email: String, booking_id: String) -> Option<Booking> {
//     STATE.with(|state| {
//         state.borrow().get_booking(&email, &booking_id).cloned()
//     })
// }

// #[ic_cdk_macros::query]
// fn get_all_bookings() -> Vec<BookingSummary> {
//     STATE.with(|state| {
//         state.borrow().get_all_bookings()
//     })
// }

#[ic_cdk_macros::query]
fn greet(GreetParams(name): GreetParams) -> GreetResponse {
    let caller = ic_cdk::caller();

    print!("greet - {caller:?}");

    let resp_strng = format!("Hello, {}!", name);
    GreetResponse(resp_strng)
}

ic_cdk_macros::export_candid!();
