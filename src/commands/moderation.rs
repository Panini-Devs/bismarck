use std::time::Duration;

use crate::{
    utilities::{
        self,
        embeds::warnings_command_embed,
        messages, models,
        modlog::{self, ensure_user, ModType},
    },
    Context, Error,
};

use chrono::{Days, NaiveDateTime, Utc};
use duration_str::parse;
use poise::serenity_prelude::UserId;
use serenity::model::Timestamp;
use tracing::{error, info};

/// Bans a user.
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

    let member = models::member(context, context.guild_id().unwrap(), user_id).await?;

    let ensure = ensure_user(&member, &user_id, &context.guild_id().unwrap(), database).await;

    if let Err(why) = ensure {
        let _ = messages::error_response(why.to_string(), true).await;
        return Ok(());
    }

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
    } else {
        let reply = messages::info_reply(result.unwrap(), true);
        context.send(reply).await?;
    }

    Ok(())
}

/// Kicks a user.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "KICK_MEMBERS",
    required_bot_permissions = "KICK_MEMBERS | SEND_MESSAGES",
    guild_only,
    ephemeral
)]
pub async fn kick(
    context: Context<'_>,
    #[description = "The user to kick."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "Reason for the kick."]
    #[max_length = 80]
    reason: Option<String>,
) -> Result<(), Error> {
    let database = &context.data().sqlite;

    let user = models::user(context, user_id).await?;

    let member = models::member(context, context.guild_id().unwrap(), user_id).await?;

    let ensure = ensure_user(&member, &user_id, &context.guild_id().unwrap(), database).await;

    if let Err(why) = ensure {
        let _ = messages::error_response(why.to_string(), true).await;
        return Ok(());
    }

    let moderator = context.author();
    let moderator_id = moderator.id;

    if user.system {
        let reply = messages::error_reply("Cannot kick a system user.", false);
        context.send(reply).await?;
        return Ok(());
    }

    if user_id == moderator_id {
        let reply = messages::error_reply("Sorry, but you cannot kick yourself.", true);
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
            "You've been kicked from {guild_name} by {moderator_mention} for {reason}.",
        ));
        let dm = user.direct_message(context, message).await;

        if let Err(why) = dm {
            error!("Couldn't send DM to @{user_name}: {why:?}");
        }

        match guild_id.kick_with_reason(context, user_id, &reason).await {
            Ok(_) => {
                modlog::insert_modlog(
                    ModType::Kick,
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

                info!("@{moderator_name} kicked @{user_name} from {guild_name}: {reason}");
                Ok(format!("{user_mention} has been kicked."))
            }
            Err(why) => {
                error!("Couldn't kick @{user_name}: {why:?}");
                Err(format!("Sorry, but I couldn't kick {user_mention}."))
            }
        }
    };

    if let Err(why) = result {
        let reply = messages::error_reply(&why, true);
        context.send(reply).await?;
    } else {
        let reply = messages::info_reply(result.unwrap(), true);

        context.send(reply).await?;
    }

    Ok(())
}

