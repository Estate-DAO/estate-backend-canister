#[cfg(test)]
mod payment_id_index_tests {
    use crate::models::*;
    use crate::rebuild_payment_id_index;
    use std::collections::BTreeMap;

    fn create_test_booking_with_payment_id_v2(
        app_ref: &str, 
        email: &str, 
        payment_id_v2: &str
    ) -> (BookingId, Booking) {
        let booking_id = BookingId::new(app_ref.to_string(), email.to_string());
        let mut payment_details = PaymentDetails::new(booking_id.clone());
        payment_details.payment_api_response.payment_id_v2 = payment_id_v2.to_string();
        payment_details.payment_api_response.provider = "test_provider".to_string();
        
        let booking = Booking {
            booking_id: booking_id.clone(),
            guests: UserDetails::default(),
            book_room_status: None,
            user_selected_hotel_room_details: HotelRoomDetails::default(),
            payment_details,
        };
        
        (booking_id, booking)
    }

    fn create_test_state_with_multiple_bookings() -> CanisterState {
        let mut state = CanisterState::new();
        
        // Create multiple bookings with different payment_id_v2 values
        let (booking_id1, booking1) = create_test_booking_with_payment_id_v2(
            "APP001", "user1@example.com", "payment_abc123"
        );
        let (booking_id2, booking2) = create_test_booking_with_payment_id_v2(
            "APP002", "user1@example.com", "payment_xyz789"
        );
        let (booking_id3, booking3) = create_test_booking_with_payment_id_v2(
            "APP003", "user2@example.com", "payment_def456"
        );
        
        // Add bookings for user1
        let mut user1_bookings = BTreeMap::new();
        user1_bookings.insert(booking_id1, booking1);
        user1_bookings.insert(booking_id2, booking2);
        
        let user1_info = UserInfoAndBookings {
            primary_user: AdultDetail::default(),
            bookings: user1_bookings,
        };
        
        // Add booking for user2
        let mut user2_bookings = BTreeMap::new();
        user2_bookings.insert(booking_id3, booking3);
        
        let user2_info = UserInfoAndBookings {
            primary_user: AdultDetail::default(),
            bookings: user2_bookings,
        };
        
        state.users.insert("user1@example.com".to_string(), user1_info);
        state.users.insert("user2@example.com".to_string(), user2_info);
        
        state
    }

    #[test]
    fn test_rebuild_payment_id_index_empty_state() {
        let state = CanisterState::new();
        
        // Manually set the global state for testing
        crate::STATE.with(|s| {
            *s.borrow_mut() = state;
        });
        
        rebuild_payment_id_index();
        
        crate::STATE.with(|s| {
            let state = s.borrow();
            assert!(state.payment_id_index.is_some());
            assert!(state.payment_id_index.as_ref().unwrap().is_empty());
        });
    }

    #[test]
    fn test_rebuild_payment_id_index_with_bookings() {
        let state = create_test_state_with_multiple_bookings();
        
        // Set the global state
        crate::STATE.with(|s| {
            *s.borrow_mut() = state;
        });
        
        rebuild_payment_id_index();
        
        crate::STATE.with(|s| {
            let state = s.borrow();
            let index = state.payment_id_index.as_ref().unwrap();
            
            // Verify all 3 payment_id_v2 values are indexed
            assert_eq!(index.len(), 3);
            
            // Verify specific mappings
            let booking_id1 = BookingId::new("APP001".to_string(), "user1@example.com".to_string());
            let booking_id2 = BookingId::new("APP002".to_string(), "user1@example.com".to_string());
            let booking_id3 = BookingId::new("APP003".to_string(), "user2@example.com".to_string());
            
            assert_eq!(index.get("payment_abc123"), Some(&booking_id1));
            assert_eq!(index.get("payment_xyz789"), Some(&booking_id2));
            assert_eq!(index.get("payment_def456"), Some(&booking_id3));
        });
    }

