This is the CanisterState.


---- 

### user_details : 

#[derive(Clone, Default, Debug)]
pub struct BlockRoomCtx {
    pub adults: RwSignal<Vec<AdultDetail>>,
    pub children: RwSignal<Vec<ChildDetail>>,
    pub terms_accepted: RwSignal<bool>,
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





##  booking_details: 
- which hotel - HoteDetails{  name, hotel_code}
- which room -
- price on block_room?
 

## payment_details:
from payment provider -> map to your own schema to capture details
- payment status + booking_id 
- 
