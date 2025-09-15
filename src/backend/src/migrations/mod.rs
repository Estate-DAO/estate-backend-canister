pub mod a1001_payment_id_v2_migration;
pub use a1001_payment_id_v2_migration::*;
pub mod a1002_default_controllers_migration;
pub use a1002_default_controllers_migration::*;


#[cfg(test)]
mod all_migration_tests{
    pub mod a1001_payment_id_v2_tests;
    pub mod a1002_default_controllers_migration_test;
}