    #[test]
    fn test_rebuild_payment_id_index_skips_empty_payment_id_v2() {
        let mut state = CanisterState::new();
        
        // Create booking with empty payment_id_v2
        let (booking_id, mut booking) = create_test_booking_with_payment_id_v2(
            "APP001", "user@example.com", ""
        );
        
        // Ensure payment_id_v2 is empty
        booking.payment_details.payment_api_response.payment_id_v2 = "".to_string();
        
        let mut user_bookings = BTreeMap::new();
        user_bookings.insert(booking_id, booking);
        
        let user_info = UserInfoAndBookings {
            primary_user: AdultDetail::default(),
            bookings: user_bookings,
        };
        
        state.users.insert("user@example.com".to_string(), user_info);
        
        crate::STATE.with(|s| {
            *s.borrow_mut() = state;
        });
        
        rebuild_payment_id_index();
        
        crate::STATE.with(|s| {
            let state = s.borrow();
            let index = state.payment_id_index.as_ref().unwrap();
            
            // Index should be empty since payment_id_v2 was empty
            assert!(index.is_empty());
        });
    }

    #[test]
    fn test_rebuild_payment_id_index_doesnt_rebuild_if_exists() {
        let state = create_test_state_with_multiple_bookings();
        
        crate::STATE.with(|s| {
            let mut state_ref = s.borrow_mut();
            *state_ref = state;
            
            // Manually create a non-empty index
            let mut existing_index = BTreeMap::new();
            existing_index.insert("existing_key".to_string(), 
                BookingId::new("EXISTING".to_string(), "existing@example.com".to_string()));
            state_ref.payment_id_index = Some(existing_index);
        });
        
        rebuild_payment_id_index();
        
        crate::STATE.with(|s| {
            let state = s.borrow();
            let index = state.payment_id_index.as_ref().unwrap();
            
            // Should still have the existing key and not rebuild
            assert_eq!(index.len(), 1);
            assert!(index.contains_key("existing_key"));
            assert!(!index.contains_key("payment_abc123"));
        });
    }

    #[test]
    fn test_update_payment_details_updates_index() {
        let mut state = create_test_state_with_multiple_bookings();
        
        // Initialize the index
        state.payment_id_index = Some(BTreeMap::new());
        
        let booking_id = BookingId::new("APP001".to_string(), "user1@example.com".to_string());
        let mut payment_details = PaymentDetails::new(booking_id.clone());
        payment_details.payment_api_response.payment_id_v2 = "new_payment_id_123".to_string();
        
        let result = state.update_payment_details(booking_id.clone(), payment_details);
        assert!(result.is_ok());
        
        // Verify index was updated
        let index = state.payment_id_index.as_ref().unwrap();
        assert_eq!(index.get("new_payment_id_123"), Some(&booking_id));
    }

    #[test]
    fn test_update_payment_details_prevents_duplicate_payment_id_v2() {
        let mut state = create_test_state_with_multiple_bookings();
        
        // Initialize the index and rebuild it
        state.payment_id_index = Some(BTreeMap::new());
        rebuild_payment_id_index_for_state(&mut state);
        
        // Try to update a different booking with an existing payment_id_v2
        let booking_id = BookingId::new("APP002".to_string(), "user1@example.com".to_string());
        let mut payment_details = PaymentDetails::new(booking_id.clone());
        payment_details.payment_api_response.payment_id_v2 = "payment_abc123".to_string(); // Already used by APP001
        
        let result = state.update_payment_details(booking_id, payment_details);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already used by other booking"));
    }

