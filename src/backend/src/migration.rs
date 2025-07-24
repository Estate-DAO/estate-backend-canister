use crate::{migrations::{AddDefaultControllersMigration, AddPaymentIdV2Migration}, CanisterState};
use candid::CandidType;
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

/// Get current timestamp, returns 0 in test environment for deterministic testing
fn get_timestamp() -> u64 {
    if cfg!(test) { 0 } else { time() }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct SchemaVersion {
    /// Migration version number
    pub version: u64,
    /// Timestamp when migration was applied
    pub applied_at: u64,
    /// Human-readable description of the migration
    pub description: String,
}

/// Schema metadata for tracking migration state and history
/// 
/// ## Migration Version System
/// - **Base Version**: 1000 (default for new canisters)
/// - **Migration Versions**: 1001, 1002, 1003, ... (incremental)
/// - **File Naming**: `a{version}_{description}.rs` (e.g., `a1001_payment_id_v2_migration.rs`)
/// 
/// ## Version Significance
/// `current_version` determines which migrations to apply:
/// - If canister is at version 1000, migrations 1001+ will be applied
/// - If canister is at version 1002, only migrations 1003+ will be applied
/// - Rollbacks decrease `current_version` and remove migration history
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct SchemaMetadata {
    /// Current schema version of the canister
    pub current_version: u64,
    /// History of all successfully applied migrations
    pub applied_migrations: Vec<SchemaVersion>,
    /// Optional target version for future migrations
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

/// Migration engine for managing schema evolution in the canister
/// 
/// ## ✅ Implemented Features
/// - **Auto-Discovery**: Automatically registers and sorts migrations by version
/// - **Sequential Application**: Applies unapplied migrations in version order
/// - **Rollback Support**: Can rollback to previous versions via `rollback_to_version()`
/// - **Validation**: Validates each migration after application
/// - **History Tracking**: Records all applied migrations with timestamps and descriptions
/// - **Pending Detection**: Identifies unapplied migrations via `get_pending_migrations()`
/// 
/// ## ❌ Not Yet Implemented
/// - **Target Version Control**: `target_version` field exists but not respected in `apply_migrations()`
/// - **Dry Run Mode**: No preview capability before applying migrations
/// - **Rollback Safeguards**: No validation of rollback safety before execution
pub struct MigrationEngine {
    migrations: Vec<Box<dyn Migration>>,
}

impl MigrationEngine {
    pub fn new() -> Self {
        Self {
            migrations: vec![
                Box::new(AddPaymentIdV2Migration),
                Box::new(AddDefaultControllersMigration),
            ],
        }
    }

    pub fn add_migration(&mut self, migration: Box<dyn Migration>) {
        self.migrations.push(migration);
        // Sort migrations by version to ensure correct order
        self.migrations.sort_by_key(|a| a.version());
    }

    pub fn apply_migrations(&self, state: &mut CanisterState) -> Result<(), String> {
        let current_version = state.schema_metadata.current_version;

        let mut unapplied: Vec<_> = self
            .migrations
            .iter()
            .filter(|m| m.version() > current_version)
            .collect();

        if unapplied.is_empty() {
            return Ok(());
        }

        // Sort unapplied migrations by version to ensure correct order
        unapplied.sort_by_key(|a| a.version());

        for migration in unapplied {
            // Apply migration
            migration.migrate_up(state)?;

            // Validate migration result
            migration.validate(state)?;

            // Record successful migration
            state.schema_metadata.applied_migrations.push(SchemaVersion {
                version: migration.version(),
                applied_at: get_timestamp(),
                description: migration.description().to_string(),
            });

            // Update current version
            state.schema_metadata.current_version = migration.version();
        }

        Ok(())
    }

    pub fn rollback_to_version(
        &self,
        state: &mut CanisterState,
        target_version: u64,
    ) -> Result<(), String> {
        let current_version = state.schema_metadata.current_version;

        if target_version >= current_version {
            return Err("Target version must be lower than current version".to_string());
        }

        let to_rollback: Vec<_> = self
            .migrations
            .iter()
            .filter(|m| m.version() > target_version && m.version() <= current_version)
            .collect();

        // Rollback in reverse order
        for migration in to_rollback.iter().rev() {
            migration.migrate_down(state)?;

            // Remove from applied migrations
            state.schema_metadata
                .applied_migrations
                .retain(|v| v.version != migration.version());
        }

        // Update current version
        state.schema_metadata.current_version = target_version;

        Ok(())
    }

    pub fn get_pending_migrations(&self, state: &CanisterState) -> Vec<&dyn Migration> {
        let current_version = state.schema_metadata.current_version;

        self.migrations
            .iter()
            .filter(|m| m.version() > current_version)
            .map(|m| m.as_ref())
            .collect()
    }

    pub fn get_applied_migrations(&self, state: &CanisterState) -> Vec<SchemaVersion> {
        state.schema_metadata.applied_migrations.clone()
    }
}

impl Default for MigrationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
