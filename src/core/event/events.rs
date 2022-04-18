use super::{EventMarker, __seal_event_marker};
use serenity::model::event;
use serenity::prelude::TypeMapKey;
use std::any::Any;
use std::future::Future;
use std::marker::PhantomData;

/// Shorthand for a vector of events for a given type.
pub(super) type Queue<T> = Vec<fn(T) -> Box<dyn Future<Output = ()>>>;

/// Newtype wrapper to allow implementing `TypeMapKey` on event types.
/// This wrapper is only used at the type level, since accessing a `TypeMap`
/// doesn't literally require the key type as a value.
pub struct Unit<T: Any>(PhantomData<T>);

macro_rules! impl_event_type {
    ($evt:ty) => {
        #[sealed]
        impl EventMarker for $evt {}

        impl TypeMapKey for Unit<$evt> {
            type Value = Queue<$evt>;
        }
    };
}

use impl_event_type;

/* -------------------------------------------------------------------------- */
/*                                 Event Types                                */
/* -------------------------------------------------------------------------- */

impl_event_type!(event::ReadyEvent);
