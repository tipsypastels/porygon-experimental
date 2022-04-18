//! The Porygon2 application.
//!
//! Contains all the actual commands, handlers,
//! and features, with `core` acting as the
//! backend for all of it.
//!
//! Add modules for each feature and do not re-export anything,
//! just stick it in `installer` as the sole entrypoint.
//!
//! Some app modules may access each others exports, but nothing
//! but `installer` should be accessed by the outside.

use crate::core::setup::Setup;

mod activity;

/// Installs all of the myriad setup steps needed for the application.
pub fn installer(setup: Setup) -> Setup {
    setup.add_from(activity::installer)
}
