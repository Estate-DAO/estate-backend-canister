use std::collections::BTreeMap;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};


#[derive(CandidType, Deserialize, Default, Serialize, Clone, Debug)]
pub struct CanisterState {
    //index (String in Btree) is supposed to be user email/phone
    pub details: BTreeMap<String, Details>
}

impl CanisterState {
    pub fn new() -> Self {
        Self { details: BTreeMap::new() }
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

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct UserBookingDetails{
    pub book_room_response: Option<BookRoomResponse>,
    pub user_selected_hotel_room_details: HotelRoomDetails
}

// HotelRoomDetails scope
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct HotelRoomDetails{
    hotel_details: HotelDetails,
    pub date_range: SelectedDateRange,
    destination: Destination,
    room_details: Vec<RoomDetails>,
    requested_payment_amount: f64
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

#[derive(Clone, Debug, Default)]
pub struct SelectedDateRange {
    pub start: (u32, u32, u32),
    pub end: (u32, u32, u32),
}
// _________________________________________

// BookRoomResponse scope
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct BookRoomResponse {
    pub status: BookingStatus,
    pub message: Option<String>,
    pub commit_booking: Vec<BookingDetails>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookingDetails {
    pub booking_id: String,
    pub booking_ref_no: String,
    pub confirmation_no: String,
    pub booking_status: BookingStatus,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum BookingStatus {
    BookFailed = 0,
    Confirmed = 1,
}
// _____________________________________

// UserDetails scope
#[derive(CandidType, Clone, Default, Debug)]
pub struct UserDetails {
    pub adults: Vec<AdultDetail>,
    pub children: Vec<ChildDetail>,
}

impl UserDetails {
    pub fn new() -> Self {
        Self {
            adults: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_adult(&mut self, adult: AdultDetail) {
        self.adults.push(adult);
    }

    pub fn add_child(&mut self, child: ChildDetail) -> Result<(), String> {
        if child.age > 18 {
            return Err("Child must be under 18 years old".into());
        }
        self.children.push(child);
        Ok(())
    }

    pub fn get_primary_contact(&self) -> Option<(String, String)> {
        self.adults.first().and_then(|adult| {
            match (&adult.email, &adult.phone) {
                (Some(email), Some(phone)) => Some((email.clone(), phone.clone())),
                _ => None
            }
        })
    }

    pub fn total_guests(&self) -> usize {
        self.adults.len() + self.children.len()
    }

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

        Ok(())
    }
}

#[derive(Default, Clone, Debug)]
pub struct AdultDetail {
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>, // Only for first adult
    pub phone: Option<String>, // Only for first adult
}

#[derive(Default, Clone, Debug)]
pub struct ChildDetail {
    pub first_name: String,
    pub last_name: Option<String>,
    pub age: u8,
}
// ______________________________________________

// PaymentDetails Scope
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct PaymentDetails {
    pub booking_id: String,
    pub payment_status: PaymentStatus,
}

impl PaymentDetails {
    pub fn new(booking_id: String) -> Self {
        Self {
            booking_id,
            payment_status: PaymentStatus::Unpaid(None),
        }
    }

    pub fn process_payment(&mut self, transaction_ref: String) -> Result<(), String> {
        self.payment_status = PaymentStatus::Paid(transaction_ref);
        Ok(())
    }

    pub fn mark_payment_failed(&mut self, error: String) {
        self.payment_status = PaymentStatus::Unpaid(Some(error));
    }

    pub fn get_status_display(&self) -> String {
        match &self.payment_status {
            PaymentStatus::Paid(ref_no) => format!("Payment confirmed (Ref: {})", ref_no),
            PaymentStatus::Unpaid(None) => "Awaiting payment".to_string(),
            PaymentStatus::Unpaid(Some(error)) => format!("Payment failed: {}", error),
        }
    }

    pub fn is_paid(&self) -> bool {
        matches!(self.payment_status, PaymentStatus::Paid(_))
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum PaymentStatus {
    /// transaction reference number from payments provider
    Paid(String),
    /// if the transaction failed, that would be here.
    Unpaid(Option<String>),
}
// ____________________________________________





// #[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
// pub struct RentalTransaction {
//     pub booking_id: u64,
//     pub car_id: u64,
//     pub customer_principal_id: Principal,
//     pub customer: Option<CustomerDetails>,
//     pub total_amount: f64,
//     pub payment_status: PaymentStatus,
// }

// #[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
// pub struct TransactionHistory {
//     pub booking_id: u64,
//     pub car_id: u64,
//     pub customer_principal_id: Principal,
//     pub customer: Option<CustomerDetails>,
//     pub start_timestamp: String, // Unix timestamp
//     pub end_timestamp: String,   // Unix timestamp
//     pub total_amount: f64,
//     pub payment_status: PaymentStatus,
// }

// impl RentalTransaction {

//     pub fn to_transaction_history(&self) -> TransactionHistory {
//         TransactionHistory {
//             booking_id: self.booking_id, 
//             car_id: self.car_id, 
//             customer_principal_id: self.customer_principal_id.clone(), 
//             customer: self.customer.clone(),
//             start_timestamp: format_datetime(self.start_timestamp),
//             end_timestamp: format_datetime(self.end_timestamp),
//             total_amount: self.total_amount,
//             payment_status: self.payment_status.clone()
//         }
//     }

// }

// #[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
// pub struct CustomerDetails {
//     pub name: String,
//     pub email: String,
//     pub country_code: String,
//     pub mobile_number: String,
//     pub age: u8,
//     pub pan: String, 
//     pub aadhar: String,
// }

// impl CustomerDetails {
//     pub fn validate_details(&self) -> Result<(), String> {
//         if self.name.trim().len() < 3 {return  Err("Invalid Name, please provide a name with more than 4 characters.".into()) ;}
//         if self.email.trim().len() < 5 {return  Err("Invalid email, please provide a valid email adress".into()) ;}
//         if self.country_code.trim().len() != 2  {return  Err("Invalid country code, please provide a valid country code".into()) ;}
//         if self.mobile_number.trim().len() != 10  {return  Err("Invalid mobile number, please provide a 10 digits mobile number".into()) ;}
//         if (self.pan.trim().is_empty() || self.pan.trim().len() < 10) && (self.aadhar.trim().is_empty() || self.aadhar.trim().len() != 12)  {return  Err("Invalid documents, please provide a PAN or Aadhar".into()) ;}
//         if self.age < 18  {return  Err("Invalid age, age should be atleast 18".into()) ;}
//         Ok(())
//     }
// }

// #[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
// pub struct Customer {
//     pub principal: Principal,
//     pub name: String,
//     pub email: String,
//     pub phone_number: String,
//     pub id_type: Option<IdType>,
// }

// #[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
// pub enum IdType {
//     Aadhar(String),
//     PAN(String),
// }
