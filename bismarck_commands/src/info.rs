use std::sync::atomic::Ordering;

use bismarck_core::{context::Context, error::Error};
use bismarck_utilities::git::{get_current_branch, get_head_revision, get_absolute_path};
use git2::Repository;
use poise::{serenity_prelude as serenity, CreateReply};

use serenity::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, PremiumType};

/// Returns bot information.
#[poise::command(
    prefix_command,
    slash_command,
    required_bot_permissions = "SEND_MESSAGES",
    aliases("botinfo", "bi"),
    category = "Info"
)]
pub async fn about(context: Context<'_>) -> Result<(), Error> {
    let repo = Repository::open(get_absolute_path())?;

    let version = env!("CARGO_PKG_VERSION").to_string();
    let codename = "Graf Zeppelin".to_string();
    let branch = get_current_branch(&repo);
    let revision = get_head_revision(&repo);

    let current_user = context.cache().current_user().clone();

    let bot_name = &current_user.name;
    let bot_avatar = &current_user.avatar_url().unwrap();
    let bot_owner = context
        .http()
        .get_current_application_info()
        .await?
        .owner
        .unwrap()
        .tag();

    let num_shards = context.cache().shard_count();
    let num_guilds = context.cache().guilds().len();
    let num_channels = context.cache().guild_channel_count();
    let num_users = context.cache().user_count();

    let about_fields = vec![
        ("Version", version, true),
        ("Codename", codename.to_string(), true),
        ("Branch", branch, true),
        ("Revision", format!("`{revision}`"), true),
        ("Owner", bot_owner, true),
        ("Shards", num_shards.to_string(), true),
        ("Guilds", num_guilds.to_string(), true),
        ("Channels", num_channels.to_string(), true),
        ("Users", num_users.to_string(), true),
    ];

    let embed = CreateEmbed::new()
        .title(format!("**{bot_name}**"))
        .url("https://github.com/Panini-Devs/bismarck")
        .thumbnail(bot_avatar)
        .color(0x00BFFF)
        .fields(about_fields)
        .footer(CreateEmbedFooter::new("Written with Rust & Poise."));

    let msg = CreateReply::default().embed(embed);

    let _ = context.send(msg).await;

    Ok(())
}

/// Returns the account age of the selected user.
#[poise::command(
    slash_command,
    prefix_command,
    required_bot_permissions = "SEND_MESSAGES",
    aliases("userinfo", "ui"),
    category = "Info"
)]
pub async fn user_info(
    context: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| context.author());
    let user_id = u.id;
    let user_name = u.tag();
    let user_created = u.created_at();
    let user_avatar = u.face();
    let user_bot = u.bot;
    let user_nitro = match u.premium_type {
        PremiumType::None => "None",
        PremiumType::NitroClassic => "Nitro Classic",
        PremiumType::Nitro => "Nitro",
        PremiumType::NitroBasic => "Nitro Basic",
        _ => "Unrecognized Premium Type",
    };
    let guild = context.guild().unwrap().clone();
    let member = guild.member(&context.http(), user_id).await.unwrap();

    let embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new(user_name.clone()).icon_url(user_avatar))
        .thumbnail(guild.icon_url().unwrap().to_string())
        .description(format!("Showing information about user {user_name}"))
        .field("ID", user_id.to_string(), true)
        .field("Created at", user_created.to_string(), true)
        .field("Is bot", user_bot.to_string(), true)
        .field(
            "Joined server at",
            member.joined_at.unwrap().to_string(),
            true,
        )
        .field("Nitro Subscription", user_nitro, true)
        .footer(CreateEmbedFooter::new(format!("User ID: {user_id}")));

    let msg = CreateReply::default().embed(embed);

    let _ = context.send(msg).await;

    Ok(())
}

/// Shows the user's avatars.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Info",
    required_bot_permissions = "SEND_MESSAGES"
)]
pub async fn user_avatars(
    context: Context<'_>,
    #[description = "Selected user."] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = match user {
        Some(user) => user,
        None => context.author().clone(),
    };
    let user_id = user.id;
    let user_name = user.tag();
    let guild = context.guild().unwrap().clone();
    let guild_avatar = guild
        .member(&context.http(), user_id)
        .await
        .unwrap()
        .clone()
        .avatar_url()
        .unwrap_or("".to_string());

    if !guild_avatar.is_empty() {
        let embed = CreateEmbed::new()
            .author(CreateEmbedAuthor::new(user_name.clone()).icon_url(user.face()))
            .description(format!("Showing profile pictures of {}", user_name.clone()))
            .image(user.face());

        let embed2 = CreateEmbed::new()
            .author(CreateEmbedAuthor::new(user_name.clone()).icon_url(guild_avatar.clone()))
            .description(format!("Showing profile pictures of {}", user_name.clone()))
            .image(guild_avatar.clone());

        let msg = CreateReply::default().embed(embed).embed(embed2);
        let _ = context.send(msg).await;
    } else {
        let embed = CreateEmbed::new()
            .author(CreateEmbedAuthor::new(user_name.clone()).icon_url(user.face()))
            .description(format!("Showing profile pictures of {user_name}"))
            .image(user.face());

        let msg = CreateReply::default().embed(embed);
        let _ = context.send(msg).await;
    }

    Ok(())
}

/// Returns the bot's stats.
#[poise::command(
    slash_command,
    prefix_command,
    category = "Info",
    required_bot_permissions = "SEND_MESSAGES"
)]
pub async fn bot_stat(context: Context<'_>) -> Result<(), Error> {
    let guild = context.guild().unwrap().id;
    let guildid = guild.get();

    let data = context.data();

    let commands_ran = data.commands_ran.get(&guildid).unwrap();
    let songs_played = data.songs_played.get(&guildid).unwrap();

    let embed = CreateEmbed::new()
        .title("**Stats**")
        .field(
            "Commands Ran (Guild)",
            commands_ran.load(Ordering::Relaxed).to_string(),
            true,
        )
        .field(
            "Songs Played (Guild)",
            songs_played.load(Ordering::Relaxed).to_string(),
            true,
        )
        .footer(CreateEmbedFooter::new(format!(
            "Bot ID: {}",
            context.cache().current_user().id
        )));

    let msg = CreateReply::default().embed(embed);

    let _ = context.send(msg).await;

    Ok(())
}
