pub mod models;
pub use models::*;
mod controller;
mod migration;
mod migrations;

use candid::Principal;
pub use controller::is_controller;

use std::cell::RefCell;

pub mod memory;
pub mod lifecycle;

#[cfg(test)]
mod payment_id_index_tests;

thread_local! {
    static STATE: RefCell<CanisterState> = RefCell::new(CanisterState::default());
}

// #[ic_cdk_macros::init]
// fn init() {
//     init_hook();
// }

// fn init_hook() {
//     STATE.with(|state| {
//         let mut state = state.borrow_mut();

//         if state.users.is_empty() {
//             // Insert a default user if the map is empty
//             state
//                 .users
//                 .insert("a@b.com".to_string(), UserInfoAndBookings::default());
//         }
//     });
// }


pub fn rebuild_payment_id_index() {
    STATE.with(|state| {
        let mut state = state.borrow_mut();

        // Only rebuild if index doesn't exist or is empty (new field after upgrade)
        if let Some(ref payment_index) = state.payment_id_index {
            if !payment_index.is_empty() {
                return; // Index already exists, no need to rebuild
            }
        }

        // Collect payment_id_v2 -> booking_id mappings
        let mut payment_mappings = Vec::new();
        for user_info in state.users.values() {
            for booking in user_info.bookings.values() {
                let payment_id_v2 = &booking.payment_details.payment_api_response.payment_id_v2;
                if !payment_id_v2.is_empty() {
                    payment_mappings.push((payment_id_v2.clone(), booking.booking_id.clone()));
                }
            }
        }

        // Initialize the index if it doesn't exist
        if state.payment_id_index.is_none() {
            state.payment_id_index = Some(std::collections::BTreeMap::new());
        }

        // Build index from collected mappings
        if let Some(ref mut payment_index) = state.payment_id_index {
            for (payment_id_v2, booking_id) in payment_mappings {
                payment_index.insert(payment_id_v2, booking_id);
            }
        }
    });
}

////////////////////////////
// CREATE / UPDATE
////////////////////////////
#[ic_cdk_macros::update(guard = "is_controller")]
fn add_booking(email: String, booking: Booking) -> Result<String, String> {
    STATE.with(|state| state.borrow_mut().add_booking_and_user(&email, booking))
}

// #[ic_cdk_macros::update(guard = "is_controller")]
// fn update_booking_message(booking_id: BookingId, message: String) -> Result<String, String> {
//     STATE.with(|state| {
//         state
//             .borrow_mut()
//             .update_booking_message(booking_id, message)
//     })
// }

// #[ic_cdk_macros::update(guard = "is_controller")]
// fn update_book_room_response(booking_id: BookingId, book_room_response: BookRoomResponse) -> Result<(), String> {
//     STATE.with(|state| {
//         state.borrow_mut().update_book_room_response(booking_id, book_room_response)
//     })
// }

#[ic_cdk_macros::update(guard = "is_controller")]
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

// #[ic_cdk_macros::update(guard = "is_controller")]
// fn update_booking(email: String, booking_id: String, booking: Booking) -> Result<(), String> {
//     STATE.with(|state| {
//         state.borrow_mut().update_booking(&email, &booking_id, booking)
//     })
// }

// #[ic_cdk_macros::update(guard = "is_controller")]
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

#[ic_cdk_macros::update(guard = "is_controller")]
fn update_book_room_response(
    booking_id: BookingId,
    book_room_response: BEBookRoomResponse,
) -> Result<String, String> {
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

#[ic_cdk_macros::query(guard = "is_controller")]
fn get_all_bookings() -> Vec<BookingSummary> {
    STATE.with(|state| state.borrow().get_all_bookings())
}

#[ic_cdk_macros::query]
fn greet(GreetParams(name): GreetParams) -> GreetResponse {
    let caller = ic_cdk::caller();

    print!("greet - {caller:?}");

    let resp_strng = format!("Hello, {}!", name);
    GreetResponse(resp_strng)
}

#[ic_cdk_macros::query(guard = "is_controller")]
fn is_booking_paid(booking_id: BookingId) -> bool {
    STATE.with(|state| {
        state
            .borrow()
            .get_booking_by_id(&booking_id)
            .map(|booking| booking.payment_details.is_paid())
            .unwrap_or(false)
    })
}

#[ic_cdk_macros::update(guard = "is_controller")]
fn update_email_sent(booking_id: BookingId, sent: bool) -> Result<(), String> {
    STATE.with(|state| state.borrow_mut().update_email_sent(booking_id, sent))
}

#[ic_cdk_macros::query]
fn get_email_sent(booking_id: BookingId) -> Result<bool, String> {
    STATE.with(|state| state.borrow_mut().get_email_sent(&booking_id))
}

#[ic_cdk_macros::query]
fn get_booking_by_id(booking_id: BookingId) -> Option<Booking> {
    print!("get_booking_by_id - {booking_id:?}");
    STATE.with(|state| state.borrow().get_booking_by_id(&booking_id).cloned())
}

#[ic_cdk_macros::query]
fn get_booking_id_by_payment_id_v2(payment_id_v2: String) -> Option<BookingId> {
    STATE.with(|state| {
        state
            .borrow()
            .payment_id_index
            .as_ref()
            .and_then(|index| index.get(&payment_id_v2).cloned())
    })
}

#[ic_cdk_macros::query]
fn get_current_migration_info() -> (u64, String) {
    STATE.with(|state| state.borrow().get_current_migration_info())
}

#[ic_cdk_macros::update(guard = "is_controller")]
fn run_migrations() -> Result<String, String> {
    use crate::migration::MigrationEngine;
    
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let engine = MigrationEngine::new();
        
        let pending_count = engine.get_pending_migrations(&state).len();
        if pending_count == 0 {
            return Ok("No pending migrations to run".to_string());
        }
        
        engine.apply_migrations(&mut state)
            .map_err(|e| format!("Migration failed: {}", e))?;
        
        Ok(format!("Successfully applied {} migration(s)", pending_count))
    })
}

#[ic_cdk_macros::update(guard = "is_controller")]
fn update_user_principal_email_index(principal: Principal, email: String) -> Result<String, String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        
        // Validate email format (basic validation)
        if email.is_empty() || !email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        
        // Update the index
        state.user_principal_email_index.insert(principal, email.clone());
        
        Ok(format!("Successfully mapped principal {} to email {}", principal, email))
    })
}

#[ic_cdk_macros::query]
fn my_bookings() -> Vec<Booking> {
   let user = ic_cdk::caller();
   STATE.with(|state| {
       state.borrow()
           .get_user_bookings_by_principal(user)
           .map(|bookings| bookings.values().cloned().collect())
           .unwrap_or_default()
   })
}

// #[ic_cdk_macros::query(guard = "is_controller")]
// fn get_booking_by_app_reference(app_reference: AppReference) -> Option<Booking> {
//     STATE.with(|state| {
//         state
//             .borrow()
//             .users
//             .values()
//             .find_map(|user_info| {
//                 user_info
//                     .bookings
//                     .values()
//                     .find(|booking| booking.booking_id.get_app_reference() == app_reference)
//             })
//             .cloned()
//     })
// }

// #[ic_cdk_macros::query(guard = "is_controller")]
// fn get_booking_by_app_reference_and_email(app_reference: AppReference, email: String) -> Option<Booking> {
//     let booking_id = BookingId::new(app_reference, email);
//     STATE.with(|state| {
//         state
//             .borrow()
//             .get_booking_by_id(&booking_id)
//             .cloned()
//     })
// }

ic_cdk_macros::export_candid!();