/// Unbans a user.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "BAN_MEMBERS",
    required_bot_permissions = "BAN_MEMBERS | SEND_MESSAGES",
    guild_only,
    ephemeral
)]
pub async fn unban(
    context: Context<'_>,
    #[description = "The user to unban."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "Reason for the unban."]
    #[max_length = 80]
    reason: Option<String>,
) -> Result<(), Error> {
    let database = &context.data().sqlite;

    let user = models::user(context, user_id).await?;

    let member = models::member(context, context.guild_id().unwrap(), user_id).await?;

    let ensure = ensure_user(&member, &user_id, &context.guild_id().unwrap(), database).await;

    if let Err(why) = ensure {
        let _ = messages::error_response(why.to_string(), true).await;
        return Ok(());
    }

    let moderator = context.author();
    let moderator_id = moderator.id;

    if user.system {
        let reply = messages::error_reply("Cannot unban a system user.", false);
        context.send(reply).await?;
        return Ok(());
    }

    if user_id == moderator_id {
        let reply = messages::error_reply("Sorry, but you cannot unban yourself.", true);
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

        let moderator_name = &moderator.name;

        let (guild_id, guild_name) = {
            let guild_id = context.guild_id().unwrap();
            let guild = context.guild().unwrap();
            (guild_id, guild.name.clone())
        };

        let created_at = Utc::now().naive_utc();

        let mut user_mod_history = modlog::select_modlog_from_users(&user_id, database).await?;

        match guild_id.unban(context, user_id).await {
            Ok(_) => {
                modlog::insert_modlog(
                    ModType::Unban,
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

                info!("@{moderator_name} unbanned @{user_name} from {guild_name}: {reason}");
                Ok(format!("{user_mention} has been unbanned."))
            }
            Err(why) => {
                error!("Couldn't unban @{user_name}: {why:?}");
                Err(format!("Sorry, but I couldn't unban {user_mention}."))
            }
        }
    };

    if let Err(why) = result {
        let reply = messages::error_reply(&why, true);
        context.send(reply).await?;
    } else {
        let reply = messages::info_reply(result.unwrap(), true);

        context.send(reply).await?;
    }

    Ok(())
}

/// Times out a user.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "MODERATE_MEMBERS | SEND_MESSAGES",
    guild_only,
    ephemeral
)]
pub async fn timeout(
    context: Context<'_>,
    #[description = "The user to timeout."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "Duration of the timeout."] duration: String,
    #[description = "Reason for the timeout."]
    #[max_length = 80]
    reason: Option<String>,
) -> Result<(), Error> {
    let database = &context.data().sqlite;

    let user = models::user(context, user_id).await?;

    let moderator = context.author();
    let moderator_id = moderator.id;

    if user.system {
        let reply = messages::error_reply("Cannot timeout a system user.", false);
        context.send(reply).await?;
        return Ok(());
    }

    if user_id == moderator_id {
        let reply = messages::error_reply("Sorry, but you cannot timeout yourself.", true);
        context.send(reply).await?;

        return Ok(());
    }

    let member = models::member(context, context.guild_id().unwrap(), user_id).await?;

    let ensure = ensure_user(&member, &user_id, &context.guild_id().unwrap(), database).await;

    if let Err(why) = ensure {
        let _ = messages::error_response(why.to_string(), true).await;
        return Ok(());
    }

    let reason = reason.unwrap_or_else(|| "No reason provided.".to_string());

    let reason_char_count = reason.chars().count();
    if reason_char_count > 80 {
        let reply = messages::info_reply("Reason must be no more than 80 characters long.", true);
        context.send(reply).await?;

        return Ok(());
    }

    let duration = match parse(&duration) {
        Ok(duration) => duration,
        Err(why) => {
            let reply = messages::error_reply(why.to_string(), true);
            context.send(reply).await?;
            return Ok(());
        }
    };

    let time = Timestamp::from(Utc::now() + duration);

    if time > Timestamp::from(Utc::now() + Days::new(28)) {
        let reply = messages::error_reply("Cannot timeout for longer than 28 days.", true);
        context.send(reply).await?;
        return Ok(());
    }

    if time < Timestamp::from(Utc::now() + Duration::from_secs(0)) {
        let reply = messages::error_reply("Cannot timeout for less than 0 seconds.", true);
        context.send(reply).await?;
        return Ok(());
    }

    let result = {
        let (user_name, user_mention) = (&user.name, models::user_mention(context, user_id).await?);

        let mut member = models::member(context, context.guild_id().unwrap(), user_id).await?;

        let moderator_name = &moderator.name;

        let (guild_id, guild_name) = {
            let guild_id = context.guild_id().unwrap();
            let guild = context.guild().unwrap();
            (guild_id, guild.name.clone())
        };

        let created_at = Utc::now().naive_utc();

        let mut user_mod_history = modlog::select_modlog_from_users(&user_id, database).await?;

        match member
            .disable_communication_until_datetime(context, time)
            .await
        {
            Ok(_) => {
                modlog::insert_modlog(
                    ModType::Timeout,
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

                info!("@{moderator_name} timed out @{user_name} from {guild_name}: {reason}");
                Ok(format!("{user_mention} has been timed out."))
            }
            Err(why) => {
                error!("Couldn't timeout @{user_name}: {why:?}");
                Err(format!("Sorry, but I couldn't timeout {user_mention}."))
            }
        }
    };

    if let Err(why) = result {
        let reply = messages::error_reply(&why, true);
        context.send(reply).await?;
    } else {
        let reply = messages::info_reply(result.unwrap(), true);

        context.send(reply).await?;
    }

    Ok(())
}

/// Un-times out a user.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "MODERATE_MEMBERS | SEND_MESSAGES",
    guild_only,
    ephemeral
)]
pub async fn untimeout(
    context: Context<'_>,
    #[description = "The user to untimeout."]
    #[rename = "user"]
    user_id: UserId,
) -> Result<(), Error> {
    let database = &context.data().sqlite;

    let user = models::user(context, user_id).await?;

    if user.system {
        let reply = messages::error_reply("Cannot untimeout a system user.", false);
        context.send(reply).await?;
        return Ok(());
    }

    let member = models::member(context, context.guild_id().unwrap(), user_id).await?;

    let ensure = ensure_user(&member, &user_id, &context.guild_id().unwrap(), database).await;

    if let Err(why) = ensure {
        let _ = messages::error_response(why.to_string(), true).await;
        return Ok(());
    }

    let result = {
        let (user_name, user_mention) = (&user.name, models::user_mention(context, user_id).await?);

        let mut member = models::member(context, context.guild_id().unwrap(), user_id).await?;

        let moderator_id = context.author().id;

        let (guild_id, guild_name) = {
            let guild_id = context.guild_id().unwrap();
            let guild = context.guild().unwrap();
            (guild_id, guild.name.clone())
        };

        let created_at = Utc::now().naive_utc();

        let mut user_mod_history = modlog::select_modlog_from_users(&user_id, database).await?;

        match member.enable_communication(context).await {
            Ok(_) => {
                modlog::insert_modlog(
                    ModType::Untimeout,
                    &guild_id,
                    &user_id,
                    &moderator_id,
                    "",
                    created_at,
                    database,
                )
                .await?;

                user_mod_history += 1;

                modlog::update_users_set_modlog(&user_id, user_mod_history, database).await?;

                info!("@{moderator_id} untimed out @{user_name} from {guild_name}");
                Ok(format!("{user_mention} has been untimed out."))
            }
            Err(why) => {
                error!("Couldn't untimeout @{user_name}: {why:?}");
                Err(format!("Sorry, but I couldn't untimeout {user_mention}."))
            }
        }
    };

    if let Err(why) = result {
        let reply = messages::error_reply(&why, true);
        context.send(reply).await?;
    } else {
        let reply = messages::info_reply(result.unwrap(), true);

        context.send(reply).await?;
    }

    Ok(())
}

