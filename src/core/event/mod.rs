mod proxy;

pub mod events;
mod registry;

pub use proxy::*;
pub use registry::*;

#[sealed]
pub trait EventMarker {}
