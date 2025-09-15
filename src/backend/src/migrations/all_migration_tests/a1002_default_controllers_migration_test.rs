mod default_controllers_tests {
    use crate::migrations::AddDefaultControllersMigration;
    use crate::models::*;
    use crate::migration::Migration;
    use candid::Principal;

    fn create_test_state() -> CanisterState {
        CanisterState::new()
    }

    fn create_default_principal_1() -> Principal {
        Principal::from_text("znxnh-f2v3a-dwwtd-jhyr2-tzdg7-xrm43-xf6xc-q4nfk-arhfb-54k5n-zae").unwrap()
    }

    fn create_default_principal_2() -> Principal {
        Principal::from_text("krnjz-sqido-sjhrl-jprhc-g6ezq-xidpq-futvc-3mwtc-cpee4-z3cls-wqe").unwrap()
    }

    fn create_different_principal() -> Principal {
        Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap()
    }

    #[test]
    fn test_default_controllers_migration_version() {
        let migration = AddDefaultControllersMigration;
        assert_eq!(migration.version(), 1002);
    }

    #[test]
    fn test_default_controllers_migration_up_none() {
        let migration = AddDefaultControllersMigration;
        let mut state = create_test_state();
        let default_principal_1 = create_default_principal_1();
        let default_principal_2 = create_default_principal_2();
        
        // Ensure controllers is None initially
        assert!(state.controllers.is_none());
        
        // Apply migration
        let result = migration.migrate_up(&mut state);
        assert!(result.is_ok());
        
        // Verify controllers now contains both default controllers
        assert!(state.controllers.is_some());
        assert_eq!(state.controllers.as_ref().unwrap().len(), 2);
        assert!(state.controllers.as_ref().unwrap().contains(&default_principal_1));
        assert!(state.controllers.as_ref().unwrap().contains(&default_principal_2));
    }

    #[test]
    fn test_default_controllers_migration_up_empty() {
        let migration = AddDefaultControllersMigration;
        let mut state = create_test_state();
        let default_principal_1 = create_default_principal_1();
        let default_principal_2 = create_default_principal_2();
        
        // Set controllers to empty vec
        state.controllers = Some(Vec::new());
        
        // Apply migration
        let result = migration.migrate_up(&mut state);
        assert!(result.is_ok());
        
        // Verify controllers now contains both default controllers
        assert!(state.controllers.is_some());
        assert_eq!(state.controllers.as_ref().unwrap().len(), 2);
        assert!(state.controllers.as_ref().unwrap().contains(&default_principal_1));
        assert!(state.controllers.as_ref().unwrap().contains(&default_principal_2));
    }

    #[test]
    fn test_default_controllers_migration_up_existing() {
        let migration = AddDefaultControllersMigration;
        let mut state = create_test_state();
        
        // Set controllers with existing different values
        let different_principal = create_different_principal();
        state.controllers = Some(vec![different_principal]);
        
        // Apply migration
        let result = migration.migrate_up(&mut state);
        assert!(result.is_ok());
        
        // Verify controllers remains unchanged
        assert!(state.controllers.is_some());
        assert_eq!(state.controllers.as_ref().unwrap().len(), 1);
        assert_eq!(state.controllers.as_ref().unwrap()[0], different_principal);
    }

    #[test]
    fn test_default_controllers_migration_down_default_only() {
        let migration = AddDefaultControllersMigration;
        let mut state = create_test_state();
        let default_principal_1 = create_default_principal_1();
        let default_principal_2 = create_default_principal_2();
        
        // Set controllers to only default controllers (as if migration was applied)
        state.controllers = Some(vec![default_principal_1, default_principal_2]);
        
        // Apply rollback
        let result = migration.migrate_down(&mut state);
        assert!(result.is_ok());
        
        // Verify controllers is back to None
        assert!(state.controllers.is_none());
    }

    #[test]
    fn test_default_controllers_migration_down_with_other_data() {
        let migration = AddDefaultControllersMigration;
        let mut state = create_test_state();
        
        // Set controllers with different data
        let different_principal = create_different_principal();
        state.controllers = Some(vec![different_principal]);
        
        // Apply rollback
        let result = migration.migrate_down(&mut state);
        assert!(result.is_ok());
        
        // Verify controllers remains unchanged (has other data)
        assert!(state.controllers.is_some());
        assert_eq!(state.controllers.as_ref().unwrap().len(), 1);
        assert_eq!(state.controllers.as_ref().unwrap()[0], different_principal);
    }

    #[test]
    fn test_default_controllers_migration_validate_success_with_default() {
        let migration = AddDefaultControllersMigration;
        let mut state = create_test_state();
        
        // Apply migration first
        let _ = migration.migrate_up(&mut state);
        
        // Validation should pass with default controller
        let result = migration.validate(&state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_default_controllers_migration_validate_success_with_data() {
        let migration = AddDefaultControllersMigration;
        let mut state = create_test_state();
        
        // Set controllers with valid principal
        let different_principal = create_different_principal();
        state.controllers = Some(vec![different_principal]);
        
        // Validation should pass
        let result = migration.validate(&state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_default_controllers_migration_validate_failure_none() {
        let migration = AddDefaultControllersMigration;
        let state = create_test_state();
        
        // Don't apply migration, controllers remains None
        // Validation should fail
        let result = migration.validate(&state);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("controllers field should be initialized"));
    }

    #[test]
    fn test_default_controllers_migration_validate_failure_anonymous() {
        let migration = AddDefaultControllersMigration;
        let mut state = create_test_state();
        
        // Set controllers with anonymous principal
        state.controllers = Some(vec![Principal::anonymous()]);
        
        // Validation should fail
        let result = migration.validate(&state);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("anonymous principal"));
    }
}