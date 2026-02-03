//! Backend library for the codegame project.
//!
//! This module exposes the backend components for use in integration tests
//! and as a library.

pub mod models;
pub mod prelude;
pub mod repositories;
pub mod routes;

pub use routes::routes;
