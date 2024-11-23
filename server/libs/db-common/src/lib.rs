pub mod db;

pub use db::*;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();
