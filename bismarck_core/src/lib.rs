use poise::serenity_prelude as serenity;

pub mod context;
pub mod data;
pub mod error;
pub mod types;

pub async fn gateway_intents() -> serenity::GatewayIntents {
    serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::GUILD_MODERATION
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT
}
