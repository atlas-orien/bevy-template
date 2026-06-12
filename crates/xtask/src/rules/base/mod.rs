//! Reusable architecture rule primitives.
//!
//! Files in this module should define small, semantic checks that crate-specific
//! rule modules can compose.

pub mod dependencies;
pub mod derives;
pub mod functions;
pub mod paths;
pub mod profiles;
pub mod source;
