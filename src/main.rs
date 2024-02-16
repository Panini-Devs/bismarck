use dashmap::DashMap;
use poise::serenity_prelude as serenity;
use sqlx::sqlite::SqliteQueryResult;
use std::env;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use tracing::{error, info};
use utilities::event_handler::event_handler;
use utilities::on_error::on_error;
use utilities::types::GuildSettings;

mod commands;
mod utilities;

use crate::commands::{info::*, math::*, moderation::*, owner::*, setup::*, utilities::*};

use sqlx::SqlitePool;

pub struct Data {
    pub reqwest: reqwest::Client,
    pub sqlite: SqlitePool,
    pub guild_data: DashMap<u64, GuildSettings>,
    pub commands_ran: DashMap<u64, AtomicU64>,
    pub songs_played: DashMap<u64, AtomicU64>,
    pub shard_manager: Arc<serenity::ShardManager>,
    pub is_loop_running: AtomicBool,
} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type FrameworkError<'a> = poise::FrameworkError<'a, Data, Error>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type PartialContext<'a> = poise::PartialContext<'a, Data, Error>;

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

    let guild_settings_map = DashMap::new();

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

    // Initialize command counter
    let bot_stats = sqlx::query!("SELECT * FROM bot_stats")
        .fetch_all(&database)
        .await
        .expect("Couldn't fetch bot stats");

    let commands_ran = DashMap::new();
    let songs_played = DashMap::new();

    for bot_stat in bot_stats {
        let guild_id = bot_stat.guild_id as u64;

        let cr = bot_stat.commands_ran as u64;
        let sp = bot_stat.songs_played as u64;

        commands_ran.insert(guild_id, AtomicU64::new(cr));
        songs_played.insert(guild_id, AtomicU64::new(sp));
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("+".to_string()),
                // tracks edits for 60 seconds
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(60),
                ))),
                case_insensitive_commands: true,
                mention_as_prefix: true,
                execute_self_messages: false,
                // dynamic prefix support
                dynamic_prefix: Some(|context: PartialContext| {
                    Box::pin(async move {
                        if let Some(guild_id) = context.guild_id {
                            let pf = &context.data.guild_data;

                            let guild_settings = pf.get(&guild_id.get());
                            match guild_settings {
                                // we now return the result instead of throwing it away with `let _`
                                Some(guild_settings) => Ok(Some(guild_settings.prefix.clone())),
                                None => {
                                    // if no guild settings found,
                                    // create new database entry and return default prefix
                                    let (guild_id, owner_id) = {
                                        let guild = guild_id
                                            .to_guild_cached(&context.serenity_context.cache)
                                            .unwrap();
                                        (i64::from(guild.id), i64::from(guild.owner_id))
                                    };

                                    let database = &context.data.sqlite;

                                    // create new guild settings into sqlite database as a failsafe
                                    // in case guild_join did not load properly
                                    let query_result: Result<SqliteQueryResult, sqlx::Error> =
                                        sqlx::query!(
                                            "INSERT INTO guild_settings (
                                            guild_id,
                                            prefix,
                                            owner_id
                                        ) VALUES (?, ?, ?)",
                                            guild_id,
                                            "+",
                                            owner_id
                                        )
                                        .execute(database)
                                        .await;

                                    // this one ended up a bit weird
                                    // we have to convert the sqlx::Error to our type alias Error
                                    // if query_result is Err
                                    // otherwise we just return Ok(Some("+".to_string))
                                    // the inner query result is unused, but can be used in the closure if desired
                                    match query_result {
                                        Ok(_query) => Ok(Some("+".to_string())),
                                        Err(sqlx_error) => Err(Error::from(sqlx_error)),
                                    }

                                    // the below code does the same as the above a bit more idomatically
                                    // go with whichever seems more readable to you

                                    // query_result.map_or_else(
                                    //     |sqlx_err| Err(Error::from(sqlx_err)),
                                    //     |_query_result| Ok(Some("+".to_string())),
                                    // )
                                }
                            }
                        } else {
                            // previously, without the else block, we were throwing away
                            // everything we did in the `if let` and always just returning Ok(Some("+".to_string()))
                            Ok(Some("+".to_string()))
                        }
                    })
                }),
                ..Default::default()
            },
            commands: vec![
                // Info commands
                about(),
                user_info(),
                user_avatars(),
                bot_stat(),
                // Math commands
                multiply(),
                add(),
                divide(),
                subtract(),
                // Moderation commands
                ban(),
                kick(),
                unban(),
                timeout(),
                untimeout(),
                warn(),
                // Utility commands
                help(),
                ping(),
                servers(),
                prefix(),
                // Owner commands
                shutdown(),
            ],
            skip_checks_for_owners: true,
            event_handler: |context, event, framework, data| {
                Box::pin(event_handler(context, event, framework, data))
            },
            pre_command: |context| {
                Box::pin(async move {
                    if let Some(guild_id) = context.guild_id() {
                        let commands_ran =
                            context.data().commands_ran.get(&guild_id.get()).unwrap();
                        commands_ran.fetch_add(1, Ordering::Relaxed);
                    }

                    let commands_ran_global = context.data().commands_ran.get(&0).unwrap();
                    commands_ran_global.fetch_add(1, Ordering::Relaxed);
                })
            },
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .setup(|context, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(context, &framework.options().commands).await?;
                Ok(Data {
                    reqwest: reqwest::Client::new(),
                    sqlite: database,
                    commands_ran,
                    songs_played,
                    guild_data: guild_settings_map,
                    shard_manager: framework.shard_manager().clone(),
                    is_loop_running: AtomicBool::new(false),
                })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();

    // Setup shard manager
    let shard_manager = client.shard_manager.clone();

    // Start shard manager
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");

        info!("Gracefully shutting down...");
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
