use crate::Context;
use poise::serenity_prelude::{model::ModelError, User, UserId};
use serenity::all::{GuildId, Member, Mention, Mentionable};
use tracing::error;

pub fn author(context: Context<'_>) -> Result<&User, ModelError> {
    Ok(context.author())
}

pub fn author_mention(context: Context<'_>) -> Result<Mention, ModelError> {
    let author = author(context)?;
    Ok(author.mention())
}

pub async fn user(context: Context<'_>, user_id: UserId) -> Result<User, ModelError> {
    match user_id.to_user(context).await {
        Ok(user) => Ok(user),
        Err(why) => {
            error!("Couldn't get user: {why:?}");
            Err(ModelError::MemberNotFound)
        }
    }
}

pub async fn user_mention(context: Context<'_>, user_id: UserId) -> Result<Mention, ModelError> {
    Ok(user(context, user_id).await?.mention())
}

pub async fn member(
    ctx: Context<'_>,
    guild_id: GuildId,
    user_id: UserId,
) -> Result<Member, ModelError> {
    match guild_id.member(&ctx, user_id).await {
        Ok(member) => Ok(member),
        Err(why) => {
            error!("Couldn't get member: {why:?}");
            Err(ModelError::MemberNotFound)
        }
    }
}