/// Warns a user.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "MODERATE_MEMBERS | SEND_MESSAGES",
    guild_only,
    ephemeral
)]
pub async fn warn(
    context: Context<'_>,
    #[description = "The user to warn."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "The reason for the warning."] reason: String,
) -> Result<(), Error> {
    let database = &context.data().sqlite;

    let user = models::user(context, user_id).await?;

    let member = models::member(context, context.guild_id().unwrap(), user_id).await?;

    let ensure = ensure_user(&member, &user_id, &context.guild_id().unwrap(), database).await;

    if let Err(why) = ensure {
        let _ = messages::error_response(why.to_string(), true).await;
        return Ok(());
    }

    if user.system {
        let reply = messages::error_reply("Cannot warn a system user.", false);
        context.send(reply).await?;
        return Ok(());
    }

    let result = {
        let (user_name, user_mention) = (&user.name, models::user_mention(context, user_id).await?);

        let moderator_id = context.author().id;

        let (guild_id, guild_name) = {
            let guild_id = context.guild_id().unwrap();
            let guild = context.guild().unwrap();
            (guild_id, guild.name.clone())
        };

        let created_at = Utc::now().naive_utc();

        let mut user_mod_history = modlog::select_modlog_from_users(&user_id, database).await?;

        match modlog::insert_modlog(
            ModType::Warn,
            &guild_id,
            &user_id,
            &moderator_id,
            &reason,
            created_at,
            database,
        )
        .await
        {
            Ok(_) => {
                user_mod_history += 1;

                modlog::update_users_set_modlog(&user_id, user_mod_history, database).await?;

                info!("@{moderator_id} warned @{user_name} from {guild_name}");
                Ok(format!("{user_mention} has been warned."))
            }

            Err(why) => {
                error!("Couldn't warn @{user_name}: {why:?}");
                Err(format!("Sorry, but I couldn't warn {user_mention}."))
            }
        }
    };

    if let Err(why) = result {
        let reply = messages::error_reply(&why, true);
        context.send(reply).await?;
    } else {
        let reply = messages::info_reply(result.unwrap(), true);

        context.send(reply).await?;
    }

    Ok(())
}

