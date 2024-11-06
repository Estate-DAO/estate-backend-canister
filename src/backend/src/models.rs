use std::collections::BTreeMap;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};


mod payment_details;
pub use payment_details::*;


mod user_details;
pub use user_details::*;

mod booking_details;
pub use booking_details::*;

#[derive(CandidType, Deserialize, Default, Serialize, Clone, Debug)]
pub struct CanisterState {
    //index (String in Btree) is supposed to be user email/phone
    pub details: BTreeMap<String, Details>
}

impl CanisterState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_booking(&mut self, email: String, details: Details) -> Result<(), String> {
        if self.details.contains_key(&email) {
            return Err("Booking already exists for this email".into());
        }
        self.details.insert(email, details);
        Ok(())
    }

    pub fn get_booking(&self, email: &str) -> Option<&Details> {
        self.details.get(email)
    }

    pub fn update_booking(&mut self, email: String, details: Details) -> Result<(), String> {
        if !self.details.contains_key(&email) {
            return Err("No booking found for this email".into());
        }
        self.details.insert(email, details);
        Ok(())
    }

    pub fn get_all_bookings(&self) -> Vec<(&String, &Details)> {
        self.details.iter().collect()
    }

    pub fn cancel_booking(&mut self, email: &str) -> Result<(), String> {
        if self.details.remove(email).is_none() {
            return Err("No booking found for this email".into());
        }
        Ok(())
    }
}

/// todo a user can have many bookings -> Vec
/// todo a user can have many payment details (corresponding to each booking?) -> Vec
/// todo booking -> payment details
/// todo query -> user(email) list all booking WITH payment details
#[derive(CandidType, Deserialize, Default, Serialize, Clone, Debug)]
pub struct Details {
    pub user_details: UserDetails,
    pub booking_details: UserBookingDetails,
    pub payment_details: PaymentDetails,
}

impl Details {
    pub fn new(
        user_details: UserDetails,
        booking_details: UserBookingDetails,
        payment_details: PaymentDetails,
    ) -> Self {
        Self {
            user_details,
            booking_details,
            payment_details,
        }
    }

    pub fn get_booking_status(&self) -> BookingStatus {
        self.booking_details
            .book_room_response
            .as_ref()
            .map(|r| r.status.clone())
            .unwrap_or(BookingStatus::BookFailed)
    }

    pub fn get_total_amount(&self) -> f64 {
        self.booking_details.user_selected_hotel_room_details.requested_payment_amount
    }

    pub fn get_booking_summary(&self) -> String {
        let hotel = &self.booking_details.user_selected_hotel_room_details.hotel_details;
        let date_range = &self.booking_details.user_selected_hotel_room_details.date_range;
        
        format!(
            "{} at {} ({} nights) - {}",
            hotel.hotel_name,
            self.booking_details.user_selected_hotel_room_details.destination.city,
            date_range.no_of_nights(),
            self.payment_details.get_status_display()
        )
    }

    pub fn get_primary_guest(&self) -> Option<&AdultDetail> {
        self.user_details.adults.first()
    }

    pub fn validate(&self) -> Result<(), String> {
        // Validate primary guest exists
        if self.user_details.adults.is_empty() {
            return Err("Primary guest details required".into());
        }

        // Validate primary guest contact info
        let primary = self.get_primary_guest().unwrap();
        if primary.email.is_none() || primary.phone.is_none() {
            return Err("Primary guest must provide email and phone".into());
        }

        // Validate room allocation matches guest count
        let total_guests = self.user_details.adults.len() + self.user_details.children.len();
        if total_guests > (self.booking_details.user_selected_hotel_room_details.room_details.len() * 4) {
            return Err("Not enough rooms for guest count".into());
        }

        Ok(())
    }

    pub fn is_confirmed(&self) -> bool {
        matches!(self.get_booking_status(), BookingStatus::Confirmed)
    }
}
