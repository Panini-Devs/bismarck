use bismarck_core::{context::Context, error::Error};
use chrono::Duration;
use chrono::Utc;
use poise::builtins::PrettyHelpConfiguration;
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;

/// Show help message
#[poise::command(
    slash_command,
    prefix_command,
    track_edits,
    category = "Utility",
    required_bot_permissions = "SEND_MESSAGES"
)]
pub async fn help(
    context: Context<'_>,
    #[description = "Command to get help for"]
    #[rest]
    mut command: Option<String>,
) -> Result<(), Error> {
    // This makes it possible to just make `help` a subcommand of any command
    if context.invoked_command_name() != "help" {
        command = match command {
            Some(c) => Some(format!("{} {}", context.invoked_command_name(), c)),
            None => Some(context.invoked_command_name().to_string()),
        };
    }

    let pf = &context.data().guild_data.clone();
    let prefix = context
        .guild_id()
        .and_then(|guild_id| pf.get(&guild_id.get()))
        .map_or_else(|| "+".to_string(), |gs| gs.prefix.clone());

    let format = format!(
        "\
    Type `{prefix}help command` for more info on a command.
    You can edit your `{prefix}help` message to the bot and the bot will edit its response."
    );

    let extra_text_at_bottom = format.as_str();

    let config = PrettyHelpConfiguration {
        show_subcommands: true,
        show_context_menu_commands: true,
        ephemeral: true,
        extra_text_at_bottom,
        include_description: true,
        ..Default::default()
    };
    poise::builtins::pretty_help(context, command.as_deref(), config).await?;
    Ok(())
}

/// Shows latency of the bot to Discord API and Shard.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Utility",
    required_bot_permissions = "SEND_MESSAGES"
)]
pub async fn ping(context: Context<'_>) -> Result<(), Error> {
    let start = Utc::now();
    let start_ts = start.timestamp();
    let start_ts_ss = start.timestamp_subsec_millis() as i64;
    let ping = context.say(":ping_pong: Pinging!").await.unwrap();
    let end = Utc::now();
    let end_ts = end.timestamp();
    let end_ts_ss = end.timestamp_subsec_millis() as i64;
    let api_response = ((end_ts - start_ts) * 1000) + (end_ts_ss - start_ts_ss);
    let context_data = context.data();
    let shard_manager = &context_data.shard_manager;

    let runners = shard_manager.runners.lock().await;
    let runner = match runners.get(&context.serenity_context().shard_id) {
        Some(runner) => runner,
        None => {
            context.reply("Could not find a shard").await?;
            return Ok(());
        }
    };

    let shard_response = match runner.latency {
        Some(latency) => {
            if let Ok(time) = Duration::from_std(latency) {
                let time_ms = time.num_milliseconds();
                format!("`{time_ms}ms`")
            } else {
                "No latency information available".to_string()
            }
        }
        None => "No data available at the moment.".to_string(),
    };

    let response = format!(
        "Pong! Succesfully retrieved the message and shard latencies. :ping_pong:\n\n\
        **API Response Time**: `{api_response}ms`\n\
        **Shard Response Time**: {shard_response}"
    );

    let embed = CreateEmbed::new()
        .color(0x008b_0000)
        .title("Discord Latency Information")
        .description(response);
    ping.edit(context, CreateReply::default().embed(embed))
        .await?;

    Ok(())
}

/// Shows the servers the bot is connected to.
#[poise::command(
    slash_command,
    prefix_command,
    category = "Utility",
    required_bot_permissions = "SEND_MESSAGES"
)]
pub async fn servers(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::servers(ctx).await?;
    Ok(())
}

/// Shows the status of the bot
#[poise::command(
    slash_command,
    prefix_command,
    category = "Utility",
    required_bot_permissions = "SEND_MESSAGES"
)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    let memory = sys.used_memory();
    let total = sys.total_memory();
    let free = sys.free_memory();

    let cpus = sys.cpus();

    let system_name = sysinfo::System::name().unwrap_or("Unknown OS".to_string());
    let kernel_version = sysinfo::System::kernel_version().unwrap_or("Unknown Kernel".to_string());
    let os_version = sysinfo::System::os_version().unwrap_or("Unknown Version".to_string());

    let embed = CreateEmbed::new()
        .title("Bot Status")
        .field(
            "Memory",
            format!(
                "```Used: {} MB\nTotal: {} MB\nFree: {} MB```",
                memory / 1000000,
                total / 1000000,
                free / 1000000
            ),
            false,
        )
        .field("CPU", format!("```{} Threads```", cpus.len()), false)
        .field(
            "System",
            format!(
                "```OS Name: {}\nKernel Version: {}\nOS Version: {}```",
                system_name, kernel_version, os_version
            ),
            false,
        )
        .color(0x008b_0000);

    let msg = CreateReply::default().embed(embed);
    ctx.send(msg).await?;

    Ok(())
}
