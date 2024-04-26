use dashmap::DashMap;
use poise::serenity_prelude as serenity;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::Arc;
use sqlx::SqlitePool;
use crate::types::{GuildSettings, User};

#[derive(Debug)]
pub struct Data {
    pub reqwest: reqwest::Client,
    pub sqlite: SqlitePool,
    pub guild_data: DashMap<u64, GuildSettings>,
    pub users: DashMap<u64, User>,
    pub commands_ran: DashMap<u64, AtomicU64>,
    pub commands_ran_users: DashMap<u64, AtomicU64>,
    pub songs_played: DashMap<u64, AtomicU64>,
    pub shard_manager: Arc<serenity::ShardManager>,
    pub is_loop_running: AtomicBool,
} // User data, which is stored and accessible in all command invocations