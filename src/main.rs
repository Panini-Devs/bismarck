use poise::serenity_prelude as serenity;
use reqwest;
use sqlx::SqlitePool;
use std::env;

mod commands;


use crate::commands::{
    info::*,
    math::*,
    neko::*,
    setup::*,
    settings::*,
    utilities::*
};

struct Data {
    reqwest: reqwest::Client,
    sqlite: SqlitePool,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    // gets token, exits if no token
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = serenity::GatewayIntents::all();

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable `RUST_LOG` to `debug`.
    tracing_subscriber::fmt::init();

    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    reqwest: reqwest::Client::new(),
                    sqlite: database,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start_autosharded().await.unwrap();
}
