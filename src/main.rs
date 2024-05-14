use bismarck_core::context::PartialContext;
use bismarck_core::data::Data;
use bismarck_core::types::{GuildSettings, User};
use bismarck_events::event_handler::event_handler;
use bismarck_events::on_error::on_error;
use dashmap::DashMap;
use poise::serenity_prelude as serenity;
use std::env;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use tracing::{error, info};

use bismarck_commands::{
    info::*, moderation::*, neko::*, owner::*, setup::*, utilities::*, wiki::*,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    // gets token, exits if no token
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = bismarck_core::gateway_intents().await;

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
    let guild_settings = sqlx::query!("SELECT * FROM guild")
        .fetch_all(&database)
        .await
        .expect("Couldn't fetch guild settings");

    let guild_settings_map = DashMap::new();

    for guild_setting in guild_settings {
        let guild_id = guild_setting.id as u64;
        let guild_settings = GuildSettings {
            prefix: guild_setting.prefix,
            owner_id: guild_setting.owner as u64,
            mute_type: guild_setting.mute_style,
            mute_role: guild_setting.mute_role.unwrap_or_default() as u64,
            default_mute_duration: guild_setting.mute_duration as u64,
        };

        guild_settings_map.insert(guild_id, guild_settings);
    }

    let users = DashMap::new();
    let commands_ran_user_map = DashMap::new();
    let users_map = sqlx::query!("SELECT * FROM user")
        .fetch_all(&database)
        .await
        .expect("Couldn't fetch users");

    for user in users_map {
        let user_id = user.id as u64;
        let user_stats = User {
            id: user_id,
            acquaint_fate: user.acquaint_fate as u64,
            intertwined_fate: user.interwined_fate as u64,
            primogems: user.primogems as u64,
            standard_pity: user.standard_pity as u64,
            weapon_pity: user.weapon_pity as u64,
            character_pity: user.character_pity as u64,
        };

        users.insert(user_id, user_stats);
        commands_ran_user_map.insert(user_id, AtomicU64::new(user.commands_run as u64));
    }

    // Initialize command counter
    let bot_stats = sqlx::query!("SELECT id, commands_ran, songs_played FROM guild")
        .fetch_all(&database)
        .await
        .expect("Couldn't fetch bot stats"); // fetch all of them, and if database is otherwise unavailable return error and quit the program

    let commands_ran = DashMap::new();
    let songs_played = DashMap::new();

    for bot_stat in bot_stats {
        let guild_id = bot_stat.id as u64;

        let cr = bot_stat.commands_ran as u64; //commands ran
        let sp = bot_stat.songs_played as u64; //songs played

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
                    Box::pin(async move { bismarck_utilities::command::get_prefix(context).await })
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
                // TODO: math(),
                // Moderation commands
                ban(),
                kick(),
                unban(),
                timeout(),
                untimeout(),
                warn(),
                warnings(),
                // Neko commands
                neko(),
                // Wiki commands
                wiki(),
                // Utility commands
                help(),
                ping(),
                servers(),
                prefix(),
                status(),
                // Owner commands
                shutdown(),
            ],
            skip_checks_for_owners: true,
            event_handler: |context, event, framework, data| {
                Box::pin(event_handler(context, event, framework, data))
            },
            pre_command: |context| {
                Box::pin(async move { bismarck_utilities::command::pre_command(context).await })
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
                    users,
                    commands_ran_users: commands_ran_user_map,
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