    #[test]
    fn test_update_payment_details_allows_same_booking_same_payment_id_v2() {
        let mut state = create_test_state_with_multiple_bookings();
        
        // Initialize the index and rebuild it
        state.payment_id_index = Some(BTreeMap::new());
        rebuild_payment_id_index_for_state(&mut state);
        
        // Update the same booking with the same payment_id_v2 should be allowed
        let booking_id = BookingId::new("APP001".to_string(), "user1@example.com".to_string());
        let mut payment_details = PaymentDetails::new(booking_id.clone());
        payment_details.payment_api_response.payment_id_v2 = "payment_abc123".to_string(); // Same as existing
        
        let result = state.update_payment_details(booking_id, payment_details);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_payment_details_removes_old_index_entry() {
        let mut state = create_test_state_with_multiple_bookings();
        
        // Initialize the index and rebuild it
        state.payment_id_index = Some(BTreeMap::new());
        rebuild_payment_id_index_for_state(&mut state);
        
        let booking_id = BookingId::new("APP001".to_string(), "user1@example.com".to_string());
        
        // Verify old payment_id_v2 is in index
        assert!(state.payment_id_index.as_ref().unwrap().contains_key("payment_abc123"));
        
        // Update with new payment_id_v2
        let mut payment_details = PaymentDetails::new(booking_id.clone());
        payment_details.payment_api_response.payment_id_v2 = "new_payment_xyz".to_string();
        
        let result = state.update_payment_details(booking_id.clone(), payment_details);
        assert!(result.is_ok());
        
        let index = state.payment_id_index.as_ref().unwrap();
        
        // Old payment_id_v2 should be removed
        assert!(!index.contains_key("payment_abc123"));
        
        // New payment_id_v2 should be added
        assert_eq!(index.get("new_payment_xyz"), Some(&booking_id));
    }

    #[test]
    fn test_update_payment_details_rejects_empty_payment_id_v2() {
        let mut state = create_test_state_with_multiple_bookings();
        state.payment_id_index = Some(BTreeMap::new());
        
        let booking_id = BookingId::new("APP001".to_string(), "user1@example.com".to_string());
        let mut payment_details = PaymentDetails::new(booking_id);
        payment_details.payment_api_response.payment_id_v2 = "".to_string(); // Empty
        
        let result = state.update_payment_details(
            BookingId::new("APP001".to_string(), "user1@example.com".to_string()),
            payment_details
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_get_booking_id_by_payment_id_v2_query() {
        let state = create_test_state_with_multiple_bookings();
        
        crate::STATE.with(|s| {
            let mut state_ref = s.borrow_mut();
            *state_ref = state;
            
            // Initialize and rebuild index
            state_ref.payment_id_index = Some(BTreeMap::new());
            rebuild_payment_id_index_for_state(&mut state_ref);
        });
        
        // Test the query function
        let result = crate::get_booking_id_by_payment_id_v2("payment_abc123".to_string());
        let expected_booking_id = BookingId::new("APP001".to_string(), "user1@example.com".to_string());
        assert_eq!(result, Some(expected_booking_id));
        
        // Test non-existent payment_id_v2
        let result = crate::get_booking_id_by_payment_id_v2("non_existent".to_string());
        assert_eq!(result, None);
    }

    // Helper function to rebuild index for a specific state (for testing)
    fn rebuild_payment_id_index_for_state(state: &mut CanisterState) {
        if let Some(ref payment_index) = state.payment_id_index {
            if !payment_index.is_empty() {
                return;
            }
        }

        let mut payment_mappings = Vec::new();
        for user_info in state.users.values() {
            for booking in user_info.bookings.values() {
                let payment_id_v2 = &booking.payment_details.payment_api_response.payment_id_v2;
                if !payment_id_v2.is_empty() {
                    payment_mappings.push((payment_id_v2.clone(), booking.booking_id.clone()));
                }
            }
        }

        if state.payment_id_index.is_none() {
            state.payment_id_index = Some(BTreeMap::new());
        }

        if let Some(ref mut payment_index) = state.payment_id_index {
            payment_index.clear(); // Clear for rebuild
            for (payment_id, booking_id) in payment_mappings {
                payment_index.insert(payment_id, booking_id);
            }
        }
    }
}