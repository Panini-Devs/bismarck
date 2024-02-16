use tracing::error;

use crate::FrameworkError;

use super::messages;

pub async fn on_error(error: FrameworkError<'_>) {
    match error {
        FrameworkError::Setup { .. } | FrameworkError::EventHandler { .. } => {}
        FrameworkError::Command { ctx, .. } => {
            let reply =
                messages::error_reply("Oh no! There's a problem in executing this command.", true);
            if let Err(why) = ctx.send(reply).await {
                if why.to_string().contains("40060") {
                    // Interaction has already been acknowledged.
                    return;
                }

                error!("Couldn't send reply: {why:?}");
            }
        }
        FrameworkError::CommandPanic { ctx, .. } => {
            let reply = messages::error_reply(
                "Oh no! A panic occurred whilst executing this command.",
                true,
            );
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {:?}", why);
            }
        }
        FrameworkError::ArgumentParse { .. }
        | poise::FrameworkError::CommandStructureMismatch { .. } => {}
        FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            let permissions = missing_permissions
                .iter()
                .map(|permission| permission.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            let reply = messages::error_reply(
                format!("Oh no! I'm missing the following permission(s): `{permissions}`"),
                true,
            );
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {:?}", why);
            }
        }
        FrameworkError::NsfwOnly { ctx, .. } => {
            let reply = messages::error_reply(
                "Sorry, but I can only execute this command in a NSFW channel.",
                true,
            );
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {:?}", why);
            }
        }
        _ => {}
    }
}
