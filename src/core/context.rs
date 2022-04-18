use super::event::EventRegistry;
use serenity::prelude::Context;
use serenity::prelude::TypeMapKey;

/// Extensions to the context type for common accessors.
#[async_trait]
pub trait ContextExt {
    async fn events(&self) -> EventRegistry;
}

#[async_trait]
impl ContextExt for Context {
    async fn events(&self) -> EventRegistry {
        self.data
            .write()
            .await
            .entry::<EventRegistry>()
            .or_insert_with(EventRegistry::new)
            .clone()
    }
}
