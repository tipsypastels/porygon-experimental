use crate::core::{
    controller::Controller,
    setup::{ControllerCollection, Step, StepArgs},
};
use custom_debug::Debug;
use serenity::{client::Client, http::client::Http, model::guild::PartialGuild};
use std::collections::HashMap;

/// An initialization function that runs at the end of setup. Usually defined
/// as a function, which the `init` macro transforms into a static struct.
///
/// Most initializers are used to set event handlers.
#[derive(Debug)]
pub struct Init {
    /// The name of the initializer.
    pub name: &'static str,

    #[debug(skip)]
    /// The function pointer for the initializer. This is public
    /// so the `init` macro can write to it, but should not be
    /// messed with, use `Init::exec` to call it.
    pub __exec: for<'a> fn(args: &'a InitArgs<'a>),
}

impl Init {
    pub fn exec<'a>(&self, args: &'a InitArgs<'a>) {
        (self.__exec)(args)
    }
}

/// Arguments given to a running initializer.
#[derive(Debug)]
pub struct InitArgs<'a> {
    #[debug(skip)]
    client: &'a Client,

    controller: Controller,
    guild: Option<PartialGuild>,
}

impl<'a> InitArgs<'a> {
    async fn new(step_args: StepArgs<'a, InitStep>) -> InitArgs<'a> {
        Self {
            client: step_args.client,
            controller: step_args.scope,
            guild: step_args.scope.try_get_guild(step_args.http).await,
        }
    }
}

/// Setup step that manages the execution of initializers at the end of setup.
#[derive(Debug)]
pub struct InitStep {
    controller: Controller,
    inits: HashMap<&'static str, Init>,
}

#[async_trait]
impl Step for InitStep {
    type Operand = Init;
    type Collection = ControllerCollection<Self>;

    const NAME: &'static str = "init";

    fn new(controller: Controller) -> Self {
        Self {
            controller,
            inits: HashMap::default(),
        }
    }

    fn append(&mut self, init: Init) {
        self.inits.insert(init.name, init);
    }

    async fn execute<'a>(self, args: StepArgs<'a, Self>) -> serenity::Result<()> {
        let init_args = InitArgs::new(args).await;

        for init in (&self.inits).values() {
            init.exec(&init_args);
        }

        Ok(())
    }
}
