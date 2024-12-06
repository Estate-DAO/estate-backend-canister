use crate::{PaymentDetails, UserDetails};
use candid::{CandidType, Principal};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub type AppReference = String;
pub type UserEmail = String;

#[derive(
    CandidType, Deserialize, Default, Serialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord,
)]
pub struct BookingId(AppReference, UserEmail);

#[derive(CandidType, Deserialize, Default, Serialize, Clone, Debug)]
pub struct Booking {
    pub booking_id: BookingId,
    pub guests: UserDetails,

    /// status of booking
    pub book_room_status: Option<BookRoomResponse>,

    /// user preferences for the hotel
    pub user_selected_hotel_room_details: HotelRoomDetails,

    pub payment_details: PaymentDetails,
}

impl Booking {
    pub fn new(
        booking_id: BookingId,
        guests: UserDetails,
        // booking_details: UserBookingPreferences,
        book_room_status: Option<BookRoomResponse>,
        user_selected_hotel_room_details: HotelRoomDetails,

        payment_details: PaymentDetails,
    ) -> Result<Self, String> {
        let booking = Self {
            booking_id,
            guests,
            book_room_status,
            user_selected_hotel_room_details,
            payment_details,
        };

        booking.validate()?;
        Ok(booking)
    }

    pub fn validate(&self) -> Result<(), String> {
        // Validate guests exist
        if self.guests.adults.is_empty() {
            return Err("At least one adult guest required".into());
        }

        // // Validate room allocation matches guest count
        // let total_guests = self.guests.total_guests();
        // if total_guests > (self.booking_details.user_selected_hotel_room_details.room_details.len() * 4) {
        //     return Err("Not enough rooms for guest count".into());
        // }

        Ok(())
    }

    pub fn get_booking_status(&self) -> BookingStatus {
        self.book_room_status
            .as_ref()
            .map(|r| r.status.clone())
            .unwrap_or(BookingStatus::BookFailed)
    }

    pub fn get_requested_payment_amount(&self) -> f64 {
        self.user_selected_hotel_room_details
            .requested_payment_amount
    }

    pub fn get_booking_summary(&self) -> String {
        let hotel = &self.user_selected_hotel_room_details.hotel_details;
        let date_range = &self.user_selected_hotel_room_details.date_range;

        format!(
            "{} at {} ({} nights) - {}",
            hotel.hotel_name,
            self.user_selected_hotel_room_details
                .destination
                .as_ref()
                .map(|d| d.city.as_str())
                .unwrap_or("Unknown"),
            date_range.no_of_nights(),
            self.payment_details.get_status_display()
        )
    }

    pub fn is_confirmed(&self) -> bool {
        matches!(self.get_booking_status(), BookingStatus::Confirmed)
    }
}

// #[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
// pub struct UserBookingPreferencesAndBookingStatus {
//     pub book_room_response: Option<BookRoomResponse>,
//     pub user_selected_hotel_room_details: HotelRoomDetails,
// }

// HotelRoomDetails scope
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct HotelRoomDetails {
    pub hotel_details: HotelDetails,
    pub date_range: SelectedDateRange,
    pub destination: Option<Destination>,
    pub room_details: Vec<RoomDetails>,
    /// amount shown on block_room
    pub requested_payment_amount: f64,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct RoomDetails {
    pub room_type_name: String,
    pub room_unique_id: String,
    pub room_price: f32,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct HotelDetails {
    pub hotel_name: String,
    pub hotel_code: String,
    pub hotel_image: String,
    pub hotel_location: String,
    pub block_room_id: String,
    pub hotel_token: String,
}

#[derive(CandidType, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Destination {
    pub city: String,
    pub country_name: String,
    pub country_code: String,
    pub city_id: String,
}

#[derive(CandidType, Deserialize, Default, Serialize, Clone, Debug)]
pub struct SelectedDateRange {
    pub start: (u32, u32, u32),
    pub end: (u32, u32, u32),
}

/// copied from frontend repo as it is.
impl SelectedDateRange {
    pub fn to_string(&self) -> String {
        let start_str = format!(
            "{:04}-{:02}-{:02}",
            self.start.0, self.start.1, self.start.2
        );
        let end_str = format!("{:04}-{:02}-{:02}", self.end.0, self.end.1, self.end.2);
        format!("{} - {}", start_str, end_str)
    }

    pub fn no_of_nights(&self) -> u32 {
        let (start_year, start_month, start_day) = self.start;
        let (end_year, end_month, end_day) = self.end;

        if self.start == (0, 0, 0) || self.end == (0, 0, 0) {
            return 0;
        }

        let start_date = chrono::NaiveDate::from_ymd_opt(start_year as i32, start_month, start_day);
        let end_date = chrono::NaiveDate::from_ymd_opt(end_year as i32, end_month, end_day);

        if let (Some(start), Some(end)) = (start_date, end_date) {
            if end > start {
                return (end - start).num_days() as u32;
            }
        }
        0
    }

    pub fn format_date(date: (u32, u32, u32)) -> String {
        format!("{:02}-{:02}-{:04}", date.2, date.1, date.0)
    }
    pub fn format_as_human_readable_date(&self) -> String {
        let format_date = |(year, month, day): (u32, u32, u32)| {
            NaiveDate::from_ymd_opt(year as i32, month, day)
                .map(|d| d.format("%a, %b %d").to_string())
                .unwrap_or_default()
        };

        format!("{} - {}", format_date(self.start), format_date(self.end))
    }
}

#[derive(CandidType, Deserialize, Default, Serialize, Clone, Debug)]

pub struct BookRoomResponse {
    pub status: BookingStatus,
    pub message: Option<String>,
    pub commit_booking: Vec<BookingDetails>,
}

#[derive(CandidType, Deserialize, Default, Serialize, Clone, Debug)]

pub struct BookingDetails {
    pub booking_id: BookingId,
    pub booking_ref_no: String,
    pub confirmation_no: String,
    pub booking_status: BookingStatus,
}

/// todo: shall we use a string for telling the details of why booking failed / or confirmed with some sort of transaction_id?
#[derive(CandidType, Deserialize, Default, Serialize, Clone, Debug)]
pub enum BookingStatus {
    #[default]
    BookFailed,
    Confirmed,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct BookingSummary {
    pub booking_id: BookingId,
    pub user_email: String,
    pub hotel_name: String,
    pub destination: String,
    pub nights: u32,
    pub payment_status: String,
    pub booking_status: BookingStatus,
}

impl From<(&str, &Booking)> for BookingSummary {
    fn from((email, booking): (&str, &Booking)) -> Self {
        let hotel = &booking.user_selected_hotel_room_details.hotel_details;
        let destination = booking
            .user_selected_hotel_room_details
            .destination
            .as_ref()
            .map(|d| d.city.clone())
            .unwrap_or_default();

        BookingSummary {
            booking_id: booking.booking_id.clone(),
            user_email: email.to_string(),
            hotel_name: hotel.hotel_name.clone(),
            destination,
            nights: booking
                .user_selected_hotel_room_details
                .date_range
                .no_of_nights(),
            payment_status: booking.payment_details.get_status_display(),
            booking_status: booking
                .book_room_status
                .as_ref()
                .map(|r| r.status.clone())
                .unwrap_or_default(),
        }
    }
}
