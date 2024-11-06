use std::collections::BTreeMap;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};


mod payment_details;
pub use payment_details::*;

mod user_details;
pub use user_details::*;

mod booking_details;
pub use booking_details::*;


/// todo a user can have many bookings -> Vec
/// He does
/// 
/// todo a user can have many payment details (corresponding to each booking?) -> Vec
/// user has many bookings, & each booking has its payment details
/// 
/// todo booking -> payment details
/// booking has payment details
/// 
/// todo query -> user(email) list all booking WITH payment details
/// via get_user_bookings


#[derive(CandidType, Deserialize, Default, Serialize, Clone, Debug)]
pub struct CanisterState {
    // Map from email/phone watever to UserProfile
    pub users: BTreeMap<String, UserProfile>
}

impl CanisterState {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn register_user(&mut self, adult: AdultDetail) -> Result<(), String> {
        let email = adult.email.as_ref()
            .ok_or("Email required for registration")?
            .clone();
            
        if self.users.contains_key(&email) {
            return Err("User already registered".into());
        }

        let profile = UserProfile::new(adult)?;
        self.users.insert(email, profile);
        Ok(())
    }

    pub fn add_booking(&mut self, email: &str, booking: Booking) -> Result<(), String> {
        let user_profile = self.users.get_mut(email)
            .ok_or("User not found")?;
        
        user_profile.add_booking(booking)
    }

    pub fn get_user_profile(&self, email: &str) -> Option<&UserProfile> {
        self.users.get(email)
    }

    pub fn get_user_bookings(&self, email: &str) -> Option<&Vec<Booking>> {
        self.users.get(email).map(|profile| &profile.bookings)
    }

    pub fn get_booking(&self, email: &str, booking_id: &str) -> Option<&Booking> {
        self.users.get(email)?.bookings.iter()
            .find(|b| b.booking_id == booking_id)
    }

    pub fn update_booking(&mut self, email: &str, booking_id: &str, booking: Booking) -> Result<(), String> {
        let user_profile = self.users.get_mut(email)
            .ok_or("User not found")?;
        
        let booking_index = user_profile.bookings.iter()
            .position(|b| b.booking_id == booking_id)
            .ok_or("Booking not found")?;

        user_profile.bookings[booking_index] = booking;
        Ok(())
    }

    pub fn cancel_booking(&mut self, email: &str, booking_id: &str) -> Result<(), String> {
        let user_profile = self.users.get_mut(email)
            .ok_or("User not found")?;
        
        let booking_index = user_profile.bookings.iter()
            .position(|b| b.booking_id == booking_id)
            .ok_or("Booking not found")?;

        user_profile.bookings.remove(booking_index);
        Ok(())
    }

    pub fn get_all_bookings(&self) -> Vec<BookingSummary> {
        self.users.iter()
            .flat_map(|(email, profile)| {
                profile.bookings.iter()
                    .map(|booking| BookingSummary::from((email.as_str(), booking)))
            })
            .collect()
    }
}