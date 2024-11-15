
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

Frontend INTEGRATION

1. (api.rs) PROCEED_TO_PAY_BUTTON -> backend_call WITH UserDetails + BookingDetails + ~~Paymentdetails~~
2. - payments_response
- success => backedn_call WITH PaymentDetails
          => book_room => book_room_response (api.rs)
- fail => backend_call WITH PaymentDetails (api.rs)