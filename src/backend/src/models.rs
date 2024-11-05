use std::collections::BTreeMap;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Default, Serialize, Clone, Debug)]
pub struct CanisterState {
    pub user_details: UserDetails,
    pub booking_details: UserBookingDetails ,
    pub payment_details: PaymentDetails,
}


#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct UserBookingDetails{
    pub book_room_response: Option<BookRoomResponse>,
    pub user_selected_hotel_details: HotelRoomDetails
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct HotelRoomDetails{
    hotel_details: HotelDetails,
    start_date: (u32,u32,u32),
    end_date: (u32,u32,u32) ,
    destination: Destination,
    room_details: Vec<RoomDetails>,
    /// shown to the user at block_room API call
    requested_payment_amount: f64
}


#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct RoomDetails{
    room_name: String,
    room_unique_id: String,
}


#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct HotelDetails {
    pub name: String,
    pub hotel_code: String,
}

/// Payment -- status "to payment gateway?"
/// payment_provider -> "TX_NUMBER_13434" /  "PAYMENT_FAILED"


#[derive(Clone, Default, Debug)]
pub struct UserDetails {
    pub adults: Vec<AdultDetail>,
    pub children: Vec<ChildDetail>,
    pub terms_accepted: bool,
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
    pub age: Option<u8>,
}
 


#[derive(CandidType,Serialize, Deserialize, Debug, Clone)]
pub struct BookRoomResponse {
    #[serde(rename = "Status")]
    pub status: BookingStatus,

    #[serde(rename = "Message")]
    pub message: Option<String>,

    #[serde(rename = "CommitBooking")]
    pub commit_booking: Vec<BookingDetails>,
}

#[derive(CandidType,Serialize, Deserialize, Debug, Clone)]
pub struct BookingDetails {
    pub booking_id: String,

    pub booking_ref_no: String,

    pub confirmation_no: String,

    pub booking_status: String,
}

#[derive(CandidType,Serialize, Deserialize, Debug, Clone)]
pub enum BookingStatus {
    #[serde(rename = "BookFailed")]
    BookFailed = 0,
    #[serde(rename = "Confirmed")]
    Confirmed = 1,
}
 

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct PaymentDetails {
    pub booking_id: String,
    pub payment_status: PaymentStatus,
 }

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum PaymentStatus {
    /// transaction reference number from payments provider
    Paid(String),
    /// if the transaction failed, that would be here.
    Unpaid(Option<String>),
}
    

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
 
