use std::collections::BTreeMap;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

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
    pub users: BTreeMap<String, UserInfoAndBookings>
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

    pub fn add_booking(&mut self, email: &str, booking: Booking) -> Result<String, String> {
        let user_profile = self.users.get_mut(email)
            .ok_or("User not found")?;
        
        user_profile.add_booking(booking);
        Ok("Success".into())
    }

    pub fn get_user_profile(&self, email: &str) -> Option<&UserInfoAndBookings> {
        self.users.get(email)
    }

    pub fn get_user_bookings(&self, email: &str) -> Option<&BTreeMap<BookingId, Booking>> {
        self.users.get(email).map(|profile| &profile.bookings)
    }

    pub fn update_payment_status(&self, booking_id: BookingId, payment_status: BackendPaymentStatus) -> Result<Booking, String> {
        // Find the booking by ID.
        let booking = self.users.values()
            .flat_map(|user| user.bookings.values())
            .find(|booking| booking.booking_id == booking_id)
            .cloned()
            .ok_or("Booking not found".to_string())?; // Return error if not found

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
}