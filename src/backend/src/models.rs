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

// mod booking_state;
// // pub use booking_state::*;

mod tests;

#[derive(CandidType, Deserialize, Default, Serialize, Clone, Debug)]
pub struct CanisterState {
    // Map from email/phone watever to UserInfoAndBookings
    // #[serde(skip, default = "_default_slot_details_map")]
    // pub users:
    pub users: BTreeMap<String, UserInfoAndBookings>,
    // pub ongoing_bookings: BTreeMap<BookingId, BookingState>,
    #[serde(default)]
    pub controllers: Option<Vec<Principal>>,
    // pub controllers: Vec<Principal>,
    // pub admin_principal: Vec<Principal>
}

impl CanisterState {
    pub fn new() -> Self {
        Self {
            users: BTreeMap::new(),
            // ongoing_bookings: BTreeMap::new(),
            controllers: None,
        }
    }

    // pub fn create_booking(&mut self, id: BookingId) -> Result<(), String> {
    //     if self.ongoing_bookings.contains_key(&id) {
    //         return Err("Booking already exists".to_string());
    //     }
    //     self.ongoing_bookings.insert(id, BookingState::new());
    //     Ok(())
    // }

    // pub fn get_booking_state(&self, id: &BookingId) -> Option<&BookingState> {
    //     self.ongoing_bookings.get(id)
    // }

    // pub fn list_bookings_by_status(&self, status: BookingStatus) -> Vec<BookingId> {
    //     self.ongoing_bookings
    //         .iter()
    //         .filter(|(_, state)| state.current_status == status)
    //         .map(|(id, _)| id.clone())
    //         .collect()
    // }

    // pub fn find_stuck_bookings(&self, threshold: std::time::Duration) -> Vec<BookingId> {
    //     self.ongoing_bookings
    //         .iter()
    //         .filter(|(_, state)| !state.is_terminal() && state.time_in_current_state() > threshold)
    //         .map(|(id, _)| id.clone())
    //         .collect()
    // }

    // pub fn update_booking_status(&mut self, id: &BookingId, new_status: BookingStatus) -> Result<(), String> {
    //     let booking = self.ongoing_bookings.get_mut(id)
    //         .ok_or_else(|| "Booking not found".to_string())?;

    //     booking.update_status(new_status)?;

    //     // Remove booking if it reached a terminal state
    //     if booking.is_terminal() {
    //         self.ongoing_bookings.remove(id);
    //     }

    //     Ok(())
    // }

    // pub fn delete_booking(&mut self, id: &BookingId) -> Result<(), String> {
    //     self.ongoing_bookings
    //         .remove(id)
    //         .map(|_| ())
    //         .ok_or_else(|| "Booking not found".to_string())
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

    pub fn get_booking_by_id(&self, booking_id: &BookingId) -> Option<&Booking> {
        // First try to find the user with the email from the booking_id
        let user_email = booking_id.get_user_email();
        if let Some(user) = self.users.get(user_email) {
            // Then try to get the booking from that user
            if let Some(booking) = user.get_booking_by_id(booking_id) {
                return Some(booking);
            }
        }

        None
    }

    pub fn get_user_bookings(&self, email: &str) -> Option<&BTreeMap<BookingId, Booking>> {
        self.users.get(email).map(|profile| &profile.bookings)
    }

    pub fn update_payment_details(
        &mut self,
        booking_id: BookingId,
        payment_details: PaymentDetails,
    ) -> Result<Booking, String> {
        // validation - booking_id MUST exist.

        // Find the user by email
        let user_email = booking_id.get_user_email();
        let user = self
            .users
            .get_mut(user_email)
            .ok_or_else(|| format!("User with email '{}' not found", user_email))?;

        // Find the booking by ID
        let booking = user.bookings.get_mut(&booking_id).ok_or_else(|| {
            format!(
                "Booking with app_reference '{}' not found",
                booking_id.get_app_reference()
            )
        })?;

        // Update booking with payment details and status
        booking.update_payment_details_with_api_response(payment_details.clone());

        Ok(booking.clone())
    }

    pub fn update_book_room_response(
        &mut self,
        booking_id: BookingId,
        book_room_response: BEBookRoomResponse,
    ) -> Result<String, String> {
        // ) -> Result<BEBookRoomResponse, String> {
        let user_email = booking_id.get_user_email();
        let result = self
            .users
            .get_mut(user_email)
            .ok_or_else(|| format!("User with email '{}' not found", user_email))
            .and_then(|user| {
                let booking = user.bookings.get_mut(&booking_id).ok_or_else(|| {
                    format!(
                        "Booking with app_refrence '{}' not found",
                        booking_id.get_app_reference()
                    )
                })?;

                booking.update_book_room_status(book_room_response.clone())?;
                Ok("Success")
            })?;
        Ok(result.into())
    }

