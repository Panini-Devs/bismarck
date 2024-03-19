use ::serenity::all::Member;
use chrono::NaiveDateTime;
use poise::serenity_prelude as serenity;
use serenity::all::{GuildId, UserId};
use sqlx::{Row, SqlitePool};
use tokio::time::Instant;
use tracing::{debug, error, info};
use uuid::Uuid;

pub enum ModType {
    Warn,
    Timeout,
    Untimeout,
    Kick,
    Ban,
    Unban,
}

impl ModType {
    pub fn as_str(&self) -> &str {
        match self {
            ModType::Warn => "warn",
            ModType::Timeout => "timeout",
            ModType::Untimeout => "untimeout",
            ModType::Kick => "kick",
            ModType::Ban => "ban",
            ModType::Unban => "unban",
        }
    }
}

pub async fn select_modlog(
    modtype: ModType,
    user_id: &UserId,
    guild_id: &GuildId,
    pool: &SqlitePool,
) -> Result<Vec<(String, i64, i64, i64, String, NaiveDateTime, String)>, sqlx::Error> {
    let start_time = Instant::now();

    let rows = sqlx::query(
        "SELECT uuid, action_type, user_id, moderator_id, reason, time_created, guild_id FROM guild_log WHERE user_id = ? AND guild_id = ? AND action_type = ?"
    )
        .bind(i64::from(*user_id))
        .bind(i64::from(*guild_id))
        .bind(modtype.as_str())
        .fetch_all(pool).await?;

    let mut logs = Vec::new();

    for row in rows {
        if row.is_empty() {
            return Err(sqlx::Error::RowNotFound);
        }

        let (uuid, guild_id, user_id, moderator_id, action_type, created_at, reason) = (
            row.get::<String, _>(0),
            row.get::<i64, _>(1),
            row.get::<i64, _>(2),
            row.get::<i64, _>(3),
            row.get::<String, _>(4),
            row.get::<NaiveDateTime, _>(5),
            row.get::<String, _>(6),
        );

        logs.push((
            uuid,
            guild_id,
            user_id,
            moderator_id,
            action_type,
            created_at,
            reason,
        ));
    }

    let elapsed_time = start_time.elapsed();
    info!("Selected from Moderation Logs in {elapsed_time:.2?}");

    Ok(logs)
}

pub async fn delete_mod_log(
    uuid: String,
    guild_id: &GuildId,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query("DELETE FROM guild_log WHERE uuid = ? AND guild_id = ?")
        .bind(uuid)
        .bind(i64::from(*guild_id));

    if let Err(why) = query.execute(pool).await {
        error!("Failed to execute query: {:?}", why);
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    info!("Deleted from Moderation Logs in {elapsed_time:.2?}");

    Ok(())
}

pub async fn insert_modlog(
    action_type: ModType,
    guild_id: &GuildId,
    user_id: &UserId,
    moderator_id: &UserId,
    reason: &str,
    created_at: NaiveDateTime,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let uuid = Uuid::new_v4().to_string();

    let query = sqlx::query(
        "INSERT INTO guild_log (uuid, action_type, user_id, moderator_id, reason, time_created, guild_id) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
        .bind(uuid)
        .bind(action_type.as_str())
        .bind(i64::from(*user_id))
        .bind(i64::from(*moderator_id))
        .bind(reason)
        .bind(created_at)
        .bind(i64::from(*guild_id));

    if let Err(why) = query.execute(pool).await {
        error!("Failed to execute query: {:?}", why);
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();

    info!("Inserted into Moderation Logs in {elapsed_time:.2?}");

    Ok(())
}

pub async fn ensure_user(
    member: &Member,
    user_id: &UserId,
    guild_id: &GuildId,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let join = member.joined_at.unwrap().to_rfc2822();

    let query = sqlx::query(
        "INSERT OR IGNORE INTO user_guild (user_id, guild_id, infractions, join_date) VALUES (?, ?, ?, ?)")
        .bind(i64::from(*user_id))
        .bind(i64::from(*guild_id))
        .bind(0)
        .bind(join);

    if let Err(why) = query.execute(pool).await {
        error!("Failed to execute query: {:?}", why);
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();

    info!("Ensured user in Users in {elapsed_time:.2?}");

    Ok(())
}

pub async fn select_modlog_from_users(
    user_id: &UserId,
    pool: &SqlitePool,
) -> Result<i32, sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query("SELECT infractions FROM user_guild WHERE user_id = ?")
        .bind(i64::from(*user_id));
    let row = match query.fetch_one(pool).await {
        Ok(infractions) => infractions,
        Err(why) => {
            error!("Couldn't select infractions from Users: {why:?}");
            return Err(why);
        }
    };

    let infractions = match row.try_get::<i32, _>("infractions") {
        Ok(infractions) => infractions,
        Err(why) => {
            error!("Couldn't get infractions: {why:?}");
            return Err(why);
        }
    };

    let elapsed_time = start_time.elapsed();
    debug!("Selected infractions from Users in {elapsed_time:.2?}");

    Ok(infractions)
}

pub async fn update_users_set_modlog(
    user_id: &UserId,
    infractions: i32,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query("UPDATE user_guild SET infractions = ? WHERE user_id = ?")
        .bind(infractions)
        .bind(i64::from(*user_id));
    if let Err(why) = query.execute(pool).await {
        error!("Couldn't update infractions for user(s) in Users: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Updated infractions for user(s) within Users in {elapsed_time:.2?}");

    Ok(())
}
