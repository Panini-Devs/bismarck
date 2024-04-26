use poise::CreateReply;
use serenity::builder::{
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage,
};

use super::embeds;

pub async fn error_response(
    message: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let embed = embeds::error_message_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(ephemeral);
    CreateInteractionResponse::Message(response_message)
}

pub fn info_message(message: impl Into<String>) -> CreateMessage {
    let embed = embeds::info_message_embed(&message.into());

    CreateMessage::default().embed(embed)
}

pub fn error_reply(message: impl Into<String>, ephemeral: bool) -> CreateReply {
    let embed = embeds::error_message_embed(&message.into());

    CreateReply::default().embed(embed).ephemeral(ephemeral)
}

pub fn info_reply(message: impl Into<String>, ephemeral: bool) -> CreateReply {
    let embed = embeds::info_message_embed(&message.into());

    CreateReply::default().embed(embed).ephemeral(ephemeral)
}
