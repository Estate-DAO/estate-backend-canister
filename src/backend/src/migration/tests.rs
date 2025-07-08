#[cfg(test)]
mod migration_engine_tests {
    use crate::migration::{Migration, MigrationEngine, SchemaMetadata};
    use crate::models::*;
    use std::collections::BTreeMap;

    fn create_test_state() -> CanisterState {
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
    fn test_schema_metadata_default() {
        let metadata = SchemaMetadata::default();
        assert_eq!(metadata.current_version, 1000);
        assert!(metadata.applied_migrations.is_empty());
        assert!(metadata.target_version.is_none());
    }

    #[test]
    fn test_migration_engine_new() {
        let engine = MigrationEngine::new();
        assert_eq!(engine.migrations.len(), 1); // Should have AddPaymentIdV2Migration
    }

    #[test]
    fn test_migration_engine_no_pending_migrations() {
        let engine = MigrationEngine::new();
        let mut state = create_test_state();
        state.schema_metadata.current_version = 2000; // Higher than any migration version
        
        let result = engine.apply_migrations(&mut state);
        assert!(result.is_ok());
        assert_eq!(state.schema_metadata.current_version, 2000); // Should remain unchanged
    }

    #[test]
    fn test_get_pending_migrations() {
        let engine = MigrationEngine::new();
        let state = create_test_state(); // Default version is 1000
        
        let pending = engine.get_pending_migrations(&state);
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].version(), 1001);
    }

    #[test]
    fn test_get_applied_migrations() {
        let engine = MigrationEngine::new();
        let mut state = create_test_state();
        
        // Apply migrations
        let result = engine.apply_migrations(&mut state);
        assert!(result.is_ok());
        
        let applied = engine.get_applied_migrations(&state);
        assert_eq!(applied.len(), 1);
        assert_eq!(applied[0].version, 1001);
        assert_eq!(applied[0].description, "Add payment_id_v2 field and migrate existing payment_id data");
    }

    #[test]
    fn test_migration_engine_add_migration() {
        let mut engine = MigrationEngine::new();
        let initial_count = engine.migrations.len();
        
        // Create a test migration
        struct TestMigration;
        impl Migration for TestMigration {
            fn version(&self) -> u64 { 1002 }
            fn description(&self) -> &str { "Test migration" }
            fn migrate_up(&self, _state: &mut CanisterState) -> Result<(), String> { Ok(()) }
            fn migrate_down(&self, _state: &mut CanisterState) -> Result<(), String> { Ok(()) }
            fn validate(&self, _state: &CanisterState) -> Result<(), String> { Ok(()) }
        }
        
        engine.add_migration(Box::new(TestMigration));
        assert_eq!(engine.migrations.len(), initial_count + 1);
    }

    #[test]
    fn test_canister_state_with_schema_metadata() {
        let state = CanisterState::new();
        assert_eq!(state.schema_metadata.current_version, 1000);
        assert!(state.schema_metadata.applied_migrations.is_empty());
    }
}