    pub fn update_booking_message(
        &mut self,
        booking_id: BookingId,
        message: String,
    ) -> Result<String, String> {
        let user_email = booking_id.get_user_email();
        self.users
            .get_mut(user_email)
            .ok_or_else(|| format!("User with email '{}' not found", user_email))
            .and_then(|user| {
                user.update_booking_message(&booking_id, message)
                    .map(|_| "Message updated successfully".to_string())
            })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn create_test_booking() -> Booking {
//         let booking_id = BookingId::new("TEST123".to_string(), "test@example.com".to_string());

//         let adult = AdultDetail {
//             first_name: "John".to_string(),
//             last_name: Some("Doe".to_string()),
//             email: Some("test@example.com".to_string()),
//             phone: Some("1234567890".to_string()),
//         };

//         let user_details = UserDetails {
//             adults: vec![adult],
//             children: vec![],
//         };

//         let hotel_details = HotelDetails {
//             hotel_name: "Test Hotel".to_string(),
//             hotel_code: "TH001".to_string(),
//             hotel_image: "image.jpg".to_string(),
//             hotel_location: "Test Location".to_string(),
//             block_room_id: "BR001".to_string(),
//             hotel_token: "token123".to_string(),
//         };

//         let date_range = SelectedDateRange {
//             start: (2023, 1, 1),
//             end: (2023, 1, 5),
//         };

//         let destination = Some(Destination {
//             city: "Test City".to_string(),
//             country_name: "Test Country".to_string(),
//             country_code: "TC".to_string(),
//             city_id: "TC001".to_string(),
//         });

//         let room_details = vec![RoomDetails {
//             room_type_name: "Deluxe".to_string(),
//             room_unique_id: "D001".to_string(),
//             room_price: 100.0,
//         }];

//         let hotel_room_details = HotelRoomDetails {
//             hotel_details,
//             date_range,
//             destination,
//             room_details,
//             requested_payment_amount: 400.0,
//         };

//         let payment_details = PaymentDetails::new(booking_id.clone());

//         Booking {
//             booking_id,
//             guests: user_details,
//             book_room_status: None,
//             user_selected_hotel_room_details: hotel_room_details,
//             payment_details,
//         }
//     }

//     #[test]
//     fn test_new() {
//         let state = CanisterState::new();
//         assert!(state.users.is_empty());
//         assert!(state.controllers.is_none() || state.controllers.as_ref().unwrap().is_empty());
//     }

//     #[test]
//     fn test_add_booking_and_user() {
//         let mut state = CanisterState::new();
//         let booking = create_test_booking();
//         let email = "test@example.com";

//         let result = state.add_booking_and_user(email, booking.clone());
//         assert!(result.is_ok());

//         // Verify user was added
//         assert!(state.users.contains_key(email));

//         // Verify booking was added to user
//         let user = state.users.get(email).unwrap();
//         assert!(user.bookings.contains_key(&booking.booking_id));

//         // Test adding duplicate booking
//         let result = state.add_booking_and_user(email, booking);
//         assert!(result.is_ok()); // The function always returns Ok
//     }

//     #[test]
//     fn test_get_user_profile() {
//         let mut state = CanisterState::new();
//         let booking = create_test_booking();
//         let email = "test@example.com";

//         // Add a user with a booking
//         let _ = state.add_booking_and_user(email, booking);

//         // Test getting existing user
//         let user_profile = state.get_user_profile(email);
//         assert!(user_profile.is_some());

//         // Test getting non-existent user
//         let user_profile = state.get_user_profile("nonexistent@example.com");
//         assert!(user_profile.is_none());
//     }

//     #[test]
//     fn test_get_booking_by_id() {
//         let mut state = CanisterState::new();
//         let booking = create_test_booking();
//         let booking_id = booking.booking_id.clone();
//         let email = "test@example.com";

//         // Add a user with a booking
//         let _ = state.add_booking_and_user(email, booking);

//         // Test getting existing booking
//         let retrieved_booking = state.get_booking_by_id(&booking_id);
//         assert!(retrieved_booking.is_some());

//         // Test getting non-existent booking
//         let non_existent_id =
//             BookingId::new("NONEXISTENT".to_string(), "test@example.com".to_string());
//         let retrieved_booking = state.get_booking_by_id(&non_existent_id);
//         assert!(retrieved_booking.is_none());
//     }

//     #[test]
//     fn test_get_user_bookings() {
//         let mut state = CanisterState::new();
//         let booking = create_test_booking();
//         let email = "test@example.com";

//         // Add a user with a booking
//         let _ = state.add_booking_and_user(email, booking);

//         // Test getting bookings for existing user
//         let bookings = state.get_user_bookings(email);
//         assert!(bookings.is_some());
//         assert_eq!(bookings.unwrap().len(), 1);

//         // Test getting bookings for non-existent user
//         let bookings = state.get_user_bookings("nonexistent@example.com");
//         assert!(bookings.is_none());
//     }

//     #[test]
//     fn test_update_payment_details() {
//         let mut state = CanisterState::new();
//         let booking = create_test_booking();
//         let booking_id = booking.booking_id.clone();
//         let email = "test@example.com";

//         // Add a user with a booking
//         let _ = state.add_booking_and_user(email, booking);

//         // Create updated payment details
//         let mut payment_details = PaymentDetails::new(booking_id.clone());
//         payment_details.payment_api_response.payment_status = "finished".to_string();
//         payment_details.payment_api_response.payment_id = 12345;

//         // Update payment details
//         let result = state.update_payment_details(booking_id.clone(), payment_details);
//         assert!(result.is_ok());

//         // Verify payment status was updated
//         let updated_booking = result.unwrap();
//         assert!(matches!(
//             updated_booking.payment_details.payment_status,
//             BackendPaymentStatus::Paid(_)
//         ));

//         // Test updating non-existent booking
//         let non_existent_id =
//             BookingId::new("NONEXISTENT".to_string(), "test@example.com".to_string());
//         let payment_details = PaymentDetails::new(non_existent_id.clone());
//         let result = state.update_payment_details(non_existent_id, payment_details);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_update_book_room_response() {
//         let mut state = CanisterState::new();
//         let booking = create_test_booking();
//         let booking_id = booking.booking_id.clone();
//         let email = "test@example.com";

//         // Add a user with a booking
//         let _ = state.add_booking_and_user(email, booking);

//         // Create book room response
//         let book_room_response = BEBookRoomResponse {
//             status: "Success".to_string(),
//             message: "Room booked successfully".to_string(),
//             commit_booking: BookingDetails {
//                 booking_id: booking_id.clone(),
//                 travelomatrix_id: "TM123".to_string(),
//                 booking_ref_no: "REF123".to_string(),
//                 confirmation_no: "CONF123".to_string(),
//                 api_status: BookingStatus::Confirmed,
//                 booking_status: "Confirmed".to_string(),
//             },
//         };

//         // Update book room response
//         let result = state.update_book_room_response(booking_id.clone(), book_room_response);
//         assert!(result.is_ok());

//         // Verify book room status was updated
//         let updated_booking = state.get_booking_by_id(&booking_id).unwrap();
//         assert!(updated_booking.book_room_status.is_some());
//         assert_eq!(updated_booking.get_booking_status(), "Confirmed");

//         // Test updating non-existent booking
//         let non_existent_id =
//             BookingId::new("NONEXISTENT".to_string(), "test@example.com".to_string());
//         let book_room_response = BEBookRoomResponse::default();
//         let result = state.update_book_room_response(non_existent_id, book_room_response);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_update_booking_message() {
//         let mut state = CanisterState::new();
//         let booking = create_test_booking();
//         let booking_id = booking.booking_id.clone();
//         let email = "test@example.com";

//         // Add a user with a booking
//         let _ = state.add_booking_and_user(email, booking);

//         // Update booking message
//         let message = "Test message";
//         let result = state.update_booking_message(booking_id.clone(), message.to_string());
//         assert!(result.is_ok());

//         // Verify message was updated
//         let updated_booking = state.get_booking_by_id(&booking_id).unwrap();
//         assert!(updated_booking.book_room_status.is_some());
//         assert_eq!(
//             updated_booking.book_room_status.as_ref().unwrap().message,
//             format!("[frontend] {}", message)
//         );

//         // Test updating non-existent booking
//         let non_existent_id =
//             BookingId::new("NONEXISTENT".to_string(), "test@example.com".to_string());
//         let result = state.update_booking_message(non_existent_id, "Test".to_string());
//         assert!(result.is_err());
//     }
// }
