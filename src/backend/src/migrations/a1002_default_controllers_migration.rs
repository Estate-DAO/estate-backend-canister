use crate::migration::Migration;
use crate::CanisterState;
use candid::Principal;

const DEFAULT_CONTROLLERS: &[&str] = &[
    "znxnh-f2v3a-dwwtd-jhyr2-tzdg7-xrm43-xf6xc-q4nfk-arhfb-54k5n-zae",
    "krnjz-sqido-sjhrl-jprhc-g6ezq-xidpq-futvc-3mwtc-cpee4-z3cls-wqe"
];


pub struct AddDefaultControllersMigration;

impl Migration for AddDefaultControllersMigration {
    fn version(&self) -> u64 {
        1002
    }

    fn description(&self) -> &str {
        "Add default controllers to the canister if not already set"
    }

    fn migrate_up(&self, state: &mut CanisterState) -> Result<(), String> {
        // Parse all default controllers
        let default_principals: Result<Vec<Principal>, String> = DEFAULT_CONTROLLERS
            .iter()
            .enumerate()
            .map(|(i, controller_str)| {
                Principal::from_text(controller_str)
                    .map_err(|e| format!("Failed to parse default controller {} principal: {}", i + 1, e))
            })
            .collect();
        
        let default_principals = default_principals?;
        
        // Only set default controllers if controllers field is None or empty
        match &state.controllers {
            None => {
                // Set default controllers
                state.controllers = Some(default_principals);
                ic_cdk::println!("AddDefaultControllersMigration: Initialized with {} default controllers", DEFAULT_CONTROLLERS.len());
            }
            Some(controllers) if controllers.is_empty() => {
                // Controllers list exists but is empty - add default controllers
                state.controllers = Some(default_principals);
                ic_cdk::println!("AddDefaultControllersMigration: Added {} default controllers to empty list", DEFAULT_CONTROLLERS.len());
            }
            Some(controllers) => {
                // Controllers already exist with values - no migration needed
                ic_cdk::println!("AddDefaultControllersMigration: Controllers already exist with {} entries", controllers.len());
            }
        }
        
        Ok(())
    }

    fn migrate_down(&self, state: &mut CanisterState) -> Result<(), String> {
        // Parse all default controllers
        let default_principals: Result<Vec<Principal>, String> = DEFAULT_CONTROLLERS
            .iter()
            .enumerate()
            .map(|(i, controller_str)| {
                Principal::from_text(controller_str)
                    .map_err(|e| format!("Failed to parse default controller {} principal: {}", i + 1, e))
            })
            .collect();
        
        let default_principals = default_principals?;
        
        // For rollback, remove default controllers if they are the only ones
        if let Some(ref controllers) = state.controllers {
            if controllers.len() == default_principals.len() 
                && default_principals.iter().all(|p| controllers.contains(p)) {
                state.controllers = None;
                ic_cdk::println!("AddDefaultControllersMigration rollback: Removed default controllers, reset to None");
            } else {
                ic_cdk::println!("AddDefaultControllersMigration rollback: Controllers have other data, keeping as is");
            }
        }
        
        Ok(())
    }

    // in migrations, we want to be using old fields for migration purpose. so clippy is allowed.
    fn validate(&self, state: &CanisterState) -> Result<(), String> {
        // Validate that controllers field is properly initialized
        match &state.controllers {
            None => {
                return Err("Validation failed: controllers field should be initialized after migration".to_string());
            }
            Some(controllers) => {
                // Validate that all entries are valid principals
                for (index, controller) in controllers.iter().enumerate() {
                    // Basic validation - ensure principal is not anonymous
                    if *controller == Principal::anonymous() {
                        return Err(format!(
                            "Validation failed: controller at index {} is anonymous principal",
                            index
                        ));
                    }
                }
                
                ic_cdk::println!("AddDefaultControllersMigration validation: controllers field properly initialized with {} entries", controllers.len());
            }
        }
        
        Ok(())
    }
}