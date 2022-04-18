use super::super::Step;
use super::{Collection, Scope, __seal_collection, __seal_scope};
use std::any::type_name;
use std::borrow::Cow;

#[sealed]
impl Scope for () {
    fn suffix(&self) -> Cow<str> {
        "".into()
    }
}

/// Collection for setup steps that do not depend on controller scoping to be
/// globally unique. This is the exception, as most steps are controller-based.
///
/// Wraps an `Option<R>` internally. As a result, trying to create multiple
/// step instances in this collection will panic.
pub struct UniqueCollection<S: Step>(Option<S>);

#[sealed]
impl<S: Step<Collection = Self>> Collection<S> for UniqueCollection<S> {
    type Scope = ();

    fn factory(&mut self, scope: ()) -> &mut S {
        if self.0.is_some() {
            unreachable!(
                "Tried to insert twice into unique collection for {}!",
                type_name::<S>()
            );
        }

        self.0.insert(S::new(scope))
    }
}

impl<S: Step> Default for UniqueCollection<S> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<S: Step> IntoIterator for UniqueCollection<S> {
    type Item = ((), S);
    type IntoIter = IntoIter<S>;

    fn into_iter(self) -> IntoIter<S> {
        IntoIter(self)
    }
}

pub struct IntoIter<S: Step>(UniqueCollection<S>);

impl<S: Step> Iterator for IntoIter<S> {
    type Item = ((), S);

    fn next(&mut self) -> Option<Self::Item> {
        self.0 .0.take().map(|s| ((), s))
    }
}
