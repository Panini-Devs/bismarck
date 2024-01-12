use std::{time::Duration, sync::Arc, collections::HashMap};
use poise::serenity_prelude as serenity;
use reqwest;
use tokio::{time::sleep, sync::RwLock};
use tracing::{error, info};
use utilities::types::GuildSettings;
use std::env;

mod commands;
mod utilities;

use crate::commands::{info::*, math::*, neko::*, settings::*, setup::*, utilities::*};

use sqlx::SqlitePool;

pub struct Data {
    pub reqwest: reqwest::Client,
    pub sqlite: SqlitePool,
    pub guild_data: RwLock<HashMap<u64, GuildSettings>>
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

    // Run migrations, which updates the database's schema to the latest version.
    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");

    // Initiate guild settings
    let guild_settings = sqlx::query!("SELECT * FROM guild_settings")
        .fetch_all(&database)
        .await
        .expect("Couldn't fetch guild settings");

    let mut guild_settings_map = HashMap::new();

    for guild_setting in guild_settings {
        let guild_id = guild_setting.guild_id as u64;
        let guild_settings = GuildSettings {
            prefix: guild_setting.prefix,
            owner_id: guild_setting.owner_id as u64,
            mute_type: guild_setting.mute_style,
            mute_role: guild_setting.mute_role_id.unwrap_or_default() as u64,
            default_mute_duration: guild_setting.mute_duration as u64,
        };

        guild_settings_map.insert(guild_id, guild_settings);
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("+".to_string()),
                // tracks edits for 60 seconds
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(std::time::Duration::from_secs(60)))),
                case_insensitive_commands: true,
                mention_as_prefix: true,
                execute_self_messages: false,
                // dynamic prefix support
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
                    guild_data: RwLock::new(guild_settings_map)
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
