//! Module that contains service modules used by the application.
//!
//! They are divided in primary and secondary services.
//!
//! - Primary services define the application logic and are used by
//!   facades to serve requests to the client.
//!
//! - Secondary services are used to support the primary services
//!   like access control and database.

pub mod access_control;
pub mod database;
pub mod user;
