use super::{Skip, Step};
use serenity::http::client::Http;
use std::borrow::Cow;
use std::hash::Hash;

mod controller;
mod unique;

pub use controller::*;
pub use unique::*;

/// A scope by which a collection of setup steps of a given type is identified.
///
/// See the documentation on `Step`.
#[sealed]
#[async_trait]
pub trait Scope: PartialEq + Eq + Hash + Send + Sync + 'static {
    /// Returns whether the step identified under this scope
    /// should be skipped during the setup process. Unconditionally
    /// returns `Proceed` by default, but `Controller` overrides it
    /// by checking if the associated guild is connected.
    async fn try_skip(&self, _http: &Http) -> Skip {
        Skip::Proceed
    }

    /// Acts as a suffix to disambiguate the name of the setup step
    /// in logging.
    fn suffix(&self) -> Cow<str>;
}

/// A collection of setup steps of a given type.
///
/// See the documentation on `Step`.
#[sealed]
pub trait Collection<R: Step<Collection = Self>>:
    Default + IntoIterator<Item = (Self::Scope, R)>
{
    /// The scope under which a given instance of the step type `S`
    /// is unique. This is either `()` or `Controller`.
    type Scope: Scope;

    /// Creates a new step instance, adds it to the internal collection
    /// of the collection type, and returns a mutable reference to it.
    ///
    /// If the step is already in the collection, it is returned
    /// unchanged at no additional cost.
    fn factory(&mut self, scope: Self::Scope) -> &mut R;
}
