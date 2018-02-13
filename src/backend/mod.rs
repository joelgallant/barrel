//! A backend module which provides a few generic traits
//! to implement SQL generation for different databases.
//! 
//! It also re-exports the generators for existing databases
//! so they can be used more conveniently.

mod pg;
pub use self::pg::Pg;
use Type;

/// A generic SQL generator trait
pub trait SqlGenerator {
    
    /// Create a new table with a name
    fn create_table(name: &str) -> String;

    /// Create a new table with a name, only if it doesn't exist
    fn create_table_if_not_exists(name: &str) -> String;

    /// Drop a table with a name 
    fn drop_table(name: &str) -> String;

    /// Drop a table with a name, only if it exists
    fn drop_table_if_exists(name: &str) -> String;

    /// Rename a table from <old> to <new>
    fn rename_table(old: &str, new: &str) -> String;

    /// Modify a table in some other way
    fn alter_table(name: &str) -> String;


    /// Create a new column with a type
    fn add_column(name: &str, _type: Type) -> String;

    /// Drop an existing column from the table
    fn drop_column(name: &str) -> String;

    /// Rename an existing column
    fn rename_column(old: &str, new: &str) -> String;
}