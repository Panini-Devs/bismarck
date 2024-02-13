use crate::{
    utilities::{
        messages, models,
        modlog::{self, ModType},
    },
    Context, Error,
};

use chrono::Utc;
use poise::serenity_prelude::UserId;
use tracing::{error, info};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "BAN_MEMBERS",
    required_bot_permissions = "BAN_MEMBERS | SEND_MESSAGES",
    guild_only,
    ephemeral
)]
pub async fn ban(
    context: Context<'_>,
    #[description = "The user to ban."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "Reason for the ban."]
    #[max_length = 80]
    reason: Option<String>,
) -> Result<(), Error> {
    let database = &context.data().sqlite;

    let user = models::user(context, user_id).await?;

    let moderator = context.author();
    let moderator_id = moderator.id;

    if user.system {
        let reply = messages::error_reply("Cannot ban a system user.", false);
        context.send(reply).await?;
        return Ok(());
    }

    if user_id == moderator_id {
        let reply = messages::error_reply("Sorry, but you cannot ban yourself.", true);
        context.send(reply).await?;

        return Ok(());
    }

    let reason = reason.unwrap_or_else(|| "No reason provided.".to_string());

    let reason_char_count = reason.chars().count();
    if reason_char_count > 80 {
        let reply = messages::info_reply("Reason must be no more than 80 characters long.", true);
        context.send(reply).await?;

        return Ok(());
    }

    let result = {
        let (user_name, user_mention) = (&user.name, models::user_mention(context, user_id).await?);

        let (moderator_name, moderator_mention) =
            (&moderator.name, models::author_mention(context)?);

        let (guild_id, guild_name) = {
            let guild_id = context.guild_id().unwrap();
            let guild = context.guild().unwrap();
            (guild_id, guild.name.clone())
        };

        let created_at = Utc::now().naive_utc();

        let mut user_mod_history = modlog::select_modlog_from_users(&user_id, database).await?;

        let message = messages::info_message(format!(
            "You've been banned from {guild_name} by {moderator_mention} for {reason}.",
        ));
        let dm = user.direct_message(context, message).await;

        if let Err(why) = dm {
            error!("Couldn't send DM to @{user_name}: {why:?}");
        }

        match guild_id.ban_with_reason(context, user_id, 0, &reason).await {
            Ok(_) => {
                modlog::insert_modlog(
                    ModType::Ban,
                    &guild_id,
                    &user_id,
                    &moderator_id,
                    &reason,
                    created_at,
                    database,
                )
                .await?;

                user_mod_history += 1;

                modlog::update_users_set_modlog(&user_id, user_mod_history, database).await?;

                info!("@{moderator_name} banned @{user_name} from {guild_name}: {reason}");
                Ok(format!("{user_mention} has been banned."))
            }
            Err(why) => {
                error!("Couldn't ban @{user_name}: {why:?}");
                Err(format!("Sorry, but I couldn't ban {user_mention}."))
            }
        }
    };

    if let Err(why) = result {
        let reply = messages::error_reply(&why, true);
        context.send(reply).await?;
    }

    Ok(())
}
