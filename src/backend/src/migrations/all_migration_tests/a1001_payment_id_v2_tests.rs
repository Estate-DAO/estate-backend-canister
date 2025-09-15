
mod payment_id_v2_tests {
    use crate::migrations::AddPaymentIdV2Migration;
    use crate::models::*;
    use crate::migration::Migration;
    use std::collections::BTreeMap;

    fn create_test_state_with_payment() -> CanisterState {
        let mut state = CanisterState::new();
        
        // Create test booking with payment details
        let booking_id = BookingId::new("TEST123".to_string(), "test@example.com".to_string());
        let mut payment_details = PaymentDetails::new(booking_id.clone());
        payment_details.payment_api_response.payment_id = 12345;
        payment_details.payment_api_response.payment_id_v2 = "".to_string(); // Empty initially
        
        let booking = Booking {
            booking_id: booking_id.clone(),
            guests: UserDetails::default(),
            book_room_status: None,
            user_selected_hotel_room_details: HotelRoomDetails::default(),
            payment_details,
        };
        
        let mut user_bookings = BTreeMap::new();
        user_bookings.insert(booking_id.clone(), booking);
        
        let user_info = UserInfoAndBookings {
            primary_user: AdultDetail::default(),
            bookings: user_bookings,
        };
        
        state.users.insert("test@example.com".to_string(), user_info);
        state
    }

    #[test]
    fn test_payment_id_v2_migration_version() {
        let migration = AddPaymentIdV2Migration;
        assert_eq!(migration.version(), 1001);
    }

   

    #[test]
    fn test_payment_id_v2_migration_up() {
        let migration = AddPaymentIdV2Migration;
        let mut state = create_test_state_with_payment();
        
        // Ensure payment_id_v2 is empty before migration
        let booking = state.users.get("test@example.com").unwrap()
            .bookings.values().next().unwrap();
        assert_eq!(booking.payment_details.payment_api_response.payment_id_v2, "");
        
        // Apply migration
        let result = migration.migrate_up(&mut state);
        assert!(result.is_ok());
        
        // Verify migration result
        let booking = state.users.get("test@example.com").unwrap()
            .bookings.values().next().unwrap();
        assert_eq!(booking.payment_details.payment_api_response.payment_id_v2, "12345");
    }

    // #[test]
    // fn test_payment_id_v2_migration_down() {
    //     let migration = AddPaymentIdV2Migration;
    //     let mut state = create_test_state_with_payment();
        
    //     // First apply migration
    //     let _ = migration.migrate_up(&mut state);
        
    //     // Verify payment_id_v2 is set
    //     let booking = state.users.get("test@example.com").unwrap()
    //         .bookings.values().next().unwrap();
    //     assert_eq!(booking.payment_details.payment_api_response.payment_id_v2, "12345");
        
    //     // Apply rollback
    //     let result = migration.migrate_down(&mut state);
    //     assert!(result.is_ok());
        
    //     // Verify payment_id_v2 is cleared
    //     let booking = state.users.get("test@example.com").unwrap()
    //         .bookings.values().next().unwrap();
    //     assert_eq!(booking.payment_details.payment_api_response.payment_id_v2, "");
    // }

    // #[test]
    // fn test_payment_id_v2_migration_validate_success() {
    //     let migration = AddPaymentIdV2Migration;
    //     let mut state = create_test_state_with_payment();
        
    //     // Apply migration
    //     let _ = migration.migrate_up(&mut state);
        
    //     // Validation should pass
    //     let result = migration.validate(&state);
    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn test_payment_id_v2_migration_validate_failure() {
    //     let migration = AddPaymentIdV2Migration;
    //     let mut state = create_test_state_with_payment();
        
    //     // Don't apply migration, so payment_id_v2 remains empty
    //     // This should fail validation
    //     let result = migration.validate(&state);
    //     assert!(result.is_err());
    //     assert!(result.unwrap_err().contains("missing payment_id_v2"));
    // }

    #[test]
    fn test_payment_id_v2_migration_skip_zero_payment_id() {
        let migration = AddPaymentIdV2Migration;
        let mut state = CanisterState::new();
        
        // Create booking with zero payment_id
        let booking_id = BookingId::new("TEST123".to_string(), "test@example.com".to_string());
        let mut payment_details = PaymentDetails::new(booking_id.clone());
        payment_details.payment_api_response.payment_id = 0; // Zero payment_id
        payment_details.payment_api_response.payment_id_v2 = "".to_string();
        
        let booking = Booking {
            booking_id: booking_id.clone(),
            guests: UserDetails::default(),
            book_room_status: None,
            user_selected_hotel_room_details: HotelRoomDetails::default(),
            payment_details,
        };
        
        let mut user_bookings = BTreeMap::new();
        user_bookings.insert(booking_id.clone(), booking);
        
        let user_info = UserInfoAndBookings {
            primary_user: AdultDetail::default(),
            bookings: user_bookings,
        };
        
        state.users.insert("test@example.com".to_string(), user_info);
        
        // Apply migration
        let result = migration.migrate_up(&mut state);
        assert!(result.is_ok());
        
        // Verify payment_id_v2 remains empty (no migration for zero payment_id)
        let booking = state.users.get("test@example.com").unwrap()
            .bookings.values().next().unwrap();
        assert_eq!(booking.payment_details.payment_api_response.payment_id_v2, "");
    }

    #[test]
    fn test_payment_id_v2_migration_skip_existing_payment_id_v2() {
        let migration = AddPaymentIdV2Migration;
        let mut state = CanisterState::new();
        
        // Create booking with existing payment_id_v2
        let booking_id = BookingId::new("TEST123".to_string(), "test@example.com".to_string());
        let mut payment_details = PaymentDetails::new(booking_id.clone());
        payment_details.payment_api_response.payment_id = 12345;
        payment_details.payment_api_response.payment_id_v2 = "existing_value".to_string();
        
        let booking = Booking {
            booking_id: booking_id.clone(),
            guests: UserDetails::default(),
            book_room_status: None,
            user_selected_hotel_room_details: HotelRoomDetails::default(),
            payment_details,
        };
        
        let mut user_bookings = BTreeMap::new();
        user_bookings.insert(booking_id.clone(), booking);
        
        let user_info = UserInfoAndBookings {
            primary_user: AdultDetail::default(),
            bookings: user_bookings,
        };
        
        state.users.insert("test@example.com".to_string(), user_info);
        
        // Apply migration
        let result = migration.migrate_up(&mut state);
        assert!(result.is_ok());
        
        // Verify payment_id_v2 remains unchanged
        let booking = state.users.get("test@example.com").unwrap()
            .bookings.values().next().unwrap();
        assert_eq!(booking.payment_details.payment_api_response.payment_id_v2, "existing_value");
    }
}