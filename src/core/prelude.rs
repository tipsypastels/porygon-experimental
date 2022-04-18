//! The core prelude. Commonly used by `app` modules to
//! grab the most common stuff easily.
//!
//! `core` modules should preferably avoid depending on
//! this themselves, and just import things directly.

pub use super::controller::*;
pub use super::init::{Init, InitArgs};
pub use super::setup::Setup;
