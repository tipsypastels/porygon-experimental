use super::{Collection, Scope};
use custom_debug::Debug;
use serenity::{client::Client, http::Http};

/// A `Step` is a type-specific component of the setup process. Implementers
/// are responsible for the setup process of a specific type of operand, such as
/// a command or initializer.
///
/// Setup steps use a "merging" step to collect all the operands they need
/// before actually executing. This is defined by their *scope* - either unique
/// to the implementing type (all operands are merged into one operation), or,
/// more commonly, unique to the implementing type and its controller (all
/// operands added under the same controller are merged into one operation).
///
/// `Step`s are stored in the `Setup` builder, which manages the setup
/// process. Each implementing type has its own manually-placed slot in
/// `Setup`, for type safety and ease.
#[async_trait]
pub trait Step: Sized + Send + Sync + 'static {
    /// The type of operands this setup step will be collecting and acting upon.
    type Operand;

    /// The type of the collection *of setup steps of this type*. This is *not*
    /// the internal type of storage that *operands* are collected in, which is
    /// an implementation detail and not represented here. Rather, a collection
    /// is how the *scope* of a setup step is defined.
    ///
    /// - `UniqueCollection` for setup steps that are globally unique.
    /// - `ControllerCollection` for setup steps that are controller-scoped.
    ///
    type Collection: Collection<Self>;

    /// Name of the initialization step. Conventionally a representation
    /// with the `Step` suffix removed, e.g. `Init` for `InitStep`.
    const NAME: &'static str;

    /// Name of the step and its scope, for logging purposes. Need not
    /// be overridden.
    fn name_in(&self, scope: &StepScope<Self>) -> String {
        format!("{}{}", Self::NAME, scope.suffix())
    }

    /// Number of current operands. Used for logging.
    fn operand_count(&self) -> usize;

    /// Returns a new instance of the setup step for a given scope. The type of
    /// the scope is based on the type of the collection.
    ///
    /// - `UniqueCollection` = `()`.
    /// - `ControllerCollection` = `Controller`.
    ///
    fn new(scope: StepScope<Self>) -> Self;

    /// Returns a new instance of the collection. Used by `Step` as a shorthand,
    /// shouldn't be overriden or called outside of that.
    fn collection() -> Self::Collection {
        Self::Collection::default()
    }

    /// Appends an operand to the internal storage of this instance of the implementing
    /// type. This normally inserts into a private `HashMap` or something similar.
    fn append(&mut self, operand: Self::Operand);

    /// Runs the setup process once all the operands have been collected. This normally
    /// involves uploading the operands somewhere.
    async fn execute<'a>(self, args: StepArgs<'a, Self>) -> serenity::Result<()>;
}

/// Shorthand for extracting the scope of a step.
pub type StepScope<S> = <<S as Step>::Collection as Collection<S>>::Scope;

/// Arguments passed to execution of a setup step.
#[derive(Debug)]
pub struct StepArgs<'a, S: Step> {
    /// The Serenity client. Passed immutably, as it has interior
    /// mutability on most of the fields we want.
    #[debug(skip)]
    pub client: &'a Client,

    /// The Serenity HTTP client. Can be retrieved from `client` but
    /// separated here because it's very commonly used.
    pub http: &'a Http,

    /// The registration scope of the setup step. This is either `()`,
    /// in which case you don't care about it, or a `Controller`, in
    /// which case you do.
    pub scope: StepScope<S>,
}