/// Gets a user's warnings.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "MODERATE_MEMBERS | SEND_MESSAGES",
    guild_only
)]
pub async fn warnings(
    context: Context<'_>,
    #[description = "The user to get warnings for."]
    #[rename = "user"]
    user_id: UserId,
) -> Result<(), Error> {
    let database = &context.data().sqlite;

    let user = models::user(context, user_id).await?;

    let member = models::member(context, context.guild_id().unwrap(), user_id).await?;

    let ensure = ensure_user(&member, &user_id, &context.guild_id().unwrap(), database).await;

    if let Err(why) = ensure {
        let _ = messages::error_response(why.to_string(), true).await;
        return Ok(());
    }

    if user.system {
        let reply = messages::error_reply("Cannot get warnings for a system user.", false);
        context.send(reply).await?;
        return Ok(());
    }

    let result = {
        let (user_name, user_mention) = (&user.name, models::user_mention(context, user_id).await?);

        let guild_id = context.guild_id().unwrap();

        let user_mod_history =
            match modlog::select_modlog(ModType::Warn, &user_id, &guild_id, database).await {
                Ok(user_mod_history) => user_mod_history,
                Err(why) => {
                    error!("Couldn't select warnings from infractions: {why:?}");
                    return Err(why.into());
                }
            };

        let warning_count = user_mod_history.len();
        if warning_count < 1 {
            let reply =
                messages::info_reply(format!("{user_mention} doesn't have any warnings."), true);
            context.send(reply).await?;

            return Ok(());
        }

        let (uuids, moderator_ids, reasons, created_ats) = (
            user_mod_history
                .iter()
                .map(|(uuid, _, _, _, _, _, _)| uuid)
                .collect::<Vec<&String>>(),
            user_mod_history
                .iter()
                .map(|(_, _, _, moderator_id, _, _, _)| moderator_id)
                .collect::<Vec<&i64>>(),
            user_mod_history
                .iter()
                .map(|(_, _, _, _, reason, _, _)| reason)
                .collect::<Vec<&String>>(),
            user_mod_history
                .iter()
                .map(|(_, _, _, _, _, created_at, _)| created_at)
                .collect::<Vec<&NaiveDateTime>>(),
        );

        // TODO: Add pagination
        let uuids_iter = uuids.chunks(25);
        let mod_ids_iter = moderator_ids.chunks(25);
        let reasons_iter = reasons.chunks(25);
        let created_ats_iter = created_ats.chunks(25);

        // Cycle through the chunks of 25, creating pagination embeds
        let mut embeds = Vec::new();
        uuids_iter
            .zip(mod_ids_iter.zip(reasons_iter.zip(created_ats_iter)))
            .for_each(|(uuids, (moderator_ids, (reasons, created_ats)))| {
                embeds.push(warnings_command_embed(
                    &user,
                    uuids,
                    moderator_ids,
                    reasons,
                    created_ats,
                ));
            });

        match utilities::paginate::paginate(context, embeds).await {
            Ok(_) => {
                let author = context.author().id;
                info!("@{author} requested @{user_name}'s warnings");
                Ok(format!("{user_mention} has {warning_count} warning(s)."))
            }
            Err(why) => {
                error!("Failed to paginate: {why:?}");
                Err(why.to_string())
            }
        }
    };

    if let Err(why) = result {
        let reply = messages::error_reply(&why, true);
        context.send(reply).await?;
    } else {
        let reply = messages::info_reply(result.unwrap(), true);

        context.send(reply).await?;
    }

    Ok(())
}
