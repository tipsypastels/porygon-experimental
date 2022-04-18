use crate::core::{
    controller::Controller,
    init::{Init, InitStep},
};
use serenity::Client;
use tokio::try_join;

mod scope;
mod skip;
mod step;

pub use scope::*;
pub use skip::*;
pub use step::*;

/// A builder for the setup process. Acts as an arena storing the setup steps that
/// have been registered, and manages their execution.
#[derive(Debug)]
pub struct Setup {
    inits: <InitStep as Step>::Collection,
}

impl Setup {
    /// Creates a builder for the setup process.
    pub fn new() -> Self {
        Self {
            inits: InitStep::collection(),
        }
    }

    /// Passes the setup builder to a callback which can customize
    /// it further. This allows for callbacks that install multiple
    /// things easily, and is the primary way that `app` installs
    /// setup steps.
    pub fn add_from(self, f: impl FnOnce(Self) -> Self) -> Self {
        f(self)
    }

    /// Registers an initializer under a given controller. See `Init`.
    pub fn add_init(mut self, controller: Controller, init: Init) -> Self {
        self.inits.factory(controller).append(init);
        self
    }

    /// Executes all setup steps and drops the arena. Calling this function
    /// marks the end of the setup process.
    ///
    /// If setup steps need their data to persist somehow (either in the same
    /// form or transformed by setup) they can stick it on `Client::data`,
    /// which is passed mutably to `Step::execute`.
    #[instrument(skip(client))]
    pub async fn setup(self, client: &Client) -> serenity::Result<()> {
        info!("Starting setup!");
        try_join!(Self::setup_step::<InitStep>(self.inits, client)).map(|_| ())
    }

    /// Runs a given type of setup step.
    /// TODO: This is current sequential, I haven't figured out all the try_join
    /// variants yet but it should definitely change to being parallel.
    #[instrument(skip_all)]
    async fn setup_step<S: Step>(
        collection: S::Collection,
        client: &Client,
    ) -> serenity::Result<()> {
        let http = client.cache_and_http.http.as_ref();

        for (scope, step) in collection {
            if scope.try_skip(http).await.should_skip() {
                continue;
            }

            let name = step.name_in(&scope);
            let args = StepArgs {
                scope,
                client,
                http,
            };

            step.execute(args).await?;

            info!("Setup step complete: {}!", name);
        }

        Ok(())
    }
}
