use super::{
    events::{Queue, Unit},
    EventMarker,
};
use serenity::prelude::{TypeMap, TypeMapKey};
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct EventRegistry {
    events: Arc<Mutex<TypeMap>>,
}

impl EventRegistry {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(TypeMap::new())),
        }
    }

    pub async fn add<E>(&self, cb: fn(E) -> Box<dyn Future<Output = ()>>)
    where
        E: EventMarker,
        Unit<E>: TypeMapKey<Value = Queue<E>>,
    {
        self.events
            .lock()
            .await
            .entry::<Unit<E>>()
            .and_modify(|v| v.push(cb))
            .or_insert_with(|| vec![cb]);
    }
}

impl TypeMapKey for EventRegistry {
    type Value = Self;
}
