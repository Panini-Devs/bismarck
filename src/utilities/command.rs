use std::sync::atomic::{AtomicU64, Ordering};

use sqlx::sqlite::SqliteQueryResult;
use tracing::error;

use crate::{Context, Error, PartialContext};

pub async fn get_prefix(context: PartialContext<'_>) -> Result<Option<String>, Error> {
    if let Some(guild_id) = context.guild_id {
        let pf = &context.data.guild_data;

        let guild_settings = pf.get(&guild_id.get());
        match guild_settings {
            // we now return the result instead of throwing it away with `let _`
            Some(guild_settings) => Ok(Some(guild_settings.prefix.clone())),
            None => {
                // if no guild settings found,
                // create new database entry and return default prefix
                let (guild_id, owner_id) = {
                    let guild = guild_id
                        .to_guild_cached(&context.serenity_context.cache)
                        .unwrap();
                    (i64::from(guild.id), i64::from(guild.owner_id))
                };

                let database = &context.data.sqlite;

                // create new guild settings into sqlite database as a failsafe
                // in case guild_join did not load properly
                let query_result: Result<SqliteQueryResult, sqlx::Error> = sqlx::query!(
                    "INSERT INTO guild (
                        id,
                        prefix,
                        owner
                    ) VALUES (?, ?, ?)",
                    guild_id,
                    "+",
                    owner_id
                )
                .execute(database)
                .await;

                // this one ended up a bit weird
                // we have to convert the sqlx::Error to our type alias Error
                // if query_result is Err
                // otherwise we just return Ok(Some("+".to_string))
                // the inner query result is unused, but can be used in the closure if desired
                match query_result {
                    Ok(_query) => Ok(Some("+".to_string())),
                    Err(sqlx_error) => Err(Error::from(sqlx_error)),
                }

                // the below code does the same as the above a bit more idomatically
                // go with whichever seems more readable to you

                // query_result.map_or_else(
                //     |sqlx_err| Err(Error::from(sqlx_err)),
                //     |_query_result| Ok(Some("+".to_string())),
                // )
            }
        }
    } else {
        // previously, without the else block, we were throwing away
        // everything we did in the `if let` and always just returning Ok(Some("+".to_string()))
        Ok(Some("+".to_string()))
    }
}

pub async fn pre_command(context: Context<'_>) -> () {
    if let Some(guild_id) = context.guild_id() {
        let commands_ran = context.data().commands_ran.get(&guild_id.get()).unwrap();
        commands_ran.fetch_add(1, Ordering::Relaxed);

        let id = guild_id.get() as i64;

        if let Err(query) = sqlx::query!(
            "UPDATE guild SET commands_ran = commands_ran + 1 WHERE id = ?",
            id
        )
        .execute(&context.data().sqlite)
        .await
        {
            error!("Failed to update guild commands ran: {}", query);
        }
    }

    let commands_ran_global = context.data().commands_ran.get(&0).unwrap();
    commands_ran_global.fetch_add(1, Ordering::Relaxed);

    if let Err(query) =
        sqlx::query!("UPDATE guild SET commands_ran = commands_ran + 1 WHERE id = 0")
            .execute(&context.data().sqlite)
            .await
    {
        error!("Failed to update global commands ran: {}", query);
    }

    let author_id = u64::from(context.author().id);
    if let Some(commands_ran_user) = context.data().commands_ran_users.get(&author_id) {
        commands_ran_user.fetch_add(1, Ordering::Relaxed);

        let author_id = i64::from(context.author().id);

        if let Err(query) = sqlx::query!(
            "UPDATE user SET commands_run = commands_run + 1 WHERE id = ?",
            author_id
        )
        .execute(&context.data().sqlite)
        .await
        {
            error!("Failed to update user commands ran: {}", query);
        }

        return;
    }

    context
        .data()
        .commands_ran_users
        .insert(author_id, AtomicU64::new(1));

    let author_id = i64::from(context.author().id);
    if let Err(query) = sqlx::query!(
        "INSERT OR IGNORE INTO user (
            id
        ) VALUES (?)",
        author_id
    )
    .execute(&context.data().sqlite)
    .await
    {
        error!("Failed to insert user: {}", query);
    }
}
