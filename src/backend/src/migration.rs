use candid::CandidType;
use serde::{Deserialize, Serialize};
use ic_cdk::api::time;
use crate::{ CanisterState};
// use crate::{migrations::AddPaymentIdV2Migration, CanisterState};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct SchemaVersion {
    pub version: u64,
    pub applied_at: u64,
    pub description: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct SchemaMetadata {
    pub current_version: u64,
    pub applied_migrations: Vec<SchemaVersion>,
    pub target_version: Option<u64>,
}

impl Default for SchemaMetadata {
    fn default() -> Self {
        Self {
            current_version: 1000,
            applied_migrations: Vec::new(),
            target_version: None,
        }
    }
}

pub trait Migration: Send + Sync {
    fn version(&self) -> u64;
    fn description(&self) -> &str;
    fn migrate_up(&self, state: &mut CanisterState) -> Result<(), String>;
    fn migrate_down(&self, state: &mut CanisterState) -> Result<(), String>;
    fn validate(&self, state: &CanisterState) -> Result<(), String>;
}

pub struct MigrationEngine {
    migrations: Vec<Box<dyn Migration>>,
}

impl MigrationEngine {
    pub fn new() -> Self {
        Self {
            migrations: vec![
                // Box::new(AddPaymentIdV2Migration),
            ],
        }
    }

    // fn ensure_schema_metadata<'a>(&self, state: &'a mut CanisterState) -> &'a mut SchemaMetadata {
    //     if state.schema_metadata.is_none() {
    //         state.schema_metadata = Some(SchemaMetadata::default());
    //     }
    //     state.schema_metadata.as_mut().unwrap()
    // }

    // pub fn add_migration(&mut self, migration: Box<dyn Migration>) {
    //     self.migrations.push(migration);
    //     // Sort migrations by version to ensure correct order
    //     self.migrations.sort_by(|a, b| a.version().cmp(&b.version()));
    // }

    // pub fn apply_migrations(&self, state: &mut CanisterState) -> Result<(), String> {
    //     let schema_metadata = self.ensure_schema_metadata(state);
    //     let current_version = schema_metadata.current_version;
        
    //     let unapplied: Vec<_> = self.migrations
    //         .iter()
    //         .filter(|m| m.version() > current_version)
    //         .collect();

    //     if unapplied.is_empty() {
    //         return Ok(());
    //     }

    //     for migration in unapplied {
    //         // Apply migration
    //         migration.migrate_up(state)?;
            
    //         // Validate migration result
    //         migration.validate(state)?;
            
    //         // Record successful migration
    //         let schema_metadata = self.ensure_schema_metadata(state);
    //         schema_metadata.applied_migrations.push(SchemaVersion {
    //             version: migration.version(),
    //             applied_at: time(),
    //             description: migration.description().to_string(),
    //         });
            
    //         // Update current version
    //         schema_metadata.current_version = migration.version();
    //     }
        
    //     Ok(())
    // }

    // pub fn rollback_to_version(&self, state: &mut CanisterState, target_version: u64) -> Result<(), String> {
    //     let schema_metadata = self.ensure_schema_metadata(state);
    //     let current_version = schema_metadata.current_version;
        
    //     if target_version >= current_version {
    //         return Err("Target version must be lower than current version".to_string());
    //     }

    //     let to_rollback: Vec<_> = self.migrations
    //         .iter()
    //         .filter(|m| m.version() > target_version && m.version() <= current_version)
    //         .collect();

    //     // Rollback in reverse order
    //     for migration in to_rollback.iter().rev() {
    //         migration.migrate_down(state)?;
            
    //         // Remove from applied migrations
    //         let schema_metadata = self.ensure_schema_metadata(state);
    //         schema_metadata.applied_migrations.retain(|v| v.version != migration.version());
    //     }
        
    //     // Update current version
    //     let schema_metadata = self.ensure_schema_metadata(state);
    //     schema_metadata.current_version = target_version;
        
    //     Ok(())
    // }

    // pub fn get_pending_migrations(&self, state: &CanisterState) -> Vec<&dyn Migration> {
    //     let current_version = state.schema_metadata
    //         .as_ref()
    //         .map(|s| s.current_version)
    //         .unwrap_or(SchemaMetadata::default().current_version);
        
    //     self.migrations
    //         .iter()
    //         .filter(|m| m.version() > current_version)
    //         .map(|m| m.as_ref())
    //         .collect()
    // }

    // pub fn get_applied_migrations(&self, state: &CanisterState) -> Vec<SchemaVersion> {
    //     state.schema_metadata
    //         .as_ref()
    //         .map(|s| s.applied_migrations.clone())
    //         .unwrap_or_default()
    // }
}

impl Default for MigrationEngine {
    fn default() -> Self {
        Self::new()
    }
}
