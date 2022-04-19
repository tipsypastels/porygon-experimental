#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//! Porygon.

#[macro_use]
extern crate sealed;

#[macro_use]
extern crate serenity;

#[macro_use]
extern crate tracing;

mod app;
mod core;

use crate::core::setup::Setup;
use dotenv::dotenv;
use serenity::Client;
use std::env;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use tracing_tree::HierarchicalLayer;

#[tokio::main]
#[instrument]
async fn main() -> serenity::Result<()> {
    dotenv().ok();
    start_tracing();

    let token = env("DISCORD_TOKEN");
    let app_id = env("APPLICATION_ID")
        .parse::<u64>()
        .expect("Application ID is not a `u64`!");

    let client = Client::builder(token)
        .application_id(app_id)
        .await
        .unwrap_or_else(|e| panic!("Failed to create client: {e}"));

    Setup::new().add_from(app::installer).setup(&client).await?;

    info!("Setup complete!");

    Ok(())
}

fn env(var: &str) -> String {
    env::var(var).unwrap_or_else(|_| panic!("Mandatory environment variable `{var}` is missing!"))
}

fn start_tracing() {
    Registry::default()
        .with(fmt::layer().compact().without_time())
        .with(EnvFilter::from_default_env())
        .with(
            HierarchicalLayer::new(2)
                .with_bracketed_fields(true)
                .with_indent_lines(true),
        )
        .init();
}
