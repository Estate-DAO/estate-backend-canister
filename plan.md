
- store data in backend 
- expose API for leptos to consume
  - save_user_details + save_booking_details
- integrate leptos with backend
  - Form - UserDetails (Adult,child,name,age) + BookingDetails (Hotel,Room)
  - BookRoom
  - PaymentProvider
- Principal ID - backend
- Principal ID - frontend

search 
block_room
  popup - "Proceed to pay" 
  - (api.rs) backend -- UserDetails + BookingDetails + Paymentdetails
  - payments provider -> redirect


book_room
payment


1. (api.rs) backend -- UserDetails + BookingDetails + Paymentdetails
2. - payments_response
- success => book_room => book_room_response (api.rs)
- fail => update backend (api.rs)