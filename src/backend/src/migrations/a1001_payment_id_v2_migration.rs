use crate::migration::Migration;
use crate::CanisterState;

pub struct AddPaymentIdV2Migration;

impl Migration for AddPaymentIdV2Migration {
    // NOTE: make sure that the version matches the filename 
    // for example
    // a1001_payment_id_v2_migration.rs should have version 1001
    // here a1001 represents the version number. this is a convention we follow while writing migrations
    fn version(&self) -> u64 {
        1001
    }

    fn description(&self) -> &str {
        "Add payment_id_v2 field and migrate existing payment_id data"
    }

    fn migrate_up(&self, state: &mut CanisterState) -> Result<(), String> {
        let mut migration_count = 0;
        
        for user in state.users.values_mut() {
            for booking in user.bookings.values_mut() {
                let payment_response = &mut booking.payment_details.payment_api_response;
                
                // Only migrate if payment_id_v2 is empty and payment_id is not zero
                if payment_response.payment_id_v2.is_empty() && payment_response.payment_id != 0 {
                    payment_response.payment_id_v2 = payment_response.payment_id.to_string();
                    migration_count += 1;
                }
            }
        }
        
        // Log successful migration
        ic_cdk::println!("AddPaymentIdV2Migration: Migrated {} payment records", migration_count);
        
        Ok(())
    }

    fn migrate_down(&self, state: &mut CanisterState) -> Result<(), String> {
        // For rollback, we can clear payment_id_v2 fields
        let mut rollback_count = 0;
        
        for user in state.users.values_mut() {
            for booking in user.bookings.values_mut() {
                let payment_response = &mut booking.payment_details.payment_api_response;
                
                if !payment_response.payment_id_v2.is_empty() {
                    payment_response.payment_id_v2 = String::new();
                    rollback_count += 1;
                }
            }
        }
        
        ic_cdk::println!("AddPaymentIdV2Migration rollback: Cleared {} payment_id_v2 fields", rollback_count);
        
        Ok(())
    }

    fn validate(&self, state: &CanisterState) -> Result<(), String> {
        let mut validation_errors = Vec::new();
        
        for (user_email, user) in state.users.iter() {
            for (booking_id, booking) in user.bookings.iter() {
                let payment_response = &booking.payment_details.payment_api_response;
                
                // Validate: if payment_id exists, payment_id_v2 should also exist
                if payment_response.payment_id != 0 && payment_response.payment_id_v2.is_empty() {
                    validation_errors.push(format!(
                        "Booking {} for user {} has payment_id {} but missing payment_id_v2",
                        booking_id.get_app_reference(),
                        user_email,
                        payment_response.payment_id
                    ));
                }
                
                // Validate: payment_id_v2 should match payment_id when both exist
                if payment_response.payment_id != 0 
                    && !payment_response.payment_id_v2.is_empty() 
                    && payment_response.payment_id_v2 != payment_response.payment_id.to_string() {
                    validation_errors.push(format!(
                        "Booking {} for user {} has mismatched payment_id ({}) and payment_id_v2 ({})",
                        booking_id.get_app_reference(),
                        user_email,
                        payment_response.payment_id,
                        payment_response.payment_id_v2
                    ));
                }
            }
        }
        
        if validation_errors.is_empty() {
            Ok(())
        } else {
            Err(format!("Validation failed: {}", validation_errors.join("; ")))
        }
    }
}