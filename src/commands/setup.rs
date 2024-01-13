use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::{CreateEmbed, CreateEmbedFooter, CreateMessage};

///
#[poise::command(
    prefix_command,
    slash_command,
    category = "Settings",
    subcommands("set", "view")
)]
pub async fn prefix(context: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    category = "Settings",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn set(context: Context<'_>, prefix: Option<String>) -> Result<(), Error> {
    Ok(())
}

/// Views current guild's prefix commands' prefix.
#[poise::command(slash_command, category = "Settings")]
pub async fn view(context: Context<'_>) -> Result<(), Error> {
    Ok(())
}
