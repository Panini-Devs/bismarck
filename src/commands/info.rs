use crate::utilities::git::{get_current_branch, get_head_revision};
use crate::{Context, Error};
use git2::Repository;
use poise::{serenity_prelude as serenity, CreateReply};

use serenity::{
    CreateEmbed,
    CreateEmbedFooter,
    CreateEmbedAuthor,
    PremiumTier,
    PremiumType,

};

/// Returns bot information.
#[poise::command(prefix_command, slash_command)]
pub async fn about(context: Context<'_>) -> Result<(), Error> {
    let repo = Repository::open(env!("CARGO_MANIFEST_DIR"))?;

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

/// Returns the account age of the selected u.
///
#[poise::command(slash_command, prefix_command)]
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

    message
        .channel_id
        .send_message(&context, CreateMessage::new().embed(embed))
        .await?;

    Ok(())
}
