use super::super::{Skip, Step};
use super::{Collection, Scope, __seal_collection, __seal_scope};
use crate::core::controller::Controller;
use serenity::http::client::Http;
use std::borrow::Cow;
use std::collections::{hash_map::IntoIter, HashMap};

#[sealed]
#[async_trait]
impl Scope for Controller {
    async fn try_skip(&self, http: &Http) -> Skip {
        if self.is_connected(http).await {
            return Skip::Proceed;
        }

        // TODO: actual error message here.
        println!("Not connected to {self}!");
        Skip::Skip
    }

    fn suffix(&self) -> Cow<str> {
        self.to_string().into()
    }
}

/// Collection for setup steps that depend on being added under a specific controller.
/// Implemented as a map of controller to step instance.
#[derive(Debug)]
pub struct ControllerCollection<S: Step>(HashMap<Controller, S>);

#[sealed]
impl<S: Step<Collection = Self>> Collection<S> for ControllerCollection<S> {
    type Scope = Controller;

    fn factory(&mut self, scope: Self::Scope) -> &mut S {
        self.0.entry(scope).or_insert_with(|| S::new(scope))
    }
}

impl<S: Step> Default for ControllerCollection<S> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl<S: Step> IntoIterator for ControllerCollection<S> {
    type Item = (Controller, S);
    type IntoIter = IntoIter<Controller, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
