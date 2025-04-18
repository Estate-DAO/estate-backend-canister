use std::collections::BTreeMap;

use crate::Booking;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use super::{BEBookRoomResponse, BookingDetails, BookingId};

#[derive(CandidType, Deserialize, Default, Serialize, Clone, Debug)]
pub struct UserInfoAndBookings {
    pub primary_user: AdultDetail,
    pub bookings: BTreeMap<BookingId, Booking>,
}

impl UserInfoAndBookings {
    // pub fn new(primary_user: AdultDetail) -> Result<Self, String> {
    //     // Validate that primary user has required contact info
    //     if primary_user.email.is_none() || primary_user.phone.is_none() {
    //         return Err("Primary user must have email and phone".into());
    //     }

    //     Ok(Self {
    //         primary_user,
    //         bookings: BTreeMap::new(),
    //     })
    // }

    pub fn get_booking_by_id(&self, booking_id: &BookingId) -> Option<&Booking> {
        self.bookings.get(booking_id)
    }

    pub fn get_contact_info(&self) -> Option<(String, String)> {
        match (&self.primary_user.email, &self.primary_user.phone) {
            (Some(email), Some(phone)) => Some((email.clone(), phone.clone())),
            _ => None,
        }
    }

    // pub fn add_booking(&mut self, booking: Booking) -> Result<(), String> {
    //     // Check for duplicate booking_id
    //     if self.bookings.iter().any(|b| b.booking_id == booking.booking_id) {
    //         return Err("Booking ID already exists".into());
    //     }

    //     self.bookings.push(booking);
    //     Ok(())
    // }

    pub fn add_booking(&mut self, booking: Booking) -> Result<(), String> {
        // Check for duplicate booking_id
        if self.bookings.contains_key(&booking.booking_id) {
            return Err("Booking ID already exists".into());
        }

        self.bookings.insert(booking.booking_id.clone(), booking); // Insert into BTreeMap
        Ok(())
    }

    pub fn update_booking_message(
        &mut self,
        booking_id: &BookingId,
        message: String,
    ) -> Result<(), String> {
        if let Some(booking) = self.bookings.get_mut(booking_id) {
            let frontend_message = format!("[frontend] {}", message);

            // If book_room_status is None, create a new one with the message
            if booking.book_room_status.is_none() {
                booking.book_room_status = Some(BEBookRoomResponse {
                    status: "Updated".to_string(),
                    message: frontend_message,
                    commit_booking: BookingDetails::default(),
                });
            } else {
                // Update the existing book_room_status message
                if let Some(status) = &mut booking.book_room_status {
                    status.message = frontend_message;
                }
            }
            Ok(())
        } else {
            Err("Booking ID does not exist".to_string())
        }
    }

    // pub fn get_all_booking_summaries(&self) -> Vec<String> {
    //     self.bookings.iter()
    //         .map(|booking| booking.get_booking_summary())
    //         .collect()
    // }
}

// UserDetails scope
#[derive(CandidType, Serialize, Deserialize, Default, Clone, Debug)]
pub struct UserDetails {
    pub adults: Vec<AdultDetail>,
    pub children: Vec<ChildDetail>,
}

impl UserDetails {
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn add_adult(&mut self, adult: AdultDetail) {
    //     self.adults.push(adult);
    // }

    // pub fn add_child(&mut self, child: ChildDetail) -> Result<(), String> {
    //     if child.age > 18 {
    //         return Err("Child must be under 18 years old".into());
    //     }
    //     self.children.push(child);
    //     Ok(())
    // }

    pub fn get_primary_contact(&self) -> Option<(String, String)> {
        self.adults
            .first()
            .and_then(|adult| match (&adult.email, &adult.phone) {
                (Some(email), Some(phone)) => Some((email.clone(), phone.clone())),
                _ => None,
            })
    }

    // pub fn total_guests(&self) -> usize {
    //     self.adults.len() + self.children.len()
    // }

    pub fn validate(&self) -> Result<(), String> {
        if self.adults.is_empty() {
            return Err("At least one adult required".into());
        }

        // Validate primary adult has contact info
        if let Some(adult) = self.adults.first() {
            if adult.email.is_none() || adult.phone.is_none() {
                return Err("Primary adult must provide email and phone".into());
            }
        }

        // todo: validate that all the ages of children are < 18

        Ok(())
    }
}

#[derive(CandidType, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct AdultDetail {
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>, // Only for first adult
    pub phone: Option<String>, // Only for first adult
}

#[derive(CandidType, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct ChildDetail {
    pub first_name: String,
    pub last_name: Option<String>,
    pub age: u8,
}
