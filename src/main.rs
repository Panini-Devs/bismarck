use std::{time::Duration, sync::Arc};
use poise::serenity_prelude as serenity;
use reqwest;
use tokio::time::sleep;
use tracing::{error, info};
use std::env;

mod commands;
mod utilities;

use crate::commands::{info::*, math::*, neko::*, settings::*, setup::*, utilities::*};

use sqlx::SqlitePool;

pub struct Data {
    pub reqwest: reqwest::Client,
    pub sqlite: SqlitePool,
} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

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
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("+".to_string()),
                // tracks edits for 60 seconds
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(std::time::Duration::from_secs(60)))),
                case_insensitive_commands: true,
                mention_as_prefix: true,
                ..Default::default()
            },
            commands: vec![
                about(),
                user_info(),
                user_avatars(),
                multiply(),
                add()
            ],
            skip_checks_for_owners: true,
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

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await.unwrap();

    // Setup shard manager
    let shard_manager = client.shard_manager.clone();

    // Start shard manager
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.shutdown_all().await;
    });

    let manager = client.shard_manager.clone();

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(30)).await;

            let shard_runners = manager.runners.lock().await;

            for (id, runner) in shard_runners.iter() {
                info!(
                    "Shard ID {} is {} with a latency of {:?}",
                    id, runner.stage, runner.latency,
                );
            }
        }
    });

    if let Err(why) = client.start_autosharded().await {
        error!("Client error: {:?}", why);
    }
}
