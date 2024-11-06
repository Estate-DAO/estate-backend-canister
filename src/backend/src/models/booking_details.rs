use std::collections::BTreeMap;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct UserBookingDetails{
    pub book_room_response: Option<BookRoomResponse>,
    pub user_selected_hotel_room_details: HotelRoomDetails
}

// HotelRoomDetails scope
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct HotelRoomDetails{
    pub hotel_details: HotelDetails,
    pub date_range: SelectedDateRange,
    pub destination: Option<Destination>,
    pub room_details: Vec<RoomDetails>,
    pub requested_payment_amount: f64
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct RoomDetails{
    pub room_type_name: String,
    pub room_unique_id: String,
    pub room_price: f32
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct HotelDetails {
    pub hotel_name: String,
    pub hotel_code: String,
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

        format!("{} - {}",
            format_date(self.start),
            format_date(self.end)
        )
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
    pub booking_id: String,
    pub booking_ref_no: String,
    pub confirmation_no: String,
    pub booking_status: BookingStatus,
}

/// todo: shall we use a string for telling the details of why booking failed / or confirmed with some sort of transaction_id?
#[derive(CandidType, Deserialize, Default, Serialize, Clone, Debug)]
pub enum BookingStatus {
    #[default]
    BookFailed = 0,
    Confirmed = 1,
}
 
