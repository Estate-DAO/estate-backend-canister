use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod payment_details;
pub use payment_details::*;

mod user_details;
pub use user_details::*;

mod booking_details;
pub use booking_details::*;

mod greet;
pub use greet::*;

#[derive(CandidType, Deserialize, Default, Serialize, Clone, Debug)]
pub struct CanisterState {
    // Map from email/phone watever to UserInfoAndBookings
    // #[serde(skip, default = "_default_slot_details_map")]
    // pub users:
    //     ic_stable_structures::btreemap::BTreeMap<String,UserInfoAndBookings, Memory>,
    pub users: BTreeMap<String, UserInfoAndBookings>,
    // pub admin_principal: Vec<Principal>
}

impl CanisterState {
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn register_user(&mut self, adult: AdultDetail) -> Result<(), String> {
    //     let email = adult.email.as_ref()
    //         .ok_or("Email required for registration")?
    //         .clone();

    //     if self.users.contains_key(&email) {
    //         return Err("User already registered".into());
    //     }

    //     let profile = UserInfoAndBookings::new(adult)?;
    //     self.users.insert(email, profile);
    //     Ok(())
    // }
    pub fn add_booking_and_user(
        &mut self,
        email: &str,
        booking: Booking,
    ) -> Result<String, String> {
        let user_profile = self
            .users
            .entry(email.to_string())
            .or_insert_with(|| UserInfoAndBookings::default());

        let user_result = user_profile.add_booking(booking);
        println!("add_booking_and_user - {user_result:?}");
        Ok("Success".into())
    }

    pub fn get_user_profile(&self, email: &str) -> Option<&UserInfoAndBookings> {
        self.users.get(email)
    }

    pub fn get_user_bookings(&self, email: &str) -> Option<&BTreeMap<BookingId, Booking>> {
        self.users.get(email).map(|profile| &profile.bookings)
    }

    pub fn update_payment_details(
        &self,
        booking_id: BookingId,
        payment_details: PaymentDetails,
    ) -> Result<Booking, String> {
        // validation - booking_id MUST exist.
        let cloned_payment_api_resp = payment_details.payment_api_response.clone();

        // Find the booking by ID.
        let mut booking = self
            .users
            .values()
            .flat_map(|user| user.bookings.values())
            .find(|booking| booking.booking_id == booking_id)
            .cloned()
            .ok_or("Booking not found".to_string())?;

        booking.payment_details = payment_details;

        let status = cloned_payment_api_resp.payment_status;

        let payment_status = match status.as_str() {
            "finished" => {
                let trans_ref = format!("{:?} - COMPLETED", cloned_payment_api_resp.payment_id);
                BackendPaymentStatus::Paid(trans_ref)
            }
            "cancelled" => {
                let trans_ref = format!("{:?} - CANCELLED", cloned_payment_api_resp.payment_id);
                BackendPaymentStatus::Unpaid(Some(trans_ref))
            }
            _ => {
                let trans_ref = format!("{:?} - PENDING", cloned_payment_api_resp.payment_id);
                BackendPaymentStatus::Unpaid(Some(trans_ref))
            }
        };
        let pay_stat_c = payment_status.clone();

        booking.update_payment_status(payment_status);
        booking.payment_details.payment_status = pay_stat_c;

        println!("{:?}", booking);
        Ok(booking)
    }
    // pub fn get_booking(&self, email: &str, booking_id: &str) -> Option<&Booking> {
    //     self.users.get(email)?.bookings.iter()
    //         .find(|b| b.booking_id == booking_id)
    // }

    // pub fn update_booking(&mut self, email: &str, booking_id: &str, booking: Booking) -> Result<(), String> {
    //     let user_profile = self.users.get_mut(email)
    //         .ok_or("User not found")?;

    //     let booking_index = user_profile.bookings.iter()
    //         .position(|b| b.booking_id == booking_id)
    //         .ok_or("Booking not found")?;

    //     user_profile.bookings[booking_index] = booking;
    //     Ok(())
    // }

    // pub fn cancel_booking(&mut self, email: &str, booking_id: &str) -> Result<(), String> {
    //     let user_profile = self.users.get_mut(email)
    //         .ok_or("User not found")?;

    //     let booking_index = user_profile.bookings.iter()
    //         .position(|b| b.booking_id == booking_id)
    //         .ok_or("Booking not found")?;

    //     user_profile.bookings.remove(booking_index);
    //     Ok(())
    // }

    // pub fn get_all_bookings(&self) -> Vec<BookingSummary> {
    //     self.users.iter()
    //         .flat_map(|(email, profile)| {
    //             profile.bookings.iter()
    //                 .map(|booking| BookingSummary::from((email.as_str(), booking)))
    //         })
    //         .collect()
    // }

    pub fn update_book_room_response(
        &mut self,
        booking_id: BookingId,
        book_room_response: BEBookRoomResponse,
    ) -> Result<(), String> {
        let user_email = booking_id.get_user_email();
        let result = self
            .users
            .get_mut(user_email)
            .ok_or_else(|| format!("User with email '{}' not found", user_email))
            .and_then(|user| {
                user.bookings
                    .get_mut(&booking_id)
                    .map(|booking| {
                        booking.book_room_status = Some(book_room_response);
                    })
                    .ok_or_else(|| {
                        format!(
                            "Booking with app_refrence '{}' not found",
                            booking_id.get_app_reference()
                        )
                    })
            });
        result.map(|_| ())
    }
}
